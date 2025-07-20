# Rusterminal Changelog

## v0.1.1
- Quick fixes
- When you enter a command needing multiple syntaxes, it now shows usage info

## v0.1.2
- Fixed issues with interactive commands
- Misc big fixes
- Cleaned up code
- Added `"ver"` command
- Added `"web"` command
- Removed pointless code

## v0.1.3
- Added `"reload"` command
- Code cleanup & optimizations
- Added `"uptime"` command
- Added `"update"` command
- Added ability to chain multiple commands using `&&`
- Added `"wait"` command
- More info added to `"ver"` command
- Bug fixes

## v0.1.35
- Bug fixes on `"update"` command
- Added ability to install Rusterminal

## v0.1.4
- Added `"uninstall"` command
- Bug fixes
- Added commands: `"ls"`, `"del"`, `"ping"`, `"title"`, `"edit"`, `"copy"`, `"rmtitle"`

## v0.1.5
- Moved functions to `funcs.rs`
- Fixed `"uninstall"` command
- Bug fixes
- Added `"upgrade"` command

## v0.1.6
- Better Fedora compatibility
- Code optimizations
- Added `"clean"`, `"in"`, `"newdir"` commands
- Removed `"reload"` command
- Added prompt to launch Rusterminal after upgrading

## v0.1.7
- Fixed support for Apt and Dnf package managers (not just Pacman)
- Moved `"cmds"` command to `/src/cmds.rs`
- Fixed typos
- `"cmds"` command now lists commands alphabetically
- Code optimizations
- Renamed `"sh"` command to `"run"`

## v0.1.8
- Added `"credits"` command
- Added `"legacy"` command
- Fixed user input bugs
- Bug fixes
- Added `"fmtdsk"` command

## v0.1.9
- Bug fixes
- Added missing dependencies install via `install.sh`
- Code optimizations
- Improved `"xray"` command: select all Rusterminal files

## v0.2.0
- Code optimizations
- Working command history
- Added `"settings"` command
- Configure Rusterminal with `"settings"` command
- Bug fixes

## v0.2.1
- Added `"build"` command
- New config options:
  - `disableUpdateCMD`
  - `useHostnameInPrompt`
  - `helpFuncOnStartup`
  - `showReminderToSaveSettings`
  - `forceUniversalOScompatability`
  - `rusterminalBuildPath`
- Bug fixes
- Code optimizations

## v0.2.2
- Moved config loading to `funcs.rs`
- Code optimizations
- Customizable terminal prompt
- New config options:
  - `showSystemInformationInVerCMD`
  - `considerYayAsAPackageManager`
  - `promptType`
  - `customPrompt`
- Improved `"help"` command
- Removed `loadconfigs.rs` (moved to `funcs.rs`)

## v0.2.3
- Fixed typo
- Added ability to register Rusterminal as default shell
- Code optimizations
- Bug fixes

## v0.2.4
- Bug fixes
- Improved `"echo"` command
- Code optimizations
- Moved `settings.conf` to `~/.config/rusterminal`
- Added `forceDisablePackageManagerCheck` setting

## v0.2.5
- Code optimizations
- Added `parted` dependency
- Removed `"uptime"` and `"legacy"` commands (considered useless)

## v0.2.6
- Added `gcc` dependency
- Code optimizations
- Added option to exit `"xray.py"`

## v0.2.7
- Code optimizations
- Added config options:
  - `considerParuAsAPackageManager`
  - `clearScreenOnStartup`
- Fixed typos
- Added `"man"` command
- `"help"` command now shows Python version

## v0.2.8
- Rewrote `"xray.py"` in Rust
- Added `cleanCompileOnStartup` config
- Code optimizations
- Tweaks to `"clean()"`

## v0.2.9
- Major code optimizations
- Installer files moved to `~/rusterminal/installer`
- Bug fixes
- Tweaked `"update"` command

## v0.3.0
- Added `"rusterminal"` command
- Moved some commands under `"rusterminal"` command
- Code optimizations
- Removed `"man"` command

## v0.3.1
- Overhauled `"rusterminal upgrade"` command
- Bug fixes
- Code optimizations

## v0.3.2
- Added `"script"` subcommand in `"rusterminal"`
- Bug fixes
- Code optimizations
- Added logging for troubleshooting
- Updated dependencies
- Added `"logs"` and `"dellogs"` subcommands in `"rusterminal"`
- Fixed typos
- Added utilities for development
- `"clean"` command now cleans system more
- `"clean"` is now a subcommand of `"rusterminal"`

## v0.3.3
- Code cleanup for better readability
- Added `"reboot"` command to restart system
- Bug fixes
- Code optimizations
- Rewrote system info file `ver.py` in Rust (`sysinfo.rs`)
- Removed `showSystemInformationInVerCMD` config
- Added Flatpak support
- New configs:
  - `considerFlatpakAsAPackageManager`
  - `rusterminalBuildCommand`
- Added `"reset"` command in `"rusterminal"` to reset settings
- Added `"rename"` command
- Quality of life improvements
- Removed unused dependencies
- `"update"` command now updates Rustup

## v0.3.4
- Code optimizations
- Removed `disableUpdateCMD` setting (unused)
- Improved `settings.conf` formatting
- Added OpenSuse/Zypper package manager support
- Bug fixes
- Default shell prompt format changed to `hostname@user$~:`
- Disabled kicking out users with unsupported package managers
- Added custom update commands via configs:
  - `customUpdateCommand`
  - `enableCustomUpdateCommand`
- Added `shellToRunShellCommands` config
- `(rusterminal) clean` command cleans Rusterminal installation
- Removed `showReminderToSaveSettings` config (unused)

## v0.3.5
- Code optimizations
- Proper command history stored at `$HOME/.rusterminal_history`
- Added `"history"` subcommand to `"rusterminal"`
- New configs:
  - `displayMemoryUsage`
  - `logTimeAndDateFormat`
- Bug fixes
- Uninstall now asks to remove history & configs
- Overhauled custom prompt setting
- Added `"cd"` command
- Added `startingDir` config
- Improved `"ls"` command
- Installer prompts to edit configs before first run
- Version retrieved from `Cargo.toml`
- `"help"` now shows Rust edition next to version
- Overhauled `format.sh`

###

### ***(Changelog format change)***

###

## v0.3.6

### beta1
- Removed useless code
- Bug fixes
- Added `"changelog"` subcommand in `"rusterminal"`
- `"changes.txt"` has been renamed to `"changes.md"` and given an overhaul

### beta2
- The subcommand `"ver"` in `"rusterminal"` now shows Rust's edition
- Compiling Rusterminal is now part of the installation process
- Command arguments
  - Example: `rusterminal <arg>`
- `"argPrefixer"` configuration

### beta3
- Bug fixes
- `"outputLoggedMessages"` configuration
- configuration `"clearScreenOnStartup"` is now disabled by default
- `"ls"` command now lists everything in that directory (including hidden items)

### beta4
- Removed config `displayMemoryUsage`
- New file `s_vars.rs`: Holds static variables, cleans up code
- File structure has been changed

### beta5
- 

### rc1
- 

### rc2
- 

### Final changes
- 
