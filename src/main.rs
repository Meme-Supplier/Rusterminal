#!/usr/bin/env rust-script
#[cfg(target_os = "linux")]
use hostname::get;
use rustyline::error::ReadlineError;
use rustyline::{Config, DefaultEditor};
use std::io::Write;
use std::process::exit;
use std::{env, io};

mod cmds;
mod funcs;
mod logger;
mod xray;

fn process_input(input: &str) {
    logger::log(&format!(
        "main::process_input(): Executing command: {input}"
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
            "restart" => funcs::run_shell_command("sudo reboot"),
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

            _ if command.starts_with("echo ") => funcs::echo(&command[5..]),
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
            _ if command.starts_with("rusterminal ") => rusterminal(&command[12..]),

            _ => println!("{command}: command not found"),
        }
    }
}

fn rusterminal(cmd: &str) {
    let config = funcs::load_configs();
    logger::log(&format!(
        "main::rusterminal(): Executing command in subcommand \"rusterminal()\": {cmd}"
    ));

    let lines: [&str; 19] = [
        "",
        "Available Commands:",
        "",
        "  build",
        "  clean",
        "  cmds",
        "  dellogs",
        "  help",
        "  logs",
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
        "help" => for line in lines.iter() { println!("{line}") },
        "credits" => println!("\nCredits:\n\nMaintainer: Meme Supplier\nLead programmer: Meme Supplier\n"),
        "rmtitle" => funcs::set_window_title("Rusterminal"),
        "xray" => xray::main(),
        "title" => println!("Usage: title <window title>"),
        "script" => println!("Usage: rusterminal script <script path>"),
        "clean" => funcs::clean(),

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
            funcs::run_shell_command("clear")
        }

        "upgrade" => {
            print!("Pick a channel to update to:\n\nbeta\nmain\n\nType \"exit\" to exit.\n\nChoice: ");

            let mut input = String::new();
            io::stdout().flush().expect("Failed to flush");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if input.trim() == "beta" {
                logger::log("main::rusterminal(): Updating Rusterminal to Beta branch...");
                funcs::run_shell_command("cd ~/ && git clone --branch beta --single-branch https://github.com/Meme-Supplier/Rusterminal && cd ~/Rusterminal/installer && bash install.sh");
                logger::log("main::rusterminal(): Update successful.");
                exit(0)
            } else if  input.trim() == "main" {
                logger::log("main::rusterminal(): Updating Rusterminal to Main branch...");
                funcs::run_shell_command("cd ~/rusterminal/installer/ && bash upgrade.sh");
                logger::log("main::rusterminal(): Update successful.");
                exit(0)
            } else {
                print!("\n");

                if input.trim() != "exit" {
                    println!("Invalid option! Please pick between \"beta\" and \"main\".")
                }
            }
        }

        "uninstall" => {
            logger::log("main::rusterminal(): Uninstalling Rusterminal...()");
            funcs::run_shell_command("cd ~/rusterminal/installer/ && bash uninstall.sh");
            logger::log("main::rusterminal(): Uninstall successful.");
            exit(0)
        }

        "build" => {
            let path = config.get("rusterminalBuildPath").map(|s| s.as_str()).unwrap_or_default();
            logger::log(&format!("main::rusterminal(): Building Rusterminal to {path}."));

            let command = format!("cd ~/rusterminal && cargo build && cd target/debug/ && cp Rusterminal {path} && echo -e \"\nBuilt Rusterminal to \\\"{path}\\\".\nYou can change the path in Rusterminal's configurations.\n\"");
            funcs::run_shell_command(&command);

            logger::log("main::rusterminal(): Build successful.")
        }

        "settings" => {
            logger::log("main::rusterminal(): Changing settings...");

            funcs::run_shell_command("nano ~/.config/rusterminal/settings.conf");

            logger::log("main::rusterminal(): Changed settings successfully.");

            match config.get("showReminderToSaveSettings").map(String::as_str) {
                Some("true") => println!("\nRestart Rusterminal for changes to take affect.\n"),
                Some(_) => {}
                None => {
                    println!("Setting 'showReminderToSaveSettings' not found in config!\nTry reloading Rusterminal!");
                    logger::log("main::rusterminal(): Setting 'showReminderToSaveSettings' not found in config!")
                }
            }
        }

        "update" => {
            match config.get("disableUpdateCMD").map(String::as_str) {
                Some("false") => funcs::update(),
                Some(_) => println!("\n\"update\" command disabled!\nRun command \"settings\" and look for the line \"disableUpdateCMD\".\n"),
                None => {
                    println!("Setting 'disableUpdateCMD' not found in config!\nTry reloading Rusterminal!");
                    logger::log("main::rusterminal(): Setting 'showReminderToSaveSettings' not found in config!")
                }
            }
        }

        _ if cmd.starts_with("title ") => funcs::set_window_title(&cmd[6..]),
        _ if cmd.starts_with("script ") => funcs::run_rusterminal_script(&cmd[7..]),

        _ => println!("Command not recognized: {cmd}")
    }
}

fn get_prompt() -> String {
    logger::log("main::get_prompt(): Setting user prompt...");

    let config = funcs::load_configs();

    let prompt = match config.get("promptType").map(String::as_str) {
        Some("default") => match config.get("useHostnameInPrompt").map(String::as_str) {
            Some("true") => {
                let hostname = get()
                    .map(|h| h.to_string_lossy().into_owned())
                    .unwrap_or_else(|_| "unknown".to_string());
                format!("{0}@{0}$~: ", hostname)
            }
            Some(_) => "rusterminal$~: ".to_string(),
            None => {
                println!("Setting 'useHostnameInPrompt' not found in config!\nTry reloading Rusterminal!");
                logger::log("Setting 'useHostnameInPrompt' not found in config!");
                "rusterminal$~: ".to_string()
            }
        },
        Some("custom") => config
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
        Some(_) => "rusterminal$~: ".to_string(), // fallback for unknown promptType
        None => {
            println!("Setting 'promptType' not found in config!\nTry reloading Rusterminal!");
            logger::log("Setting 'promptType' not found in config!");
            "rusterminal$~: ".to_string()
        }
    };

    prompt.to_string()
}

fn check_compatability() {
    logger::log("main::rusterminal(): Checking compatability...");

    let config = funcs::load_configs();

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

    match config
        .get("forceUniversalOScompatability")
        .map(String::as_str)
    {
        Some("false") => {
            if env::consts::OS != "linux" {
                println!("Rusterminal is designed for Linux only!\nExiting...");
                logger::log(
                    "main::check_compatability(): System isn't Linux, quitting Rusterminal.",
                );
                exit(0)
            }
        }
        Some(_) => {
            if env::consts::OS != "linux" {
                println!("Since you're OS isn't Linux, expect tons of errors and instability.");
                logger::log("main::check_compatability(): Running Rusterminal with tons on errors and instability...")
            }
        }
        None => {
            println!("Setting 'forceUniversalOScompatability' not found in config!\nTry reloading Rusterminal!");
            logger::log("main::check_compatability(): Setting 'forceUniversalOScompatability' not found in config!")
        }
    }

    match config
        .get("forceDisablePackageManagerCheck")
        .map(String::as_str)
    {
        Some("false") => {
            if funcs::detect_package_manager() == "apt"
                || funcs::detect_package_manager() == "dnf"
                || funcs::detect_package_manager() == "pacman"
            {
            } else {
                println!("You're using an unsupported package manager! Rusterminal will now exit.");
                logger::log("main::check_compatability(): User is using an unsupported package manager. Exiting...");
                exit(0)
            }
        }
        Some(_) => {}
        None => {
            println!("Setting 'forceDisablePackageManagerCheck' not found in config!\nTry reloading Rusterminal!");
            logger::log("main::check_compatibility(): Setting 'forceDisablePackageManagerCheck' not found in config!")
        }
    }
}

fn init() {
    funcs::set_window_title("Rusterminal");
    let config = funcs::load_configs();

    check_compatability();

    match config.get("clearScreenOnStartup").map(String::as_str) {
        Some("true") => funcs::run_shell_command("clear"),
        Some(_) => print!("\n"),
        None => {
            println!(
                "Setting 'clearScreenOnStartup' not found in config!\nTry reloading Rusterminal!"
            );
            logger::log("main::init(): Setting 'clearScreenOnStartup' not found in config!")
        }
    }

    match config.get("helpFuncOnStartup").map(String::as_str) {
        Some("true") => funcs::help(),
        Some(_) => {}
        None => {
            println!(
                "Setting 'helpFuncOnStartup' not found in config!\nTry reloading Rusterminal!"
            );
            logger::log("main::init(): Setting 'helpFuncOnStartup' not found in config!")
        }
    }
}

fn main() {
    logger::init(&format!(
        "===== Start session {} =====\n",
        &logger::get_time()
    ));

    let mut rl = DefaultEditor::with_config(Config::default()).expect("Failed to create editor");
    let config = funcs::load_configs();
    logger::log("main::main(): Loaded configurations");
    let prompt: String = get_prompt();

    init();

    loop {
        match rl.readline(&prompt) {
            Ok(line) => {
                let input = line.trim();
                if !input.is_empty() {
                    if let Some("true") = config.get("commandHistoryEnabled").map(String::as_str) {
                        let _ = rl.add_history_entry(input);
                    }
                    process_input(input)
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                eprintln!("Exiting...");
                break;
            }
            Err(err) => {
                eprintln!("Error: {err:?}");
                break;
            }
        }
    }
}
