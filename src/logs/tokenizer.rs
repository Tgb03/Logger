
use crate::run::time::Time;

use super::token::Token;

pub struct Tokenizer;

impl Tokenizer {

  /// Takes as input one log file string
  /// it then tokenizes the entire log file
  pub fn tokenize(log_string: &str) -> Vec<(Time, Token)> {
    let mut result = Vec::new();

    for line in log_string.split('\n') {
      if let Some(token) = Self::tokenize_str(line) {
        println!("{:?} tokenized", token);
        if let Ok(time) = Time::from(line.trim()) {
          result.push((time, token));
        }
      }
    }

    //println!("Size of token vec: {}", result.len());

    result
  }

  pub fn tokenize_str(line: &str) -> Option<Token> {
    
    if line.contains("<color=#C84800>>>>>>>>>>>>>> SetSessionIDSeed, forcedSeed: ") { return Some(Token::create_session_seed(line)); }
    if line.contains("GAMESTATEMANAGER CHANGE STATE FROM : Lobby TO: Generating") { return Some(Token::GeneratingLevel); }
    if line.contains("GAMESTATEMANAGER CHANGE STATE FROM : Generating TO: ReadyToStopElevatorRide") { return Some(Token::GeneratingFinished); }
    if line.contains("CreateKeyItemDistribution") { return Some(Token::create_item_alloc(line)); }
    if line.contains("TryGetExistingGenericFunctionDistributionForSession") { return Some(Token::create_item_spawn(line)); }
    if line.contains("<color=#C84800>LG_Distribute_WardenObjective.SelectZoneFromPlacementAndKeepTrackOnCount") { return Some(Token::create_collectable_allocated(line)); }
    if line.contains("TryGetRandomPlacementZone.  Determine wardenobjective zone. Found zone with LocalIndex") { return Some(Token::create_hsu_alloc(line)); }
    if line.contains("<color=#C84800>>>>> LG_Distribute_WardenObjective, placing warden objective item with function") { return Some(Token::create_objective_spawned_override(line)); }
    if line.contains("<color=#C84800>LG_Distribute_WardenObjective.DistributeGatherRetrieveItems") { return Some(Token::create_collectable_item_id(line)); }
    if line.contains("GenericSmallPickupItem_Core.SetupFromLevelgen, seed:") { return Some(Token::create_collectable_item_seed(line)); }
    if line.contains("SelectActiveExpedition : Selected!") { return Some(Token::create_expedition(line)); }
    if line.contains("GAMESTATEMANAGER CHANGE STATE FROM : StopElevatorRide TO: ReadyToStartLevel") { return Some(Token::GameStarting); }
    if line.contains("GAMESTATEMANAGER CHANGE STATE FROM : ReadyToStartLevel TO: InLevel") { return Some(Token::GameStarted); }
    if line.contains("exits PLOC_InElevator") { return Some(Token::create_player(line)); }
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
      (Time::from("00:00:00.000").unwrap(), Token::SelectExpedition("R1C1".to_string())),
      (Time::from("00:00:10.000").unwrap(), Token::GameStarted),
      (Time::from("00:00:10.000").unwrap(), Token::PlayerDroppedInLevel(1)),
      (Time::from("00:00:10.100").unwrap(), Token::PlayerDroppedInLevel(23423)),
      (Time::from("00:00:10.110").unwrap(), Token::PlayerDroppedInLevel(3)),
      (Time::from("00:00:10.250").unwrap(), Token::PlayerDroppedInLevel(4)),
      (Time::from("00:01:12.135").unwrap(), Token::DoorOpen),
      (Time::from("00:03:12.198").unwrap(), Token::DoorOpen),
      (Time::from("00:04:06.000").unwrap(), Token::DoorOpen),
      (Time::from("00:14:12.135").unwrap(), Token::DoorOpen),
      (Time::from("00:16:11.890").unwrap(), Token::DoorOpen),
      (Time::from("00:17:59.343").unwrap(), Token::GameEndWin),
    ]);
  }
}
