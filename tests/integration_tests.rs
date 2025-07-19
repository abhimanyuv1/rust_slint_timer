use std::rc::Rc;
use std::cell::RefCell;
use rust_slint_timer::timer::{TimerLogic, TimerState};

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_timer_logic_integration() {
        // Test complete timer workflow
        let mut timer = TimerLogic::with_time(0, 0, 3).unwrap();
        
        // Test initial state
        assert!(!timer.is_running());
        assert!(!timer.is_completed());
        assert_eq!(timer.get_remaining_time_string(), "00:00:03");
        
        // Test start functionality
        timer.start_timer();
        assert!(timer.is_running());
        assert!(!timer.is_completed());
        
        // Test tick progression
        assert!(!timer.tick()); // 3 -> 2
        assert_eq!(timer.get_remaining_time_string(), "00:00:02");
        assert!(timer.is_running());
        
        assert!(!timer.tick()); // 2 -> 1
        assert_eq!(timer.get_remaining_time_string(), "00:00:01");
        assert!(timer.is_running());
        
        assert!(timer.tick()); // 1 -> 0 (completion)
        assert_eq!(timer.get_remaining_time_string(), "00:00:00");
        assert!(!timer.is_running());
        assert!(timer.is_completed());
        
        // Test reset functionality
        timer.reset_timer();
        assert!(!timer.is_running());
        assert!(!timer.is_completed());
        assert_eq!(timer.get_remaining_time_string(), "00:00:03");
    }

    #[test]
    fn test_callback_integration() {
        use std::sync::{Arc, Mutex};
        
        let mut timer = TimerLogic::with_time(0, 0, 2).unwrap();
        let callback_states = Arc::new(Mutex::new(Vec::new()));
        let callback_states_clone = callback_states.clone();
        
        // Set up callback to capture state changes
        timer.set_state_callback(move |state| {
            callback_states_clone.lock().unwrap().push(state.clone());
        });
        
        // Test complete workflow with callbacks
        timer.start_timer(); // Should trigger callback
        timer.tick(); // Should trigger callback
        timer.tick(); // Should trigger callback twice (tick + completion)
        
        let states = callback_states.lock().unwrap();
        assert_eq!(states.len(), 4); // start + tick + tick + completion
        
        // Verify state progression
        assert!(states[0].is_running && !states[0].is_completed); // start
        assert_eq!(states[1].remaining_seconds, 1); // first tick
        assert_eq!(states[2].remaining_seconds, 0); // second tick
        assert!(!states[3].is_running && states[3].is_completed); // completion
    }

    #[test]
    fn test_validation_integration() {
        // Test validation with various inputs
        assert!(TimerLogic::with_time(0, 0, 1).is_ok());
        assert!(TimerLogic::with_time(23, 59, 59).is_ok());
        assert!(TimerLogic::with_time(1, 30, 45).is_ok());
        
        // Test invalid inputs
        assert!(TimerLogic::with_time(24, 0, 0).is_err());
        assert!(TimerLogic::with_time(0, 60, 0).is_err());
        assert!(TimerLogic::with_time(0, 0, 60).is_err());
        assert!(TimerLogic::with_time(0, 0, 0).is_err());
        
        // Test set_time validation
        let mut timer = TimerLogic::new();
        assert!(timer.set_time(2, 15, 30).is_ok());
        assert_eq!(timer.get_remaining_time_string(), "02:15:30");
        
        // Invalid set_time should not change state
        assert!(timer.set_time(25, 0, 0).is_err());
        assert_eq!(timer.get_remaining_time_string(), "02:15:30");
    }

    #[test]
    fn test_pause_resume_integration() {
        let mut timer = TimerLogic::with_time(0, 0, 5).unwrap();
        
        // Start and tick once
        timer.start_timer();
        timer.tick();
        assert_eq!(timer.get_remaining_time_string(), "00:00:04");
        assert!(timer.is_running());
        
        // Pause and verify tick doesn't progress
        timer.pause_timer();
        assert!(!timer.is_running());
        timer.tick();
        assert_eq!(timer.get_remaining_time_string(), "00:00:04"); // Should not change
        
        // Resume and verify tick works again
        timer.start_timer();
        assert!(timer.is_running());
        timer.tick();
        assert_eq!(timer.get_remaining_time_string(), "00:00:03");
    }
}