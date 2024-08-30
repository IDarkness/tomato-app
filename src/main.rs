use rodio::Source;
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use tauri::{Manager, State, Window};

#[derive(Clone, serde::Serialize)]
struct Payload {
    data: String,
}

#[tauri::command]
fn run_timer(
    state: State<'_, Mutex<AppState>>,
    window: Window,
    start_time: String,
    handle: tauri::AppHandle,
) {
    let mut time_in_seconds: u32 = start_time
        .parse::<u32>()
        .expect("error with parsing String to u32");
    let curr_window: Window = window.clone();
    std::thread::spawn(move || {
        while time_in_seconds > 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));
            time_in_seconds = time_in_seconds - 1;
            curr_window
                .emit(
                    "run-timer-event",
                    Payload {
                        data: time_in_seconds.to_string().into(),
                    },
                )
                .unwrap();
        }
    });
    //set timer up ture
    window.emit("timer-end-event", "").unwrap();
    blink_timer(&state, &window);
    play_alarm_sound(handle);
}

fn play_alarm_sound(handle: tauri::AppHandle) {
    let sound_file_path = handle
        .path_resolver()
        .resolve_resource("resources/retro-game-alarm.wav")
        .expect("failed to resolve resource");

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let file = File::open(&sound_file_path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let _ = stream_handle.play_raw(source.convert_samples());
}

fn blink_timer(state: &State<'_, Mutex<AppState>>, window: &Window) {
    let is_timer_up: bool = state.lock().unwrap().is_timer_up;
    let curr_window: Window = window.clone();
    std::thread::spawn(move || {
        while is_timer_up == true {
            println!("here");
            std::thread::sleep(std::time::Duration::from_secs(1));
            curr_window
                .emit("turn-timer-red-event", Payload { data: "OK".into() })
                .unwrap();
        }
    });
}

#[derive(Default)]
struct AppState {
    is_timer_up: bool,
}
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![run_timer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
