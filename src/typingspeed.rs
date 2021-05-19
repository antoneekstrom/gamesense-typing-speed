use std::time::{Duration, Instant};

#[derive(Debug)]
struct KeyPress {
    pub time: Instant,
}
pub struct TypingSpeedTracker {
    key_presses: Vec<KeyPress>,
    timeout: Duration,
}

pub struct TypingSpeed {
    duration: Duration,
    keys_pressed: usize,
}

impl TypingSpeedTracker {
    const MAX_KEY_PRESSES: usize = 700;

    pub fn new(timeout: Duration) -> Self {
        return TypingSpeedTracker {
            key_presses: Vec::with_capacity(TypingSpeedTracker::MAX_KEY_PRESSES),
            timeout,
        };
    }

    pub fn press_key(&mut self) {
        let key_presses = &mut self.key_presses;

        if key_presses.len() > 0 {
            let last_key_press = &key_presses[0];
            if last_key_press.time.elapsed() > self.timeout {
                key_presses.clear();
            }
        }

        key_presses.insert(
            0,
            KeyPress {
                time: Instant::now(),
            },
        );

        if key_presses.len() >= TypingSpeedTracker::MAX_KEY_PRESSES {
            key_presses.truncate(TypingSpeedTracker::MAX_KEY_PRESSES);
        }
    }

    pub fn typing_speed_within_timespan(&self, timespan: Duration) -> TypingSpeed {
        let keys_pressed = self.get_presses_within_timespan(timespan);
        println!("keys_pressed: {}", keys_pressed.len());
        return TypingSpeed::new(timespan, keys_pressed.len());
    }

    fn get_presses_within_timespan<'a>(&'a self, timespan: Duration) -> Vec<&'a KeyPress> {
        return self
            .key_presses
            .iter()
            .filter(|key| key.time.elapsed() < timespan)
            .collect();
    }
}

impl TypingSpeed {
    pub const WPM_MODIFIER: f32 = 1.0 / 5.0;

    pub fn new(duration: Duration, keys_pressed: usize) -> Self {
        return TypingSpeed {
            duration,
            keys_pressed,
        };
    }

    pub fn wpm(&self) -> f32 {
        return self.cpm() * TypingSpeed::WPM_MODIFIER;
    }

    pub fn cpm(&self) -> f32 {
        let TypingSpeed {
            duration,
            keys_pressed,
        } = self;
        return ((*keys_pressed as f32) / duration.as_secs_f32()) * 60.0;
    }
}
