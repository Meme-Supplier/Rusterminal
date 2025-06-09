#!/usr/bin/env rust-script

pub fn list() {
    let lines: [&str; 21] = [
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
        "rename <files>",
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
