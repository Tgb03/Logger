use std::ops::Deref;

use super::token::Token;
use crate::time::Time;

pub trait Tokenizer {
    fn tokenize_single(&self, line: &str) -> Option<Token>;
    fn tokenize(&self, lines: &str) -> Vec<(Time, Token)> {
        let mut result = Vec::new();

        for line in lines.split('\n').map(|v| v.trim_start()) {
            if let Some(token) = self.tokenize_single(line) {
                if let Some(time) = Time::from(line) {
                    // #[cfg(debug_assertions)]
                    // eprintln!("{} Token parsed:{:?}", time.to_string(), token);
                    result.push((time, token));
                }
            }
        }

        result
    }
}

pub struct TokenizeIter<'a, I, T>
where
    I: Iterator<Item = &'a str>,
    T: Tokenizer, {

    iter: I,
    tokenizer: &'a T

}

impl<'a, I, T> Iterator for TokenizeIter<'a, I, T> 
where
    I: Iterator<Item = &'a str>,
    T: Tokenizer, {
    
    type Item = (Time, Token);
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.iter
                .next()?
                .trim_start();

            match (
                self.tokenizer.tokenize_single(line),
                Time::from(line)
            ) {
                (Some(token), Some(time)) => return Some((time, token)),
                _ => {}
            }
        }
    }
}

impl<'a, I, T> TokenizeIter<'a, I, T> 
where
    I: Iterator<Item = &'a str>,
    T: Tokenizer, {

    pub fn new(iter: I, tokenizer: &'a T) -> Self {
        Self {
            iter,
            tokenizer,
        }
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
    for<'a> &'a I: IntoIterator<Item = &'a T>,
{
    fn tokenize_single(&self, line: &str) -> Option<Token> {
        self.into_iter().find_map(|v| v.tokenize_single(line))
    }
}

struct BaseTokenizer;
pub struct RunTokenizer;
pub struct GenerationTokenizer;

impl Tokenizer for BaseTokenizer {
    fn tokenize_single(&self, line: &str) -> Option<Token> {
        if line.get(44..60).is_some_and(|v| v == "SetSessionIDSeed") {
            return Some(Token::create_session_seed(line));
        }
        if line
            .get(30..52)
            .is_some_and(|v| v == "SelectActiveExpedition")
        {
            return Some(Token::create_expedition(line));
        }
        if line.get(15..32).is_some_and(|v| v == "OnApplicationQuit") {
            return Some(Token::LogFileEnd);
        }

        let len = line.len();

        if line
            .get(len.saturating_sub(21)..len.saturating_sub(1))
            .is_some_and(|v| v == "was added to session")
        {
            return Some(Token::PlayerJoinedLobby);
        }
        if line
            .get(15..45)
            .is_some_and(|v| v == "DEBUG : Closed connection with")
        {
            return Some(Token::PlayerLeftLobby);
        }
        if line
            .get(15..43)
            .is_some_and(|v| v == "DEBUG : Leaving session hub!")
        {
            return Some(Token::UserExitLobby);
        }

        None
    }
}

impl Tokenizer for RunTokenizer {
    fn tokenize_single(&self, line: &str) -> Option<Token> {
        if line.contains("exits PLOC_InElevator") {
            return Some(Token::create_player(line));
        }
        if line
            .get(69..109)
            .is_some_and(|v| v == ": StopElevatorRide TO: ReadyToStartLevel")
        {
            return Some(Token::GameStarting);
        }
        if line
            .get(69..100)
            .is_some_and(|v| v == ": ReadyToStartLevel TO: InLevel")
        {
            return Some(Token::GameStarted);
        }
        if line
            .get(31..61)
            .is_some_and(|v| v == "LinkedToZoneData.EventsOnEnter")
        {
            return Some(Token::DoorOpen);
        }
        if line
            .get(15..42)
            .is_some_and(|v| v == "BulkheadDoorController_Core")
        {
            return Some(Token::BulkheadScanDone);
        }
        if line
            .get(116..141)
            .is_some_and(|v| v == "WardenObjectiveItemSolved")
        {
            return Some(Token::SecondaryDone);
        }
        if line
            .get(112..137)
            .is_some_and(|v| v == "WardenObjectiveItemSolved")
        {
            return Some(Token::OverloadDone);
        }
        if line
            .get(71..100)
            .is_some_and(|v| v == "InLevel TO: ExpeditionSuccess")
        {
            return Some(Token::GameEndWin);
        }
        if line
            .get(15..63)
            .is_some_and(|v| v == "RundownManager.OnExpeditionEnded(endState: Abort")
        {
            return Some(Token::GameEndAbort);
        }
        if line
            .get(15..48)
            .is_some_and(|v| v == "CleanupAfterExpedition AfterLevel")
        {
            return Some(Token::GameEndAbort);
        }
        if line
            .get(15..58)
            .is_some_and(|v| v == "DEBUG : Leaving session hub! : IsInHub:True")
        {
            return Some(Token::GameEndAbort);
        }
        if line
            .get(71..97)
            .is_some_and(|v| v == "InLevel TO: ExpeditionFail")
        {
            return Some(Token::GameEndLost);
        }

        None
    }
}

impl Tokenizer for GenerationTokenizer {
    fn tokenize_single(&self, line: &str) -> Option<Token> {
        if line
            .get(69..91)
            .is_some_and(|v| v == ": Lobby TO: Generating")
        {
            return Some(Token::GeneratingLevel);
        }
        if line
            .get(69..109)
            .is_some_and(|v| v == ": Generating TO: ReadyToStopElevatorRide")
        {
            return Some(Token::GeneratingFinished);
        }
        if line
            .get(29..54)
            .is_some_and(|v| v == "CreateKeyItemDistribution")
        {
            return Some(Token::create_item_alloc(line));
        }
        if line
            .get(30..81)
            .is_some_and(|v| v == "TryGetExistingGenericFunctionDistributionForSession")
        {
            return Some(Token::create_item_spawn(line));
        }
        if line.get(30..102).is_some_and(|v| {
            v == "LG_Distribute_WardenObjective.SelectZoneFromPlacementAndKeepTrackOnCount"
        }) {
            return Some(Token::create_collectable_allocated(line));
        }
        if line.get(35..121).is_some_and(|v| v == "TryGetRandomPlacementZone.  Determine wardenobjective zone. Found zone with LocalIndex") 
        { 
            return Some(Token::create_hsu_alloc(line)); 
        }
        if line.get(35..109).is_some_and(|v| {
            v == "LG_Distribute_WardenObjective, placing warden objective item with function"
        }) {
            return Some(Token::create_objective_spawned_override(line));
        }
        if line
            .get(30..89)
            .is_some_and(|v| v == "LG_Distribute_WardenObjective.DistributeGatherRetrieveItems")
        {
            return Some(Token::create_collectable_item_id(line));
        }
        if line
            .get(15..67)
            .is_some_and(|v| v == "GenericSmallPickupItem_Core.SetupFromLevelgen, seed:")
        {
            return Some(Token::create_collectable_item_seed(line));
        }
        if line
            .get(15..44)
            .is_some_and(|v| v == "RESET placementDataIndex to 0") {
            
            return Some(Token::DimensionReset);
        }
        if line
            .get(15..47)
            .is_some_and(|v| v == "Increment placementDataIndex to ") {
            
            return Some(Token::DimensionIncrease);
        }

        None
    }
}

pub struct GenericTokenizer {
    tokenizers: Vec<Box<dyn Tokenizer>>,
}

impl Tokenizer for GenericTokenizer {
    fn tokenize_single(&self, line: &str) -> Option<Token> {
        self.tokenizers.tokenize_single(line)
    }
}

impl Default for GenericTokenizer {
    fn default() -> Self {
        Self {
            tokenizers: vec![Box::new(BaseTokenizer)],
        }
    }
}

impl GenericTokenizer {
    pub fn add_tokenizer<T>(mut self, tokenizer: T) -> Self
    where
        T: Tokenizer + 'static,
    {
        self.tokenizers.push(Box::new(tokenizer));

        self
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs::File, io::Read};

    use crate::logs::data::ObjectiveFunction;

    use super::*;

    fn create_tokenizer() -> GenericTokenizer {
        GenericTokenizer::default()
            .add_tokenizer(RunTokenizer)
            .add_tokenizer(GenerationTokenizer)
    }

    fn load_file(name: &str) -> Option<String> {
        let mut result = String::default();
        let path_buf = env::current_dir()
            .ok()?
            .parent()?
            .join("examples")
            .join("log_files")
            .join(name)
            .with_extension("txt");

        println!("{:?}", path_buf);

        let mut f = File::open(path_buf).ok()?;

        match f.read_to_string(&mut result) {
            Ok(_) => Some(result),
            Err(_) => None,
        }
    }

    fn tokenize_file(name: &str, tokenizer: &GenericTokenizer) -> Vec<Token> {
        let file_str = load_file(name).unwrap();

        tokenizer
            .tokenize(&file_str)
            .into_iter()
            .filter_map(|(_, v)| {
                match v {
                    Token::PlayerJoinedLobby |
                    Token::PlayerLeftLobby |
                    Token::UserExitLobby |
                    Token::SessionSeed(_) |
                    Token::PlayerDroppedInLevel(_) |
                    Token::SelectExpedition(_) |
                    Token::LogFileEnd => None,
                    _ => Some(v),
                }
            })
            .collect()
    }

    #[test]
    fn test_generation_r1a1() {
        let tokenizer = create_tokenizer();

        let tokens_v= vec![
            tokenize_file("R1A1_client.frosty_exp_comp.txt", &tokenizer),
            tokenize_file("R1A1_host.maid_exp_comp.txt", &tokenizer),
        ];
        
        for tokens in tokens_v {
            assert_eq!(tokens, vec![
                Token::GeneratingLevel,
                Token::ItemAllocated("KEY_GREEN_245".try_into().unwrap()),
                Token::ItemSpawn(50, 48),
                Token::CollectableAllocated(3),
                Token::ObjectiveSpawnedOverride(18, ObjectiveFunction::HSU_FindTakeSample),
                Token::GeneratingFinished,
                Token::GameStarting,
                Token::GameStarted,
                Token::DoorOpen,
                Token::DoorOpen,
                Token::GameEndWin,
                Token::GameEndAbort,
            ]);
        }
    }

    #[test]
    fn test_run_r1b1() {
        let tokenizer = create_tokenizer();

        let tokens_v= vec![
            tokenize_file("R1B2_client.alex_hsu_3keys.txt", &tokenizer),
            tokenize_file("R1B2_client.frosty_hsu_3keys.txt", &tokenizer),
            tokenize_file("R1B2_host.maid_hsu_3keys.txt", &tokenizer),
        ];
        
        for tokens in tokens_v {
            assert_eq!(tokens, vec![
                Token::GeneratingLevel,
                Token::ItemAllocated("KEY_BLUE_184".try_into().unwrap()),
                Token::ItemSpawn(18, 8),
                Token::ItemAllocated("KEY_PURPLE_421".try_into().unwrap()),
                Token::ItemSpawn(23, 20),
                Token::ItemAllocated("KEY_YELLOW_990".try_into().unwrap()),
                Token::ItemSpawn(23, 37),
                Token::CollectableAllocated(5),
                Token::ObjectiveSpawnedOverride(16, ObjectiveFunction::HSU_FindTakeSample),
                Token::GeneratingFinished,
                Token::GameStarting,
                Token::GameStarted,
                Token::DoorOpen,
                Token::DoorOpen,
                Token::DoorOpen,
                Token::DoorOpen,
                Token::GameEndLost,
                Token::GameEndAbort,
                Token::GeneratingLevel,
                Token::ItemAllocated("KEY_PURPLE_389".try_into().unwrap()),
                Token::ItemSpawn(18, 1),
                Token::ItemAllocated("KEY_GREY_560".try_into().unwrap()),
                Token::ItemSpawn(23, 21),
                Token::ItemAllocated("KEY_ORANGE_338".try_into().unwrap()),
                Token::ItemSpawn(22, 14),
                Token::CollectableAllocated(5),
                Token::ObjectiveSpawnedOverride(16, ObjectiveFunction::HSU_FindTakeSample),
                Token::GeneratingFinished,
                Token::GameStarting,
                Token::GameStarted,
                Token::DoorOpen,
                Token::DoorOpen,
                Token::DoorOpen,
                Token::DoorOpen,
                Token::GameEndWin,
                Token::GameEndAbort,
            ]);
        }   
    }

    #[test]
    fn test_r6c2() {
        let tokenizer = create_tokenizer();

        let tokens = tokenize_file("R6C2_host_hisec.txt", &tokenizer);

        assert_eq!(tokens, vec![
            Token::GeneratingLevel,
            Token::ItemAllocated("BULKHEAD_KEY_538".try_into().unwrap()),
            Token::ItemSpawn(123, 1),
            Token::ItemAllocated("BULKHEAD_KEY_585".try_into().unwrap()),
            Token::ItemSpawn(247, 5),
            Token::CollectableAllocated(246),
            Token::CollectableItemID(154),
            Token::DimensionReset,
            Token::GeneratingFinished,
            Token::GameStarting,
            Token::GameStarted,
            Token::DoorOpen,
            Token::GameEndAbort,
            Token::GameEndAbort,
        ]);
    }
    
}
