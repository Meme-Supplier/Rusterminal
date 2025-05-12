#!/usr/bin/env rust-script

/* 2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier */

use std::io::{self, Write};
use std::process::{Command, Stdio};

static VERSION: &str = "2.0";

pub fn run_shell_command(cmd: &str) {
    if cmd.trim().is_empty() {
        return;
    }

    let _ = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
}

fn list() {
    let lines: [&str; 10] = [
        "1] main.rs",
        "2] cmds.rs",
        "3] funcs.rs",
        "4] diskfmt.py",
        "5] ver.py",
        "6] xray.rs",
        "7] install.sh",
        "8] upgrade.sh",
        "9] launch.sh",
        "10] uninstall.sh\n"];

    for  line in lines.iter() {
        println!("{line}");
    }
}

fn choose(input: &str) {
    let input = input.trim(); // Trim whitespace and newlines

    let file = match input {
        "1" => Some("main.rs"),
        "2" => Some("cmds.rs"),
        "3" => Some("funcs.rs"),
        "4" => Some("diskfmt.py"),
        "5" => Some("ver.py"),
        "6" => Some("xray.rs"),
        "7" => Some("install.sh"),
        "8" => Some("upgrade.sh"),
        "9" => Some("launch.sh"),
        "10" => Some("uninstall.sh"),
        _ => {
            println!("{input}: Invalid filename!");
            None
        }
    };

    if let Some(file) = file {
        let command = format!("nano ~/rusterminal/src/{file}");
        run_shell_command(&command);
    }
}

pub fn main() {
    println!("\nRusterminal Viewer v{VERSION}\n\nChoose file to view (select number):\nType \"0\" or \"exit\" to exit.\n");
    list();

    print!("Choice: ");
    let mut input = String::new();

    io::stdout().flush().expect("Failed to flush");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    choose(&input);
    print!("\n");
    run_shell_command("clear");
}
