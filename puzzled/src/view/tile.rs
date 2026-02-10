use crate::offset::CellOffset;
use crate::offset::PixelOffset;
use adw::gdk::RGBA;
use adw::gio;
use adw::glib;
use adw::glib::random_double;
use adw::prelude::GdkCairoContextExt;
use adw::subclass::prelude::*;
use gtk::cairo::Context;
use gtk::prelude::{DrawingAreaExtManual, WidgetExt};
use ndarray::{Array2, Axis};

#[derive(Debug, Default)]
pub enum HighlightMode {
    #[default]
    None,
    Overlapping,
    OutOfBounds,
}

mod imp {
    use super::*;
    use std::cell::RefCell;

    #[derive(Debug, Default)]
    pub struct PuzzledTileView {
        pub id: RefCell<usize>,
        pub base: RefCell<Array2<bool>>,
        pub current_rotation: RefCell<Array2<bool>>,
        pub position_cells: RefCell<Option<CellOffset>>,
        pub position_pixels: RefCell<PixelOffset>,
        pub color: RefCell<Option<RGBA>>,
        pub highlights: RefCell<Array2<HighlightMode>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledTileView {
        const NAME: &'static str = "PuzzledTileView";
        type Type = TileView;
        type ParentType = gtk::DrawingArea;

        fn class_init(_: &mut Self::Class) {}

        fn instance_init(_: &glib::subclass::InitializingObject<Self>) {}
    }

    impl ObjectImpl for PuzzledTileView {}
    impl WidgetImpl for PuzzledTileView {}
    impl DrawingAreaImpl for PuzzledTileView {}
}

glib::wrapper! {
    pub struct TileView(ObjectSubclass<imp::PuzzledTileView>)
        @extends gtk::Widget, gtk::DrawingArea,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl TileView {
    pub fn new(id: usize, base: Array2<bool>) -> Self {
        let obj: TileView = glib::Object::builder().build();

        obj.imp().id.replace(id);
        obj.imp().base.replace(base.clone());
        obj.imp().current_rotation.replace(base);
        obj.imp().color.replace(Some(random_color()));

        obj.set_draw_func({
            let self_clone = obj.clone();
            move |_, cr, width, height| self_clone.draw(cr, width, height)
        });

        obj
    }

    fn draw(&self, cr: &Context, width: i32, height: i32) {
        let current_rotation = self.imp().current_rotation.borrow();
        cr.set_source_color(&self.imp().color.borrow().unwrap_or_else(|| RGBA::BLACK));
        for ((x, y), cell) in current_rotation.indexed_iter() {
            if *cell {
                cr.rectangle(
                    x as f64 * (width as f64 / current_rotation.dim().0 as f64),
                    y as f64 * (height as f64 / current_rotation.dim().1 as f64),
                    width as f64 / current_rotation.dim().0 as f64,
                    height as f64 / current_rotation.dim().1 as f64,
                );
                cr.fill().expect("Failed to fill");
            }
        }
    }

    pub fn id(&self) -> usize {
        *self.imp().id.borrow()
    }

    pub fn base(&self) -> Array2<bool> {
        self.imp().base.borrow().clone()
    }

    pub fn set_current_rotation(&self, rotation: Array2<bool>) {
        self.imp().current_rotation.replace(rotation);
        self.queue_draw();
    }

    pub fn current_rotation(&self) -> Array2<bool> {
        self.imp().current_rotation.borrow().clone()
    }

    pub fn set_position_cells(&self, position_cells: Option<CellOffset>) {
        self.imp().position_cells.replace(position_cells);
    }

    pub fn position_cells(&self) -> Option<CellOffset> {
        self.imp().position_cells.borrow().clone()
    }

    pub fn position_pixels(&self) -> PixelOffset {
        self.imp().position_pixels.borrow().clone()
    }

    pub fn set_position_pixels(&self, position_pixels: PixelOffset) {
        self.imp().position_pixels.replace(position_pixels);
    }

    pub fn set_highlights(&self, highlights: Array2<HighlightMode>) {
        self.imp().highlights.replace(highlights);
        self.queue_draw();
    }

    pub fn rotate_clockwise(&self) {
        let base = self.current_rotation();
        let mut rotated = base.reversed_axes();
        rotated.invert_axis(Axis(0));
        self.set_current_rotation(rotated);
    }

    pub fn flip_horizontal(&self) {
        let mut base = self.current_rotation();
        base.invert_axis(Axis(0));
        self.set_current_rotation(base);
    }
}

fn random_color() -> RGBA {
    RGBA::new(
        random_double() as f32,
        random_double() as f32,
        random_double() as f32,
        1.0,
    )
}
