var modes_enabled = new Map();

function set_mode(mode_name, is_enabled) {
    modes_enabled.set(mode_name, is_enabled);

    var id = mode_name + "-text";
    var element = document.getElementById(id);
    var on_off = is_enabled ? "ON" : "OFF";
    var button_text = mode_name.charAt(0).toUpperCase() + mode_name.slice(1) + " Mode(yay): " + on_off;
    element.innerHTML = button_text;
}

function exit_with_modes() {
    if (event && event.keyCode !== 13) return
    var enabled_str = "";
    for (const [key, value] of modes_enabled) {
        if (value === true) {
            enabled_str += key + "-";
        }
    }
    if (enabled_str.endsWith('-')) {
        enabled_str = enabled_str.substring(0, enabled_str.length - 1);
    }
    if (enabled_str.length = 0) {
        enabled_str = "none";
    }
    location.href = "http://localhost/" + enabled_str;
}