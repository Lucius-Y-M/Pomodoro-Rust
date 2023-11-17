

////// ====================== MOVED from APP.rs


use std::sync::{Arc, Mutex};

use crate::{ChrLocal, ChrDuration, ChrDateTime, ArMut};



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

    is_ongoing: ArMut<bool>,
    study_or_relax: StudyRelaxStatus,
    is_paused: ArMut<bool>, // NOT is_stopped

    pub study_len: i64,
    pub study_len_slider_enable: bool,
    pub study_len_btn_stat: &'static str,

    pub relax_len: i64,
    pub relax_len_slider_enable: bool,
    pub relax_len_btn_stat: &'static str,

}

pub type SharedState = ArMut<CountdownState>;






impl Default for AppStatus {
    fn default() -> Self {
        Self {
            shared_state: Arc::new(Mutex::new(CountdownState::default())),
            study_len_btn_stat: BTN_STATUS_CONF[0],
            relax_len_btn_stat: BTN_STATUS_CONF[0],
            


            is_ongoing: Arc::new(Mutex::new(false)),
            study_or_relax: StudyRelaxStatus::Study,
            is_paused: Arc::new(Mutex::new(false)),
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
        *self.is_ongoing.clone().lock().unwrap()
    }

    pub fn is_paused(&self) -> bool {
        *self.is_paused.clone().lock().unwrap()
    }


    pub fn is_running(&self) -> bool {
        self.is_ongoing() & !self.is_paused()
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
                
                match self.shared_state.try_lock() {
                    Ok(mut s) => {
                        *self.is_ongoing.lock().unwrap() = true;
                        *self.is_paused.lock().unwrap() = false;


                        //// TODO: perhaps not assert
                        assert!(s.duration.is_some() && s.start_time.clone().lock().unwrap().is_some(), "!! Assertion failed: No duration or start time set");

                        let t_dur = s.duration.as_ref().unwrap();
                        let t_start_armut = s.start_time.clone().lock().unwrap().unwrap();

                        s.remaining_time = Some(t_start_armut.checked_sub_signed(*t_dur).unwrap());


                        // 1 sec

                        todo!("Finish Countdown Thread part")
                        // let countdown_thread = std::thread::spawn(move || {
                        //     for i in 1..t_dur.num_seconds() {

                                
                        //     }
                        // });


                    },
                    Err(_) => {
                        println!("!! Run: Failed to lock.");
                    },
                }

            },
            Err(_) => {},
        }


    }



    //// === Logic for Pause() and Resume()
    fn __aux_pause_resume_logic(&mut self, should_pause: bool) -> Result<(), ()> {
        match self.execute_command(RuntimeCommand::Pause) {
            Ok(_) => {

                match self.is_paused.try_lock() {
                    Ok(mut ip) => {
                        *ip = should_pause;
                        Ok(())
                    },
                    Err(e) => {
                        println!("!! Failed to Pause/Resume! Reason: {e}");
                        Err(())
                    },
                }
            },
            
            Err(e) => {
                println!("!! Failed to Pause/Resume! Reason: {e}");
                Err(())
            },
        }
    }
    //// === Logic for Pause() and Resume()


    pub fn pause(&mut self) -> Result<(), ()> {
        self.__aux_pause_resume_logic(true)
    }

    pub fn resume(&mut self) -> Result<(), ()> {
        self.__aux_pause_resume_logic(false)
    }



    pub fn stop(&mut self) -> Result<(), ()> {

        match self.execute_command(RuntimeCommand::Stop) {
            Ok(_) => {

                match self.is_ongoing.try_lock() {
                    Ok(mut io) => {
                        *io = false;
                    },
                    Err(e) => {
                        println!("!! Failed to Stop! Reason: {e}");
                        return Err(());
                    },
                }

                match self.is_paused.try_lock() {
                    Ok(mut ip) => {
                        *ip = false;
                        Ok(())
                    },
                    Err(e) => {
                        println!("!! Failed to Stop! Reason: {e}");
                        println!("!!!! Potential Catastrophic Failure, REOPEN THE PROGRAM TO AVOID ABNORMAL STATE");
                        Err(())
                    },
                }


            },
            
            Err(e) => {
                println!("!! Failed to Stop! Reason: {e}");
                Err(())
            },
        }
    }




    fn execute_command(&mut self, command: RuntimeCommand) -> Result<(), &'static str> {
        
        let mut state = match self.shared_state.try_lock() {
            Ok(state) => state,
            Err(_) => return Err("!! Failed to lock state"),
        };
    
        match command {
            RuntimeCommand::Run => {
                if let Ok(mut s_t_mg) = state.start_time.try_lock() {
                    *s_t_mg = Some(ChrLocal::now());
                                       

                    Ok(())

                    //// TODO: 可能還有問題
                    // todo!()                
                } else {
                    Err(">> Countdown already running.")
                }
            },
            RuntimeCommand::Pause => {

                if let Ok(start_time) = state.start_time.try_lock() {
                    
                    match *start_time {
                        Some(start_time) => {
                            
                            state.paused_time = Some( ChrLocal::now().signed_duration_since(start_time) );

                            Ok(())
                            
                        },
                        None => { Err("!! Failed to acquire start time mutex for Pause Command!") },
                    }
                } else {
                    Err(">> Countdown not running.")
                }
            },
            RuntimeCommand::Resume => {

                if let Ok(start_time) = state.start_time.try_lock() {
                    
                    match *start_time {
                        Some(mut start_time) => {
                            
                            start_time = ChrLocal::now() - state.paused_time.expect("!!>> Unwrapped Paused Time When None!");
                            state.paused_time = None;

                            Ok(())

                        },
                        None => { Err("!! Failed to acquire start time mutex for Resume Command!") },
                    }
                } else {
                    Err(">> Countdown not running.")
                }

                // if let Some(paused_time) = state.paused_time {
                //     state.start_time = Some(ChrLocal::now() - paused_time);
                //     state.paused_time = None;

                //     Ok(())
                // } else {
                //     Err(">> Countdown is not running.")
                // }

            },
            RuntimeCommand::Stop => {

                if let Ok(mut s_t_mg) = state.start_time.try_lock() {
                    *s_t_mg = None;
                    state.paused_time = None;
                    
                    Ok(())

                } else {
                    Err("!! Failed to acquire start time lock for Stop Command!")
                }

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


#[derive(Debug, Default, Clone)]
pub struct CountdownState {
    duration: Option<ChrDuration>,
    start_time: ArMut<Option<ChrDateTime<ChrLocal>>>,
    paused_time: Option<ChrDuration>,
    remaining_time: Option<ChrDateTime<ChrLocal>>

}

// impl Copy for CountdownState {
    

// }



impl CountdownState {
    fn new(duration: ChrDuration) -> Self {
        CountdownState {
            duration: Some(duration),
            start_time: Arc::new(Mutex::new(None)),
            paused_time: None,
            remaining_time: None
        }
    }

    pub fn update_duration(&mut self, new_dur: Option<ChrDuration>) {
        self.duration = new_dur;
    }
}