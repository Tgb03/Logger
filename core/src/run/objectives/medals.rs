use crate::time::Time;

pub const MEDALS: [&str; 4] = [
    "Bronze",
    "Silver",
    "Gold",
    "Champion",
];

pub const BRONZE_MEDALS: &[Time] = &[
    
];

pub const SILVER_MEDALS: &[Time] = &[

];

pub const GOLD_MEDALS: &[Time] = &[

];

pub const CHAMPION_MEDALS: &[Time] = &[

];

pub fn get_medal(time: Time, level_name: &str) -> &str {
    todo!()
}