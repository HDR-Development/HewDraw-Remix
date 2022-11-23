var enabled_modes = new Map();
enabled_modes.set('tag', false);
enabled_modes.set('turbo', false);
enabled_modes.set('hitfall', false);

function toggle_mode(mode_name) {
    if (event && event.keyCode !== 13) return
    alert("toggling: " + mode_name);
    var is_now_enabled = !enabled_modes.get(mode_name.toLowerCase());
    enabled_modes.set(mode_name.toLowerCase(), is_now_enabled);
    var button_text = mode_name + " Mode (" + (is_now_enabled ? "ON" : "OFF") + ")";
    document.getElementById(mode_name.toLowerCase() + "-text-field").innerHTML = button_text;
}

function exit_with_id() {
    if (event && event.keyCode !== 13) return
    var str = "";
    enabled_modes.forEach((v, k) => {
        if (v === true) {
            str += k + '-';
        }
    });
    if (str.endsWith('-')) {
        str = str.substring(0, str.length - 1);
    }
    if (str.length == 0) {
        str = "none";
    }
    alert("exiting with str: " + str);
    location.href = "http://localhost/" + str;
}