use orbtk::prelude::*;

use crate::MainView;

pub enum Message {
	Encrypt(String),
	Decrypt(String),
	Clear,
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
				Message::Encrypt(p) => {
					println!("Encrypt: {}", p);
					self.path = p.clone();
					MainView::target_file_set(&mut ctx.widget(), p);
				},
				Message::Decrypt(p) => {
					println!("Decrypt: {}", p);
				},
				Message::Clear => {
					println!("Clear");
				}
			}
		}
	}
}
