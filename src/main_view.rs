use orbtk::prelude::*;

use crate::{
	MainState, Message,
	get_file_type, FileType
};

widget!(
	DropArea: DropHandler {
		text: String
	}
);

impl Template for DropArea {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {

		self.name("Drop Area")
			.text("Hello")
			.child(Stack::new()
				.margin(10)
				.spacing(10)
				.child(ImageWidget::new()
					.image("assets/dnd.png")
					.h_align("center")
					.build(ctx)
				)
				.child(TextBlock::new()
					.text(id)
					.id("path_text")
					.h_align("center")
					.localizable(false)
					.build(ctx)
				)
				.build(ctx)
			)

	}

}

fn build_confirm_button(id: Entity, ctx: &mut BuildContext) -> Entity {
	Button::new()
		.text("Confirm")
		.on_click(move |states, _| {
			states.send_message(Message::ClearFile, id);
			true
		})
		.build(ctx)
}

fn build_cancel_button(id: Entity, ctx: &mut BuildContext) -> Entity {
	Button::new()
		.text("Cancel")
		.on_click(move |states, _| {
			states.send_message(Message::ClearFile, id);
			true
		})
		.build(ctx)
}

widget!(
    MainView<MainState> {
        title: String,
		target_file: String,
		password: String,
		confirmation_password: String
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {

		let blank_page = Container::new().build(ctx);

		let encrypt_page = Stack::new()
			.child(PasswordBox::new().water_mark("Password...").build(ctx))
			.child(PasswordBox::new().water_mark("Confirm password...").build(ctx))
			.child(build_confirm_button(id, ctx))
			.child(build_cancel_button(id, ctx))
			.build(ctx);

		let decrypt_page = Stack::new()
			.child(PasswordBox::new()
				.id("password_thing")
				.text(("password", id))
				.water_mark("Password...")
				.build(ctx)
			)
			.child(build_confirm_button(id, ctx))
			.child(build_cancel_button(id, ctx))
			.build(ctx);

		let pager = Pager::new()
			.id("pager")
			.child(blank_page)
			.child(encrypt_page)
			.child(decrypt_page)
			.v_align("end")
			.h_align("center")
			.build(ctx);

		let drop_area = DropArea::new()
			.on_drop_file(move |states,path,_| {
				match get_file_type(&path) {
					FileType::Encrypted => {
						states.send_message(
							PagerAction::Navigate(1), pager
						);
					}
					FileType::Decrypted => {
						states.send_message(
							PagerAction::Navigate(2), pager
						);
					}
				}
				states.send_message(Message::NewFile(path), id);
				true
			})
			.text(("target_file", id))
			.v_align("stretch")
			.h_align("center")
			.build(ctx);

		self.name("MainView")
			.target_file("No file")
			.child(Stack::new()
				.margin(10)
				.spacing(10)
				.child(drop_area)
				.child(pager)
				.build(ctx)
			)
    }
}

