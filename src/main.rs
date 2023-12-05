use pomodoro::Pomodoro;


fn main() -> eframe::Result<()> {
    println!("Hello, world!");



    let native_options = eframe::NativeOptions {
        
        /* Newer ver */
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_drag_and_drop(true),
        /* No longer works in new ver */
        // initial_window_size: Some([200.0, 200.0].into()),
        // min_window_size: Some([300.0, 220.0].into()),
        ..Default::default()
    };

    eframe::run_native(
        "Eframe App Defined in Main",
        native_options,
        Box::new(|cc| Box::new(Pomodoro::new(cc)))
    )

}
