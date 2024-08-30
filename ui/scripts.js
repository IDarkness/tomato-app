const listen = window.__TAURI__.event.listen;
const emit = window.__TAURI__.event.listen;
const invoke = window.__TAURI__.invoke;
const currWindow = window.__TAURI__.window;

listen('turn-timer-red-event', () => {
  document.getElementById('timer').style.color = "red";
});

listen('turn-timer-black-event', () => {
  document.getElementById('timer').style.color = "black";
});

listen('run-timer-event', (event) => {
  document.getElementById('timer').innerHTML = event.payload.data;
});

listen('timer-end-event', (event) => {
});

function runOnFocus() {
  document.getElementById('timer').style.color = "black";
}

function showElement(element_name) {
  document.getElementById(element_name).style.display = "inline";
}

function hideElement(element_name) {
  document.getElementById(element_name).style.display = "none";
}

function startTimer(startTime) {
  document.getElementById('timer').innerHTML = startTime;
  invoke('run_timer', { startTime }).then(
    (result) => {
      //if it works, do nothing
    },
    (error) => {
      console.log("Error occred while engaging the run_timer() rust function: " + error);
    });
}

function stopTimer() {
  console.log("stopTimer triggert");
}

window.showElement = showElement;
window.hideElement = hideElement;
window.runOnFocus = runOnFocus;
window.startTimer = startTimer;
window.stopTimer = stopTimer;

