var modes_enabled = new Map();

window.onerror = function (msg, url, lineNo, columnNo, error) {
    alert("Error Message: " + msg + "\nURL: " + url + " : " + lineNo + "," + columnNo + "\nError Object: " + error);
    return false;
}

function do_alert(...data) {
    alert(data);
}

console.error = do_alert;
console.warn = do_alert;
console.trace = do_alert;

function toggle_mode(mode_name) {
    alert("toggling mode: " + mode_name);
    // toggle the mode
    if (modes_enabled.has(mode_name)) {
        modes_enabled.set(mode_name, !modes_enabled.get(mode_name));
    } else {
        modes_enabled.set(mode_name, true);
    }

    var is_enabled = modes_enabled.get(mode_name);
    var id = mode_name + "-text";
    var element = document.getElementById(id);
    var on_off = is_enabled ? "ON" : "OFF";
    var button_text = mode_name.charAt(0).toUpperCase() + mode_name.slice(1) + " Mode(yay): " + on_off;
    element.innerHTML = button_text;
}

function exit_with_modes(e) {
    alert("exit called");
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