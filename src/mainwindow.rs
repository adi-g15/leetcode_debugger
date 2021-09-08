use orbtk::prelude::*;

pub struct MainWindow {
}

impl MainWindow {
    pub fn new(context: &mut BuildContext<'_>) -> Entity {
        const APP_WIDTH:  u32 = 800;
        const APP_HEIGHT: u32 = 600;

        let window = Window::new()
            .title("Leetcode Debugger")
            .size(APP_WIDTH, APP_HEIGHT)
            .resizeable(true)
            .borderless(false);

        //Window::
        window.build(context)
    }
}
