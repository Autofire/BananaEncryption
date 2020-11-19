use orbtk::prelude::*;

use crate::MainState;

widget!(
    MainView<MainState> {
        title: String
    }
);

widget!(
    MyWidget: DropHandler {
      background: Brush,
      count: u32,
      text: String
    }
);

impl Template for MyWidget {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
         self.name("MyWidget")
            .style("my_widget_style")
            .background("#000000")
            .count(0)
            .text("Initial text HIII")
			.on_drop_file(|_,_,_| {
				println!("event triggered");
				true
			})
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
    }
}

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
			.child(TextBlock::new().text(("title", id)).build(ctx))
			.child(MyWidget::new().build(ctx))
    }
}
