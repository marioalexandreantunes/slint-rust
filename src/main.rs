// main.rs
use slint::SharedString;
use std::thread;
use std::time::Duration;

mod logger;

slint::include_modules!();


/// Function that simulates a background process that logs messages
fn background_process() {
    let mut counter = 0;
    loop {
        // Simulate some processing
        thread::sleep(Duration::from_secs(2));
        
        // Log a message
        counter += 1;
        logger::log(&format!("Background process: event #{}", counter));
    }
}

fn main() {
    // Load the interface
    let app = AppWindow::new().unwrap();
    
    // Initialize the logger with a weak reference to the UI
    logger::init_logger(app.as_weak());
    
    // Initialize our interface
    logger::log("Application started");
    
    // Create a background thread to demonstrate logging outside the main thread
    thread::spawn(|| {
        // Add a small delay to ensure the UI is ready
        thread::sleep(Duration::from_millis(500));
        
        logger::log("Background thread started");
        background_process();
    });
    
    // Button to manually add a message
    let weak_window = app.as_weak();
    app.on_add_log_message(move |message| {
        if !message.is_empty() {
            logger::log(&format!("Interface: {}", message));
            
            // Clear the input field
            weak_window.unwrap().set_input_text(SharedString::new());
        }
    });
    
    // Run the main application loop
    app.run().unwrap();
}
