use std::sync::{Arc, Mutex};

use crate::{StdDuration, Instant};



pub enum RuntimeCommand {
    Run,
    Pause,
    Resume,
    Stop
}


#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct AppStatus {

    #[serde(skip)]
    shared_state: SharedState,

    is_ongoing: bool,
    study_or_relax: StudyRelaxStatus,
    is_paused: bool, // NOT is_stopped
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

        if let Ok(_) = self.execute_command(RuntimeCommand::Run) {
            self.is_ongoing = true;
            self.is_paused = false;
        }


    }

    pub fn pause(&mut self) {
        if let Ok(_) = self.execute_command(RuntimeCommand::Pause) {
            self.is_paused = true;
        }
    }

    pub fn resume(&mut self) {
        if let Ok(_) = self.execute_command(RuntimeCommand::Resume) {
            self.is_paused = false;
        }
    }



    pub fn stop(&mut self) {

        if let Ok(_) = self.execute_command(RuntimeCommand::Stop) {
            self.is_ongoing = false;
            self.is_paused = false;    
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
                    state.start_time = Some(Instant::now());
                    state.paused_time = None;

                    Ok(())
                } else {
                    Err(">> Countdown already running.")
                }
            },
            RuntimeCommand::Pause => {
                if let Some(start_time) = state.start_time {
                    if state.paused_time.is_none() {
                        state.paused_time = Some(Instant::now() - start_time);
                        
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
                    state.start_time = Some(Instant::now() - paused_time);
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
    duration: StdDuration,
    start_time: Option<Instant>,
    paused_time: Option<StdDuration>
}

impl CountdownState {
    fn new(duration: StdDuration) -> Self {
        CountdownState { duration, start_time: None, paused_time: None }
    }


}




pub type SharedState = Arc<Mutex<CountdownState>>;


