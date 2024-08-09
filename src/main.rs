use std::io;
use std::time::{Duration, Instant};
use std::io::Write;
//use crossterm::cursor::{Hide, Show};

fn main() {
    println!("Please enter time for your pomodoro session.");
    println!("Format must be 'xxmyys' Example: 25m10s ");
    let mut time = String::new();
    io::stdin()
        .read_line(&mut time)
        .expect("Failed to read line");
    let time = time.trim();

    let parts: Vec<&str> = time.split(|c| c == 'm' || c == 's').collect();

    let m = parts[0].parse::<u64>().unwrap_or(0);
    let s = parts[1].parse::<u64>().unwrap_or(0);

    let total = m*60 + s;
    let duration = Duration::from_secs(total);
    let start = Instant::now();
    let mut current = 0;
    loop {
        if start.elapsed().as_secs() >= (current + 1) {
             print!("\rTime elapsed: {}m {}s", start.elapsed().as_secs() / 60, start.elapsed().as_secs()%60);
            io::stdout().flush();
            current = start.elapsed().as_secs();
        }
        if start.elapsed() >= duration {
            break;
        }
    }
    print!("\rTime up!       ");
    io::stdout().flush();
}
