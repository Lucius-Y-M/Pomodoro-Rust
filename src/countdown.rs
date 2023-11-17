

////// ====================== MOVED from APP.rs

use std::sync::{Arc, Mutex};

use crate::{ChrLocal, ChrDuration, ChrDateTime};



pub enum EnOrDis {
    Enable,
    Disable
}



////// ====================== MOVED from APP.rs

pub const BTN_CONFIRM_STR : &str = "Confirm";
pub const BTN_RECHOOSE_STR : &str = "Re-Choose";
pub const BTN_STATUS_CONF : [&str; 2] = [BTN_CONFIRM_STR, BTN_RECHOOSE_STR];


pub const BTN_PAUSE_STR : &str = "PAUSE";
pub const BTN_RESUME_STR : &str = "RESUME";
pub const BTN_STATUS_PAUSE : [&str; 2] = [BTN_PAUSE_STR, BTN_RESUME_STR];




pub enum RuntimeCommand {
    Run,
    Pause,
    Resume,
    Stop
}


#[derive(serde::Deserialize, serde::Serialize)]
pub struct AppStatus {

    #[serde(skip)]
    pub shared_state: SharedState,

    is_ongoing: bool,
    study_or_relax: StudyRelaxStatus,
    is_paused: bool, // NOT is_stopped

    pub study_len: i64,
    pub study_len_slider_enable: bool,
    pub study_len_btn_stat: &'static str,

    pub relax_len: i64,
    pub relax_len_slider_enable: bool,
    pub relax_len_btn_stat: &'static str,


}

pub type SharedState = Arc<Mutex<CountdownState>>;






impl Default for AppStatus {
    fn default() -> Self {
        Self {
            shared_state: Arc::new(Mutex::new(CountdownState::default())),
            study_len_btn_stat: BTN_STATUS_CONF[0],
            relax_len_btn_stat: BTN_STATUS_CONF[0],
            


            is_ongoing: false,
            study_or_relax: StudyRelaxStatus::Study,
            is_paused: false,
            study_len: 0i64,
            study_len_slider_enable: true,
            relax_len: 0i64,
            relax_len_slider_enable: true,
        }
    }
}


impl AppStatus {

    // ======== GETTERS
    pub fn is_ongoing(&self) -> bool {
        self.is_ongoing
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }


    pub fn is_running(&self) -> bool {
        self.is_ongoing & !self.is_paused
    }

    pub fn study_or_relax(&self) -> StudyRelaxStatus {
        self.study_or_relax
    }


    // ======== SETTERS
    pub fn revert_study_relax(&mut self) {
        self.study_or_relax = match self.study_or_relax {
            StudyRelaxStatus::Study => StudyRelaxStatus::Relax,
            StudyRelaxStatus::Relax => StudyRelaxStatus::Study,
        };
    }



    // ======== RUNNERS



    pub fn run(&mut self) {

        match self.execute_command(RuntimeCommand::Run) {
            Ok(_) => {
                self.is_ongoing = true;
                self.is_paused = false;
            },
            Err(_) => {},
        }


    }

    pub fn pause(&mut self) {
        match self.execute_command(RuntimeCommand::Pause) {
            Ok(_) => { self.is_paused = true; },
            Err(_) => {},
            // Err(e) => { println!("{e}"); },
        }
    }

    pub fn resume(&mut self) {
        match self.execute_command(RuntimeCommand::Resume) {
            Ok(_) => { self.is_paused = false; },
            Err(_) => {},
            // Err(e) => { println!("{e}"); },
            
        }
    }



    pub fn stop(&mut self) {

        match self.execute_command(RuntimeCommand::Stop) {
            Ok(_) => {
                self.is_ongoing = false;
                self.is_paused = false;
            },
            Err(_) => todo!(),
        }
    }




    fn execute_command(&mut self, command: RuntimeCommand) -> Result<(), &'static str> {
        
        let mut state = match self.shared_state.try_lock() {
            Ok(state) => state,
            Err(_) => return Err("!! Failed to lock state"),
        };
    
        match command {
            RuntimeCommand::Run => {
                if state.start_time.is_none() {
                    state.start_time = Some(ChrLocal::now());
                    state.paused_time = None;

                    Ok(())
                } else {
                    Err(">> Countdown already running.")
                }
            },
            RuntimeCommand::Pause => {
                if let Some(start_time) = state.start_time {
                    if state.paused_time.is_none() {
                        state.paused_time = Some( ChrLocal::now().signed_duration_since(start_time) );
                        
                        Ok(())
                    } else {
                        Err(">> Countdown already paused.")
                    }
                } else {
                    Err(">> Countdown not running.")
                }
            },
            RuntimeCommand::Resume => {
                if let Some(paused_time) = state.paused_time {
                    state.start_time = Some(ChrLocal::now() - paused_time);
                    state.paused_time = None;

                    Ok(())
                } else {
                    Err(">> Countdown is not running.")
                }
            },
            RuntimeCommand::Stop => {
                state.start_time = None;
                state.paused_time = None;
                
                Ok(())
            },
        }
    
    }    

}



#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Copy)]
pub enum StudyRelaxStatus {
    #[default]
    Study,
    Relax
}


#[derive(Debug, Default, Clone, Copy)]
pub struct CountdownState {
    duration: Option<ChrDuration>,
    start_time: Option<ChrDateTime<ChrLocal>>,
    paused_time: Option<ChrDuration>,
    remaining_time: Option<ChrDateTime<ChrLocal>>

}


impl CountdownState {
    fn new(duration: ChrDuration) -> Self {
        CountdownState { duration: Some(duration), start_time: None, paused_time: None, remaining_time: None }
    }

    pub fn update_duration(&mut self, new_dur: Option<ChrDuration>) {
        self.duration = new_dur;
    }
}