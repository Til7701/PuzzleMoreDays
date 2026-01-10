use crate::puzzle::config::Target;
use crate::puzzle_state::{Cell, PuzzleState};
use crate::state::{get_state, SolverStatus, State};
use log::debug;
use puzzle_solver::board::Board;
use puzzle_solver::tile::Tile;
use std::cmp::PartialEq;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::{runtime, task};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SolverCallId(u64);

pub type OnCompleteCallback = Box<dyn FnOnce(SolverStatus) + Send>;

static SOLVER_CALL_ID_ATOMIC_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn create_solver_call_id() -> SolverCallId {
    SolverCallId(SOLVER_CALL_ID_ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst))
}

pub fn solve_for_target(
    solver_call_id: &SolverCallId,
    puzzle_state: &PuzzleState,
    target: &Target,
    on_complete: OnCompleteCallback,
) {
    init_runtime_if_needed();
    let cancel_token = CancellationToken::new();
    let board = create_board(puzzle_state, target);
    let tiles: Vec<Tile> = puzzle_state
        .unused_tiles
        .iter()
        .map(|tile_state| Tile::new(tile_state.base.clone()))
        .collect();
    let mut state = get_state();
    state.solver_call_id = Some(solver_call_id.clone());
    state.solver_cancel_token = Some(cancel_token.clone());
    let runtime = state.runtime.as_ref().unwrap();
    runtime.spawn({
        let solver_call_id = solver_call_id.clone();
        let cancel_token = cancel_token.clone();
        async move {
            let result = puzzle_solver::solve_all_filling(board, &tiles, cancel_token).await;
            handle_on_complete(solver_call_id, result.is_ok(), on_complete);
        }
    });
    drop(state);
}

fn init_runtime_if_needed() {
    let mut state = get_state();
    if state.runtime.is_none() {
        let runtime = runtime::Builder::new_multi_thread().build().unwrap();
        state.runtime = Some(runtime);
    }
}

fn handle_on_complete(
    solver_call_id: SolverCallId,
    solvable: bool,
    on_complete: OnCompleteCallback,
) {
    let mut state = get_state();
    if state.solver_call_id == Some(solver_call_id.clone()) {
        state.solver_call_id = None;
        drop(state);
        on_complete(SolverStatus::Done { solvable });
    }
}

pub fn interrupt_solver_call(call_id: &SolverCallId, state: &State) {
    debug!("Interrupting solver call: {:?}", call_id);
    if state.solver_call_id == Some(call_id.clone()) {
        if let Some(cancel_token) = &state.solver_cancel_token {
            cancel_token.cancel();
            debug!("Solver call {:?} aborted.", call_id);
        }
    }
}

pub fn is_solved(puzzle_state: &PuzzleState, target: &Target) -> bool {
    let board = create_board(puzzle_state, target);
    board.get_array().iter().all(|cell| *cell)
}

fn create_board(puzzle_state: &PuzzleState, target: &Target) -> Board {
    let dims = puzzle_state.grid.dim();
    let mut board = Board::new(dims);

    puzzle_state.grid.indexed_iter().for_each(|((x, y), cell)| {
        let is_filled = match cell {
            Cell::Empty(cell_data) => !cell_data.is_on_board,
            Cell::One(_, _) => true,
            Cell::Many(_, _) => true,
        };

        board[[x, y]] = is_filled;
    });

    for index in target.indices.iter() {
        let x = index.0 + 1;
        let y = index.1 + 1;
        board[[x, y]] = true;
    }

    board
}
