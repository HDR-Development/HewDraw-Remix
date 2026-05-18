var AButtonHeld = false;
var buttonCount = 0;
var rows = 0;
var currPos = 0;
var currRow = 0;
var last = 0;

window.addEventListener("DOMContentLoaded", (e) => {
  var buttons = document.querySelectorAll("button");
  buttonCount = buttons.length;
  last = buttonCount - 1;
  rows = (buttonCount - 2) % 3;
  [].forEach.call(buttons, function (btn) {
    btn.addEventListener("focus", () => {
      btn.classList.add("is-focused");
    });

    btn.addEventListener("focusout", () => {
      btn.classList.remove("is-focused");
    });
  });

  // Listen to the keydown event and prevent the default
  window.addEventListener("keydown", function (e) {
    if (e.keyCode == UP) {
      var target = document.querySelector(".is-focused").previousElementSibling;
      if (target != undefined) {
        getCurrentActiveContainer()[0].scrollTop = target.offsetTop + 50;
        target.focus();
        currPos--;
        currRow--;
      }
    } else if (e.keyCode == DOWN) {
      var target = document.querySelector(".is-focused").nextElementSibling;
      if (currRow == rows) {
        target = buttons[last];
      }
      if (target != undefined) {
        getCurrentActiveContainer()[0].scrollTop = target.offsetTop - 50;
        target.focus();
        currPos++;
        currRow++;
      }
    } else if (e.keyCode == RIGHT) {
      var target =
        document.querySelector(".is-focused").nextElementSibling
          .nextElementSibling.nextElementSibling;
      if (target != undefined) {
        getCurrentActiveContainer()[0].scrollTop = target.offsetTop - 50;
        target.focus();
        currPos += 3;
      }
    } else if (e.keyCode == LEFT) {
      var target =
        document.querySelector(".is-focused").previousElementSibling
          .previousElementSibling.previousElementSibling;
      if (target != undefined) {
        getCurrentActiveContainer()[0].scrollTop = target.offsetTop - 50;
        target.focus();
        currPos -= 3;
      }
    }
  });

  window.nx.footer.setAssign("A", "", () => {
    $(".is-focused").last().click();
  });
  window.nx.footer.setAssign("B", "", () => {
    window.location.href = "http://localhost/";
  });
  window.nx.footer.setAssign("X", "", () => {});
  window.nx.footer.setAssign("Y", "", () => {});

  if ($(".is-focused").length <= 0) {
    $("#list").find("button").get(0).focus();
  }
});
