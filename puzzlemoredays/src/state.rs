use crate::puzzle::config::Target;
use crate::puzzle::PuzzleConfig;
use crate::solver::SolverCallId;
use once_cell::sync::Lazy;
use std::backtrace::Backtrace;
use std::sync::{Mutex, MutexGuard, TryLockError};
use tokio::runtime::Runtime;
use tokio_util::sync::CancellationToken;

static APP_STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::default()));

#[derive(Debug)]
pub struct State {
    pub puzzle_config: PuzzleConfig,
    pub target_selection: Option<Target>,
    pub solver_status: SolverStatus,
    pub solver_call_id: Option<SolverCallId>,
    pub solver_cancel_token: Option<CancellationToken>,
    pub runtime: Option<Runtime>,
}

pub fn get_state() -> MutexGuard<'static, State> {
    match APP_STATE.try_lock() {
        Ok(guard) => guard,
        Err(TryLockError::WouldBlock) => {
            eprintln!(
                "get_state: mutex busy (possible deadlock). PID={} Backtrace:\n{:?}",
                std::process::id(),
                Backtrace::force_capture()
            );
            std::thread::sleep(std::time::Duration::from_secs(1));
            APP_STATE.lock().unwrap()
        }
        Err(TryLockError::Poisoned(_)) => APP_STATE.lock().unwrap(),
    }
}

impl Default for State {
    fn default() -> Self {
        let puzzle_config = PuzzleConfig::default();
        let default_target = puzzle_config.default_target.clone();
        State {
            puzzle_config,
            target_selection: default_target,
            solver_status: SolverStatus::Disabled,
            solver_call_id: None,
            solver_cancel_token: None,
            runtime: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SolverStatus {
    Disabled,
    Running { call_id: SolverCallId },
    Done { solvable: bool },
}
