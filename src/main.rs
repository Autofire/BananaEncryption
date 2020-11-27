use orbtk::prelude::*;

pub use self::main_state::*;
pub use self::main_view::*;
pub use self::encryption::*;

mod main_state;
mod main_view;
mod encryption;

fn main() {
    Application::from_name("orbtk-sandbox")
        .window(move |ctx| {
            Window::new()
                .title("orbtk-sandbox")
                .position((100.0, 100.0))
                .size(300.0, 300.0)
                .resizeable(true)
                .child(MainView::new().title("Hello OrbTk").build(ctx))
                .build(ctx)
        })
        .run();
}
