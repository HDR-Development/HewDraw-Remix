function exit_with_id(e) {
    if (event && event.keyCode !== 13) return
    location.href = "http://localhost/" + e.id;
}