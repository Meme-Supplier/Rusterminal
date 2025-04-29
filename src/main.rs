#!/usr/bin/env rust-script
#[cfg(target_os = "linux")]

/* 2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier */

use gethostname::gethostname;
use rustyline::error::ReadlineError;
use rustyline::{Config, DefaultEditor};
use std::env;
use std::process::exit;

mod cmds;
mod funcs;
mod loadconfigs;

fn process_input(input: &str) {
    // Load configurations
    let config = loadconfigs::load();

    for command in input.split("&&").map(|s| s.trim()) {
        if command.is_empty() {
            continue;
        }

        match command {
            "clear" => funcs::run_shell_command("clear"),
            "exit" => exit(0),
            "cmds" => cmds::list(),
            "ver" => funcs::ver(),
            "help" => funcs::help(),
            "shutdown" => funcs::run_shell_command("sudo shutdown now"),
            "restart" => funcs::run_shell_command("sudo reboot"),
            "uptime" => funcs::run_shell_command("uptime"),
            "python" | "python3" => funcs::run_shell_command("python3"),
            "xray" => funcs::xray(),
            "rmtitle" => funcs::set_window_title("Rusterminal"),
            "clean" => funcs::clean(),
            "credits" => funcs::credits(),
            "legacy" => funcs::run_shell_command("cd ~/ && git clone https://github.com/Meme-Supplier/PYShell.git && cd ~/PYShell && bash installer.sh"),
            "fmtdsk" => funcs::fmtdsk(),

            "build" => {
                let path = config.get("rusterminalBuildPath").map(|s| s.as_str()).unwrap_or_default();
                let command = format!("cd ~/rusterminal && cargo build && cd target/debug/ && cp rusterminal {path} && echo -e \"\nBuilt Rusterminal to \"{path}\".\nYou can change the path in Rusterminal's configurations.\n\"");
                funcs::run_shell_command(&command);
            }
            
            "settings" => {
                funcs::run_shell_command("nano ~/rusterminal/src/settings.conf");

                match config.get("showReminderToSaveSettings").map(String::as_str) {
                    Some("true") => {
                        println!("\nRestart Rusterminal for changes to take affect.\n")
                    }
                    Some(_) => {}
                    None => {
                        println!("Setting 'showReminderToSaveSettings' not found in config!\nTry reloading Rusterminal!");
                    }
                }
            }

            "update" => {
                match config.get("disableUpdateCMD").map(String::as_str) {
                    Some("false") => {
                        funcs::update()
                    }
                    Some(_) => {
                        println!("\n\"update\" command disabled!\nRun command \"settings\" and look for the line \"disableUpdateCMD\".\n")
                    }
                    None => {
                        println!("Setting 'disableUpdateCMD' not found in config!\nTry reloading Rusterminal!");
                    }
                }
            },

            "upgrade" => {
                funcs::run_shell_command("cd ~/rusterminal/src/ && bash upgrade.sh");
                process_input("exit");
            }
            "uninstall" => {
                funcs::run_shell_command("cd ~/rusterminal/src/ && bash uninstall.sh");
                process_input("exit");
            }

            // Commands that require extra usage
            "echo" => println!("Usage: echo <text>"),
            "run" => println!("Usage: run <command>"),
            "web" => println!("Usage: web <website>"),
            "expr" => println!("Usage: expr <equation>"),
            "wait" => println!("Usage: wait <time>"),
            "ping" => println!("Usage: ping <domain>"),
            "ls" => println!("Usage: ls <directory>"),
            "del" => println!("Usage: del <flag> <file/directory>"),
            "title" => println!("Usage: title <string>"),
            "edit" => println!("Usage: edit <path>"),
            "copy" => println!("Usage: copy <flag> <path>"),
            "newdir" => println!("Usage: newdir <path>"),

            _ if command.starts_with("echo ") => println!("{}", &command[5..]),
            _ if command.starts_with("run ") => funcs::run(&command[3..]),
            _ if command.starts_with("web ") => funcs::web(&command[4..]),
            _ if command.starts_with("expr ") => funcs::run(command),
            _ if command.starts_with("wait ") => funcs::wait(&command[5..]),
            _ if command.starts_with("ping ") => funcs::ping(&command[5..]),
            _ if command.starts_with("ls ") => funcs::ls(&command[3..]),
            _ if command.starts_with("del ") => funcs::del(&command[4..]),
            _ if command.starts_with("title ") => funcs::set_window_title(&command[6..]),
            _ if command.starts_with("edit ") => funcs::edit(&command[5..]),
            _ if command.starts_with("copy ") => funcs::copy(&command[5..]),
            _ if command.starts_with("in ") => funcs::input(&command[3..]),
            _ if command.starts_with("newdir ") => funcs::new_dir(&command[7..]),

            _ => println!("{command}: command not found"),
        }
    }
}

fn main() {
    funcs::run_shell_command("clear");

    // Load configurations
    let config = loadconfigs::load();

    match config
        .get("forceUniversalOScompatability")
        .map(String::as_str)
    {
        Some("false") => {
            if env::consts::OS == "linux" {
                // OK
            } else {
                println!("Rusterminal is designed for Linux only!\nExiting...");
                exit(0);
            }
        }
        Some(_) => {
            if env::consts::OS != "linux" {
                println!("Since you're OS isn't Linux, expect tons of errors and instability.");
            }
        }
        None => {
            println!("Setting 'forceUniversalOScompatability' not found in config!\nTry reloading Rusterminal!");
        }
    }

    if funcs::detect_package_manager() == "apt"
        || funcs::detect_package_manager() == "dnf"
        || funcs::detect_package_manager() == "pacman"
    {}

    funcs::set_window_title("Rusterminal");

    match config.get("helpFuncOnStartup").map(String::as_str) {
        Some("true") => {
            funcs::help();
        }
        Some(_) => {}
        None => {
            println!("Setting 'helpFuncOnStartup' not found in config!\nTry reloading Rusterminal!");
        }
    }

    let mut rl = DefaultEditor::with_config(Config::default()).expect("Failed to create editor");

    loop {
        let prompt = match config.get("useHostnameInPrompt").map(String::as_str) {
            Some("true") => format!(
                "{}@{}$~: ",
                gethostname().to_string_lossy(),
                gethostname().to_string_lossy()
            ),
            Some(_) => "rusterminal$~: ".to_string(),
            None => {
                println!("Setting 'useHostnameInPrompt' not found in config!\nTry reloading Rusterminal!");
                "rusterminal$~: ".to_string() // fallback
            }
        };

        match rl.readline(&prompt) {
            Ok(line) => {
                let input = line.trim();
                if !input.is_empty() {
                    match config.get("commandHistoryEnabled").map(String::as_str) {
                        Some("true") => {
                            let _ = rl.add_history_entry(input);
                        }
                        Some(_) => {
                            // Do nothing
                        }
                        None => {
                            println!("Setting 'commandHistoryEnabled' not found in config!\nTry reloading Rusterminal!");
                        }
                    }
                    process_input(input);
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Exiting...");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
