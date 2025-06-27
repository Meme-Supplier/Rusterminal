#!/usr/bin/env rust-script

use hostname::get;
use rustyline::error::ReadlineError;
use rustyline::{Config, DefaultEditor};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::process::exit;
use std::sync::LazyLock;
use std::{env, io};

use crate::funcs::run_shell_command;

mod cmds;
mod funcs;
mod logger;
mod sysinfo;
mod xray;

static CONFIGS: LazyLock<HashMap<String, String>> = LazyLock::new(|| funcs::load_configs());

fn process_input(input: &str) {
    logger::log(&format!(
        "main::process_input(): Executing command: \"{input}\""
    ));

    for command in input.split("&&").map(|s| s.trim()) {
        if command.is_empty() {
            continue;
        }

        match command {
            "clear" => funcs::run_shell_command("clear"),
            "exit" => funcs::exit_rusterminal(),
            "help" => funcs::help(),
            "shutdown" => funcs::run_shell_command("sudo shutdown now"),
            "restart" | "reboot" => funcs::run_shell_command("sudo reboot"),
            "python" | "python3" => funcs::run_shell_command("python3"),
            "fmtdsk" => funcs::fmtdsk(),

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
            "rusterminal" => rusterminal("help"),
            "rename" => println!("Usage: rename <files>"),

            _ if command.starts_with("echo ") => println!("{}", &command[5..]),
            _ if command.starts_with("run ") => funcs::run_shell_command(&command[4..]),
            _ if command.starts_with("web ") => funcs::web(&command[4..]),
            _ if command.starts_with("expr ") => funcs::run_shell_command(command),
            _ if command.starts_with("wait ") => funcs::wait(&command[5..]),
            _ if command.starts_with("ping ") => funcs::ping(&command[5..]),
            _ if command.starts_with("ls ") => funcs::ls(&command[3..]),
            _ if command.starts_with("del ") => funcs::del(&command[4..]),
            _ if command.starts_with("edit ") => funcs::edit(&command[5..]),
            _ if command.starts_with("copy ") => funcs::copy(&command[5..]),
            _ if command.starts_with("in ") => funcs::input(&command[3..]),
            _ if command.starts_with("newdir ") => funcs::new_dir(&command[7..]),
            _ if command.starts_with("rename ") => funcs::rename(&command[7..]),
            _ if command.starts_with("rusterminal ") => rusterminal(&command[12..]),

            _ => {
                match CONFIGS
                    .get("treatRusterminalLikeARealShell")
                    .map(String::as_str)
                {
                    Some("true") => funcs::run_shell_command(command),
                    Some(_) => eprintln!("{command}: Command not found"),
                    None => {
                        eprintln!("Setting \"treatRusterminalLikeARealShell\" not found in config!\nTry reloading Rusterminal!");
                        logger::log("main::process_input(): Setting \"treatRusterminalLikeARealShell\" not found in config!")
                    }
                }
            }
        }
    }
}

fn rusterminal(cmd: &str) {
    logger::log(&format!(
        "main::rusterminal(): Executing command in subcommand \"rusterminal()\": \"{cmd}\""
    ));

    let lines: [&str; 22] = [
        "",
        "Available Commands:",
        "",
        "rusterminal <subcommand>",
        "",
        "  build",
        "  clean",
        "  cmds",
        "  dellogs",
        "  help",
        "  logs",
        "  reset",
        "  rmtitle",
        "  script <file path>",
        "  settings",
        "  title <title>",
        "  update",
        "  upgrade",
        "  uninstall",
        "  ver",
        "  xray",
        "",
    ];

    match cmd {
        "cmds" => cmds::list(),
        "ver" => funcs::ver(),
        "help" => {
            for line in lines.iter() {
                println!("{line}")
            }
        }
        "credits" => {
            println!("\nCredits:\n\nMaintainer: Meme Supplier\nLead programmer: Meme Supplier\n")
        }
        "rmtitle" => funcs::set_window_title("Rusterminal"),
        "xray" => xray::main(),
        "title" => println!("Usage: title <window title>"),
        "script" => println!("Usage: rusterminal script <script path>"),
        "clean" => funcs::clean(),
        "update" => funcs::update(),

        "history" => {
            logger::log("main:rusterminal(): Viewing Rusterminal's command history...");
            run_shell_command("nano ~/.rusterminal_history")
        }

        "reset" => {
            logger::log("main::rusterminal(): Resetting Rusterminal to it's default settings...");
            println!("Resetting Rusterminal to it's default settings...");

            funcs::run_shell_command("cd ~/.config/rusterminal/; rm -f settings.conf; mv defaults.conf settings.conf; cp settings.conf settings2.conf; mv settings2.conf defaults.conf");

            logger::log("main::rusterminal(): Rusterminal has been reset to its defaults.");
            println!("\nRusterminal has been reset to its defaults.\nPlease relaunch Rusterminal for changes to take effect.");

            funcs::run_shell_command("rm -rf ~/rusterminal/target");

            exit(0)
        }

        "dellogs" => {
            println!("Relaunch Rusterminal to reset your logs.");
            logger::log("main::rusterminal(): Deleting logs...");
            funcs::run_shell_command("rm -f ~/rusterminal/log.txt");
            exit(0)
        }

        "logs" => {
            logger::log("main::rusterminal(): Opening logs");
            funcs::run_shell_command("nano ~/rusterminal/log.txt");
            logger::log("main::rusterminal(): Logs closed");
        }

        "upgrade" => {
            print!("Pick a channel to update to:\n\n1] beta (Newest features, may not be stable)\n2] main (Latest stable version)\n\nType \"exit\" to exit.\n\nChoice: ");

            let mut input = String::new();

            io::stdout().flush().expect("Failed to flush");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if input.trim() == "beta" || input.trim() == "1" {
                logger::log("main::rusterminal(): Updating Rusterminal to Beta branch...");
                funcs::run_shell_command("cd ~/ && git clone --branch beta --single-branch https://github.com/Meme-Supplier/Rusterminal && cd ~/Rusterminal/installer && bash install.sh");
                logger::log("main::rusterminal(): Update successful.");
                exit(0)
            } else if input.trim() == "main" || input.trim() == "2" {
                logger::log("main::rusterminal(): Updating Rusterminal to Main branch...");
                funcs::run_shell_command("cd ~/rusterminal/installer/ && bash upgrade.sh");
                logger::log("main::rusterminal(): Update successful.");
                exit(0)
            } else {
                if input.trim() != "exit" {
                    eprintln!("\nInvalid option! Please pick between \"beta\" and \"main\".")
                } else {
                    eprintln!("\nOperation canceled.")
                }
            }
        }

        "uninstall" => {
            logger::log("main::rusterminal(): Uninstalling Rusterminal...");
            funcs::run_shell_command("cd ~/rusterminal/installer/ && bash uninstall.sh");
            logger::log("main::rusterminal(): Uninstall successful.");
            exit(0)
        }

        "build" => {
            let path = &CONFIGS
                .get("rusterminalBuildPath")
                .map(|s| s.as_str())
                .unwrap_or_default()[1..CONFIGS
                .get("rusterminalBuildPath")
                .map(|s| s.as_str())
                .unwrap_or_default()[1..]
                .len()];

            let build_command: &str = &CONFIGS
                .get("rusterminalBuildCommand")
                .map(|s| s.as_str())
                .unwrap_or_default()[1..CONFIGS
                .get("rusterminalBuildCommand")
                .map(|s| s.as_str())
                .unwrap_or_default()[1..]
                .len()];

            logger::log(&format!("main::rusterminal(): Building Rusterminal to \"{path}\" using command \"{build_command}\"."));
            funcs::run_shell_command(&format!("cd ~/rusterminal && {build_command} && cd target/debug || cd target/release && cp Rusterminal {path} && echo -e \"\nBuilt Rusterminal to \\\"{path}\\\".\nYou can change the path in Rusterminal's configurations.\n\""));
            logger::log("main::rusterminal(): Build successful.")
        }

        "settings" => {
            logger::log("main::rusterminal(): Changing settings...");

            funcs::run_shell_command("nano ~/.config/rusterminal/settings.conf");
            logger::log("main::rusterminal(): Changed settings successfully.");
        }

        _ if cmd.starts_with("title ") => funcs::set_window_title(&cmd[6..]),
        _ if cmd.starts_with("script ") => funcs::run_rusterminal_script(&cmd[7..]),

        _ => println!("Command not recognized: {cmd}"),
    }
}

fn get_prompt() -> String {
    logger::log("main::get_prompt(): Setting user prompt...");

    let prompt = match CONFIGS.get("promptType").map(String::as_str) {
        Some("default") => match CONFIGS.get("useHostnameInPrompt").map(String::as_str) {
            Some("true") => {
                let hostname = get()
                    .map(|h| h.to_string_lossy().into_owned())
                    .unwrap_or_else(|_| "unknown".to_string());
                let username = &env::var("USER").expect("Failed to get USER");

                format!("{hostname}@{username}$~: ")
            }
            Some(_) => "rusterminal$~: ".to_string(),
            None => {
                eprintln!("Setting \"useHostnameInPrompt\" not found in config!\nTry reloading Rusterminal!");
                logger::log("\"Setting 'useHostnameInPrompt\" not found in config!");
                "rusterminal$~: ".to_string()
            }
        },
        Some("custom") => CONFIGS
            .get("customPrompt")
            .map(|s| {
                let s = s.trim();
                if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                    s[1..s.len() - 1].to_string()
                } else {
                    s.to_string()
                }
            })
            .unwrap_or_else(|| "rusterminal$~: ".to_string()),
        Some(_) => "rusterminal$~: ".to_string(),
        None => {
            eprintln!("Setting \"promptType\" not found in config!\nTry reloading Rusterminal!");
            logger::log("Setting \"promptType\" not found in config!");
            "rusterminal$~: ".to_string()
        }
    };

    prompt.to_string()
}

fn check_compatability() {
    logger::log("main::rusterminal(): Checking compatability...");

    let package_manager = funcs::detect_package_manager();
    let os = env::consts::OS;

    logger::log(&format!(
        "main::check_compatability(): Package Manager: {package_manager}"
    ));
    logger::log(&format!("main::check_compatability(): OS: {os}"));
    logger::log(&format!(
        "main::check_compatability(): Rusterminal Version: {}",
        funcs::VERSION
    ));

    match CONFIGS
        .get("forceUniversalOScompatability")
        .map(String::as_str)
    {
        Some("false") => {
            if os != "linux" {
                eprintln!("Rusterminal is designed for Linux only!\nExiting...");
                logger::log(
                    "main::check_compatability(): System isn't Linux, quitting Rusterminal.",
                );
                exit(0)
            }
        }
        Some(_) => {
            if os != "linux" {
                println!("Since you're OS isn't Linux, expect tons of errors and instability.");
                logger::log("main::check_compatability(): Running Rusterminal with tons on errors and instability...")
            }
        }
        None => {
            eprintln!("Setting \"forceUniversalOScompatability\" not found in config!\nTry reloading Rusterminal!");
            logger::log("main::check_compatability(): Setting \"forceUniversalOScompatability\" not found in config!")
        }
    }

    if package_manager == "apt"
        || package_manager == "dnf"
        || package_manager == "pacman"
        || package_manager == "zypper"
    {
    } else {
        println!("You're using an unsupported package manager! Expect errors and incompatability!");
        logger::log("main::check_compatability(): User is using an unsupported package manager. Errors and incompatability are imminent.");
    }
}

fn init() {
    funcs::set_window_title("Rusterminal");

    check_compatability();

    logger::log("main::init(): System is compatible, continuing...");

    match CONFIGS.get("clearScreenOnStartup").map(String::as_str) {
        Some("true") => funcs::run_shell_command("clear"),
        Some(_) => println!(),
        None => {
            eprintln!(
                "Setting \"clearScreenOnStartup\" not found in config!\nTry reloading Rusterminal!"
            );
            logger::log("main::init(): Setting \"clearScreenOnStartup\" not found in config!")
        }
    }

    match CONFIGS.get("helpFuncOnStartup").map(String::as_str) {
        Some("true") => funcs::help(),
        Some(_) => {}
        None => {
            eprintln!(
                "Setting \"helpFuncOnStartup\" not found in config!\nTry reloading Rusterminal!"
            );
            logger::log("main::init(): Setting \"helpFuncOnStartup\" not found in config!")
        }
    }
}

fn get_rusterminal_history() -> io::Result<Vec<String>> {
    let home = logger::get_home();
    let content = fs::read_to_string(format!("{home}/.rusterminal_history"))?;
    let words = content.lines().map(|s| s.to_string()).collect();

    Ok(words)
}

fn main() {
    logger::init(&format!(
        "\n===== Start session {} =====\n",
        &logger::get_time()
    ));

    logger::log(&format!(
        "logger::get_time_format(): Using time/date format: {}",
        logger::get_time_format()
    ));

    let mut rl = DefaultEditor::with_config(Config::default()).expect("Failed to create editor");

    if let Ok(history) = get_rusterminal_history() {
        for cmd in history {
            let _ = rl.add_history_entry(cmd);
        }
    }

    init();

    loop {
        match rl.readline(&get_prompt()) {
            Ok(line) => {
                let input = line.trim();
                if !input.is_empty() {
                    if let Some("true") = CONFIGS.get("commandHistoryEnabled").map(String::as_str) {
                        let home = logger::get_home();

                        let _ = rl.add_history_entry(input);
                        let _ =
                            logger::write_to_file(input, &format!("{home}/.rusterminal_history"));
                    }
                    process_input(input)
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                eprintln!("Force-exiting Rusterminal...");
                break;
            }
            Err(err) => {
                eprintln!("Error: {err:?}");
                break;
            }
        }
    }
}
