use orbtk::prelude::*;

use crate::MainState;

widget!(
	DropArea: DropHandler {
	}
);

impl Template for DropArea {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
		self.name("Drop Area")
			.child(ImageWidget::new()
				.image("assets/orbtk_logo.png")
				.build(ctx)
			)

	}

}

widget!(
    MyWidget: DropHandler {
      background: Brush,
      count: u32,
      text: String
    }
);

impl Template for MyWidget {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
		let pager = Pager::new()
			.child(TextBlock::new().text("Page 1").build(ctx))
			.child(TextBlock::new().text("Page 2").build(ctx))
			.build(ctx);

		let next_button = Button::new()
			.enabled(("next_enabled", pager))
			.text("next")
			.on_click(move |states, _| {
					//states.get_mut::<PagerState>(pager).next();
					states.send_message(PagerAction::Next, pager);
					//orbtk::widgets::Pager::next(ctx, pager);
					true
					})
			.build(ctx);

		let previous_button = Button::new()
			.enabled(("previous_enabled", pager))
			.text("previous")
			.on_click(move |states, _| {
					//states.get_mut::<PagerState>(pager).previous();
					states.send_message(PagerAction::Previous, pager);
					true
			})
			.build(ctx);

		self.name("MyWidget")
			.style("my_widget_style")
			.background("#000000")
			.count(0)
			.text("Initial text HIII")
			.on_drop_file(|_,path,_| {
					println!("event triggered");
					println!("{}", path);
					true
					})
					/*
			.child(Container::new()
				// Container references the same background as MyWidget
				.background(id)
				.child(TextBlock::new()
					// TextBlock references the same text as MyWidget
					.text(id)
					.build(ctx)
				)
				.build(ctx)
			  )
			  */
			.child(Stack::new()
				.width(120)
				.margin(10)
				.spacing(10)
				.child(pager)
				.child(next_button)
				.child(previous_button)
				.build(ctx)
			)
    }
}


widget!(
    MainView<MainState> {
        title: String
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
			//.child(TextBlock::new().text(("title", id)).build(ctx))
			//.child(MyWidget::new().build(ctx))
			.child(DropArea::new()
				.on_drop_file(|_,path,_| {
					println!("event triggered");
					println!("{}", path);
					true
				})
				.v_align("center")
				.h_align("center")
				.build(ctx)
			)
    }
}
