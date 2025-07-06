#!/usr/bin/env rust-script

use regex::Regex;
use rustc_version_runtime::version;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process::{exit, Command, Stdio};
use std::{env, fs};

use crate::logger::{get_home, get_time, init, log};
use crate::process_input;
use crate::sysinfo::get_system_info;

pub const VERSION: &str = "v0.3.5-beta5";

pub fn load_configs() -> HashMap<String, String> {
    let home_dir: String = env::var("HOME").expect("Failed to get HOME directory");

    let content = fs::read_to_string(format!("{home_dir}/.config/rusterminal/settings.conf"))
        .expect("Failed to read config");

    let mut config: HashMap<String, String> = HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue; // skip empty lines or comments
        }
        if let Some((key, value)) = line.split_once('=') {
            config.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    config
}

pub fn run_rusterminal_script(path: &str) {
    log(&format!(
        "funcs::run_rusterminal_script(): Running Rusterminal script: {path}"
    ));

    let file: Result<File, io::Error> = File::open(path);

    if let Ok(file) = file {
        let reader: BufReader<File> = BufReader::new(file);
        for line_result in reader.lines() {
            if let Ok(line) = line_result {
                process_input(&line)
            } else {
                eprintln!("Failed to read a line in {path}");
                log(&format!(
                    "funcs::run_rusterminal_script(): Failed to read a line in Rusterminal script: {path}"
                ))
            }
        }
    } else {
        eprintln!("Failed to open script file: {path}");
        log(&format!(
            "funcs::run_rusterminal_script(): Failed to open script file in Rusterminal script: {path}"
        ))
    }
}

pub fn set_current_cwd(dir: &str) -> io::Result<()> {
    let home = get_home(); // assuming this returns String

    // Expand ~ and $HOME
    let resolved_dir = if dir.starts_with("~/") {
        format!("{home}/{}", &dir[2..])
    } else if dir == "~" || dir == "$HOME" {
        home
    } else if dir.starts_with("$HOME/") {
        format!("{home}/{}", &dir[6..])
    } else {
        dir.to_string()
    };

    let path = Path::new(&resolved_dir);

    if !path.exists() {
        eprintln!("Directory \"{resolved_dir}\" doesn't exist");
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Directory does not exist: {resolved_dir}"),
        ));
    }

    if !path.is_dir() {
        eprintln!("Directory \"{resolved_dir}\" doesn't exist");
        log(&format!(
            "funcs::set_current_cwd(): Directory \"{resolved_dir}\" doesn't exist."
        ));
    }

    env::set_current_dir(path)?;

    Ok(())
}

pub fn exit_rusterminal() {
    match load_configs()
        .get("cleanCompileOnStartup")
        .map(String::as_str)
    {
        Some("true") => {
            log("funcs::exit_rusterminal(): Cleaning up Rusterminal before closing...");
            run_shell_command("cd $HOME/rusterminal; cargo clean");
            exit(0)
        }
        Some(_) => {
            log("funcs::exit_rusterminal(): Exiting Rusterminal...");
            init(&format!("\n===== End Session {} =====", get_time()));
            exit(0)
        }
        None => {
            eprintln!(
                "Setting \"cleanCompileOnStartup\" not found in config!\nTry reloading Rusterminal!"
            );
            log("funcs::exit_rusterminal(): Setting \"cleanCompileOnStartup\" not found in config!")
        }
    }
}

pub fn run_python(script: &str) {
    log(&format!(
        "funcs::run_python(): Running Python script: {script}"
    ));
    run_shell_command(&format!("python3 {script}"));
}

pub fn fmtdsk() {
    log("funcs::fmtdsk(): Running disk formatter script.");

    let home_dir = get_home();
    run_python(&format!("{home_dir}/rusterminal/src/diskfmt.py"));

    log("funcs::fmtdsk(): Disk formatting successful.");
}

pub fn rename(files: &str) {
    log(&format!("funcs::rename(): Renaming... {files}"));
    run_shell_command(&format!("mv {files}"));
}

pub fn new_dir(dir: &str) {
    log(&format!("funcs::new_dir(): Creating directory: {dir}"));
    run_shell_command(&format!("mkdir {dir}"))
}

pub fn input(str: &str) {
    let mut input: String = String::new();

    println!("{str}");
    io::stdout().flush().expect("Failed to flush");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    print!("{input}");
}

pub fn clean() {
    log("funcs::clean(): Cleaning up your package manager...");

    if detect_package_manager().as_str() == "apt" {
        run_shell_command("sudo apt autoremove -y")
    } else if detect_package_manager().as_str() == "dnf" {
        run_shell_command("sudo dnf autoremove -y")
    } else if detect_package_manager().as_str() == "zypper" {
        run_shell_command(
            "sudo zypper remove $(zypper packages --orphaned | awk '/^i/ {print $5}') -y; sudo zypper clean --all -y; sudo rm -rf /var/cache/zypp/packages/* || exit",
        );
    } else {
        run_shell_command(
            "sudo pacman -Rns $(pacman -Qdtq) --noconfirm; sudo pacman -S -cc --noconfirm",
        );

        match load_configs()
            .get("considerYayAsAPackageManager")
            .map(String::as_str)
        {
            Some("true") => {
                run_shell_command("yay -Rns $(yay -Qdtq) --noconfirm; yay -S -cc --noconfirm");
            }
            Some(_) => {}
            None => {
                eprintln!(
                    "Setting \"considerYayAsAPackageManager\" not found in config!\nTry reloading Rusterminal!"
                );
                log("funcs::clean(): Setting \"considerYayAsAPackageManager\" not found in config!")
            }
        }

        match load_configs()
            .get("considerParuAsAPackageManager")
            .map(String::as_str)
        {
            Some("true") => {
                run_shell_command("paru -Rns $(paru -Qdtq) --noconfirm; paru -S -cc --noconfirm");
            }
            Some(_) => {}
            None => {
                eprintln!(
                    "Setting \"considerParuAsAPackageManager\" not found in config!\nTry reloading Rusterminal!"
                );
                log(
                    "funcs::clean(): Setting \"considerParuAsAPackageManager\" not found in config!",
                )
            }
        }
    }

    log("funcs::clean(): Cleaning up Rust(erminal)...");
    run_shell_command("cd ~/rusterminal; cargo clean");

    log("funcs::clean(): Cleaning up system cache...");
    run_shell_command("sudo rm -rf ~/.cache || exit 0")
}

pub fn copy(path: &str) {
    log(&format!("funcs::copy(): Copying file: {path}"));
    run_shell_command(&format!("cp {path}"))
}

pub fn edit(file: &str) {
    log(&format!("funcs::edit(): Editing file: {file}"));
    run_shell_command(&format!("nano {file}"))
}

pub fn set_window_title(title: &str) {
    log(&format!(
        "funcs::set_window_title(): Setting window title: {title}"
    ));
    print!("\x1b]0;{title}\x07");
    io::stdout().flush().unwrap()
}

pub fn del(file: &str) {
    log(&format!("funcs::del(): Deleting file: {file}"));
    run_shell_command(&format!("rm {file}"))
}

pub fn ls(path: &str) {
    log(&format!("funcs::ls(): Listing directory: {path}"));
    run_shell_command(&format!("ls {path}"))
}

pub fn ping(add: &str) {
    log(&format!("funcs::ping(): Pinging web address: {add}"));
    run_shell_command(&format!("ping {add}"))
}

pub fn wait(time: &str) {
    log(&format!("funcs::wait(): Waiting {time} seconds..."));
    run_shell_command(&format!("sleep {time}"))
}

pub fn update() {
    let package_manager = &detect_package_manager();
    let config = &load_configs();

    log("funcs::update(): Updating Rust...");
    run_shell_command("rustup update"); // Update Rust
    run_shell_command("cd ~/rusterminal; cargo update"); // Update Rusterminal's dependencies

    log("funcs::update(): Updating system packages...");

    if package_manager == "apt" {
        run_shell_command("sudo apt update; sudo apt upgrade")
    } else if package_manager == "dnf" {
        run_shell_command("sudo dnf update")
    } else if package_manager == "zypper" {
        run_shell_command("sudo zypper refresh; sudo zypper update");
    } else {
        run_shell_command("sudo pacman -Syyu");

        match config
            .get("considerYayAsAPackageManager")
            .map(String::as_str)
        {
            Some("true") => run_shell_command("yay -Syyu"),
            Some(_) => {}
            None => {
                eprintln!(
                    "Setting \"considerYayAsAPackageManager\" not found in config!\nTry reloading Rusterminal!"
                );
                log("Setting \"considerYayAsAPackageManager\" not found in config!")
            }
        }

        match config
            .get("considerParuAsAPackageManager")
            .map(String::as_str)
        {
            Some("true") => run_shell_command("paru -Syyu"),
            Some(_) => {}
            None => {
                eprintln!(
                    "Setting \"considerParuAsAPackageManager\" not found in config!\nTry reloading Rusterminal!"
                );
                log(
                    "funcs::update(): Setting \"considerParuAsAPackageManager\" not found in config!",
                )
            }
        }

        match config
            .get("considerFlatpakAsAPackageManager")
            .map(String::as_str)
        {
            Some("true") => run_shell_command("flatpak update"),
            Some(_) => {}
            None => {
                eprintln!(
                    "Setting \"considerFlatpakAsAPackageManager\" not found in config!\nTry reloading Rusterminal!"
                );
                log(
                    "funcs::update(): Setting \"considerFlatpakAsAPackageManager\" not found in config!",
                )
            }
        }
    }

    match config.get("enableCustomUpdateCommand").map(String::as_str) {
        Some("true") => {
            let path = &config
                .get("customUpdateCommand")
                .map(|s| s.as_str())
                .unwrap_or_default()[1..config
                .get("customUpdateCommand")
                .map(|s| s.as_str())
                .unwrap_or_default()[1..]
                .len()];

            log(&format!(
                "funcs::run_custom_update_command(): Running custom update command: \"{path}\""
            ));

            run_shell_command(path);
        }
        Some(_) => {}
        None => {
            println!(
                "Setting \"enableCustomUpdateCommand\" not found in config!\nTry reloading Rusterminal!"
            );
            log("funcs::update(): Setting \"enableCustomUpdateCommand\" not found in config!")
        }
    }

    log("funcs::update(): Updated system.")
}

pub fn web(url: &str) {
    log(&format!("funcs::web(): Opening webpage: {url}"));
    run_shell_command(&format!("xdg-open {url}"));
}

pub fn ver() {
    println!("\nRusterminal version: {VERSION}");
    println!("Rust version: {}", rustc_version::version().unwrap());
    println!("Python version: {}\n", &get_python_version());

    let system_info = get_system_info();

    println!("Desktop Environment: {}", system_info.desktop_environment);
    println!(
        "Window Manger: {} ({})",
        system_info.window_manager, system_info.display_protocol
    );
    println!("Distro: {}", system_info.distro);
    println!("Shell: {}", system_info.shell);
    println!("Preferred package manager: {}\n", detect_package_manager())
}

pub fn run_shell_command(cmd: &str) {
    let config = load_configs();

    let shell = &config
        .get("shellToRunShellCommands")
        .map(|s| s.as_str())
        .unwrap_or_default()[1..config
        .get("shellToRunShellCommands")
        .map(|s| s.as_str())
        .unwrap_or_default()[1..]
        .len()];

    if cmd.trim().is_empty() {
        log("funcs::run_shell_command(): Empty command received, skipping.");
        return;
    }

    log(&format!(
        "funcs::run_shell_command(): Running shell command: {cmd}"
    ));

    match Command::new(shell)
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
        Ok(_) => {}
        Err(e) => log(&format!(
            "funcs::run_shell_command(): Failed to execute command: {e}"
        )),
    }
}

pub fn detect_package_manager() -> String {
    if let Some(val) = load_configs()
        .get("forceDisablePackageManagerCheck")
        .map(String::as_str)
    {
        if val == "false" {
            /* Supported package managers are listed here */
            for pm in ["pacman", "dnf", "apt", "zypper"] {
                if Command::new(pm).output().is_ok() {
                    return pm.to_string();
                }
            }
        }

        log("funcs::detect_package_manager(): No package manager has been detected!");

        return "none".to_string();
    }

    log("funcs::detect_package_manager(): Missing \"forceDisablePackageManagerCheck\" in config!");
    eprintln!("Missing \"forceDisablePackageManagerCheck\" in config!\nTry reloading Rusterminal!");

    "none".to_string()
}

pub fn get_python_version() -> String {
    let output = Command::new("python3")
        .arg("--version")
        .output()
        .expect("failed to execute process");

    let raw_output = if output.stdout.is_empty() {
        String::from_utf8_lossy(&output.stderr).to_string()
    } else {
        String::from_utf8_lossy(&output.stdout).to_string()
    };

    let re = Regex::new(r"\b(\d+\.\d+\.\d+)\b").unwrap();
    let python_version = re
        .captures(&raw_output)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str())
        .unwrap_or("unknown");

    python_version.to_string()
}

pub fn help() {
    let rust_version = &version();
    let python_version = &get_python_version();

    println!("Rusterminal {VERSION} (Rustc {rust_version}) (Python {python_version})");
    println!("Type \"rusterminal\" to get started.\n")
}
