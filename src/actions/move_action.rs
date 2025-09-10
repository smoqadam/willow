use crate::actions::Action;
use crate::engine::EngineCtx;
use crate::template::Template;
use log::{debug, error, info};
use std::path::Path;

pub struct MoveAction {
    destination: String,
}

impl MoveAction {
    pub fn new(destination: String) -> Self {
        MoveAction { destination }
    }
}

impl Action for MoveAction {
    fn run(&self, path: &Path, ctx: &EngineCtx) -> anyhow::Result<()> {
        debug!("Starting move action for path: {:?}", path);

        let template = Template::new(self.destination.clone());
        let rendered_destination = template.render(path);

        let dest_path = Path::new(&rendered_destination);

        // append the filename if the rendered destination is a directory
        let final_dest_path =
            if rendered_destination.ends_with('/') || rendered_destination.ends_with('\\') {
                let filename = path
                    .file_name()
                    .ok_or_else(|| anyhow::anyhow!("No filename in path {:?}", path))?;
                dest_path.join(filename)
            } else {
                dest_path.to_path_buf()
            };

        debug!("Moving {:?} to {:?}", path, final_dest_path);

        // create parent directory if it doesn't exist
        if let Some(parent) = final_dest_path.parent() {
            ctx.fs.create_dir_all(parent)?;
        }

        // todo: check for overwrite
        ctx.fs.rename(path, &final_dest_path).map_err(|e| {
            error!("Move action error: {:?}", e);
            anyhow::anyhow!("Failed to move {:?} to {:?}: {}", path, final_dest_path, e)
        })?;
        info!("moved {:?} to {:?}", path, final_dest_path);
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
        pub created_dirs: std::sync::Mutex<Vec<PathBuf>>,
    }

    impl Fs for MockFs {
        fn metadata(&self, _path: &Path) -> io::Result<fs::Metadata> {
            Err(io::Error::new(io::ErrorKind::Other, "not used"))
        }
        fn create_dir_all(&self, path: &Path) -> io::Result<()> {
            self.created_dirs.lock().unwrap().push(path.to_path_buf());
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
    fn moves_into_directory_when_destination_ends_with_slash() {
        let fs = Arc::new(MockFs::default());
        let ctx = EngineCtx::new(fs.clone(), Arc::new(AtomicBool::new(false)));
        let action = MoveAction::new("/dest/dir/".to_string());
        let path = PathBuf::from("/src/path/file.txt");

        action.run(&path, &ctx).unwrap();

        let dirs = fs.created_dirs.lock().unwrap().clone();
        let renames = fs.renames.lock().unwrap().clone();
        assert_eq!(dirs, vec![PathBuf::from("/dest/dir")]);
        assert_eq!(renames.len(), 1);
        assert_eq!(renames[0].0, PathBuf::from("/src/path/file.txt"));
        assert_eq!(renames[0].1, PathBuf::from("/dest/dir/file.txt"));
    }

    #[test]
    fn moves_to_exact_destination_when_path_given() {
        let fs = Arc::new(MockFs::default());
        let ctx = EngineCtx::new(fs.clone(), Arc::new(AtomicBool::new(false)));
        let action = MoveAction::new("/dest/final/name.txt".to_string());
        let path = PathBuf::from("/src/path/file.txt");

        action.run(&path, &ctx).unwrap();

        let dirs = fs.created_dirs.lock().unwrap().clone();
        let renames = fs.renames.lock().unwrap().clone();
        assert_eq!(dirs, vec![PathBuf::from("/dest/final")]);
        assert_eq!(renames.len(), 1);
        assert_eq!(renames[0].0, PathBuf::from("/src/path/file.txt"));
        assert_eq!(renames[0].1, PathBuf::from("/dest/final/name.txt"));
    }
}
