#[derive(Debug, PartialEq)]
pub enum Token {
    GeneratingLevel,

    PlayerJoinedLobby,
    PlayerLeftLobby,
    UserExitLobby,

    SessionSeed(u64),
    GeneratingFinished,
    ItemAllocated(String, bool),           // name
    ItemSpawn(u64, u64),                   // zone, id
    CollectableAllocated(u64),             // zone
    ObjectiveSpawnedOverride(u64, String), // id, name of objective
    CollectableItemID(u8),                 // item id
    CollectableItemSeed(u64),              // item seed
    SelectExpedition(String),
    GameStarting,
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

    Invalid,
}

impl Token {
    pub fn create_session_seed(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 6 {
            return Token::Invalid;
        }

        match words[5].parse::<u64>() {
            Ok(seed) => Token::SessionSeed(seed),
            Err(_) => Token::Invalid,
        }
    }

    pub fn create_item_alloc(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 6 {
            return Token::Invalid;
        }

        let name = words[5];

        Token::ItemAllocated(name.to_owned(), name.contains("BULKHEAD"))
    }

    pub fn create_item_spawn(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 15 {
            return Token::Invalid;
        }
        if words[6].len() < 4 {
            return Token::Invalid;
        }

        let zone = words[6][4..].parse().ok();
        let id = words[14].parse::<u64>();

        match (zone, id) {
            (Some(zone), Ok(id)) => Token::ItemSpawn(zone, id),
            _ => Token::Invalid,
        }
    }

    pub fn create_collectable_allocated(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 8 {
            return Token::Invalid;
        }
        if words[7].len() < 4 {
            return Token::Invalid;
        }

        match words[7][4..].parse() {
            Ok(zone) => Token::CollectableAllocated(zone),
            Err(_) => Token::Invalid,
        }
    }

    pub fn create_objective_spawned_override(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 19 {
            return Token::Invalid;
        }

        let name = match words[13] {
            "HSU_FindTakeSample" => "HSU".to_owned(),
            "TerminalUplink" => "Uplink".to_owned(),
            val => val.to_owned(),
        };

        if let Some(first) = words[18].split('_').collect::<Vec<&str>>().get(0) {
            match first.parse::<u64>() {
                Ok(i) => return Token::ObjectiveSpawnedOverride(i, name),
                Err(_) => return Token::Invalid,
            }
        }

        Token::Invalid
    }

    pub fn create_hsu_alloc(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 13 {
            return Token::Invalid;
        }
        if words[12].len() < 5 {
            return Token::Invalid;
        }

        match words[12][5..words[12].len() - 1].parse() {
            Ok(zone) => Token::CollectableAllocated(zone),
            Err(_) => Token::Invalid,
        }
    }

    pub fn create_collectable_item_id(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 9 {
            return Token::Invalid;
        }

        match words[8].parse() {
            Ok(id) => Token::CollectableItemID(id),
            Err(_) => Token::Invalid,
        }
    }

    pub fn create_collectable_item_seed(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 4 {
            return Token::Invalid;
        }

        match words[4].parse() {
            Ok(seed) => Token::CollectableItemSeed(seed),
            Err(_) => Token::Invalid,
        }
    }

    pub fn create_expedition(line: &str) -> Token {
        //println!("LINE: {}", line);

        let words: Vec<&str> = line.split(" ").collect();

        if words.len() < 8 {
            return Token::Invalid;
        }
        if words[6].len() < 6 {
            return Token::Invalid;
        }
        if words[7].len() < 5 {
            return Token::Invalid;
        }

        let rundown_id = &words[6][6..];
        let tier = &words[7][4..5];
        let level = match words[8].parse::<i32>() {
            Ok(val) => val + 1,
            Err(_) => return Token::Invalid,
        };

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
            _ => "$R",
        };

        if rundown_name == "training" {
            return Token::SelectExpedition("TRAINING".to_string());
        }
        let result = format!("{}{}{}", rundown_name, tier, level);

        Token::SelectExpedition(result)
    }

    pub fn create_player(line: &str) -> Token {
        let words: Vec<&str> = line.split(" ").collect();

        let player_id = words[words.len() - 1].trim();
        let player_id = &player_id[0..player_id.len() - 8];

        match player_id.parse::<u32>() {
            Ok(id) => Token::PlayerDroppedInLevel(id),
            Err(_) => Token::Invalid,
        }
    }
}
