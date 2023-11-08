use eframe::egui::{self, Ui};
use chrono::Duration;
#[allow(unused_imports)]
use chrono::serde::ts_seconds;



const BTN_CONFIRM_STR : &str = "Confirm";
const BTN_RECHOOSE_STR : &str = "Re-Choose";

const BTN_STATUSES : [&str; 2] = [BTN_CONFIRM_STR, BTN_RECHOOSE_STR];


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Pomodoro {
    label: String,


    value: f32,

    // #[serde(skip)]
    time_sett: TimeSettings,
    music_sett: IntercessionMusic,
}


#[derive(serde::Serialize, serde::Deserialize)]
struct TimeSettings {

    // #[serde(with = "ts_seconds")]
    #[serde(skip)]
    study_len: DurationWrapper,
    study_len_slider_enable: bool,
    #[serde(skip)]
    study_len_btn_stat: &'static str,

    // #[serde(with = "ts_seconds")]
    #[serde(skip)]
    relax_len: DurationWrapper,
    relax_len_slider_enable: bool,
    #[serde(skip)]
    relax_len_btn_stat: &'static str,

    enable_cycles: bool,
    cycle_times: i32,
    cycle_count: i32,
}

struct DurationWrapper {
    mins: i64,
    dur: Duration,
}

impl Default for DurationWrapper {
    fn default() -> Self {
        Self {
            mins: 0,
            dur: Duration::minutes(0)
        }
    }
}


impl Default for TimeSettings {
    fn default() -> Self {
        Self {
            study_len: DurationWrapper {
                mins: 50,
                dur: Duration::minutes(50)
            },
            study_len_slider_enable: true,
            study_len_btn_stat: BTN_STATUSES[0],


            relax_len: DurationWrapper {
                mins: 10,
                dur: Duration::minutes(10)
            },
            relax_len_slider_enable: true,
            relax_len_btn_stat: BTN_STATUSES[0],

            enable_cycles: false,
            cycle_times: 3,
            cycle_count: 0,

        }
    }
}



#[derive(serde::Deserialize, serde::Serialize, Default)]
struct IntercessionMusic {
    play_relax_start: bool,
    file_path_rlx: String,
    play_study_start: bool,
    file_path_std: String,
}






impl Default for Pomodoro {
    fn default() -> Self {
        Self {
            label: "My App".to_owned(),
            value: 1.1,
            time_sett: TimeSettings::default(),
            music_sett: IntercessionMusic::default()
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
                egui::widgets::global_dark_light_mode_switch(ui);

            });


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
                ui.label("Study Time Setting");
                
                let slider =
                    egui::Slider::new(
                        &mut self.time_sett.study_len.mins,
                        0..=120
                    )
                    .text("Minutes")
                    .text_color(egui::Color32::from_rgb(150, 150, 50));

                ui.add_enabled(self.time_sett.study_len_slider_enable, slider);

                let btn_lock_unlock = egui::Button::new(self.time_sett.study_len_btn_stat)
                    .fill(egui::Color32::from_rgb(3,3,3))
                    .rounding(25.0);
                let btn_reset = egui::Button::new("RESET")
                    .fill(egui::Color32::from_rgb(200, 0, 0))
                    .rounding(25.0);

                if ui.add(btn_lock_unlock).clicked() {
                    let enable = &mut self.time_sett.study_len_slider_enable;
                    if *enable {
                        *enable = false;

                        self.time_sett.study_len_btn_stat = BTN_STATUSES[1];
                        
                        self.time_sett.study_len.dur = Duration::minutes(self.time_sett.study_len.mins);
                        println!(">> Debug: study time Duration len now = {x:?}", x=self.time_sett.study_len.dur);
                    } else {
                        *enable = true;
                        self.time_sett.study_len_btn_stat = BTN_STATUSES[0];

                    }
                }

                if ui.add(btn_reset).clicked() {
                    self.time_sett.study_len.mins = 0;
                    self.time_sett.study_len.dur = Duration::minutes(0);
                }

            });

            // Relax Time
            ui.horizontal(|ui| {
                ui.label("Relax Time Setting");
                
                
                let slider =
                    egui::Slider::new(
                        &mut self.time_sett.relax_len.mins,
                        0..=60
                    )
                    .text("Minutes")
                    .text_color(egui::Color32::from_rgb(150, 150, 50));

                ui.add_enabled(self.time_sett.relax_len_slider_enable, slider);
                

                // // >>> two buttons:
                // // 1. lock / unlock slider
                let btn_lock_unlock = egui::Button::new(self.time_sett.relax_len_btn_stat)
                    .fill(egui::Color32::from_rgb(3,3,3))
                    .rounding(25.0);

                // // 2. reset slider
                let btn_reset = egui::Button::new("RESET")
                    .fill(egui::Color32::from_rgb(200, 0, 0))
                    .rounding(25.0);

                if ui.add(btn_lock_unlock).clicked() {
                    let enable = &mut self.time_sett.relax_len_slider_enable;
                    if *enable {
                        *enable = false;

                        self.time_sett.study_len_btn_stat = BTN_STATUSES[1];                        
                        self.time_sett.relax_len.dur = Duration::minutes(self.time_sett.relax_len.mins);
                        println!(">> Debug: relax time Duration len now = {x:?}", x=self.time_sett.relax_len.dur);
                    } else {
                        *enable = true;
                        self.time_sett.study_len_btn_stat = BTN_STATUSES[0];
                    }
                }

                if ui.add(btn_reset).clicked() {
                    self.time_sett.relax_len.mins = 0;
                    self.time_sett.relax_len.dur = Duration::minutes(0);
                }
            });
            


            // ======== EXTRA

            // row 1
            ui.horizontal(|ui| {

                let e = ui.checkbox(&mut false, egui::WidgetText::from("Play Music At Relax Start"));
                // only if checkbox ticked
                ui.add_enabled(self.music_sett.play_relax_start, |ui: &mut Ui| {
                    let s = &mut self.music_sett.file_path_rlx;
                    ui.text_edit_singleline(s)
                });

                if e.enabled() {
                    self.music_sett.play_relax_start = true;
                } else {
                    self.music_sett.play_relax_start = false;
                }
                
            });

            // row 2
            ui.horizontal(|ui| {

                let e = ui.checkbox(&mut false, egui::WidgetText::from("Play Music At Study Start"));
                // only if checkbox ticked
                ui.add_enabled(self.music_sett.play_study_start, |ui: &mut Ui| {
                    let s = &mut self.music_sett.file_path_std;
                    ui.text_edit_singleline(s)
                });

                if e.enabled() {
                    self.music_sett.play_study_start = true;
                } else {
                    self.music_sett.play_study_start = false;
                }

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