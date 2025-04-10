#!/usr/bin/env rust-script
#[cfg(target_os = "linux")]

/*
2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier
*/

pub fn list() {
    let lines: [&str; 28] = [
        "",
        "clean",
        "clear",
        "cmds",
        "copy <path>",
        "del <path>",
        "echo <text>",
        "edit <path>",
        "exit",
        "expr <equation>",
        "help",
        "ls <path>",
        "newdir <path>",
        "ping <site>",
        "python / python3",
        "run <command>",
        "restart",
        "rmtitle",
        "shutdown",
        "title <title>",
        "uninstall",
        "update",
        "upgrade",
        "uptime",
        "ver",
        "wait <time>",
        "xray",
        "",
    ];

    for line in lines.iter() {
        println!("{}", line);
    }
}
