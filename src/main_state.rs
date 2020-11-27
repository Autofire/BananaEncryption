use orbtk::prelude::*;

use crate::MainView;

pub enum Message {
	NewFile(String),
	ClearFile,
}

#[derive(Default, AsAny)]
pub struct MainState {
	path: String
}

impl State for MainState {
	//fn update(&mut self, _: &mut Registry, _: &mut Context) {
	//}

	fn messages(
			&mut self,
			mut messages: MessageReader,
			_registry: &mut Registry,
			ctx: &mut Context,
	) {

		for message in messages.read::<Message>() {
			match message {
				Message::NewFile(p) => {
					println!("Encrypt: {}", p);
					self.path = p.clone();
					MainView::target_file_set(&mut ctx.widget(), p);
				},
				Message::ClearFile => {
					println!("Clear");
					MainView::target_file_set(&mut ctx.widget(), String::from("No file"));
				}
			}
		}
	}
}
