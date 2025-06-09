#!/usr/bin/env rust-script

use std::process::Command;
use std::{env, fmt, fs};

fn get_desktop_environment() -> String {
    env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| env::var("DESKTOP_SESSION"))
        .or_else(|_| env::var("XDG_SESSION_TYPE"))
        .unwrap_or_else(|_| "unknown".to_string())
}

fn detect_display_protocol() -> String {
    env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "unknown".to_string())
}

fn detect_window_manager() -> Option<String> {
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

fn get_distro_name() -> String {
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

fn get_shell() -> String {
    env::var("SHELL").unwrap_or_else(|_| "unknown".to_string())
}

pub struct SystemInformation {
    pub desktop_environment: String,
    pub window_manager: String,
    pub display_protocol: String,
    pub distro: String,
    pub shell: String,
}

impl fmt::Display for SystemInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Desktop Environment: {}", self.desktop_environment)?;
        writeln!(f, "Window Manager: {}", self.window_manager)?;
        writeln!(f, "Display Protocol: {}", self.display_protocol)?;
        writeln!(f, "Distro: {}", self.distro)?;
        writeln!(f, "Shell: {}", self.shell)
    }
}

pub fn get_system_info() -> SystemInformation {
    SystemInformation {
        desktop_environment: get_desktop_environment(),
        window_manager: detect_window_manager().unwrap_or_else(|| "unknown".to_string()),
        display_protocol: detect_display_protocol(),
        distro: get_distro_name(),
        shell: get_shell(),
    }
}
