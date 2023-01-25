use crate::errors::CliResult;
use anyhow::format_err;
use std::fmt::{Debug, Display};
use std::process::{Command, Output};

pub struct JuliaCommand {
    cmd: Command,
    script: String,
}

impl Debug for JuliaCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.cmd)?;
        write!(f, "{}", self.script)
    }
}

impl JuliaCommand {
    pub fn arg(&mut self, arg: &str) -> &mut Self {
        self.cmd.arg(arg);
        self
    }

    pub fn project(&mut self, project: &str) -> &mut Self {
        self.cmd.arg(format!("--project={}", project));
        self
    }

    pub fn compile(&mut self, option: &str) -> &mut Self {
        self.cmd.arg(format!("--compile={}", option));
        self
    }

    pub fn no_startup_file(&mut self) -> &mut Self {
        self.cmd.arg("--startup-file=no");
        self
    }

    pub fn color(&mut self) -> &mut Self {
        self.cmd.arg("--color=yes");
        self
    }

    pub fn output(&mut self) -> Result<Output, std::io::Error> {
        self.cmd.arg(format!("-e {}", self.script)).output()
    }

    pub fn status(&mut self) -> Result<std::process::ExitStatus, std::io::Error> {
        self.cmd.arg(format!("-e {}", self.script)).status()
    }
}

pub trait Julia {
    fn as_julia_command(&self) -> JuliaCommand;

    fn julia_exec_cmd(&self, project: &str) -> JuliaCommand {
        let mut cmd = self.as_julia_command();
        cmd.project(project)
            .no_startup_file()
            .color()
            .compile("min");
        cmd
    }

    fn julia_exec_project_quiet(&self, project: &str) -> CliResult {
        let p = self.julia_exec_cmd(project).output()?;

        if p.status.success() {
            Ok(())
        } else {
            Err(format_err!("Julia command failed").into())
        }
    }

    fn julia_exec_project(&self, project: &str) -> CliResult {
        let p = self.julia_exec_cmd(project).status()?;

        if p.success() {
            Ok(())
        } else {
            Err(format_err!("Julia command failed").into())
        }
    }

    fn julia_exec(&self) -> CliResult {
        self.julia_exec_project("@.")
    }

    fn julia_exec_quiet(&self) -> CliResult {
        self.julia_exec_project_quiet("@.")
    }
}

impl<T: Display> Julia for T {
    fn as_julia_command(&self) -> JuliaCommand {
        let script = self.to_string();
        let cmd = Command::new("julia");
        JuliaCommand { cmd, script }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn test_julia_command() {
        let cmd = "using Pkg; Pkg.add(\"Foo\")".as_julia_command();
        assert_eq!(cmd.cmd.get_program(), "julia");
        let args: Vec<&OsStr> = cmd.cmd.get_args().collect();
        assert_eq!(args.is_empty(), true);
        assert_eq!(cmd.script, "using Pkg; Pkg.add(\"Foo\")");

        let mut cmd = "using Pkg; Pkg.add(\"Foo\")".as_julia_command();
        cmd.project("Foo").arg("Bar").arg("Baz");
        let args: Vec<&OsStr> = cmd.cmd.get_args().collect();
        assert_eq!(args.len(), 3);
        assert_eq!(args[0], "--project=Foo");
        assert_eq!(args[1], "Bar");
        assert_eq!(args[2], "Baz");
    }
}
