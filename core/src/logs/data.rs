
use std::fmt::Display;

use num_enum::{FromPrimitive, IntoPrimitive};

use super::location::Location;

#[derive(Default, Debug, PartialEq, Eq, FromPrimitive, IntoPrimitive, strum::IntoStaticStr)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Rundown {
    #[default]
    #[strum(to_string="$R")]
    Modded,
    R7 = 31,
    R1 = 32,
    R2 = 33,
    R3 = 34,
    R8 = 35,
    R4 = 37,
    R5 = 38,
    TRAINING = 39,
    R6 = 41,
    
    #[strum(to_string="OG.R1")] OG_R1 = 17,
    #[strum(to_string="OG.R2")] OG_R2 = 19,
    #[strum(to_string="OG.R3")] OG_R3 = 22,
    #[strum(to_string="OG.R4")] OG_R4 = 25,
    #[strum(to_string="OG.R5")] OG_R5 = 26,
    #[strum(to_string="OG.R6")] OG_R6 = 29,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct LevelDescriptor {

    rundown: Rundown,
    tier: u8,
    level: u8,

}

impl LevelDescriptor {

    pub fn new(rundown: Rundown, tier: u8, level: u8) -> Self {
        Self { rundown, tier, level }
    }
}

impl Display for LevelDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.rundown {
            Rundown::TRAINING => write!(f, "TRAINING"),
            _ => write!(f, "{}{}{}", Into::<&str>::into(&self.rundown), (self.tier + 'A' as u8) as char, (self.level + '1' as u8) as char)
        }
    }
}


#[derive(Default, Debug, PartialEq, Eq, strum::IntoStaticStr, Clone)]
enum KeyColor {

    PURPLE,
    GREY,
    YELLOW,
    GREEN,
    ORANGE,
    WHITE,
    RED,
    BLACK,
    BLUE,

    #[default]
    COLORED,

}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct KeyDescriptor {

    color: Option<KeyColor>,
    key_number: u16,

}

impl KeyDescriptor {

    pub fn into_location(&self, zone: u64, id: u64) -> Location {
        match &self.color {
            Some(_) => Location::ColoredKey(format!("{}", self), zone, id),
            None => Location::BulkheadKey(format!("{}", self), zone, id),
        }
    }

}

impl TryFrom<&str> for KeyDescriptor {
    type Error = ();
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut words = value.split('_').skip(1);

        let color = match words.next() {
            Some("PURPLE") => Some(KeyColor::PURPLE),
            Some("GREY") => Some(KeyColor::GREY),
            Some("YELLOW") => Some(KeyColor::YELLOW),
            Some("GREEN") => Some(KeyColor::GREEN),
            Some("ORANGE") => Some(KeyColor::ORANGE),
            Some("WHITE") => Some(KeyColor::WHITE),
            Some("RED") => Some(KeyColor::RED),
            Some("BLACK") => Some(KeyColor::BLACK),
            Some("BLUE") => Some(KeyColor::BLUE),
            Some("KEY") => None,
            Some(_) => Some(KeyColor::COLORED),
            None => return Err(()),
        };

        let key_number = words.next()
            .ok_or(())?
            .parse::<u16>()
            .map_err(|_| ())?;

        Ok(Self {
            color,
            key_number,
        })
    }
}

impl Display for KeyDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.color {
            Some(color) => write!(f, "KEY_{}_{}", Into::<&str>::into(color), self.key_number),
            None => write!(f, "BULKHEAD_KEY_{}", self.key_number),
        }
    }
}
