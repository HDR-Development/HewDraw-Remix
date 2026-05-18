const maxModes = 3;
var selectedOverhaul = '';
/**
 * @type {string[]}
 */
const enabledModes = [];

/**
 * @param {string} mode_name
 * @param {{ blur: () => void; focus: () => void; }} self
 * @param {string} elementID
 */
function toggle(mode_name, self, elementID) {
  // Enforce maximum number of enabled modes
  if (enabledModes.length >= maxModes && !enabledModes.includes(mode_name)) {
    textFocus('Too many modes selected!\nDisable another mode to enable this one.'); // Update focus text
    return;
  }

  // Toggle mode
  if (enabledModes.includes(mode_name)) {
    enabledModes.splice(enabledModes.indexOf(mode_name), 1);
  } else {
    enabledModes.push(mode_name);
  }
  const x = document.getElementById(elementID);
  if (x.style.display === "none") {
    x.style.display = "block";
  } else {
    x.style.display = "none";
  }
  self.blur();
  self.focus();
}

/**
 * @param {string} text
 */
function textFocus(text) {
  document.getElementById("focus-text").innerHTML = text;
}

/**
 * @param {string} imageSrc
 */
function updatePreview(imageSrc) {
  const preview = document.getElementById("gamemode-preview");
  if (preview) {
    preview.src = imageSrc;
  }
}

var currentModeIndex = 0;
const overhauls = [
  { id: '', name: 'Select Game Overhaul', desc: 'Click to toggle various alternate system mechanics!', image: 'placeholder.png' },
  { id: 'smash64', name: 'Smash 64 Mode', desc: 'Clash in classic fashion! Removes DI, airdodges, walltechs, and landing lag. Also raises hitstun and shieldstun, and alters character physics.', image: 'placeholder.png' },
  { id: 'rivalsofaether', name: 'Rivals of Aether Mode', desc: 'Removes shields, grabs, ledges, & spotdodges, and walljump from specials, hitfalling, and improved movement!', image: 'placeholder.png' }
];

/**
 * @param {string} elementID
 */
function cycleOverhaul(elementID) {
  // Cycle to next mode
  currentModeIndex = (currentModeIndex + 1) % overhauls.length;
  const currOverhaul = overhauls[currentModeIndex];

  // Update button text
  const button = document.getElementById('gamemode-cycle-button');
  if (button) {
    button.querySelector('h2').textContent = currOverhaul.name;
  }

  textFocus(currOverhaul.desc); // Update focus text
  updatePreview(currOverhaul.image); // Update preview image

  // Remove all game modes from enabled array
  overhauls.forEach(mode => {
    // Hide checkmark
    if (mode.id) {
      const checkmark = document.getElementById(mode.id);
      if (checkmark) {
        checkmark.style.display = "none";
      }
    }
  });

  // If a valid mode was selected, enable it
  const x = document.getElementById(elementID);
  if (currOverhaul.id && currOverhaul.id !== "") {
    selectedOverhaul = currOverhaul.id;
    x.style.display = "block"; // Show checkmark
  } else {
    selectedOverhaul = '';
    x.style.display = "none";
  }
}

var startColumnIndex = 0;
const columnsPerPage = 2;

function updateColumnVisibility() {
  // Get all columns (excluding column-left)
  const allColumns = [];
  var i = 1;
  while (document.getElementById('column-' + i)) {
    allColumns.push(document.getElementById('column-' + i));
    i++;
  }

  // Calculate which columns should be visible
  const endIndex = startColumnIndex + columnsPerPage;

  // Hide all columns, then show only the current page
  allColumns.forEach(function(col, index) {
    if (index >= startColumnIndex && index < endIndex) {
      col.style.display = 'block';
    } else {
      col.style.display = 'none';
    }
  });
}

function nextPage() {
  const allColumns = [];
  var i = 1;
  while (document.getElementById('column-' + i)) {
    allColumns.push(document.getElementById('column-' + i));
    i++;
  }

  const maxStartIndex = allColumns.length - columnsPerPage;
  startColumnIndex++;
  if (startColumnIndex > maxStartIndex) {
    startColumnIndex = 0;
  }
  updateColumnVisibility();
}

function previousPage() {
  const allColumns = [];
  var i = 1;
  while (document.getElementById('column-' + i)) {
    allColumns.push(document.getElementById('column-' + i));
    i++;
  }

  startColumnIndex--;
  if (startColumnIndex < 0) {
    startColumnIndex = allColumns.length - columnsPerPage;
  }
  updateColumnVisibility();
}

function saveAndExit() {
  if (enabledModes === undefined || enabledModes.length == 0) {
    location.href = "http://localhost/";
  }

  var str = "";
  if (!!selectedOverhaul && selectedOverhaul !== '') {
    var allModes = [selectedOverhaul].concat(enabledModes);
    str = allModes.join("-");
  } else {
    str = enabledModes.join("-");
  }

  location.href = "http://localhost/" + str;
}
