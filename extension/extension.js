import { Extension } from 'resource:///org/gnome/shell/extensions/extension.js';
import * as Main from 'resource:///org/gnome/shell/ui/main.js';
import { Button } from 'resource:///org/gnome/shell/ui/panelMenu.js';
import GObject from 'gi://GObject';
import GLib from 'gi://GLib';
import Gio from 'gi://Gio';
import St from 'gi://St';

const APP_ID = 'de.seblebs.ff-profiles';
const APP_TITLE = 'Firefox Profiles';

const FfProfilesIndicator = GObject.registerClass(
class FfProfilesIndicator extends Button {
    _init(binary) {
        super._init(0.0, 'Firefox Profiles', true);

        this._binary = binary;
        this._windowCreatedId = 0;
        this._cleanupTimeoutId = 0;

        this.add_child(new St.Icon({
            icon_name: 'firefox-symbolic',
            style_class: 'system-status-icon',
        }));

        this.connect('button-press-event', (_actor, event) => {
            if (event.get_button() !== 1) return false;
            this._launch();
            return true;
        });
    }

    _isOurWindow(metaWindow) {
        const cls = metaWindow.get_wm_class() ?? '';
        const title = metaWindow.get_title() ?? '';
        return cls === APP_ID || cls.toLowerCase() === 'ff-profiles' || title === APP_TITLE;
    }

    _findOurWindow() {
        for (const actor of global.get_window_actors()) {
            if (this._isOurWindow(actor.meta_window))
                return actor.meta_window;
        }
        return null;
    }

    _positionNearIndicator(metaWindow) {
        const [bx] = this.get_transformed_position();
        const monitorIdx = global.display.get_monitor_at_point(bx, 0);
        const workArea = global.display.get_monitor_workarea(monitorIdx);
        const frame = metaWindow.get_frame_rect();
        // frame.width is 0 before first commit; fall back to default_width in ui.rs (keep in sync)
        const windowWidth = frame.width > 0 ? frame.width : 360;

        // Center horizontally on the indicator, clamped to the monitor's work area
        let targetX = Math.round(bx + this.width / 2 - windowWidth / 2);
        targetX = Math.max(workArea.x, Math.min(targetX, workArea.x + workArea.width - windowWidth));

        metaWindow.move_frame(true, targetX, workArea.y);
    }

    _disconnectWindowCreated() {
        if (this._windowCreatedId) {
            global.display.disconnect(this._windowCreatedId);
            this._windowCreatedId = 0;
        }
    }

    _cancelCleanupTimeout() {
        if (this._cleanupTimeoutId) {
            GLib.source_remove(this._cleanupTimeoutId);
            this._cleanupTimeoutId = 0;
        }
    }

    _launch() {
        // If the window is already open, reposition it and bring it forward — do not
        // spawn a second binary; that would open a duplicate even if GTK single-instance
        // catches it, because the D-Bus guard can silently fail under some environments.
        const existing = this._findOurWindow();
        if (existing) {
            this._positionNearIndicator(existing);
            Main.activateWindow(existing);
            return;
        }

        // Fail fast with a helpful message when the binary is not installed
        if (!Gio.File.new_for_path(this._binary).query_exists(null)) {
            Main.notify('Firefox Profiles',
                `Binary not found: ${this._binary}. Run "make install" to install ff-profiles.`);
            return;
        }

        // Connect before spawning so no window-created event can slip through the gap
        this._disconnectWindowCreated();
        this._windowCreatedId = global.display.connect('window-created', (_d, metaWindow) => {
            if (!this._isOurWindow(metaWindow)) return;
            this._disconnectWindowCreated();
            this._cancelCleanupTimeout();
            // Call synchronously before meta_window_show() runs. This sets user_has_move=true
            // on the MetaWindow, which causes Mutter's placement algorithm to skip
            // auto-centering and use our coordinates for the very first paint — no flicker.
            this._positionNearIndicator(metaWindow);
        });

        // Store the source ID so it can be cancelled if the extension is disabled mid-flight
        this._cancelCleanupTimeout();
        this._cleanupTimeoutId = GLib.timeout_add(GLib.PRIORITY_DEFAULT, 5000, () => {
            this._cleanupTimeoutId = 0;
            this._disconnectWindowCreated();
            return GLib.SOURCE_REMOVE;
        });

        try {
            Gio.Subprocess.new([this._binary], Gio.SubprocessFlags.NONE);
        } catch (e) {
            Main.notify('Firefox Profiles', `Failed to launch ff-profiles: ${e.message}`);
            this._disconnectWindowCreated();
            this._cancelCleanupTimeout();
        }
    }

    destroy() {
        this._disconnectWindowCreated();
        this._cancelCleanupTimeout();
        super.destroy();
    }
});

export default class FfProfilesExtension extends Extension {
    enable() {
        // Prefer PATH lookup; fall back to the local-install location used by `make install`.
        const binary =
            GLib.find_program_in_path('ff-profiles') ??
            `${GLib.get_home_dir()}/.local/bin/ff-profiles`;

        this._indicator = new FfProfilesIndicator(binary);
        Main.panel.addToStatusArea(this.uuid, this._indicator);
    }

    disable() {
        this._indicator?.destroy();
        this._indicator = null;
    }
}
