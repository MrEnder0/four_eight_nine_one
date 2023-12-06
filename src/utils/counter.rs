use std::sync::Mutex;

use rand::Rng;

static CURRENT_COUNT: Mutex<usize> = Mutex::new(0);
static TARGET_COUNT: Mutex<usize> = Mutex::new(0);

pub struct CountStruct {}

impl CountStruct {
    pub fn get_count() -> usize {
        let binding = &CURRENT_COUNT;
        let guard = binding.lock().unwrap();

        *guard
    }

    pub fn get_target_count() -> usize {
        let binding = &TARGET_COUNT;
        let guard = binding.lock().unwrap();

        *guard
    }

    pub fn reset_counts() {
        let binding = &CURRENT_COUNT;
        let mut guard = binding.lock().unwrap();

        *guard = 0;

        Self::reset_target_count();
    }

    pub fn increment_count() {
        let binding = &CURRENT_COUNT;
        let mut guard = binding.lock().unwrap();

        *guard += 1;
    }

    pub fn reset_target_count() {
        let binding = &TARGET_COUNT;
        let mut guard = binding.lock().unwrap();

        let range = super::config::non_async_read_config().target_count_range;

        *guard = rand::thread_rng().gen_range(range.0..range.1) as usize;
    }

    pub fn check_count() -> bool {
        let current_count = Self::get_count();
        let target_count = Self::get_target_count();

        if current_count >= target_count {
            Self::reset_counts();

            return true;
        }

        false
    }
}
