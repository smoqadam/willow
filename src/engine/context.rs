use std::sync::Arc;

use crate::fs::Fs;

pub struct EngineCtx {
    pub fs: Arc<dyn Fs>,
}

impl EngineCtx {
    pub fn new(fs: Arc<dyn Fs>) -> Self {
        Self { fs }
    }
}

