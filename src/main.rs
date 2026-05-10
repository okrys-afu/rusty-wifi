mod app;
mod error;
mod wifi;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Rusty Wifi",
        options,
        Box::new(|_cc| Result::Ok(Box::new(app::App::new()))),
    )
}
