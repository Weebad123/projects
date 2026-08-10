use std::{io, thread, time::Duration};
use std::sync::mpsc;
use std::thread::sleep;
use std::io::Write;




/*
/// code documentation
/// A simple CLI timer tool that allows users to set a time for a specified duration
/// and notifies them when the time is up
*/

fn main() {
    println!("Basic Timer Tool");

    println!("Enter the time duration separated by space (format: hours minutes seconds)");

    let duration = match get_user_timer_input() {
        Some(dur) => dur,
        None => {
            println!("Invalid input or format. Enter numbers only (e.g.. 0 1 30 for one minute thirty seconds");
            return;
        }
    };
    
    println!("Timer set for: {} hours, {} minutes, {} seconds", duration.0, duration.1, duration.2);

    println!("Press 'p' + Enter to pause, 'r' + Enter to resume.");

    start_timer(duration.0, duration.1, duration.2);
    println!("Time's up!");
}



fn get_user_timer_input() -> Option<(u64, u64, u64)> {
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input).expect("Failed to read user timer input");

    let parts_input = input.trim().split_whitespace().collect::<Vec<&str>>();

    // if the parts is not exactly 3, return None
    if parts_input.len() != 3 {
        return None;
    }

    // Parse the input
    let hours = parts_input[0].parse::<u64>().ok()?;
    let minutes = parts_input[1].parse::<u64>().ok()?;
    let seconds = parts_input[2].parse::<u64>().ok()?;

    Some((hours, minutes, seconds))
}

fn start_timer(hours: u64, minutes: u64, seconds: u64) {

    let total_seconds = hours * 3600 + minutes * 60 + seconds;

    let (schannel, rchannel) = mpsc::channel::<char>();

    thread::spawn( move || {
        loop {
            let mut command = String::new();
            if io::stdin().read_line(&mut command).is_ok() {
                if let Some(cmd) = command.trim().chars().next() {
                    if cmd == 'p' || cmd == 'r' {
                        let _ = schannel.send(cmd);
                    }
                }
            }
        }
    });

    let mut remaining = total_seconds;
    let mut paused = false;

    while remaining > 0 {
        if let Ok(cmd) = rchannel.try_recv() {
            match cmd {
                'p' => {
                    paused = true;
                    println!("\n Paused at {:02}:{:02}:{:02}", remaining / 3600, (remaining % 3600)/ 60, remaining % 60);
                }
                'r' => {
                    if paused {
                        paused = false;
                        println!("Resumed");
                    }
                }
                _ => {}
            }
        }

        if paused  {
            sleep(Duration::from_millis(200));
            continue;
        }

        let hrs = remaining / 3600;
        let mins = (remaining % 3600) / 60;
        let secs = remaining % 60;
        println!("\r Time remaining: {:02}:{:02}:{:02}", hrs, mins, secs);
        io::stdout().flush().unwrap();

        sleep(Duration::from_secs(1));
        remaining -= 1;
    }

    println!();
}