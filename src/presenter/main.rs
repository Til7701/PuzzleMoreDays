use crate::presenter::puzzle_area::PuzzleAreaPresenter;
use crate::puzzle;
use crate::state::get_state;
use crate::view::create_puzzle_info;
use crate::window::PuzzlemoredaysWindow;
use adw::prelude::AdwDialogExt;
use gtk::prelude::ButtonExt;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct MainPresenter {
    puzzle_area_presenter: Rc<RefCell<PuzzleAreaPresenter>>,
    window: Rc<RefCell<Option<PuzzlemoredaysWindow>>>,
}

impl MainPresenter {
    pub fn set_puzzle_area_presenter(&self, presenter: &PuzzleAreaPresenter) {
        self.puzzle_area_presenter.replace(presenter.clone());
    }

    pub fn setup(&self, window: &PuzzlemoredaysWindow) {
        self.window.replace(Some(window.clone()));
        let puzzle_area_presenter = self.puzzle_area_presenter.borrow();
        puzzle_area_presenter.set_view(window.grid());
        puzzle_area_presenter.setup_puzzle_config_from_state();

        let puzzle_selection = window.puzzle_selection();
        puzzle_selection.set_selected(0);

        puzzle_selection.connect_selected_notify({
            let puzzle_area_presenter = puzzle_area_presenter.clone();
            move |dropdown| {
                let index = dropdown.selected();
                let puzzle_config = match index {
                    0 => puzzle::get_default_config(),
                    1 => puzzle::get_year_config(),
                    _ => panic!("Unknown puzzle selection index: {}", index),
                };
                get_state().puzzle_config = puzzle_config;

                puzzle_area_presenter.setup_puzzle_config_from_state();
            }
        });

        let puzzle_info_button = window.puzzle_info_button();
        puzzle_info_button.connect_clicked({
            let main_presenter = self.clone();
            move |_| {
                main_presenter.show_puzzle_info_dialog();
            }
        });
    }

    fn show_puzzle_info_dialog(&self) {
        if let Some(window) = self.window.borrow().as_ref() {
            let state = get_state();
            let puzzle_config = &state.puzzle_config;

            let dialog = create_puzzle_info(puzzle_config);
            dialog.present(Some(window));
        }
    }

    pub fn update_layout(&self) {
        self.puzzle_area_presenter.borrow().update_layout();
    }
}
