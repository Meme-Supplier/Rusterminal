use std::fmt;

use crate::s_vars;

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
        desktop_environment: s_vars::get_desktop_environment(),
        window_manager: s_vars::get_window_manager().unwrap_or_else(|| "unknown".to_string()),
        display_protocol: s_vars::get_display_protocol(),
        distro: s_vars::get_distro_name(),
        shell: s_vars::get_shell(),
    }
}
