use orbtk::prelude::*;

mod mainwindow;
use mainwindow::MainWindow;

fn main() {
    let application = Application::new();

    application
        .window(|context| MainWindow::new(context) )
        .run();

    // application is moved into the application.window() call, can't refer object here
    // application.run();
}
