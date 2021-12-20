use std::env;
use std::process;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;
use std::time::Instant;
use colored::*;

struct TimedResult {
    day: String,
    duration: f64,
}

impl TimedResult {
    fn new(day: &String, duration: Duration) -> TimedResult {
        return TimedResult {
            day: day.to_string(),
            duration: duration.as_secs_f64(),
        };
    }
}

fn main() {
    const DAYS: i64 = 19;

    for arg in env::args() {
        if arg == "--help" {
            println!("Usage:");
            println!("    cargo run               # Run performance benchmark");
            println!("    cargo run --bin day_XY  # Run a specific days executable");
            println!();
            println!("Options:");
            println!("    --help                  # Display help information");
            println!("    --version               # Display version information");
            process::exit(0);
        } else if arg == "--version" {
            println!("Advent of Code 2021");
            println!("Copyright (C) 2021 Brian Rowlett");
            process::exit(0);
        }
    }

    println!("Building...");
    for day in 1..=DAYS {
        let binary = format!("day_{:0>2}", day);

        let mut build_cmd = Command::new("cargo");
        build_cmd.stdout(Stdio::null());
        build_cmd.stderr(Stdio::null());
        build_cmd.args(["build", "--bin", binary.as_str(), "--release"]);

        if let Ok(mut build_process) = build_cmd.spawn() {
            if let Ok(build_status) = build_process.wait() {
                if !build_status.success() {
                    if let Some(build_code) = build_status.code() {
                        panic!("Build process failed: Day {} (exit status: {})", day, build_code);
                    }
                }
            }
        }
    }

    println!("Executing...");
    let mut timed_results = vec![];
    for day in 1..=DAYS {
        let binary = format!("target/release/day_{:0>2}", day);

        let now = Instant::now();

        let mut execute_cmd = Command::new(binary);
        execute_cmd.stdout(Stdio::null());
        execute_cmd.stderr(Stdio::null());

        if let Ok(mut execute_process) = execute_cmd.spawn() {
            if let Ok(execute_status) = execute_process.wait() {
                if !execute_status.success() {
                    if let Some(execute_code) = execute_status.code() {
                        panic!("Execute process failed: Day {} (exit status: {})", day, execute_code);
                    }
                }
            }
        }

        let duration = now.elapsed();

        let day = format!("Day {:0>2}", day);

        timed_results.push(TimedResult::new(&day, duration));
    }

    for timed_result in &timed_results {
        let mut color = Color::Red;
        if timed_result.duration < 0.04 {
            color = Color::Green;
        } else if timed_result.duration < 0.10 {
            color = Color::Yellow;
        }

        let duration = timed_result.duration.to_string();

        println!("    {}: {:.8} seconds", timed_result.day, duration.color(color));
    }
}
