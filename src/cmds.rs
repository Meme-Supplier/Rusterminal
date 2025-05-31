#!/usr/bin/env rust-script
#[cfg(target_os = "linux")]

pub fn list() {
    let lines: [&str; 20] = [
        "",
        "clear",
        "copy <path>",
        "del <path>",
        "echo <text>",
        "edit <path>",
        "exit",
        "expr <equation>",
        "fmtdsk",
        "help",
        "ls <path>",
        "newdir <path>",
        "ping <site>",
        "python / python3",
        "run <command>",
        "rusterminal",
        "restart / reboot",
        "shutdown",
        "wait <time>",
        "",
    ];

    for line in lines.iter() {
        println!("{line}")
    }
}
