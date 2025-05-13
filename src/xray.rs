#!/usr/bin/env rust-script

/* 2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier */

use std::io::{self, Write};
use crate::funcs::run_shell_command;

static VERSION: &str = "2.1";

fn list() {
    let lines: [&str; 9] = [
        "1] main.rs",
        "2] cmds.rs",
        "3] funcs.rs",
        "4] diskfmt.py",
        "5] ver.py",
        "6] xray.rs",
        "7] upgrade.sh",
        "8] launch.sh",
        "9] uninstall.sh\n"];

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
        "7" => Some("upgrade.sh"),
        "8" => Some("launch.sh"),
        "9" => Some("uninstall.sh"),
        _ => {
            println!("{input}: Invalid filename!");
            None
        }
    };

    if let Some(file) = file {
        if file == "upgrade.sh" || file == "uninstall.sh"
        {
            run_shell_command(&format!("nano ~/rusterminal/installer/{file}"))
        } else {
            run_shell_command(&format!("nano ~/rusterminal/src/{file}"));
        }
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
