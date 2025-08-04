use std::io::{self, Write};

use crate::funcs::run_shell_command;
use crate::logger::log;

static VERSION: &str = "2.6";

fn list() {
    let lines: [&str; 11] = [
        "1] src/main.rs",
        "2] src/cmds.rs",
        "3] src/funcs.rs",
        "4] src/logger.rs",
        "5] src/diskfmt.py",
        "6] src/sys/s_info.rs",
        "7] src/xray.rs",
        "8] installer/upgrade.sh",
        "9] launch.sh",
        "10] installer/uninstall.sh",
        "11] src/sys/s_vars.rs\n",
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
        "6" => Some("s_info.rs"),
        "7" => Some("xray.rs"),
        "8" => Some("upgrade.sh"),
        "9" => Some("launch.sh"),
        "10" => Some("uninstall.sh"),
        "11" => Some("s_vars.rs"),
        _ => {
            println!("{input}: Invalid filename!");
            None
        }
    };

    if let Some(file) = file {
        log(&format!("xray::choose(): Viewing file: {file}"));

        if file == "upgrade.sh" || file == "uninstall.sh" {
            run_shell_command(&format!("nano ~/rusterminal/installer/{file}"))
        } else if file == "s_vars.rs" || file == "s_info.rs" {
            run_shell_command(&format!("nano ~/rusterminal/src/sys/{file}"))
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
