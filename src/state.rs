use crate::application::WINDOW_TO_BOARD_RATIO;
use crate::puzzle::PuzzleConfig;
use once_cell::sync::Lazy;
use std::backtrace::Backtrace;
use std::sync::{Mutex, MutexGuard, TryLockError};

static APP_STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::default()));

#[derive(Debug, Clone)]
pub struct State {
    pub current_puzzle_index: u32,
    pub puzzle_config: PuzzleConfig,
    pub grid_cell_width_pixel: u32,
    pub grid_h_cell_count: u32,
    pub board_offset_x_cells: i32,
}

pub fn get_state() -> MutexGuard<'static, State> {
    match APP_STATE.try_lock() {
        Ok(guard) => guard,
        Err(TryLockError::WouldBlock) => {
            eprintln!(
                "get_state: mutex busy (possible deadlock). PID={} Backtrace:\n{:?}",
                std::process::id(),
                Backtrace::capture()
            );
            // pause so you can attach a debugger (gdb/rust-gdb) to inspect threads/stacks
            std::thread::sleep(std::time::Duration::from_secs(2));
            // fallback to blocking lock so program can continue after inspection
            APP_STATE.lock().unwrap()
        }
        Err(TryLockError::Poisoned(_)) => {
            // preserve original behavior on poisoned lock
            APP_STATE.lock().unwrap()
        }
    }
}

impl Default for State {
    fn default() -> Self {
        let puzzle_config = PuzzleConfig::default();
        let grid_h_cell_count =
            (puzzle_config.board_layout.dim().1 as f64 * WINDOW_TO_BOARD_RATIO) as u32;
        let board_offset_x_cells =
            ((grid_h_cell_count - puzzle_config.board_layout.dim().1 as u32) / 2) as i32;
        State {
            current_puzzle_index: 0,
            puzzle_config,
            grid_cell_width_pixel: 32,
            grid_h_cell_count,
            board_offset_x_cells,
        }
    }
}
