use crate::actions::Action;
use crate::engine::EngineCtx;
use crate::template::Template;
use log::{debug, error, info};
use serde_derive::Deserialize;
use std::path::Path;

pub struct MoveAction {
    destination: String,
    overwrite: MoveOverwritePolicy,
}

impl MoveAction {
    pub fn new(destination: String, overwrite: Option<MoveOverwritePolicy>) -> Self {
        MoveAction {
            destination,
            overwrite: overwrite.unwrap_or_default(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum MoveOverwritePolicy {
    #[default]
    Error,
    Skip,
    Overwrite,
    Suffix,
}

impl Action for MoveAction {
    fn run(&self, path: &Path, ctx: &EngineCtx) -> anyhow::Result<()> {
        debug!("Starting move action for path: {path:?}");

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

        debug!("Moving {path:?} to {final_dest_path:?}");

        // create parent directory if it doesn't exist
        if let Some(parent) = final_dest_path.parent() {
            ctx.fs.create_dir_all(parent)?;
        }

        // handle overwrite policy
        let mut target = final_dest_path.clone();
        if ctx.fs.exists(&target) {
            match self.overwrite {
                MoveOverwritePolicy::Error => {
                    return Err(anyhow::anyhow!(
                        "Destination exists and overwrite policy=error: {:?}",
                        target
                    ));
                }
                MoveOverwritePolicy::Skip => {
                    info!("destination exists, skipping move to {target:?}");
                    return Ok(());
                }
                MoveOverwritePolicy::Suffix => {
                    let parent = target.parent().unwrap_or_else(|| Path::new(""));
                    let stem = target.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                    let ext = target.extension().and_then(|e| e.to_str()).unwrap_or("");
                    let mut i = 1u32;
                    loop {
                        let candidate_name = if ext.is_empty() {
                            format!("{stem}_{i}")
                        } else {
                            format!("{stem}_{i}.{ext}")
                        };
                        let candidate = parent.join(candidate_name);
                        if !ctx.fs.exists(&candidate) {
                            target = candidate;
                            break;
                        }
                        i += 1;
                        if i > 10_000 {
                            return Err(anyhow::anyhow!(
                                "Too many collisions for {:?}",
                                final_dest_path
                            ));
                        }
                    }
                }
                MoveOverwritePolicy::Overwrite => { /* proceed */ }
            }
        }

        ctx.fs.rename(path, &target).map_err(|e| {
            error!("Move action error: {e:?}");
            anyhow::anyhow!("Failed to move {:?} to {:?}: {}", path, target, e)
        })?;
        info!("moved {path:?} to {target:?}");
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
        pub existing: std::sync::Mutex<Vec<PathBuf>>,
    }

    impl Fs for MockFs {
        fn metadata(&self, _path: &Path) -> io::Result<fs::Metadata> {
            Err(io::Error::other("not used"))
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
        fn exists(&self, path: &Path) -> bool {
            self.existing.lock().unwrap().contains(&path.to_path_buf())
        }
        fn read_to_string(&self, _path: &Path) -> io::Result<String> {
            Err(io::Error::other("not used"))
        }
    }

    #[test]
    fn moves_into_directory_when_destination_ends_with_slash() {
        let fs = Arc::new(MockFs::default());
        let ctx = EngineCtx::new(fs.clone(), Arc::new(AtomicBool::new(false)));
        let action = MoveAction::new("/dest/dir/".to_string(), None);
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
        let action = MoveAction::new("/dest/final/name.txt".to_string(), None);
        let path = PathBuf::from("/src/path/file.txt");

        action.run(&path, &ctx).unwrap();

        let dirs = fs.created_dirs.lock().unwrap().clone();
        let renames = fs.renames.lock().unwrap().clone();
        assert_eq!(dirs, vec![PathBuf::from("/dest/final")]);
        assert_eq!(renames.len(), 1);
        assert_eq!(renames[0].0, PathBuf::from("/src/path/file.txt"));
        assert_eq!(renames[0].1, PathBuf::from("/dest/final/name.txt"));
    }

    #[test]
    fn skip_when_destination_exists() {
        let fs = Arc::new(MockFs {
            existing: std::sync::Mutex::new(vec![PathBuf::from("/dest/dir/file.txt")]),
            ..Default::default()
        });
        let ctx = EngineCtx::new(fs.clone(), Arc::new(AtomicBool::new(false)));
        let action = MoveAction::new("/dest/dir/".to_string(), Some(MoveOverwritePolicy::Skip));
        let path = PathBuf::from("/src/path/file.txt");
        action.run(&path, &ctx).unwrap();
        assert!(fs.renames.lock().unwrap().is_empty());
    }

    #[test]
    fn suffix_when_destination_exists() {
        let existing = vec![
            PathBuf::from("/dest/dir/file.txt"),
            PathBuf::from("/dest/dir/file_1.txt"),
        ];
        let fs = Arc::new(MockFs {
            existing: std::sync::Mutex::new(existing),
            ..Default::default()
        });
        let ctx = EngineCtx::new(fs.clone(), Arc::new(AtomicBool::new(false)));
        let action = MoveAction::new("/dest/dir/".to_string(), Some(MoveOverwritePolicy::Suffix));
        let path = PathBuf::from("/src/path/file.txt");
        action.run(&path, &ctx).unwrap();
        let renames = fs.renames.lock().unwrap().clone();
        assert_eq!(renames.len(), 1);
        assert_eq!(renames[0].1, PathBuf::from("/dest/dir/file_2.txt"));
    }

    #[test]
    fn error_when_destination_exists() {
        let fs = Arc::new(MockFs {
            existing: std::sync::Mutex::new(vec![PathBuf::from("/dest/dir/file.txt")]),
            ..Default::default()
        });
        let ctx = EngineCtx::new(fs.clone(), Arc::new(AtomicBool::new(false)));
        let action = MoveAction::new("/dest/dir/".to_string(), Some(MoveOverwritePolicy::Error));
        let path = PathBuf::from("/src/path/file.txt");
        let res = action.run(&path, &ctx);
        assert!(res.is_err());
        assert!(fs.renames.lock().unwrap().is_empty());
    }

    #[test]
    fn overwrite_when_destination_exists() {
        let fs = Arc::new(MockFs {
            existing: std::sync::Mutex::new(vec![PathBuf::from("/dest/dir/file.txt")]),
            ..Default::default()
        });
        let ctx = EngineCtx::new(fs.clone(), Arc::new(AtomicBool::new(false)));
        let action = MoveAction::new(
            "/dest/dir/".to_string(),
            Some(MoveOverwritePolicy::Overwrite),
        );
        let path = PathBuf::from("/src/path/file.txt");
        action.run(&path, &ctx).unwrap();
        let renames = fs.renames.lock().unwrap().clone();
        assert_eq!(renames.len(), 1);
        assert_eq!(renames[0].1, PathBuf::from("/dest/dir/file.txt"));
    }
}
