use std::fmt::Display;


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum LocationType {
  
  Unknown,
  ColoredKey,
  BulkheadKey,
  Objective,

}


impl Default for LocationType {
  fn default() -> Self {
    LocationType::Unknown
  }
}


#[derive(Default, PartialEq, Eq, Ord, Hash, Debug)]
pub struct Location {

  item_name: Option<String>,
  
  zone: Option<u64>,
  id: Option<u64>,

  location_type: LocationType,

}

impl Display for Location {

  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}: {} at {}", 
      self.item_name.as_ref().map(|v| v.as_str()).unwrap_or("No Name"), 
      match &self.zone { None => "No Zone".to_owned(), Some(zone) => format!("ZONE {}", zone) }, 
      match &self.id { None => "No ID".to_owned(), Some(id) => id.to_string() },
    )
  }

}

impl Location {

  pub fn with_name(mut self, item_name: String) -> Self {
    self.item_name = Some(item_name);

    self
  }

  pub fn with_zone(mut self, zone: u64) -> Self {
    self.zone = Some(zone);

    self
  }

  pub fn with_id(mut self, id: u64) -> Self {
    self.id = Some(id);

    self
  }

  pub fn with_type(mut self, location_type: LocationType) -> Self {
    self.location_type = location_type;

    self
  }

  pub fn has_type(&self, location_type: &LocationType) -> bool {
    self.location_type == *location_type
  }

  pub fn get_name(&self) -> Option<&String> {
    
    self.item_name.as_ref()
    
  }

  pub fn set_name(&mut self, item_name: String) {
    self.item_name = Some(item_name);
  }

  pub fn set_zone(&mut self, zone: u64) {
    self.zone = Some(zone);
  }

  pub fn set_id(&mut self, id: u64) {
    self.id = Some(id);
  }

  pub fn get_id(&self) -> Option<u64> {
    self.id
  }

  pub fn get_zone(&self) -> Option<u64> {
    self.zone
  }

  pub fn get_type(&self) -> &LocationType {
    &self.location_type
  }

}

impl PartialOrd for Location {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match self.location_type.partial_cmp(&other.location_type) {
      Some(core::cmp::Ordering::Equal) => {}
      ord => return ord,
    }

    self.zone.partial_cmp(&other.zone)
  }
}