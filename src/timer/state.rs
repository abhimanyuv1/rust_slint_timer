#[derive(Debug, Clone)]
pub struct TimerState {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub remaining_seconds: u32,
    pub is_running: bool,
    pub is_completed: bool,
}

impl Default for TimerState {
    fn default() -> Self {
        Self {
            hours: 0,
            minutes: 0,
            seconds: 0,
            remaining_seconds: 0,
            is_running: false,
            is_completed: false,
        }
    }
}

impl TimerState {
    pub fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        let total_seconds = hours * 3600 + minutes * 60 + seconds;
        Self {
            hours,
            minutes,
            seconds,
            remaining_seconds: total_seconds,
            is_running: false,
            is_completed: false,
        }
    }

    pub fn reset(&mut self) {
        self.remaining_seconds = self.hours * 3600 + self.minutes * 60 + self.seconds;
        self.is_running = false;
        self.is_completed = false;
    }

    pub fn format_remaining_time(&self) -> String {
        let hours = self.remaining_seconds / 3600;
        let minutes = (self.remaining_seconds % 3600) / 60;
        let seconds = self.remaining_seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}