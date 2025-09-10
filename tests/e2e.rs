use std::fs as stdfs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, Once, OnceLock};
use std::thread;
use std::io::Write as _;
use std::time::{Duration, SystemTime};

use log::{LevelFilter, Log, Metadata, Record};

use willow::engine::{self, ActionSink, IoFilterStage, PipelineBuilder, StabilityStage, StaticFilterStage};
use willow::engine::EngineCtx;
use willow::fs::{Fs, StdFs};
use willow::models::{Event, EventInfo, RuntimeRule};

struct TestLogger;
static INIT_LOGGER: Once = Once::new();
static LOGS: OnceLock<Arc<Mutex<Vec<String>>>> = OnceLock::new();

impl Log for TestLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool { true }
    fn log(&self, record: &Record) { if let Some(buf) = LOGS.get() { let mut g = buf.lock().unwrap(); g.push(format!("{}", record.args())); } }
    fn flush(&self) {}
}

fn init_test_logger() -> Arc<Mutex<Vec<String>>> {
    INIT_LOGGER.call_once(|| {
        let buf = Arc::new(Mutex::new(Vec::new()));
        let _ = LOGS.set(buf.clone());
        let _ = log::set_boxed_logger(Box::new(TestLogger));
        log::set_max_level(LevelFilter::Info);
    });
    LOGS.get().unwrap().clone()
}

fn unique_test_dir(name: &str) -> PathBuf {
    let base = PathBuf::from("target");
    let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    let dir = base.join(format!("e2e_{}_{}", name, ts));
    stdfs::create_dir_all(&dir).unwrap();
    dir
}

fn wait_for(predicate: impl Fn() -> bool, timeout: Duration) -> bool {
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        if predicate() { return true; }
        thread::sleep(Duration::from_millis(100));
    }
    false
}

#[test]
fn e2e_move_with_static_and_io_conditions_and_logs() {
    let logs = init_test_logger();

    // Prepare temp dirs
    let dir = unique_test_dir("move");
    let dest = dir.join("out");
    stdfs::create_dir_all(&dest).unwrap();

    // Build pipeline directly; rule assembled below

    // Build pipeline directly to avoid relying on OS watchers (flaky in CI)
    let ctx = Arc::new(EngineCtx::new(Arc::new(StdFs::new()) as Arc<dyn Fs>, Arc::new(std::sync::atomic::AtomicBool::new(false))));
    let builder = PipelineBuilder::new(ctx.clone(), ActionSink::new())
        .add_stage(StaticFilterStage::new())
        .add_stage(StabilityStage::new())
        .add_stage(IoFilterStage::new());
    let (ingress, handles) = builder.build();

    // Create a matching file
    let input_rel = dir.join("image.jpg");
    stdfs::write(&input_rel, b"abc").unwrap();
    let input = stdfs::canonicalize(&input_rel).unwrap();
    thread::sleep(Duration::from_millis(150));
    stdfs::OpenOptions::new().append(true).open(&input).unwrap().write_all(b"!").unwrap();

    // Build runtime rule matching Any + extension + size; actions log + move
    let rule = Arc::new(RuntimeRule {
        event: Event::Any,
        conditions: vec![
            willow::condition::ConditionConfig::Extension { value: "jpg".into() }.into_condition().unwrap(),
            willow::condition::ConditionConfig::SizeGt { value: 1 }.into_condition().unwrap(),
        ],
        actions: vec![
            willow::action::ActionConfig::Log { message: "processing {filename}".into() }.into_action(),
            willow::action::ActionConfig::Move { destination: dest.to_string_lossy().to_string() + "/" }.into_action(),
        ],
    });

    // Simulate watcher: send created, then modified
    ingress.send(engine::PipelineMsg { event: EventInfo { path: input.clone(), event: Event::Created, meta: None }, rules: vec![rule.clone()] }).unwrap();
    thread::sleep(Duration::from_millis(200));
    ingress.send(engine::PipelineMsg { event: EventInfo { path: input.clone(), event: Event::Modified, meta: None }, rules: vec![rule.clone()] }).unwrap();

    // Expect it to be moved after stability window and IO filter
    let moved_path = dest.join("image.jpg");
    let ok = wait_for(|| moved_path.exists(), Duration::from_secs(8));
    if !ok {
        let log_dump = { logs.lock().unwrap().clone().join("\n") };
        panic!("expected file to be moved to {}\nlogs:\n{}", moved_path.display(), log_dump);
    }

    // Check logs
    let log_dump = { logs.lock().unwrap().clone().join("\n") };
    assert!(log_dump.contains("processing image.jpg"), "missing custom log in: {}", log_dump);
    assert!(log_dump.contains("moved") && log_dump.contains("image.jpg"), "missing move log in: {}", log_dump);

    drop(ingress);
    for h in handles { let _ = h.join(); }
}
