use log::{info, Level};
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;

mod components;
mod containers;
mod theme;
use crate::containers::*;
use crate::theme::Theme;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct App {
    clicks: u32,
}

#[derive(Clone)]
enum AppModel {
    Click,
}

#[derive(Clone)]
enum AppView {
    Clicked(u32),
}

impl Component for App {
    type DomNode = HtmlElement;
    type ModelMsg = AppModel;
    type ViewMsg = AppView;

    fn update(&mut self, msg: &AppModel, tx: &Transmitter<AppView>, _sub: &Subscriber<AppModel>) {
        match msg {
            AppModel::Click => {
                self.clicks += 1;
                tx.send(&AppView::Clicked(self.clicks));
            }
        }
    }

    fn view(&self, tx: &Transmitter<AppModel>, rx: &Receiver<AppView>) -> ViewBuilder<HtmlElement> {
        containers::layout::set_layout()
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();

    let mut grid = components::grid::Grid::new(20, 20);
    grid.binary_tree();
    info!("{}", grid);

    let gizmo = Gizmo::from(App { clicks: 0 });
    let view = View::from(gizmo.view_builder());
    view.run()
}
