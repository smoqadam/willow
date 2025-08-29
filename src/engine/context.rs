use std::sync::{Arc, atomic::AtomicBool};

use crate::fs::Fs;

pub struct EngineCtx {
    pub fs: Arc<dyn Fs>,
    pub shutdown: Arc<AtomicBool>,
}

impl EngineCtx {
    pub fn new(fs: Arc<dyn Fs>, shutdown: Arc<AtomicBool>) -> Self {
        Self { fs, shutdown }
    }
}
