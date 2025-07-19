mod timer;

use std::rc::Rc;
use std::cell::RefCell;
use slint::ComponentHandle;
use timer::{TimerLogic, TimerState};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = TimerWindow::new()?;
    
    // Create timer logic instance
    let timer_logic = Rc::new(RefCell::new(TimerLogic::new()));
    
    // Set up initial UI state
    ui.set_input_hours(0);
    ui.set_input_minutes(5);
    ui.set_input_seconds(0);
    ui.set_display_time("00:05:00".into());
    ui.set_is_running(false);
    ui.set_is_completed(false);
    
    // Set up time input change callback
    {
        let ui_weak = ui.as_weak();
        let timer_logic_clone = timer_logic.clone();
        ui.on_time_input_changed(move |hours, minutes, seconds| {
            let ui = ui_weak.unwrap();
            let mut timer = timer_logic_clone.borrow_mut();
            
            // Sanitize inputs - ensure non-negative values
            let hours = if hours < 0 { 0 } else { hours as u32 };
            let minutes = if minutes < 0 { 0 } else { minutes as u32 };
            let seconds = if seconds < 0 { 0 } else { seconds as u32 };
            
            // Validate and set time
            match timer.set_time(hours, minutes, seconds) {
                Ok(()) => {
                    let time_str = timer.get_remaining_time_string();
                    ui.set_display_time(time_str.into());
                    ui.set_is_completed(false);
                }
                Err(error) => {
                    // Handle validation error with graceful recovery
                    println!("Timer validation error: {}", error);
                    // Keep the previous valid state - don't update display
                }
            }
        });
    }
    
    // Set up start/pause button callback
    {
        let ui_weak = ui.as_weak();
        let timer_logic_clone = timer_logic.clone();
        ui.on_start_pause_clicked(move || {
            let ui = ui_weak.unwrap();
            let mut timer = timer_logic_clone.borrow_mut();
            
            if timer.is_running() {
                timer.pause_timer();
                println!("Timer paused");
            } else {
                // Before starting, ensure timer has the current input values
                let hours = ui.get_input_hours() as u32;
                let minutes = ui.get_input_minutes() as u32;
                let seconds = ui.get_input_seconds() as u32;
                
                println!("Starting timer with: {}:{}:{}", hours, minutes, seconds);
                
                // Always set the time from current inputs when starting
                match timer.set_time(hours, minutes, seconds) {
                    Ok(()) => {
                        let time_str = timer.get_remaining_time_string();
                        ui.set_display_time(time_str.into());
                        ui.set_is_completed(false);
                        
                        timer.start_timer();
                        println!("Timer started, is_running: {}, remaining: {}", 
                                timer.is_running(), timer.get_state().remaining_seconds);
                    }
                    Err(error) => {
                        println!("Timer validation error: {}", error);
                        return; // Don't start if invalid time
                    }
                }
            }
            
            // Update UI state
            ui.set_is_running(timer.is_running());
            ui.set_is_completed(timer.is_completed());
        });
    }
    
    // Set up reset button callback
    {
        let ui_weak = ui.as_weak();
        let timer_logic_clone = timer_logic.clone();
        ui.on_reset_clicked(move || {
            let ui = ui_weak.unwrap();
            let mut timer = timer_logic_clone.borrow_mut();
            
            timer.reset_timer();
            
            // Update UI state
            let time_str = timer.get_remaining_time_string();
            ui.set_display_time(time_str.into());
            ui.set_is_running(timer.is_running());
            ui.set_is_completed(timer.is_completed());
        });
    }
    
    // Set up real-time timer updates with direct UI updates
    let ui_weak = ui.as_weak();
    let timer_logic_clone = timer_logic.clone();
    let timer = slint::Timer::default();
    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_secs(1), move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut timer_logic = timer_logic_clone.borrow_mut();
            let was_running = timer_logic.is_running();
            let old_remaining = timer_logic.get_state().remaining_seconds;
            
            timer_logic.tick();
            
            // Update UI directly after tick
            let state = timer_logic.get_state();
            let time_str = format!("{:02}:{:02}:{:02}", 
                state.remaining_seconds / 3600,
                (state.remaining_seconds % 3600) / 60,
                state.remaining_seconds % 60
            );
            
            if was_running {
                println!("Tick: {} -> {}, running: {}", old_remaining, state.remaining_seconds, state.is_running);
            }
            
            ui.set_display_time(time_str.into());
            ui.set_is_running(state.is_running);
            ui.set_is_completed(state.is_completed);
        }
    });
    
    // Keep the timer alive by storing it
    std::mem::forget(timer);
    
    ui.run()
}
