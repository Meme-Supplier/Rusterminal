use std::process::Command;
use std::{env, fs};
use chrono::Local;

use hostname::get;

use crate::funcs::load_configs;
use crate::logger::log;

// System //

pub const OS: &str = env::consts::OS;

pub fn get_hostname() -> String {
    let hostname = get()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "unknown".to_string());
    hostname
}

pub fn get_package_manager() -> String {
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

pub fn get_distro_name() -> String {
    if let Ok(contents) = fs::read_to_string("/etc/os-release") {
        for line in contents.lines() {
            if line.starts_with("PRETTY_NAME=") {
                return line
                    .trim_start_matches("PRETTY_NAME=")
                    .trim_matches('"')
                    .to_string();
            }
        }
    }
    "unknown".to_string()
}

pub fn get_shell() -> String {
    env::var("SHELL").unwrap_or_else(|_| "unknown".to_string())
}

// Desktop //

pub fn get_desktop_environment() -> String {
    env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| env::var("DESKTOP_SESSION"))
        .or_else(|_| env::var("XDG_SESSION_TYPE"))
        .unwrap_or_else(|_| "unknown".to_string())
}

pub fn get_display_protocol() -> String {
    env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "unknown".to_string())
}

pub fn get_window_manager() -> Option<String> {
    let output = Command::new("ps").arg("-eo").arg("comm").output().ok()?;

    let ps_output = String::from_utf8_lossy(&output.stdout);

    let known_wms = [
        "kwin", "mutter", "sway", "i3", "xmonad", "openbox", "xfwm4", "marco", "fluxbox", "bspwm",
    ];

    for line in ps_output.lines() {
        for wm in &known_wms {
            if line.to_lowercase().contains(wm) {
                return Some(wm.to_string());
            }
        }
    }

    None
}

// User //

pub fn get_cwd() -> String  {
    let cwd = env::current_dir()
        .ok()
        .and_then(|p| p.to_str().map(String::from))
        .unwrap_or_else(|| "~".to_string());
    cwd
}

pub fn get_home() -> String {
    env::var("HOME").expect("Failed to get HOME directory")
}

pub fn get_user() -> String {
    env::var("USER").unwrap_or_else(|_| "unknown".to_string())
}

// Time //

pub fn get_time_format() -> String {
    let config = load_configs();

    let format = &config
        .get("logTimeAndDateFormat")
        .map(|s| s.as_str())
        .unwrap_or_default()[1..config
        .get("logTimeAndDateFormat")
        .map(|s| s.as_str())
        .unwrap_or_default()[1..]
        .len()];

    format.to_string()
}

pub fn get_time() -> String {
    let format = &get_time_format();
    Local::now().format(format).to_string()
}