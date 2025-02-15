
use std::u32;

use crate::run::time::Time;


/// These are Tokens. We parse the log file into these Tokens
/// so the data can be treated far easier.
#[derive(Debug, PartialEq)]
pub enum Token {
 
  GeneratingLevel,
  GeneratingFinished,
  ItemAllocated(String), // name
  ItemSpawn(u64, u64), // zone, id
  ObjectiveGather(u64, u64), // item id, item count
  ObjectiveAllocated(u64, Option<u64>), // zone, id
  ObjectiveSpawned(String), // name
  SelectExpedition(String),
  GameStarted,
  PlayerDroppedInLevel(u32),
  DoorOpen,
  BulkheadScanDone,
  SecondaryDone,
  OverloadDone,
  GameEndWin,
  GameEndLost,
  GameEndAbort,
  LogFileEnd,

}

impl Token {

  fn create_gather_objective_spawn(line: &str) -> Token {
    let words: Vec<&str> = line.split(" ").collect();

    let zone_count = words[10]
      .strip_suffix(']')
      .unwrap_or(words[10])
      .parse::<u64>()
      .unwrap_or_default();

    let id = words[14 + zone_count as usize]
      .strip_suffix(',')
      .unwrap_or(words[14 + zone_count as usize])
      .parse::<u64>()
      .unwrap_or_default();

    let count = words[13 + zone_count as usize]
      .strip_suffix(']')
      .unwrap_or(words[13 + zone_count as usize])
      .parse::<u64>()
      .unwrap_or_default();

    Token::ObjectiveGather(id, count)
  }

  fn create_basic_objective_alloc(line: &str) -> Token {
    let words: Vec<&str> = line.split(" ").collect();

    let zone = words[7];
    let zone = zone.to_owned()[4..].parse::<u64>().ok();

    Token::ObjectiveAllocated(zone.unwrap_or_default(), None)
  }

  fn create_hsu_objective_alloc(line: &str) -> Token {
    let words: Vec<&str> = line.split(" ").collect();

    let zone = words[16];
    let id = words[18].split('_').collect::<Vec<&str>>()[0].parse::<u64>().ok();
    let zone = zone.trim_end_matches(',');
    let zone = match zone.len() > 3 {
      true => &zone[4..],
      false => ""
    }.parse().ok();

    Token::ObjectiveAllocated(zone.unwrap_or_default(), id)
  }

  fn create_object_spawn(line: &str) -> Token {
    let words: Vec<&str> = line.split(" ").collect();

    let name = words[5];

    Token::ObjectiveSpawned(name.to_owned())
  }

  fn create_item_alloc(line: &str) -> Token {
    
    let words: Vec<&str> = line.split(" ").collect();

    let name = words[5];

    Token::ItemAllocated(name.to_owned())
  }

  fn create_item_spawn(line: &str) -> Token {

    let words: Vec<&str> = line.split(" ").collect(); 

    let zone = words[6][4..].parse().ok();
    let id = words[14].parse::<u64>().unwrap_or_default();

    Token::ItemSpawn(zone.unwrap_or_default(), id)
  }

  fn create_expedition(line: &str) -> Token {
    //println!("LINE: {}", line);

    let words: Vec<&str> = line.split(" ").collect();
    
    let rundown_id = &words[6][6..];
    let tier = &words[7][4..5];
    let level = words[8].parse::<i32>().unwrap() + 1;
 
    let rundown_name = match rundown_id {
      "31" => "R7",
      "32" => "R1",
      "33" => "R2",
      "34" => "R3",
      "35" => "R8",
      "37" => "R4",
      "38" => "R5",
      "39" => "training",
      "41" => "R6",
      _ => "$R_"
    };

    if rundown_name == "training" { return Token::SelectExpedition("TRAINING".to_string()); }
    let result = format!("{}{}{}", rundown_name, tier, level);

    Token::SelectExpedition(result)
  }

  fn create_player(line: &str) -> Token {

    let words: Vec<&str> = line.split(" ").collect();

    let player_id = words[words.len() - 1].trim();
    let player_id = &player_id[0..player_id.len() - 8];

    match player_id.parse::<u32>() {
      Ok(id) => Token::PlayerDroppedInLevel(id),
      Err(_) => Token::PlayerDroppedInLevel(u32::MAX),
    }
  }

  pub fn tokenize_str(line: &str) -> Option<Token> {
    
    if line.contains("GAMESTATEMANAGER CHANGE STATE FROM : Lobby TO: Generating") { return Some(Token::GeneratingLevel); }
    if line.contains("GAMESTATEMANAGER CHANGE STATE FROM : Generating TO: ReadyToStopElevatorRide") { return Some(Token::GeneratingFinished); }
    if line.contains("CreateKeyItemDistribution") { return Some(Token::create_item_alloc(line)); }
    if line.contains("TryGetExistingGenericFunctionDistributionForSession") { return Some(Token::create_item_spawn(line)); }
    if line.contains("LG_Distribute_WardenObjective.SelectZoneFromPlacementAndKeepTrackOnCount") { return Some(Token::create_basic_objective_alloc(line)); }
    if line.contains("LG_Distribute_WardenObjective, placing warden objective item with function HydroStatisUnit for wardenObjectiveType: HSU_FindTakeSample") { return Some(Token::create_hsu_objective_alloc(line)); }
    if line.contains("LG_Distribute_WardenObjective, PLACE GATHER ITEMS") { return Some(Token::create_gather_objective_spawn(line)); }
    if line.contains("WardenObjectiveManager.RegisterObjectiveItemForCollection") { return Some(Token::create_object_spawn(line)); }
    if line.contains("SNET : OnMasterCommand : ReceivingSync_Dropin") { return Some(Token::GameStarted); }
    if line.contains("SelectActiveExpedition : Selected!") { return Some(Self::create_expedition(line)); }
    if line.contains("GAMESTATEMANAGER CHANGE STATE FROM : ReadyToStartLevel TO: InLevel") { return Some(Token::GameStarted); }
    if line.contains("exits PLOC_InElevator") { return Some(Self::create_player(line)); }
    if line.contains("OnDoorIsOpened, LinkedToZoneData.EventsOnEnter") { return Some(Token::DoorOpen); }
    if line.contains("BulkheadDoorController_Core.OnScanDone") { return Some(Token::BulkheadScanDone); }
    if line.contains("WardenObjectiveManager.CheckWardenObjectiveStatus, layer: SecondaryLayer, status is diff! newStatus: WardenObjectiveItemSolved") { return Some(Token::SecondaryDone) }
    if line.contains("WardenObjectiveManager.CheckWardenObjectiveStatus, layer: ThirdLayer, status is diff! newStatus: WardenObjectiveItemSolved") { return Some(Token::OverloadDone) }
    if line.contains("GAMESTATEMANAGER CHANGE STATE FROM : InLevel TO: ExpeditionSuccess") { return Some(Token::GameEndWin); }
    if line.contains("RundownManager.OnExpeditionEnded(endState: Abort") { return Some(Token::GameEndAbort); }
    if line.contains("RundownManager.EndGameSession") { return Some(Token::GameEndAbort); }
    if line.contains("GAMESTATEMANAGER CHANGE STATE FROM : InLevel TO: ExpeditionFail") { return Some(Token::GameEndLost); }
    if line.contains("OnApplicationQuit") { return Some(Token::LogFileEnd); }

    None
  }

}

pub struct Tokenizer;

impl Tokenizer {

  /// Takes as input one log file string
  /// it then tokenizes the entire log file
  pub fn tokenize(log_string: &str) -> Vec<(Time, Token)> {
    let mut result = Vec::new();

    for line in log_string.split('\n') {
      if let Some(token) = Token::tokenize_str(line) {
        // println!("{:?} tokenized", token);
        result.push((Time::from(line.trim()), token));
      }
    }

    //println!("Size of token vec: {}", result.len());

    result
  }

}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_basic_game() {
    let token_arr = Tokenizer::tokenize(
      "00:00:00.000 - <color=#C84800>SelectActiveExpedition : Selected! Local Local_32 TierC 0 433572712 1571494152  sessionGUID:SNetwork.SNetStructs+pSessionGUID FriendsData expID set to: Local_32,3,0 ActiveExpeditionUniqueKey: Local_32_TierC_0</color>
      00:00:10.000 - GAMESTATEMANAGER CHANGE STATE FROM : ReadyToStartLevel TO: InLevel
      00:00:10.000 - Player1 exits PLOC_InElevator 1</color>
      00:00:10:055 - Useless line
      00:00:10.100 - Player2 exits PLOC_InElevator 23423</color>
      00:00:10.110 - Player3 exits PLOC_InElevator 3</color>
      00:00:10.250 - Player4 exits PLOC_InElevator 4</color>
      00:01:12.135 - OnDoorIsOpened, LinkedToZoneData.EventsOnEnter
      00:03:12.198 - OnDoorIsOpened, LinkedToZoneData.EventsOnEnter
      00:04:06.000 - OnDoorIsOpened, LinkedToZoneData.EventsOnEnter
      00:14:12.135 - OnDoorIsOpened, LinkedToZoneData.EventsOnEnter
      00:16:11.890 - OnDoorIsOpened, LinkedToZoneData.EventsOnEnter
      00:17:59.343 - GAMESTATEMANAGER CHANGE STATE FROM : InLevel TO: ExpeditionSuccess"
    );

    assert_eq!(token_arr, vec![
      (Time::from("00:00:00.000"), Token::SelectExpedition("R1C1".to_string())),
      (Time::from("00:00:10.000"), Token::GameStarted),
      (Time::from("00:00:10.000"), Token::PlayerDroppedInLevel(1)),
      (Time::from("00:00:10.100"), Token::PlayerDroppedInLevel(23423)),
      (Time::from("00:00:10.110"), Token::PlayerDroppedInLevel(3)),
      (Time::from("00:00:10.250"), Token::PlayerDroppedInLevel(4)),
      (Time::from("00:01:12.135"), Token::DoorOpen),
      (Time::from("00:03:12.198"), Token::DoorOpen),
      (Time::from("00:04:06.000"), Token::DoorOpen),
      (Time::from("00:14:12.135"), Token::DoorOpen),
      (Time::from("00:16:11.890"), Token::DoorOpen),
      (Time::from("00:17:59.343"), Token::GameEndWin),
    ]);
  }
}
