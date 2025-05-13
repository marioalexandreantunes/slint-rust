
use slint::{SharedString, Weak};
use std::sync::{Arc, Mutex};
use crate::AppWindow;

pub struct LoggerState {
    ui: Weak<AppWindow>,
}


impl LoggerState {
    pub fn new(ui: Weak<AppWindow>) -> Self {
        Self { ui }
    }

    pub fn log(&self, message: &str) {
        let message = SharedString::from(message);
        let ui_handle = self.ui.clone();
        
        // We execute the UI update on the main thread
        slint::invoke_from_event_loop(move || {
            let ui = ui_handle.unwrap();
            let mut current_log = ui.get_log_content();
            
            // We add the new message to the existing log
            if !current_log.is_empty() {
                current_log.push_str("\n");
            }
            current_log.push_str(&message);
            
            // We update the text in the UI
            ui.set_log_content(current_log);
        }).expect("Failed to update UI from background thread");
    }
}

// We create a global logger that can be accessed from anywhere
lazy_static::lazy_static! {
    static ref LOGGER: Arc<Mutex<Option<LoggerState>>> = Arc::new(Mutex::new(None));
}


// Function to initialize the global logger
pub fn init_logger(ui: Weak<AppWindow>) {
    let mut logger = LOGGER.lock().unwrap();
    *logger = Some(LoggerState::new(ui));
}

// Public function to add messages to the log from anywhere in the code
pub fn log(message: &str) {
    if let Some(logger) = &*LOGGER.lock().unwrap() {
        logger.log(message);
    }
}
