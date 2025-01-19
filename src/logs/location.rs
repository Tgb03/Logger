
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum LocationType {
  
  Unknown,
  Key,
  Objective,

}


impl Default for LocationType {
  fn default() -> Self {
    LocationType::Unknown
  }
}


#[derive(Default, PartialEq, Eq, Ord)]
pub struct Location {

  item_name: Option<String>,
  
  zone: Option<u64>,
  id: Option<u64>,

  location_type: LocationType, 

}

impl From<&Location> for String {

  fn from(value: &Location) -> Self {
    format!("{}: {} at {}", 
      match &value.item_name { None => "No Name", Some(name) => name }, 
      match &value.zone { None => "Unknown Zone".to_owned(), Some(zone) => format!("ZONE {}", zone) }, 
      match &value.id { None => "Unknown ID".to_owned(), Some(id) => id.to_string() },
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

  pub fn has_type(&self, location_type: LocationType) -> bool {
    self.location_type == location_type
  }

  pub fn get_name(&self) -> Option<&String> {
    
    self.item_name.as_ref()
    
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