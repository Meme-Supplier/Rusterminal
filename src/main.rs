#!/usr/bin/env rust-script

/*
2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier
*/

use gethostname::gethostname;
use std::env;
use std::io::{self, Write};
use std::process::{exit, Command, Stdio};

static version: &str = "v0.1.2";

fn process_input(input: &str) {
    match input {
        // Clear
        "clear" => run_shell_command("clear"),

        // Exit
        "exit" => exit(0),

        // cmds
        "cmds" => cmds(),

        // ver
        "ver" => ver(),

        // Empty input
        "" => (), // Do nothing on empty input

        // help
        "help" => help(),

        // Shutdown
        "shutdown" => {
            let _ = run_shell_command("sudo shutdown now");
        }

        // reboot
        "restart" => {
            let _ = run_shell_command("sudo reboot");
        }

        // things with extra syntax

        // echo
        "echo" => println!("Usage: echo <text>"),

        // Web
        "web" => println!("Usage: web <page>"),

        // sh
        "sh" => println!("Usage: sh <command>"),

        // expr
        "expr" => println!("Usage: expr <equation>"),

        // Echo
        _ if input.starts_with("echo ") => {
            let echo_content = &input[5..]; // Strip "echo " from the input
            println!("{}", echo_content); // Display the echo content
        }

        // Shell command execution
        _ if input.starts_with("sh ") => {
            let cmd = &input[3..];
            sh(cmd); // Run the shell command
        }

        // Open a web page
        _ if input.starts_with("web ") => {
            let page = &input[4..];
            web(page); // Run the shell command
        }

        // expr
        _ if input.starts_with("expr ") => {
            sh(input);
        }

        _ => {
            println!("{}: command not found", input);
        }
    }
}

fn web(url: &str) {
    #[cfg(target_os = "linux")]
    Command::new("xdg-open")
        .arg(url)
        .spawn()
        .expect("Failed to open webpage");
}

fn ver() {
    println!("\nRusterminal version: {}\n", version);

    // Path to the Python file
    let python_script = "./ver.py"; // Replace with your Python file name

    // Run the Python script using 'python' or 'python3'
    let _ = Command::new("python3") // or "python3"
        .arg(python_script)
        .status(); // Run the command without checking status

    println!();
}

fn sh(cmd: &str) {
    // Use sh to execute commands that may require shell features
    let _ = Command::new("sh")
        .arg("-c") // Use the -c option to pass the command as a string to the shell
        .arg(cmd) // Pass the command to be executed
        .stdin(Stdio::inherit()) // Allows interactive commands like sudo
        .stdout(Stdio::inherit()) // Preserves colored output
        .stderr(Stdio::inherit()) // Preserves error messages
        .status(); // Use `.status()` instead of `.output()`
}

fn cmds() {
    println!();

    let lines = [
        "clear",
        "exit",
        "cmds",
        "help",
        "echo <text>",
        "sh <command>",
        "expr <equation>",
        "restart",
        "shutdown",
        "ver",
    ];

    for line in lines.iter() {
        println!("{}", line);
    }

    println!();
}

fn run_shell_command(cmd: &str) {
    if cmd.trim().is_empty() {
        return;
    }

    // Use a shell (`sh -c`) so multi-word commands and colors work properly
    let _ = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::inherit()) // Allows interactive commands (like sudo pacman -Syu)
        .stdout(Stdio::inherit()) // Preserves output colors
        .stderr(Stdio::inherit()) // Preserves error messages
        .status(); // Run the command and wait
}

fn help() {
    println!("Welcome to Rusterminal {}!", version);
    println!("Type \"cmds\" for a list of commands!\n")
}

fn main() {
    // Attempt to clear the screen, ignoring errors
    run_shell_command("clear");

    if env::consts::OS == "linux" {
        // Do nothing if it's Linux
    } else {
        println!("Rusterminal is designed for Linux only!\nExiting...");
        exit(0);
    }

    println!();
    help();

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
