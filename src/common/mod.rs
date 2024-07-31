pub mod client;
pub mod framework;
pub mod builtins;
pub mod structs;
pub mod read_json;
pub mod database;
pub mod command_enable_checks;
pub mod color_conversion;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_time() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

}

pub fn concatenate_integers(nums: &[u8; 3]) -> u8 {
    nums.iter()
        .fold(0, |acc, num| acc * 10 + num)
}