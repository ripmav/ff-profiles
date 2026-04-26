import { Extension } from 'resource:///org/gnome/shell/extensions/extension.js';
import * as Main from 'resource:///org/gnome/shell/ui/main.js';
import { Button } from 'resource:///org/gnome/shell/ui/panelMenu.js';
import GObject from 'gi://GObject';
import GLib from 'gi://GLib';
import Gio from 'gi://Gio';
import St from 'gi://St';

const FfProfilesIndicator = GObject.registerClass(
class FfProfilesIndicator extends Button {
    _init(binary) {
        super._init(0.0, 'Firefox Profiles', true); // true = no built-in popup menu

        this._binary = binary;

        this.add_child(new St.Icon({
            icon_name: 'firefox-symbolic',
            style_class: 'system-status-icon',
        }));

        this.connect('button-press-event', () => {
            this._launch();
            return false;
        });
    }

    _launch() {
        try {
            Gio.Subprocess.new([this._binary], Gio.SubprocessFlags.NONE);
        } catch (e) {
            Main.notify('Firefox Profiles', `Failed to launch ff-profiles: ${e.message}`);
        }
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
