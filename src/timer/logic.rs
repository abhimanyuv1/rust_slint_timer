/// Validates time input values according to standard time constraints
/// 
/// # Arguments
/// * `hours` - Hours value (must be 0-23)
/// * `minutes` - Minutes value (must be 0-59)  
/// * `seconds` - Seconds value (must be 0-59)
/// 
/// # Returns
/// * `Ok(())` if all values are valid
/// * `Err(String)` with descriptive error message if any value is invalid
pub fn validate_time(hours: u32, minutes: u32, seconds: u32) -> Result<(), String> {
    if hours > 23 {
        return Err(format!("Hours must be between 0 and 23, got {}", hours));
    }
    
    if minutes > 59 {
        return Err(format!("Minutes must be between 0 and 59, got {}", minutes));
    }
    
    if seconds > 59 {
        return Err(format!("Seconds must be between 0 and 59, got {}", seconds));
    }
    
    // Check if all values are zero (invalid timer)
    if hours == 0 && minutes == 0 && seconds == 0 {
        return Err("Timer duration cannot be zero".to_string());
    }
    
    Ok(())
}

use crate::timer::TimerState;

/// TimerLogic manages the timer state and provides methods for controlling the timer
pub struct TimerLogic {
    state: TimerState,
    callback: Option<Box<dyn Fn(TimerState) + Send>>,
}

impl TimerLogic {
    /// Creates a new TimerLogic instance with default state
    pub fn new() -> Self {
        Self {
            state: TimerState::default(),
            callback: None,
        }
    }

    /// Creates a new TimerLogic instance with specified time
    pub fn with_time(hours: u32, minutes: u32, seconds: u32) -> Result<Self, String> {
        validate_time(hours, minutes, seconds)?;
        Ok(Self {
            state: TimerState::new(hours, minutes, seconds),
            callback: None,
        })
    }

    /// Sets a callback function to be called when the timer state changes
    pub fn set_state_callback<F>(&mut self, callback: F)
    where
        F: Fn(TimerState) + Send + 'static,
    {
        self.callback = Some(Box::new(callback));
    }

    /// Gets the current timer state
    pub fn get_state(&self) -> &TimerState {
        &self.state
    }

    /// Sets new time values for the timer
    pub fn set_time(&mut self, hours: u32, minutes: u32, seconds: u32) -> Result<(), String> {
        validate_time(hours, minutes, seconds)?;
        self.state = TimerState::new(hours, minutes, seconds);
        self.notify_state_change();
        Ok(())
    }

    /// Starts the timer
    pub fn start_timer(&mut self) {
        if !self.state.is_completed && self.state.remaining_seconds > 0 {
            self.state.is_running = true;
            self.state.is_completed = false;
            self.notify_state_change();
        }
    }

    /// Pauses the timer
    pub fn pause_timer(&mut self) {
        if self.state.is_running {
            self.state.is_running = false;
            self.notify_state_change();
        }
    }

    /// Resets the timer to its original time
    pub fn reset_timer(&mut self) {
        self.state.reset();
        self.notify_state_change();
    }

    /// Checks if the timer is currently running
    pub fn is_running(&self) -> bool {
        self.state.is_running
    }

    /// Checks if the timer has completed
    pub fn is_completed(&self) -> bool {
        self.state.is_completed
    }

    /// Gets the remaining time as a formatted string
    pub fn get_remaining_time_string(&self) -> String {
        self.state.format_remaining_time()
    }

    /// Called every second to update countdown when timer is running
    /// Returns true if timer completed, false otherwise
    pub fn tick(&mut self) -> bool {
        if !self.state.is_running || self.state.is_completed {
            return false;
        }

        if self.state.remaining_seconds > 0 {
            self.state.remaining_seconds -= 1;
            self.notify_state_change();
            
            // Check if timer completed
            if self.state.remaining_seconds == 0 {
                self.state.is_running = false;
                self.state.is_completed = true;
                self.notify_state_change();
                return true;
            }
        }
        
        false
    }

    /// Private method to notify state changes via callback
    fn notify_state_change(&self) {
        if let Some(ref callback) = self.callback {
            callback(self.state.clone());
        }
    }
}

impl Default for TimerLogic {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_time_inputs() {
        // Test valid boundary values
        assert!(validate_time(0, 0, 1).is_ok());
        assert!(validate_time(23, 59, 59).is_ok());
        assert!(validate_time(1, 30, 45).is_ok());
        assert!(validate_time(0, 5, 0).is_ok());
        assert!(validate_time(2, 0, 0).is_ok());
    }

    #[test]
    fn test_invalid_hours() {
        assert!(validate_time(24, 0, 0).is_err());
        assert!(validate_time(25, 30, 45).is_err());
        
        let error = validate_time(24, 0, 0).unwrap_err();
        assert!(error.contains("Hours must be between 0 and 23"));
    }

    #[test]
    fn test_invalid_minutes() {
        assert!(validate_time(0, 60, 0).is_err());
        assert!(validate_time(12, 75, 30).is_err());
        
        let error = validate_time(0, 60, 0).unwrap_err();
        assert!(error.contains("Minutes must be between 0 and 59"));
    }

    #[test]
    fn test_invalid_seconds() {
        assert!(validate_time(0, 0, 60).is_err());
        assert!(validate_time(5, 30, 90).is_err());
        
        let error = validate_time(0, 0, 60).unwrap_err();
        assert!(error.contains("Seconds must be between 0 and 59"));
    }

    #[test]
    fn test_zero_duration() {
        assert!(validate_time(0, 0, 0).is_err());
        
        let error = validate_time(0, 0, 0).unwrap_err();
        assert!(error.contains("Timer duration cannot be zero"));
    }

    // TimerLogic tests
    #[test]
    fn test_timer_logic_creation() {
        let timer = TimerLogic::new();
        assert!(!timer.is_running());
        assert!(!timer.is_completed());
        assert_eq!(timer.get_remaining_time_string(), "00:00:00");
    }

    #[test]
    fn test_timer_logic_with_time() {
        let timer = TimerLogic::with_time(1, 30, 45).unwrap();
        assert!(!timer.is_running());
        assert!(!timer.is_completed());
        assert_eq!(timer.get_remaining_time_string(), "01:30:45");
    }

    #[test]
    fn test_timer_logic_invalid_time() {
        assert!(TimerLogic::with_time(25, 0, 0).is_err());
        assert!(TimerLogic::with_time(0, 60, 0).is_err());
        assert!(TimerLogic::with_time(0, 0, 60).is_err());
        assert!(TimerLogic::with_time(0, 0, 0).is_err());
    }

    #[test]
    fn test_timer_state_transitions() {
        let mut timer = TimerLogic::with_time(0, 0, 5).unwrap();
        
        // Initial state
        assert!(!timer.is_running());
        assert!(!timer.is_completed());
        
        // Start timer
        timer.start_timer();
        assert!(timer.is_running());
        assert!(!timer.is_completed());
        
        // Pause timer
        timer.pause_timer();
        assert!(!timer.is_running());
        assert!(!timer.is_completed());
        
        // Reset timer
        timer.reset_timer();
        assert!(!timer.is_running());
        assert!(!timer.is_completed());
        assert_eq!(timer.get_remaining_time_string(), "00:00:05");
    }

    #[test]
    fn test_set_time() {
        let mut timer = TimerLogic::new();
        
        // Set valid time
        assert!(timer.set_time(2, 15, 30).is_ok());
        assert_eq!(timer.get_remaining_time_string(), "02:15:30");
        
        // Set invalid time
        assert!(timer.set_time(25, 0, 0).is_err());
        // State should remain unchanged after error
        assert_eq!(timer.get_remaining_time_string(), "02:15:30");
    }

    #[test]
    fn test_tick_functionality() {
        let mut timer = TimerLogic::with_time(0, 0, 3).unwrap();
        
        // Tick should return false when timer is not running
        assert!(!timer.tick());
        assert_eq!(timer.get_remaining_time_string(), "00:00:03");
        
        // Start timer and test countdown
        timer.start_timer();
        assert!(timer.is_running());
        assert!(!timer.is_completed());
        
        // First tick: 3 -> 2 seconds
        assert!(!timer.tick());
        assert_eq!(timer.get_remaining_time_string(), "00:00:02");
        assert!(timer.is_running());
        assert!(!timer.is_completed());
        
        // Second tick: 2 -> 1 seconds
        assert!(!timer.tick());
        assert_eq!(timer.get_remaining_time_string(), "00:00:01");
        assert!(timer.is_running());
        assert!(!timer.is_completed());
        
        // Final tick: 1 -> 0 seconds (completion)
        assert!(timer.tick());
        assert_eq!(timer.get_remaining_time_string(), "00:00:00");
        assert!(!timer.is_running());
        assert!(timer.is_completed());
        
        // Tick after completion should return false
        assert!(!timer.tick());
        assert_eq!(timer.get_remaining_time_string(), "00:00:00");
    }

    #[test]
    fn test_tick_when_paused() {
        let mut timer = TimerLogic::with_time(0, 0, 5).unwrap();
        timer.start_timer();
        
        // Tick once to decrement
        assert!(!timer.tick());
        assert_eq!(timer.get_remaining_time_string(), "00:00:04");
        
        // Pause timer
        timer.pause_timer();
        assert!(!timer.is_running());
        
        // Tick should not decrement when paused
        assert!(!timer.tick());
        assert_eq!(timer.get_remaining_time_string(), "00:00:04");
        
        // Resume and tick should work again
        timer.start_timer();
        assert!(!timer.tick());
        assert_eq!(timer.get_remaining_time_string(), "00:00:03");
    }

    #[test]
    fn test_callback_on_tick() {
        use std::sync::{Arc, Mutex};
        
        let mut timer = TimerLogic::with_time(0, 0, 2).unwrap();
        let callback_states = Arc::new(Mutex::new(Vec::new()));
        let callback_states_clone = callback_states.clone();
        
        timer.set_state_callback(move |state| {
            callback_states_clone.lock().unwrap().push(state.clone());
        });
        
        timer.start_timer(); // Should trigger callback
        timer.tick(); // Should trigger callback
        timer.tick(); // Should trigger callback twice (tick + completion)
        
        let states = callback_states.lock().unwrap();
        assert_eq!(states.len(), 4); // start + tick + tick + completion
        
        // Check that remaining seconds decreased properly
        assert_eq!(states[1].remaining_seconds, 1);
        assert_eq!(states[2].remaining_seconds, 0);
        assert!(states[3].is_completed);
    }
}