# Project to demo Design and Development of a Pomodoro Application

## Techstack
Frameworks:
Tauri <br>
htmx <br>
TailwindCSS <br>

Languages:
Rust <br>
JavaScript <br>

Frontend: 
UI design based on shadcn
added components: <rootdir>/ui/components/*

## Build

To build the whole application run:
''' 
cargo tauri dev
'''

To build tailwind/ css files:
'''
npx tailwindcss -i ./ui/input.css -o ./ui/output.css --watch
'''
