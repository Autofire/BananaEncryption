use orbtk::prelude::*;

use crate::{MainState, Message};

widget!(
	DropArea: DropHandler {
		text: String
	}
);

impl Template for DropArea {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {

		let test = Message::Encrypt(String::from("Hi"));

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


widget!(
    MainView<MainState> {
        title: String,
		target_file: String
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {

		let drop_area = DropArea::new()
			.on_drop_file(move |states,path,_| {
				println!("event triggered");
				println!("{}", path);
				//DropArea::text_set(&mut sctx.get(), path);
				states.send_message(Message::Encrypt(path), id);
				true
			})
			.text(("target_file", id))
			.v_align("center")
			.h_align("center")
			.build(ctx);

		self.name("MainView")
			.target_file("No file")
			.child(drop_area)
    }
}

