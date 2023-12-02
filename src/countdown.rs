

////// ====================== MOVED from APP.rs


use std::{sync::{Arc, Mutex}, time::Duration};

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

    #[serde(skip)]
    pub timer_vals: Arc<Mutex<TimerVals>>,
}

#[derive(Debug, Default)]
pub struct TimerVals {
    hr: u8,
    min: u8,
    sec: u8,
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

            timer_vals: Arc::new(Mutex::new(TimerVals::default())),
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
                        assert!(
                            s.remaining_time.is_some()
                            && s.start_time.as_ref().lock().unwrap().is_some(),
                            "!! Assertion failed: Failed to read remaining and/or starting time, OR they are not set"
                        );

                        let t_dur = s.remaining_time.clone().unwrap();
                        let t_start_armut = s.start_time.clone().lock().unwrap().unwrap();

                        // 1 sec

                        // todo!("Finish Countdown Thread part")
                        let countdown_thread = std::thread::spawn(move || {
                            for _ in 1..t_dur.num_seconds() {
                                // every one second, alter the text
                                
                                self.timer_vals.get_mut().expect("!! Failed to get Arc Mut timer_vals").sec;

                                std::thread::sleep(Duration::from_secs(1));
                                // todo!("FINISH COUNTDOWN");
                            }
                        });

                        countdown_thread.join().expect("!! Thread error not expected");
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
        
        let mut curr_state = match self.shared_state.try_lock() {
            Ok(state) => state,
            Err(_) => return Err("!! Failed to lock state"),
        };
    
        match command {
            RuntimeCommand::Run => {
                if let Ok(mut st_mutgrd) = curr_state.start_time.try_lock() {
                    *st_mutgrd = Some(ChrLocal::now());
                                       

                    Ok(())

                    //// TODO: 開始倒計時
                    // todo!()                
                } else {
                    Err(">> Countdown already running.")
                }
            },
            RuntimeCommand::Pause => {

                if let Ok(mut st_mutgrd) = curr_state.start_time.clone().try_lock() {


                    let now = ChrLocal::now();
                    let old = if let Some(time) = *st_mutgrd {
                        time
                    } else {
                        return Err("!! When running Pause Command: Old start time not found!");
                    };
                    //// update "start time" to curr
                    *st_mutgrd = Some( now );

                    //// reduce "duration" to original start - new "start time"
                    curr_state.remaining_time = Some ( now.signed_duration_since(old) );

                    Ok(())


                } else {
                    Err(">> Countdown not running.")
                }
            },
            RuntimeCommand::Resume => {

                if let Ok(mut st_mutgrd) = curr_state.start_time.try_lock() {
                    
                    // TODO: 開始倒計時

                    Ok(())
                } else {
                    Err(">> Countdown not running.")
                }

            },
            RuntimeCommand::Stop => {

                if let Ok(mut st_mutg) = curr_state.start_time.clone().try_lock() {
                    *st_mutg = None;

                    curr_state.remaining_time = None;
                    
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
    start_time: ArMut<Option<ChrDateTime<ChrLocal>>>,
    remaining_time: Option<ChrDuration>

}

// impl Copy for CountdownState {
    

// }



impl CountdownState {
    fn new(duration: ChrDuration) -> Self {
        CountdownState {
            start_time: Arc::new(Mutex::new(None)),
            remaining_time: None
        }
    }

    pub fn update_remaining(&mut self, new_dur: Option<ChrDuration>) {
        self.remaining_time = new_dur;
    }
}