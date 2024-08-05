use tauri::Window;

#[derive(Clone, serde::Serialize)]
struct Payload {
    data: String,
}

#[tauri::command]
fn run_timer(window: Window, start_time: String) {
    let mut time_in_seconds: u32 = start_time
        .parse::<u32>()
        .expect("error with parsing String to u32");
    std::thread::spawn(move || {
        while time_in_seconds > 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));
            time_in_seconds = time_in_seconds - 1;
            window
                .emit(
                    "run-timer-event",
                    Payload {
                        data: time_in_seconds.to_string().into(),
                    },
                )
                .unwrap();
        }
        window.emit("timer-end-event", "").unwrap();
    });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_timer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
