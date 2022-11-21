const modes_enabled = new Map();

/**
 * 
 * @param {*} mode_name the mode name
 * @param {*} is_enabled whether the mode is enabled
 */
function set_mode(mode_name, is_enabled) {
    modes_enabled.set(mode_name, is_enabled);

    var id = mode_name + "-text";
    var element = document.getElementById(id);
    var on_off = is_enabled ? "ON" : "OFF";
    var button_text = mode_name.charAt(0).toUpperCase() + mode_name.slice(1) + " Mode: " + on_off;
    element.innerHTML = button_text;
}

function exit_with_modes() {
    if (event && event.keyCode !== 13) return
    let enabled_str = "";
    for (const [key, value] of modes_enabled) {
        if (value === true) {
            enabled_str += key + "-";
        }
      }
    location.href = "http://localhost/modes/" + enabled_str;
}