use clap::{Parser, CommandFactory};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::channel;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, override_usage = "saw --it <PATH> --do <COMMAND>")]
struct Args {
    /// Directory or file to watch
    #[arg(short = 'i', long = "it", default_value = ".")]
    path: String,

    /// Command to execute on change
    #[arg(short = 'd', long = "do")]
    command: Option<String>,

    /// Clear screen before executing command
    #[arg(short = 'c', long = "clear")]
    clear: bool,
}

fn main() -> notify::Result<()> {
    let args = Args::parse();
    
    // If command is not provided, print help and exit
    let command_str = match args.command {
        Some(cmd) => cmd,
        None => {
            let _ = Args::command().print_help();
            return Ok(());
        }
    };

    let path = Path::new(&args.path);
    let clear_screen = args.clear;

    println!("Watching path: {:?}", path);
    println!("Command to run: '{}'", command_str);

    let (tx, rx) = channel();

    // Initialize watcher
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Start watching
    watcher.watch(path, RecursiveMode::Recursive)?;

    println!("Waiting for changes...");

    // Event loop
    for res in rx {
        match res {
            Ok(event) => {
                // Simple logging of the event
                println!("Change detected: {:?}", event.kind);
                
                if clear_screen {
                    print!("\x1B[2J\x1B[1;1H");
                    let _ = std::io::Write::flush(&mut std::io::stdout());
                }
                
                println!("--- Executing: {} ---", command_str);
                
                let status = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(["/C", &command_str])
                        .status()
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .arg(&command_str)
                        .status()
                };

                match status {
                    Ok(s) => {
                        if s.success() {
                            println!("--- Success ---");
                        } else {
                            println!("--- Failed ({}) ---", s);
                        }
                    },
                    Err(e) => eprintln!("Failed to start command: {}", e),
                }
            },
            Err(e) => println!("Watch error: {:?}", e),
        }
    }

    Ok(())
}