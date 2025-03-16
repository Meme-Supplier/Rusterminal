#!/usr/bin/env rust-script

/*
2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier
*/

use gethostname::gethostname;
use std::env;
use std::io::Error;
use std::io::{self, Write};
use std::process::{exit, Command};

fn process_input(input: &str) -> Result<(), Error> {
    match input {
        // Clear
        "clear" => run_shell_command("clear"),

        // Exit
        "exit" => exit(0),

        // cmds
        "cmds" => {
            cmds();
            Ok(()) // Return Ok for consistency
        }

        // Empty input
        "" => Ok(()),

        // help
        "help" => {
            help();
            Ok(())
        }

        // Shutdown
        "shutdown" => {
            let _ = run_shell_command("sudo shutdown now");
            Ok(())
        }

        // reboot
        "restart" => {
            let _ = run_shell_command("sudo reboot");
            Ok(())
        }

        // things with extra syntax

        // echo
        "echo" => {
            println!("Usage: echo <text>");
            Ok(())
        }

        // sh
        "sh" => {
            println!("Usage: sh <command>");
            Ok(())
        }

        // sh
        "sh" => {
            println!("Usage: expr <equation>");
            Ok(())
        }

        // Echo
        _ if input.starts_with("echo ") => {
            let echo_content = &input[5..]; // Strip "echo " from the input
            println!("{}", echo_content); // Display the echo content
            Ok(())
        }

        // Shell command execution
        _ if input.starts_with("sh ") => {
            let cmd = &input[3..]; // This removes the first 3 characters ("sh ")
            sh(cmd); // Run the shell command
            Ok(())
        }

        // expr
        _ if input.starts_with("expr ") => {
            sh(input);
            Ok(())
        }

        _ => {
            println!("{}: command not found", input);
            Ok(())
        }
    }
}

fn sh(cmd: &str) {
    println!();

    // Use sh to execute commands that may require shell features (like expr)
    let output = Command::new("sh")
        .arg("-c") // Use the -c option to pass the command as a string to the shell
        .arg(cmd) // Pass the command to be executed
        .output()
        .expect("Failed to execute shell command");

    // Check if there was any output from stdout
    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    // Check if there was any error output
    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
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
    ];

    for line in lines.iter() {
        println!("{}", line);
    }

    println!();
}

fn run_shell_command(cmd: &str) -> Result<(), Error> {
    // Check if the command exists in the system path
    let output = Command::new("which").arg(cmd).output();

    match output {
        Ok(output) if !output.stdout.is_empty() => {
            // If the command is found, execute it
            let command_output = Command::new(cmd)
                .output()
                .expect("Failed to execute command");

            println!("{}", String::from_utf8_lossy(&command_output.stdout));
            Ok(())
        }
        _ => Err(Error::new(
            std::io::ErrorKind::NotFound,
            format!("{} command not found", cmd),
        )),
    }
}

fn help() {
    println!("Welcome to Rusterminal v0.1.1!");
    println!("Type \"cmds\" for a list of commands!\n")
}

fn main() {
    // Attempt to clear the screen, handling errors if they occur
    if let Err(e) = run_shell_command("clear") {
        eprintln!("Error clearing screen: {}", e);
    }

    if env::consts::OS == "linux" {
        //let is_linux: bool = true;
    } else {
        println!("Rusterminal is designed for Linux only!\nExiting...");
        exit(0);
    }

    let hostname = gethostname();

    help();

    loop {
        let mut input = String::new();

        // Print the prompt and flush stdout
        print!(
            "{}@{}$~: ",
            hostname.to_string_lossy(),
            hostname.to_string_lossy()
        );
        io::stdout().flush().expect("Failed to flush");

        // Read user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Process the input
        if let Err(e) = process_input(input.trim()) {
            eprintln!("Error processing input: {}", e);
        }
    }
}
