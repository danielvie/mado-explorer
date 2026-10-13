//! Launch captured directories using structured process arguments. No path is
//! interpolated into a shell command; PowerShell receives only fixed switches.
use std::{
    ffi::OsString,
    os::windows::process::CommandExt,
    path::{Path, PathBuf},
    process::{Child, Command},
};
#[derive(Clone, Copy, Debug)]
pub enum ExternalTool {
    Explorer,
    Terminal,
}
#[derive(Clone, Debug)]
pub struct LaunchSpec {
    pub executable: PathBuf,
    pub arguments: Vec<OsString>,
    pub directory: PathBuf,
}
pub fn launch_spec(tool: ExternalTool, path: &Path) -> Result<LaunchSpec, String> {
    if !path.is_absolute() || !path.is_dir() {
        return Err("External tools require an existing absolute directory".into());
    }
    let windows = std::env::var_os("SystemRoot")
        .map(PathBuf::from)
        .ok_or("Windows installation directory unavailable")?;
    if !windows.is_absolute() {
        return Err("Windows installation directory is invalid".into());
    }
    let (executable, arguments) = match tool {
        ExternalTool::Explorer => (
            windows.join("explorer.exe"),
            vec![path.as_os_str().to_owned()],
        ),
        ExternalTool::Terminal => (
            windows.join("System32/WindowsPowerShell/v1.0/powershell.exe"),
            vec![
                OsString::from("-NoLogo"),
                OsString::from("-NoProfile"),
                OsString::from("-NoExit"),
            ],
        ),
    };
    if !executable.is_file() {
        return Err(format!(
            "External executable unavailable: {}",
            executable.display()
        ));
    }
    Ok(LaunchSpec {
        executable,
        arguments,
        directory: path.to_owned(),
    })
}
fn launch(tool: ExternalTool, path: &Path) -> Result<Child, String> {
    let spec = launch_spec(tool, path)?;
    let mut command = Command::new(spec.executable);
    command.args(spec.arguments).current_dir(spec.directory);
    if matches!(tool, ExternalTool::Terminal) {
        command.creation_flags(0x00000010);
    } // CREATE_NEW_CONSOLE: explicitly invoked interactive terminal.
    command
        .spawn()
        .map_err(|e| format!("Could not launch external tool: {e}"))
}
pub fn open_terminal(path: &Path) -> Result<Child, String> {
    launch(ExternalTool::Terminal, path)
}
pub fn open_explorer(path: &Path) -> Result<Child, String> {
    launch(ExternalTool::Explorer, path)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn external_tools_reject_relative() {
        assert!(launch_spec(ExternalTool::Terminal, Path::new(".")).is_err());
    }
    #[test]
    fn external_tools_paths_never_become_commands() {
        let root = std::env::temp_dir().join(format!(
            "wayfarer external & ' fixture {}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&root).unwrap();
        let terminal = launch_spec(ExternalTool::Terminal, &root).unwrap();
        assert_eq!(terminal.directory, root);
        assert_eq!(
            terminal.arguments,
            vec![
                OsString::from("-NoLogo"),
                OsString::from("-NoProfile"),
                OsString::from("-NoExit")
            ]
        );
        let explorer = launch_spec(ExternalTool::Explorer, &root).unwrap();
        assert_eq!(explorer.arguments, vec![root.as_os_str().to_owned()]);
        std::fs::remove_dir(root).unwrap();
    }
}
