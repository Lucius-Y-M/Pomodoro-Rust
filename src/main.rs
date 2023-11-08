use pomodoro::Pomodoro;


fn main() -> eframe::Result<()> {
    println!("Hello, world!");



    let native_options = eframe::NativeOptions {
        initial_window_size: Some([200.0, 200.0].into()),
        min_window_size: Some([300.0, 220.0].into()),
        ..Default::default()
    };

    eframe::run_native(
        "Eframe App Defined in Main",
        native_options,
        Box::new(|cc| Box::new(Pomodoro::new(cc)))
    )

}
