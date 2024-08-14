use enigo::{Enigo, MouseControllable};
use rdev::{listen, Event, EventType, Key};
use std::{io, sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc}, thread, time};

fn main() {
    print_menu();

    let is_running = Arc::new(AtomicBool::new(false));
    let delay = Arc::new(AtomicUsize::new(1000));

    let running_flag = Arc::clone(&is_running);
    let delay_flag = Arc::clone(&delay);

    let clicker_thread = thread::spawn(move || {
        let mut enigo = Enigo::new();

        loop {
            if running_flag.load(Ordering::SeqCst) {
                enigo.mouse_click(enigo::MouseButton::Left);
            }
            thread::sleep(time::Duration::from_millis(delay_flag.load(Ordering::SeqCst) as u64));
        }
    });

    if let Err(error) = listen(move |event| callback(event, Arc::clone(&is_running), Arc::clone(&delay))) {
        println!("Error: {:?}", error);
    }

    clicker_thread.join().unwrap();
}

fn print_menu() {
    println!("===============================");
    println!("         Rust Clicker           ");
    println!("===============================");
    println!("Commands:");
    println!("  [s]  Start/Stop Clicking");
    println!("  [d]  Set Delay Between Clicks");
    println!("  [q]  Quit");
    println!("===============================");
}

fn callback(event: Event, is_running: Arc<AtomicBool>, delay: Arc<AtomicUsize>) {
    match event.event_type {
        EventType::KeyPress(key) => {

            if key == Key::KeyS {
                let current_state = is_running.load(Ordering::SeqCst);
                is_running.store(!current_state, Ordering::SeqCst);
                println!("Auto clicker is now {}", if !current_state { "running" } else { "stopped" });
            }

            if key == Key::KeyD {
                println!("Enter new delay in milliseconds:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                if let Ok(ms) = input.trim().parse::<usize>() {
                    delay.store(ms, Ordering::SeqCst);
                    println!("Delay set to {} ms", ms);
                } else {
                    println!("Invalid input. Please enter a valid number.");
                }
            }

            if key == Key::KeyQ {
                std::process::exit(0);
            }
        }
        _ => (),
    }
}
