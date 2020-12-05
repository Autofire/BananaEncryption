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

fn build_confirm_button(id: Entity, _ctx: &mut BuildContext) -> Button {
	Button::new()
		.text("Confirm")
		.on_click(move |states, _| {
			states.send_message(Message::Confirm, id);
			true
		})
}

fn build_cancel_button(id: Entity, _ctx: &mut BuildContext, pager: Entity) -> Button {
	Button::new()
		.text("Cancel")
		.on_click(move |states, _| {
			states.send_message(Message::ClearFile, id);
            //states.send_message(PagerAction::Navigate(0), pager);
			true
		})
}

widget!(
    MainView<MainState> {
        title: String,
		target_file: String,
		decrypt_ok: bool
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {

		let pager = Pager::new()
			.id("pager")
			.v_align("end")
			.h_align("center")
			.build(ctx);

		let blank_page = Container::new().build(ctx);

		let encrypt_page = Stack::new()
			.child(PasswordBox::new()
				.id("encrypt_password")
                .water_mark("Password...")
                .build(ctx)
            )
			.child(PasswordBox::new()
				.id("encrypt_password2")
                .water_mark("Confirm password...")
                .build(ctx)
            )
			.child(build_confirm_button(id, ctx).build(ctx))
			.child(build_cancel_button(id, ctx, pager).build(ctx))
			.build(ctx);

		let decrypt_page = Stack::new()
			.child(PasswordBox::new()
				.id("decrypt_password")
				.water_mark("Password...")
				.build(ctx)
			)
			.child(build_confirm_button(id, ctx)
				.id("decrypt_confirm")
				//.enabled(("decrypt_ok", id))
				.build(ctx)
			)
			.child(build_cancel_button(id, ctx, pager).build(ctx))
			.build(ctx);


		let drop_area = DropArea::new()
			.on_drop_file(move |states,path,_| {
				match get_file_type(&path) {
					FileType::Encrypted => {
						states.send_message(
							PagerAction::Navigate(2), pager
						);
					}
					FileType::Decrypted => {
						states.send_message(
							PagerAction::Navigate(1), pager
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

        ctx.append_child(pager, blank_page);
        ctx.append_child(pager, encrypt_page);
        ctx.append_child(pager, decrypt_page);

		self.name("MainView")
			.target_file("No file")
			.decrypt_ok(true)
			.child(Stack::new()
				.margin(10)
				.spacing(10)
				.child(drop_area)
				.child(pager)
				.build(ctx)
			)
    }
}

