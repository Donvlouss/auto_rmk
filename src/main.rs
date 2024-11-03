use app::RmkApp;

mod app;
mod core;
mod record;

fn main() {
    let app = RmkApp::new();

    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options
        .viewport
        .with_min_inner_size([360., 180.])
        .with_inner_size([640., 360.]);
    eframe::run_native(
        "Flow Monitor",
        native_options,
        Box::new(|_| Ok(Box::new(app))),
    )
    .unwrap();
}
