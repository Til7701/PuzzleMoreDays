use crate::puzzle::PuzzleConfig;
use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};

static APP_STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::default()));

#[derive(Debug, Clone, Default)]
pub struct State {
    pub current_puzzle_index: u32,
    pub puzzle_config: PuzzleConfig,
}

pub fn get_state() -> MutexGuard<'static, State> {
    APP_STATE.lock().unwrap()
}
