use std::io;
use std::time::{Duration, Instant};

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
    loop {
        if start.elapsed() >= duration {
            println!("Time Up!");
            break;
        }
    }

}
