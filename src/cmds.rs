#!/usr/bin/env rust-script
#[cfg(target_os = "linux")]

/* 2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier */

pub fn list() {
    let lines: [&str; 31] = [
        "",
        "build",
        "clean",
        "clear",
        "credits",
        "cmds",
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
        "restart",
        "rmtitle",
        "settings",
        "shutdown",
        "title <title>",
        "uninstall",
        "update",
        "upgrade",
        "ver",
        "wait <time>",
        "xray",
        "",
    ];

    for line in lines.iter() {
        println!("{line}");
    }
}
