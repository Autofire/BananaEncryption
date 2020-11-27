use orbtk::prelude::*;

use crate::{
	MainState, Message,
	get_file_type,
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


widget!(
    MainView<MainState> {
        title: String,
		target_file: String
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {

		let pager = Pager::new()
			.child(TextBlock::new().text("Page 1").build(ctx))
			.child(Container::new().build(ctx))
			.child(TextBlock::new().text("Page 2").build(ctx))
			.v_align("end")
			.h_align("center")
			.build(ctx);

		let drop_area = DropArea::new()
			.on_drop_file(move |states,path,_| {
				println!("event triggered");
				println!("{}", path);
				println!("{:?}", get_file_type(&path));
				states.send_message(Message::NewFile(path), id);
				states.send_message(PagerAction::Next, pager);
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

