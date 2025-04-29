#!/usr/bin/env rust-script
#[cfg(target_os = "linux")]

/* 2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier */

use std::collections::HashMap;
use std::env;
use std::fs;

pub fn load() -> HashMap<String, String> {
    // Get the home directory
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");

    // Read the config file
    let content = fs::read_to_string(format!("{home_dir}/rusterminal/src/settings.conf"))
        .expect("Failed to read config");

    // Create a HashMap to store configurations
    let mut config = HashMap::new();

    // Parse each line
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue; // skip empty lines or comments
        }
        if let Some((key, value)) = line.split_once('=') {
            config.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    config // <--- Return the config HashMap
}
