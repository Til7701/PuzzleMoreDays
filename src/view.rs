use crate::offset::{CellOffset, PixelOffset};
use crate::puzzle::PuzzleConfig;
use crate::state::{get_state, MeaningSelection, TargetSelection};
use adw::prelude::{AdwDialogExt, PreferencesGroupExt};
use adw::prelude::{Cast, PreferencesDialogExt};
use adw::prelude::{ComboRowExt, PreferencesPageExt};
use adw::{ActionRow, ComboRow, Dialog, PreferencesDialog, PreferencesGroup, PreferencesPage};
use gtk::prelude::{BoxExt, ButtonExt, FrameExt, GridExt};
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{Button, Frame, Grid, Label, StringList, Widget};
use ndarray::Array2;

#[derive(Debug, Clone)]
pub struct TileView {
    pub elements_with_offset: Vec<(Widget, PixelOffset)>,
    pub draggables: Vec<Widget>,
    pub position_pixels: PixelOffset,
    pub position_cells: Option<CellOffset>,
}

impl TileView {
    pub fn new(id: i32, base: Array2<bool>) -> Self {
        let mut draggables: Vec<Widget> = Vec::new();
        let mut elements_with_offset: Vec<(Widget, PixelOffset)> = Vec::new();

        for ((x, y), value) in base.indexed_iter() {
            if *value {
                let css_classes: Vec<String> =
                    vec!["tile-cell".to_string(), format!("tile-cell-{}", id)];
                let cell = Frame::builder().css_classes(css_classes).build();

                elements_with_offset.push((
                    cell.clone().upcast::<Widget>(),
                    PixelOffset(x as f64, y as f64),
                ));
                draggables.push(cell.upcast::<Widget>());
            }
        }

        TileView {
            elements_with_offset,
            draggables,
            position_pixels: PixelOffset::default(),
            position_cells: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoardView {
    pub parent: Grid,
    pub elements: Vec<Widget>,
}

impl BoardView {
    pub fn new(
        board_layout: &Array2<bool>,
        meaning_areas: &Array2<i32>,
        meaning_values: &Array2<i32>,
        display_values: &Array2<String>,
    ) -> Result<BoardView, String> {
        if board_layout.dim() != meaning_areas.dim() || board_layout.dim() != meaning_values.dim() {
            return Err(
                "Dimensions of board_layout, meaning_areas, and meaning_values must match"
                    .to_string(),
            );
        }

        let grid = Grid::builder()
            .css_classes(vec!["board-grid".to_string()])
            .build();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);

        let mut elements: Vec<Widget> = Vec::new();

        for ((x, y), value) in board_layout.indexed_iter() {
            if *value {
                let css_classes: Vec<String> = vec![
                    "board-cell".to_string(),
                    format!("board-cell-{}", meaning_areas[[x, y]]),
                ];
                let cell = Frame::builder().css_classes(css_classes).build();

                if meaning_areas[[x, y]] != -1 {
                    let label = Label::new(Some(&display_values[[x, y]]));
                    cell.set_child(Some(&label));
                } else {
                    return Err(format!(
                        "Meaning area is -1 while board layout is true at position ({}, {})",
                        x, y,
                    ));
                }

                grid.attach(&cell, x as i32, y as i32, 1, 1);
                elements.push(cell.upcast::<Widget>());
            }
        }

        Ok(BoardView {
            parent: grid,
            elements,
        })
    }
}

pub fn create_puzzle_info(puzzle_config: &PuzzleConfig) -> Dialog {
    let page = create_content_for_puzzle_info(puzzle_config);

    let dialog = PreferencesDialog::builder()
        .title("Puzzle Information")
        .build();
    dialog.add(&page);

    dialog.upcast()
}

fn create_content_for_puzzle_info(puzzle_config: &PuzzleConfig) -> PreferencesPage {
    let page = PreferencesPage::builder()
        .title("Puzzle Information")
        .build();

    let general_group = PreferencesGroup::builder()
        .title("General Information")
        .build();

    let name = create_row("Puzzle Name", &puzzle_config.name);
    general_group.add(&name);

    let board_dimensions = create_row(
        "Board Dimensions",
        &format!(
            "{} x {}",
            puzzle_config.board_layout.nrows(),
            puzzle_config.board_layout.ncols()
        ),
    );
    general_group.add(&board_dimensions);

    let tile_count = create_row("Number of Tiles", &format!("{}", puzzle_config.tiles.len()));
    general_group.add(&tile_count);

    page.add(&general_group);

    if let Some(stats) = &puzzle_config.solution_statistics {
        let solution_statistics_group = PreferencesGroup::builder()
            .title("Solution Statistics")
            .build();
        let min_per_meaning = create_row(
            "Minimum Solutions per Day",
            &format!("{}", stats.min_per_meaning),
        );
        solution_statistics_group.add(&min_per_meaning);

        let max_per_meaning = create_row(
            "Maximum Solutions per Day",
            &format!("{}", stats.max_per_meaning),
        );
        solution_statistics_group.add(&max_per_meaning);

        let average_per_meaning = create_row(
            "Average Solutions per Day",
            &format!("{:.2}", stats.average_per_meaning),
        );
        solution_statistics_group.add(&average_per_meaning);

        let mean_per_meaning = create_row(
            "Mean Solutions per Day",
            &format!("{}", stats.mean_per_meaning),
        );
        solution_statistics_group.add(&mean_per_meaning);

        let total_solutions = create_row("Total Solutions", &format!("{}", stats.total_solutions));
        solution_statistics_group.add(&total_solutions);

        page.add(&solution_statistics_group);
    }

    page
}

fn create_row(title: &str, value: &str) -> ActionRow {
    ActionRow::builder()
        .title(title)
        .subtitle(value)
        .focusable(false)
        .selectable(false)
        .can_focus(false)
        .css_classes(vec!["property".to_string()])
        .build()
}

pub fn create_target_selection_dialog() -> Dialog {
    let dialog = Dialog::builder().title("Select Target Day").build();

    let content = PreferencesGroup::builder()
        .title("Select Target Day")
        .build();

    let state = get_state();
    let puzzle_config = &state.puzzle_config;
    let current_selection = &state.target_selection;
    let meaning_options = &puzzle_config.meaning_options;
    let mut dropdowns: Vec<ComboRow> = Vec::new();
    for meaning in meaning_options {
        let mut items: Vec<String> = Vec::new();
        for value in meaning.min..=meaning.max {
            items.push(value.to_string());
        }
        let item_slices: Vec<&str> = items.iter().map(String::as_str).collect();
        let model = StringList::new(&item_slices);

        let meaning = meaning.clone();
        let current_index: u32 = current_selection
            .as_ref()
            .map(|target_selection| {
                target_selection.meaning_selections[meaning.index as usize].selected_value
                    - meaning.min
            })
            .unwrap_or(0) as u32;
        let dropdown = ComboRow::builder()
            .title(&meaning.name)
            .model(&model)
            .selected(current_index)
            .build();
        content.add(&dropdown);
        dropdowns.push(dropdown);
    }
    let dropdowns = dropdowns;

    let accept_button = Button::builder()
        .label("Accept")
        .css_classes(vec!["suggested-action".to_string()])
        .build();
    accept_button.connect_clicked({
        let dialog = dialog.clone();
        let dropdowns = dropdowns.clone();
        let puzzle_config = puzzle_config.clone();
        move |_| {
            let mut selected_values: Vec<MeaningSelection> = Vec::new();
            for (i, dropdown) in dropdowns.iter().enumerate() {
                let selected = dropdown.selected();
                selected_values.push(MeaningSelection {
                    meaning_index: i as i32,
                    selected_value: selected as i32 + puzzle_config.meaning_options[i].min,
                });
            }
            let mut state = get_state();
            state.target_selection = Some(TargetSelection {
                meaning_selections: selected_values,
            });
            drop(state);
            dialog.close();
        }
    });
    let cancel_button = Button::builder().label("Cancel").build();
    cancel_button.connect_clicked({
        let dialog = dialog.clone();
        move |_| {
            dialog.close();
        }
    });
    let clear_button = Button::builder()
        .css_classes(vec!["destructive-action".to_string()])
        .label("Clear")
        .build();
    clear_button.connect_clicked({
        let dialog = dialog.clone();
        move |_| {
            let mut state = get_state();
            state.target_selection = None;
            drop(state);
            dialog.close();
        }
    });

    let box_buttons = gtk::Box::builder()
        .orientation(Horizontal)
        .spacing(5)
        .build();
    box_buttons.append(&accept_button);
    box_buttons.append(&cancel_button);
    box_buttons.append(&clear_button);

    let box_content = gtk::Box::builder().orientation(Vertical).spacing(2).build();
    box_content.append(&content);
    box_content.append(&box_buttons);

    drop(state);
    dialog.set_child(Some(&box_content));
    dialog.upcast()
}
