use orbtk::prelude::*;

pub use self::main_state::*;
pub use self::main_view::*;
pub use self::encryption::*;
pub use self::primes::*;

mod main_state;
mod main_view;
mod encryption;
mod primes;

fn main() {
    Application::from_name("Banana Encryption")
        .window(move |ctx| {
            Window::new()
                .title("Banana Encryption")
                .position((100.0, 100.0))
                .size(300.0, 300.0)
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
