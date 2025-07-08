#!/usr/bin/env rust-script

use std::io::{self, Write};

use crate::funcs::run_shell_command;
use crate::logger::log;

static VERSION: &str = "2.5";

fn list() {
    let lines: [&str; 10] = [
        "1] main.rs",
        "2] cmds.rs",
        "3] funcs.rs",
        "4] logger.rs",
        "5] diskfmt.py",
        "6] sysinfo.rs",
        "7] xray.rs",
        "8] upgrade.sh",
        "9] launch.sh",
        "10] uninstall.sh\n",
    ];

    for line in lines.iter() {
        println!("{line}")
    }
}

fn choose(input: &str) {
    let input = input.trim();

    let file = match input {
        "1" => Some("main.rs"),
        "2" => Some("cmds.rs"),
        "3" => Some("funcs.rs"),
        "4" => Some("logger.rs"),
        "5" => Some("diskfmt.py"),
        "6" => Some("sysinfo.rs"),
        "7" => Some("xray.rs"),
        "8" => Some("upgrade.sh"),
        "9" => Some("launch.sh"),
        "10" => Some("uninstall.sh"),
        _ => {
            println!("{input}: Invalid filename!");
            None
        }
    };

    if let Some(file) = file {
        log(&format!("xray::choose(): Viewing file: {file}"));

        if file == "upgrade.sh" || file == "uninstall.sh" {
            run_shell_command(&format!("nano ~/rusterminal/installer/{file}"))
        } else {
            run_shell_command(&format!("nano ~/rusterminal/src/{file}"))
        }
    }
}

pub fn main() {
    log(&format!(
        "xray::main(): Rusterminal Viewer version: {VERSION}"
    ));

    println!("\nRusterminal Viewer v{VERSION}\n\nChoose file to view (select number):\nType \"0\" or \"exit\" to exit.\n");
    list();

    print!("Choice: ");
    let mut input = String::new();

    io::stdout().flush().expect("Failed to flush");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    choose(&input);
    println!();
    run_shell_command("clear")
}
