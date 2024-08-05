const listen = window.__TAURI__.event.listen;
const emit = window.__TAURI__.event.listen;
const invoke = window.__TAURI__.invoke;
const currWindow = window.__TAURI__.window;

listen('run-timer-event', (event) => {
  document.getElementById('timer').innerHTML = event.payload.data;
});

listen('timer-end-event', (event) => {
  document.getElementById('timer').style.color = "red";
});

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

window.startTimer = startTimer;
window.stopTimer = stopTimer;

