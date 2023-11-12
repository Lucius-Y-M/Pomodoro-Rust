use eframe::egui::{self, Ui};
use chrono::Duration;
#[allow(unused_imports)]
use chrono::serde::ts_seconds;

use crate::countdown::{SharedState, StudyRelaxStatus, AppStatus};



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

    app_status: AppStatus
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




















//============================ Impl for MAIN

impl Default for Pomodoro {
    fn default() -> Self {
        Self {
            label: "My App".to_owned(),
            value: 1.1,


            time_sett: Default::default(),
            music_sett: Default::default(),
            app_status: Default::default(),
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


    fn play_track(&self) {
        match self.app_status.study_or_relax() {
            StudyRelaxStatus::Study => if self.music_sett.play_study_start {
                // TODO
                println!(">> Play Study");
            },
            StudyRelaxStatus::Relax => if self.music_sett.play_relax_start {
                // TODO
                println!(">> Play Relax");
            },
        }
    }

    // ===== corresponding to
    // run, pause, stop
    fn start_timer(&mut self) {
        self.app_status.run();
    }


    pub fn run(&mut self) {
        self.app_status.run();

        // == conversion
        loop {
            let std_dur = match self.app_status.study_or_relax() {
                StudyRelaxStatus::Study => std::time::Duration::from_secs(self.time_sett.study_len.dur.num_seconds() as u64),
                StudyRelaxStatus::Relax => std::time::Duration::from_secs(self.time_sett.relax_len.dur.num_seconds() as u64),
            };
    
            std::thread::sleep(std_dur);

            // == play music
            self.play_track();

            // == study or relax
            self.app_status.revert_study_relax();
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
            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/eframe_template/blob/master/",
            //     "Source code."
            // ));


            // // ===== from original template, disable for now

            // ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                
                
            //     powered_by_egui_and_eframe(ui);
            //     egui::warn_if_debug_build(ui);
            // });

            ui.heading("Time Settings");

            // ======== MAIN


            // components used later
            let btn_txt_confirm_study = egui::RichText::new(self.time_sett.study_len_btn_stat)
                .color(egui::Color32::from_rgb(255,255,255));
            let btn_txt_confirm_relax = egui::RichText::new(self.time_sett.relax_len_btn_stat)
                .color(egui::Color32::from_rgb(255,255,255));


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



                let btn_lock_unlock = egui::Button::new(btn_txt_confirm_study)
                    .fill(egui::Color32::from_rgb(0,0,50))
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


                let btn_reset = egui::Button::new("RESET")
                    .fill(egui::Color32::from_rgb(200, 0, 0))
                    .rounding(25.0);

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

                let btn_lock_unlock = egui::Button::new(btn_txt_confirm_relax)
                    .fill(egui::Color32::from_rgb(0,0,50))
                    .rounding(25.0);

                if ui.add(btn_lock_unlock).clicked() {
                    let enable = &mut self.time_sett.relax_len_slider_enable;
                    if *enable {
                        *enable = false;

                        self.time_sett.relax_len_btn_stat = BTN_STATUSES[1];
                        self.time_sett.relax_len.dur = Duration::minutes(self.time_sett.relax_len.mins);
                        println!(">> Debug: relax time Duration len now = {x:?}", x=self.time_sett.relax_len.dur);
                    } else {
                        *enable = true;
                        self.time_sett.relax_len_btn_stat = BTN_STATUSES[0];
                    }
                }

                // // 2. reset slider
                let btn_reset = egui::Button::new("RESET")
                    .fill(egui::Color32::from_rgb(200, 0, 0))
                    .rounding(25.0);

                if ui.add(btn_reset).clicked() {
                    self.time_sett.relax_len.mins = 0;
                    self.time_sett.relax_len.dur = Duration::minutes(0);
                    println!(">> Debug: relax time Duration len now = {x:?}", x=self.time_sett.relax_len.dur);
                }
            });
            


            // ======== EXTRA

            // row 1
            ui.horizontal(|ui| {

                ui.checkbox(&mut self.music_sett.play_relax_start, egui::WidgetText::from("Play Music At Relax Start"));
                // only if checkbox ticked
                ui.add_enabled(self.music_sett.play_relax_start, |ui: &mut Ui| {
                    let s = &mut self.music_sett.file_path_rlx;
                    ui.text_edit_singleline(s)
                });                
            });

            // row 2
            ui.horizontal(|ui| {

                let chkbox = egui::Checkbox::new(&mut self.music_sett.play_study_start, egui::WidgetText::from("Play Music At Study Start"));
                ui.add(chkbox);
                
                ui.add_enabled(self.music_sett.play_study_start, |ui: &mut Ui| {
                    let s = &mut self.music_sett.file_path_std;
                    ui.text_edit_singleline(s)
                });

            });



            // ===== MAIN BUTTONS

            ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(10.0, 5.0);
                
                let btn_start = egui::Button::new(
                    egui::RichText::new("START NOW")
                        .size(24.0)
                        .family(egui::FontFamily::Monospace)
                );
                let btn_pause = egui::Button::new(
                    egui::RichText::new("PAUSE")
                        .size(24.0)
                        .family(egui::FontFamily::Monospace)
                );
                let btn_stop = egui::Button::new(
                    egui::RichText::new("STOP")
                        .size(24.0)
                        .family(egui::FontFamily::Monospace)
                );

                let start = ui.add_enabled(!self.app_status.is_ongoing(), btn_start);
                let pause = ui.add_enabled(!self.app_status.is_paused(), btn_pause);
                let stop = ui.add_enabled(self.app_status.is_running(), btn_stop);

                if start.clicked() {
                    self.app_status.run();
                }
                if pause.clicked() {
                    self.app_status.pause();
                }
                if stop.clicked() {
                    self.app_status.stop();
                }

            });


        });


        egui::TopBottomPanel::bottom("bottom_panel")
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                ui.label("Made by Lucius Men. Written in Rust & Powered by ");
                ui.hyperlink_to("egui", "https://github.com/emilk/egui");
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