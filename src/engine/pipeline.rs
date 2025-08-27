use crate::models::{EventInfo, RuntimeRule};
use std::sync::{mpsc::{Receiver, Sender}, Arc};

    /// Stage trait for pipeline stages that filter and transform events
pub trait Stage: Send + Sync {
    fn run(
        &mut self,
        rx: Receiver<(EventInfo, Vec<Arc<RuntimeRule>>)>,
        tx: Sender<(EventInfo, Vec<Arc<RuntimeRule>>)>,
    );
}

/// Sink trait for final stages that consume events without forwarding
pub trait Sink: Send + Sync {
    fn run(&mut self, rx: Receiver<(EventInfo, Vec<Arc<RuntimeRule>>)>);
}
