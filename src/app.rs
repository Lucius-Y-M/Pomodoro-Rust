
use eframe::egui::{self, Ui};
use chrono::Duration;
#[allow(unused_imports)]
use chrono::serde::ts_seconds;


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Pomodoro {
    label: String,


    value: f32,

    // #[serde(skip)]
    time_settings: TimeSettings,
    music_settings: IntercessionMusic,
}


#[derive(serde::Serialize, serde::Deserialize)]
struct TimeSettings {

    // #[serde(with = "ts_seconds")]
    #[serde(skip)]
    study_len: DurationWrapper,
    // #[serde(with = "ts_seconds")]
    #[serde(skip)]
    relax_len: DurationWrapper,

    enable_cycles: bool,
    cycle_times: i32,
    cycle_count: i32,
}

struct DurationWrapper {
    dur: Duration,
}

impl Default for DurationWrapper {
    fn default() -> Self {
        Self { dur: Duration::minutes(40) }
    }
}


impl Default for TimeSettings {
    fn default() -> Self {
        Self {
            study_len: DurationWrapper { dur: Duration::minutes(50) },
            relax_len: DurationWrapper { dur: Duration::minutes(10) },
            enable_cycles: false,
            cycle_times: 3,
            cycle_count: 0
        }
    }
}



#[derive(serde::Deserialize, serde::Serialize)]
struct IntercessionMusic {
    play_relax_start: bool,
    file_path_relax_start: String,
    play_study_start: bool,
    file_path_study_start: String,
}






impl Default for Pomodoro {
    fn default() -> Self {
        Self {
            label: "My App".to_owned(),
            value: 1.1,
            time_settings: todo!(),
            music_settings: todo!(),            
        }
    }
}

impl Pomodoro {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Pomodoro::default()
        }
    }
}

impl eframe::App for Pomodoro {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        

        // ========== TOP
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            
            egui::menu::bar(ui, |ui| {

                ui.menu_button("file", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
                ui.add_space(16.0);
            });

            egui::widgets::global_dark_light_mode_switch(ui);
        });

               

        // ========== CENTRAL
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Time here");

            ui.horizontal(|ui| {
                ui.label("Set your profile name here:");
                ui.text_edit_singleline(&mut self.label);
            });


            // # notice how self fields are called
            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            // // === GITHUB Connection
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });

        });

        // ========== BOTTOM

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.heading("Time Settings");

            // ======== MAIN

            // Study Time
            ui.horizontal(|ui| {
                ui.label("Juche");
                let sl1 = ui.add(
                    egui::Slider::new(
                        &mut self.time_settings.study_len.dur.num_minutes(),
                        0..=120
                    )
                    .text("Minutes")
                    .text_color(egui::Color32::from_rgb(150, 150, 50))
                );

                if ui.button("Reset").clicked() {
                }
            });

            // Relax Time
            ui.horizontal(|ui| {
                
                let sl1 = ui.add(
                    egui::Slider::new(
                        &mut self.time_settings.study_len.dur.num_minutes(),
                        0..=60
                    )
                    .text("Minutes")
                    .text_color(egui::Color32::from_rgb(150, 150, 50))
                );
                
                if ui.button("Reset").clicked() {
                }
            });
            


            // ======== EXTRA

            // row 1
            ui.horizontal(|ui| {

                let r1 = ui.checkbox(&mut false, egui::WidgetText::from("Play Music At Relax Start"));
                // only if checkbox ticked
                ui.add_enabled(r1.enabled(), |ui: &mut Ui| {
                    ui.text_edit_singleline(&mut self.music_settings.file_path_relax_start)
                });                

            });

            // row 2
            ui.horizontal(|ui| {

                let r1 = ui.checkbox(&mut false, egui::WidgetText::from("Play Music At Study Start"));
                // only if checkbox ticked
                ui.add_enabled(r1.enabled(), |ui: &mut Ui| {
                    ui.text_edit_singleline(&mut self.music_settings.file_path_study_start)
                });

            });
        });

    }
}


fn powered_by_egui_and_eframe(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}