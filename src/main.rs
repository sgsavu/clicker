use enigo::{Enigo, MouseControllable};
use rdev::{listen, Event, EventType, Key};
use std::{io, sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Mutex}, thread, time};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    print_menu();

    let is_running = Arc::new(AtomicBool::new(false));
    let delay = Arc::new(AtomicUsize::new(1000));
    let points = Arc::new(Mutex::new(Vec::new()));
    let current_position = Arc::new(Mutex::new(Point { x: 0.0, y: 0.0 }));

    let running_flag = Arc::clone(&is_running);
    let delay_flag = Arc::clone(&delay);
    let points_clone = Arc::clone(&points);

    let clicker_thread = thread::spawn(move || {
        let mut enigo = Enigo::new();
        let mut point_index = 0;

        loop {
            if running_flag.load(Ordering::SeqCst) {
                let points = points_clone.lock().unwrap();
                if !points.is_empty() {
                    let point: Point = points[point_index];
                    println!("Clicking point {}: x:{} , y: {}", point_index + 1, point.x, point.y);
                    enigo.mouse_move_to(point.x as i32, point.y as i32);
                    point_index = (point_index + 1) % points.len();
                }
                enigo.mouse_click(enigo::MouseButton::Left);
            }
            thread::sleep(time::Duration::from_millis(delay_flag.load(Ordering::SeqCst) as u64));
        }
    });

    if let Err(error) = listen(move |event| callback(event, 
                                                     Arc::clone(&is_running), 
                                                     Arc::clone(&delay), 
                                                     Arc::clone(&points),
                                                     Arc::clone(&current_position))) {
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
    println!("  [a]  Add Current Position as Point");
    println!("  [c]  Clear All Points");
    println!("  [l]  List All Points");
    println!("  [q]  Quit");
    println!("===============================");
}

fn callback(event: Event, 
            is_running: Arc<AtomicBool>, 
            delay: Arc<AtomicUsize>,
            points: Arc<Mutex<Vec<Point>>>,
            current_position: Arc<Mutex<Point>>) {
    match event.event_type {
        EventType::KeyPress(key) => {
            match key {
                Key::KeyS => {
                    let current_state = is_running.load(Ordering::SeqCst);
                    is_running.store(!current_state, Ordering::SeqCst);
                    println!("Auto clicker is now {}", if !current_state { "running" } else { "stopped" });
                }
                Key::KeyD => {
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
                Key::KeyA => {
                    let current_pos = *current_position.lock().unwrap();
                    points.lock().unwrap().push(current_pos);
                    println!("Added point: ({:.2}, {:.2})", current_pos.x, current_pos.y);
                }
                Key::KeyC => {
                    points.lock().unwrap().clear();
                    println!("All points cleared");
                }
                Key::KeyL => {
                    let points = points.lock().unwrap();
                    if points.is_empty() {
                        println!("No points set");
                    } else {
                        for (i, point) in points.iter().enumerate() {
                            println!("Point {}: ({:.2}, {:.2})", i + 1, point.x, point.y);
                        }
                    }
                }
                Key::KeyQ => {
                    std::process::exit(0);
                }
                _ => (),
            }
        }
        EventType::MouseMove { x, y } => {
            let mut current_pos = current_position.lock().unwrap();
            current_pos.x = x;
            current_pos.y = y;
        }
        _ => (),
    }
}