
use std::ops::Deref;

use crate::run::time::Time;
use super::token::Token;

pub trait Tokenizer {

  fn tokenize_single(&self, line: &str) -> Option<Token>;
  fn tokenize(&self, lines: &str) -> Vec<(Time, Token)> {
    let mut result = Vec::new();

    for line in lines.split('\n') {
      if let Some(token) = self.tokenize_single(line) {
        if let Ok(time) = Time::from(line.trim_start()) {
          result.push((time, token));
        }
      }  
    }

    result
  }

}

impl Tokenizer for Box<dyn Tokenizer> {
  fn tokenize_single(&self, line: &str) -> Option<Token> {
    self.deref().tokenize_single(line)  
  }
}

impl<I, T> Tokenizer for I
where
  T: Tokenizer,
  for<'a> &'a I: IntoIterator<Item = &'a T> {
  
  fn tokenize_single(&self, line: &str) -> Option<Token> {
    self
      .into_iter()
      .find_map(|v| v.tokenize_single(line))
  }

}

pub struct BaseTokenizer;
pub struct RunTokenizer;
pub struct GenerationTokenizer;

impl Tokenizer for BaseTokenizer {
  fn tokenize_single(&self, line: &str) -> Option<Token> {
    if line.contains("<color=#C84800>>>>>>>>>>>>>> SetSessionIDSeed, forcedSeed: ") { return Some(Token::create_session_seed(line)); }
    if line.contains("SelectActiveExpedition : Selected!") { return Some(Token::create_expedition(line)); }
    if line.contains("OnApplicationQuit") { return Some(Token::LogFileEnd); }
  
    None
  }
}

impl Tokenizer for RunTokenizer {
  fn tokenize_single(&self, line: &str) -> Option<Token> {
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

    None
  }
}

impl Tokenizer for GenerationTokenizer {
  fn tokenize_single(&self, line: &str) -> Option<Token> {
    if line.contains("GAMESTATEMANAGER CHANGE STATE FROM : Lobby TO: Generating") { return Some(Token::GeneratingLevel); }
    if line.contains("GAMESTATEMANAGER CHANGE STATE FROM : Generating TO: ReadyToStopElevatorRide") { return Some(Token::GeneratingFinished); }
    if line.contains("CreateKeyItemDistribution") { return Some(Token::create_item_alloc(line)); }
    if line.contains("TryGetExistingGenericFunctionDistributionForSession") { return Some(Token::create_item_spawn(line)); }
    if line.contains("<color=#C84800>LG_Distribute_WardenObjective.SelectZoneFromPlacementAndKeepTrackOnCount") { return Some(Token::create_collectable_allocated(line)); }
    if line.contains("TryGetRandomPlacementZone.  Determine wardenobjective zone. Found zone with LocalIndex") { return Some(Token::create_hsu_alloc(line)); }
    if line.contains("<color=#C84800>>>>> LG_Distribute_WardenObjective, placing warden objective item with function") { return Some(Token::create_objective_spawned_override(line)); }
    if line.contains("<color=#C84800>LG_Distribute_WardenObjective.DistributeGatherRetrieveItems") { return Some(Token::create_collectable_item_id(line)); }
    if line.contains("GenericSmallPickupItem_Core.SetupFromLevelgen, seed:") { return Some(Token::create_collectable_item_seed(line)); }

    None
  }
}

pub struct GenericTokenizer {

  tokenizers: Vec<Box<dyn Tokenizer>>

}

impl Tokenizer for GenericTokenizer {
  fn tokenize_single(&self, line: &str) -> Option<Token> {
    self.tokenizers.tokenize_single(line)
  }
}

impl Default for GenericTokenizer {
  fn default() -> Self {
    Self { 
      tokenizers: vec![Box::new(BaseTokenizer)] 
    }
  }
}

impl GenericTokenizer {

  pub fn add_tokenizer<T>(mut self, tokenizer: T) -> Self
  where T: Tokenizer + 'static {
    self.tokenizers.push(Box::new(tokenizer));

    self
  }

}


#[cfg(test)]
mod tests {
  use super::*;

  fn create_tokenizer_max() -> impl Tokenizer {
    GenericTokenizer::default()
      .add_tokenizer(RunTokenizer)
      .add_tokenizer(GenerationTokenizer)
  }

  #[test]
  fn test_basic_game() {
    let token_arr = create_tokenizer_max().tokenize(
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
