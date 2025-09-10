use crate::actions::Action;
use crate::engine::EngineCtx;
use crate::template::Template;
use log::{debug, info};
use std::path::Path;

pub struct RenameAction {
    template: String,
}

impl RenameAction {
    pub fn new(template: String) -> Self {
        RenameAction { template }
    }
}

impl Action for RenameAction {
    fn run(&self, path: &Path, ctx: &EngineCtx) -> anyhow::Result<()> {
        debug!(
            "Starting rename action for path: {:?} with template: {}",
            path, self.template
        );

        let template = Template::new(self.template.clone());
        let rendered_name = template.render(path);

        let parent_dir = path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("No parent directory for path {:?}", path))?;
        let new_path = parent_dir.join(&rendered_name);

        info!("Renaming {:?} to {:?}", path, new_path);
        ctx.fs.rename(path, &new_path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::Fs;
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, atomic::AtomicBool};
    use std::{fs, io};

    #[derive(Default)]
    struct MockFs {
        pub renames: std::sync::Mutex<Vec<(PathBuf, PathBuf)>>,
    }

    impl Fs for MockFs {
        fn metadata(&self, _path: &Path) -> io::Result<fs::Metadata> {
            // Not used in these tests
            Err(io::Error::new(io::ErrorKind::Other, "not used"))
        }
        fn create_dir_all(&self, _path: &Path) -> io::Result<()> {
            Ok(())
        }
        fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
            self.renames
                .lock()
                .unwrap()
                .push((from.to_path_buf(), to.to_path_buf()));
            Ok(())
        }
        fn exists(&self, _path: &Path) -> bool {
            false
        }
        fn read_to_string(&self, _path: &Path) -> io::Result<String> {
            Err(io::Error::new(io::ErrorKind::Other, "not used"))
        }
    }

    #[test]
    fn renames_with_template_in_same_dir() {
        let fs = Arc::new(MockFs::default());
        let ctx = EngineCtx::new(fs.clone(), Arc::new(AtomicBool::new(false)));
        let action = RenameAction::new("{name}_renamed.{ext}".to_string());
        let path = PathBuf::from("/tmp/dir/file.txt");

        action.run(&path, &ctx).unwrap();

        let renames = fs.renames.lock().unwrap().clone();
        assert_eq!(renames.len(), 1);
        assert_eq!(renames[0].0, PathBuf::from("/tmp/dir/file.txt"));
        assert_eq!(renames[0].1, PathBuf::from("/tmp/dir/file_renamed.txt"));
    }
}
