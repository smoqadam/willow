use crate::actions::Action;
use crate::engine::EngineCtx;
use crate::template::Template;
use log::{error, info};
use serde_derive::Deserialize;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Deserialize, Debug, Clone)]
pub struct ExecActionConfig {
    pub command: String,
    #[serde(default)]
    pub args: Option<Vec<String>>,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(default)]
    pub env: Option<Vec<(String, String)>>,
    #[serde(default)]
    pub timeout_secs: Option<u64>,
}

pub struct ExecAction {
    cfg: ExecActionConfig,
}

impl ExecAction {
    pub fn new(cfg: ExecActionConfig) -> Self {
        ExecAction { cfg }
    }
}

impl Action for ExecAction {
    fn run(&self, path: &Path, _ctx: &EngineCtx) -> anyhow::Result<()> {
        let t = |s: &str| Template::new(s.to_string()).render(path);
        let cmd_str = t(&self.cfg.command);
        let mut cmd = Command::new(&cmd_str);
        if let Some(args) = &self.cfg.args {
            let rendered: Vec<String> = args.iter().map(|a| t(a)).collect();
            cmd.args(rendered);
        }
        if let Some(cwd) = &self.cfg.cwd {
            cmd.current_dir(t(cwd));
        }
        if let Some(envs) = &self.cfg.env {
            for (k, v) in envs {
                cmd.env(t(k), t(v));
            }
        }
        info!("exec.start path={} cmd={}", path.display(), cmd_str);
        let timeout = self.cfg.timeout_secs.map(Duration::from_secs);
        let start = Instant::now();
        let mut child = cmd.spawn()?;
        let status = if let Some(to) = timeout {
            loop {
                if let Some(st) = child.try_wait()? {
                    break st;
                }
                if start.elapsed() >= to {
                    child.kill()?;
                    anyhow::bail!("exec timeout after {:?}", to);
                }
                thread::sleep(Duration::from_millis(50));
            }
        } else {
            child.wait()?
        };
        if status.success() {
            info!(
                "exec.ok path={} exit={} elapsed_ms={}",
                path.display(),
                status.code().unwrap_or_default(),
                start.elapsed().as_millis()
            );
            Ok(())
        } else {
            error!(
                "exec.fail path={} exit={} elapsed_ms={}",
                path.display(),
                status.code().unwrap_or_default(),
                start.elapsed().as_millis()
            );
            anyhow::bail!("exec failed: {:?}", status)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::EngineCtx;
    use crate::fs::StdFs;
    use std::path::PathBuf;
    use std::sync::{Arc, atomic::AtomicBool};

    fn ctx() -> EngineCtx {
        EngineCtx::new(Arc::new(StdFs::new()), Arc::new(AtomicBool::new(false)))
    }

    #[test]
    fn exec_echo_succeeds() {
        let action = ExecAction::new(ExecActionConfig {
            command: "/bin/echo".into(),
            args: Some(vec!["Hello".into(), "{filename}".into()]),
            cwd: None,
            env: None,
            timeout_secs: Some(3),
        });
        let path = PathBuf::from("/tmp/file.txt");
        action.run(&path, &ctx()).unwrap();
    }

    #[test]
    fn exec_timeout_errors() {
        let action = ExecAction::new(ExecActionConfig {
            command: "/bin/sleep".into(),
            args: Some(vec!["2".into()]),
            cwd: None,
            env: None,
            timeout_secs: Some(0),
        });
        let path = PathBuf::from("/tmp/file.txt");
        let res = action.run(&path, &ctx());
        assert!(res.is_err());
    }
}
