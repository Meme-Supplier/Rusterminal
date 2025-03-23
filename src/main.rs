#!/usr/bin/env rust-script

/*
2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier
*/

use gethostname::gethostname;
use std::env;
use std::io::{self, Write};
use std::process::exit;

mod funcs;

#[cfg(target_os = "linux")]

fn process_input(input: &str) {
    for command in input.split("&&").map(|s| s.trim()) {
        if command.is_empty() {
            continue;
        }

        match command {
            "clear" => funcs::run_shell_command("clear"),
            "exit" => exit(0),
            "cmds" => funcs::cmds(),
            "ver" => funcs::ver(),
            "reload" | "rusterminal" => funcs::reload(),
            "help" => funcs::help(),
            "shutdown" => funcs::run_shell_command("sudo shutdown now"),
            "restart" => funcs::run_shell_command("sudo reboot"),
            "uptime" => funcs::run_shell_command("uptime"),
            "python" | "python3" => funcs::run_shell_command("python3"),
            "update" => funcs::update(),
            "xray" => funcs::run_shell_command("nano ~/rusterminal/src/main.rs"),
            "uninstall" => {funcs::run_shell_command("cd ~/rusterminal/src/ && bash uninstall.sh"); process_input("exit");},
            "rmtitle" => funcs::set_window_title("Rusterminal"),
            "upgrade" => {funcs::run_shell_command("cd ~/rusterminal/src/ && bash upgrade.sh"); process_input("exit");},

            // Commands that require syntax
            "echo" => println!("Usage: echo <text>"),
            "sh" => println!("Usage: sh <command>"),
            "web" => println!("Usage: web <website>"),
            "expr" => println!("Usage: expr <equation>"),
            "wait" => println!("Usage: wait <time>"),
            "ping" => println!("Usage: ping <domain>"),
            "ls" => println!("Usage: ls <directory>"),
            "del" => println!("Usage: del <flag> <file/directory>"),
            "title" => println!("Usage: title <string>"),
            "edit" => println!("Usage: edit <path>"),
            "copy" => println!("Usage: copy <flag> <path>"),

            // Commands with arguments
            _ if command.starts_with("echo ") => println!("{}", &command[5..]),
            _ if command.starts_with("sh ") => funcs::sh(&command[3..]),
            _ if command.starts_with("web ") => funcs::web(&command[4..]),
            _ if command.starts_with("expr ") => funcs::sh(command),
            _ if command.starts_with("wait ") => funcs::wait(&command[5..]),
            _ if command.starts_with("ping ") => funcs::ping(&command[5..]),
            _ if command.starts_with("ls ") => funcs::ls(&command[3..]),
            _ if command.starts_with("del ") => funcs::del(&command[4..]),
            _ if command.starts_with("title ") => funcs::set_window_title(&command[6..]),
            _ if command.starts_with("edit ") => funcs::edit(&command[5..]),
            _ if command.starts_with("copy ") => funcs::copy(&command[5..]),

            _ => println!("{}: command not found", command),
        }
    }
}

fn main() {
    // Attempt to clear the screen
    funcs::run_shell_command("clear");

    if env::consts::OS == "linux" {
        // Do nothing if it's Linux
    } else {
        println!("Rusterminal is designed for Linux only!\nExiting...");
        exit(0);
    }

    if matches!(funcs::detect_package_manager().as_str(), "apt" | "dnf" | "pacman") {
        // Do nothing if a supported package manager
    } else {
        println!("Unsupported package manager! Rusterminal only supports Apt, Dnf, and Pacman.");
        exit(0);
    }

    funcs::set_window_title("Rusterminal");
    funcs::help();

    loop {
        let mut input = String::new();

        // Print the prompt and flush stdout
        print!(
            "{}@{}$~: ",
            gethostname().to_string_lossy(),
            gethostname().to_string_lossy()
        );
        io::stdout().flush().expect("Failed to flush");

        // Read user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Process the input
        process_input(input.trim());
    }
}
