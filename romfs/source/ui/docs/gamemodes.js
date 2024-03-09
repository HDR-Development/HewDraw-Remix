var enabled_modes = new Map();
enabled_modes.set('tag', false);
enabled_modes.set('turbo', false);
enabled_modes.set('hitfall', false);
enabled_modes.set('airdash', false);
enabled_modes.set('smash64', false);

function toggle_mode(mode_name, self) {
    if (event && event.keyCode !== 13) return
    //alert("toggling: " + mode_name);
    var is_now_enabled = !enabled_modes.get(mode_name.replace(/\s+/g, '').toLowerCase());
    enabled_modes.set(mode_name.replace(/\s+/g, '').toLowerCase(), is_now_enabled);
    var button_text = mode_name + " Mode (" + (is_now_enabled ? "ON" : "OFF") + ")";
    var element = document.getElementById(mode_name.replace(/\s+/g, '').toLowerCase() + "-text-field");
    element.innerHTML = button_text;
    self.blur();
    self.focus();
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
    //alert("exiting with str: " + str);
    location.href = "http://localhost/" + str;
}