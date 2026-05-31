use std::{cmp::Ordering, time::{Duration, Instant}};

use egui::Color32;
use glr_core::token::{GameState::{self, Lobby}, Token};
use glr_lib::dll_exports::enums::SubscribeCode;

use crate::{dll::parse_continously::ContinousParser, render::Render};

struct PlayerData {
    player_name: String,
    time_start: Instant,
    text_start: String,
    text_end: Option<(String, Duration)>,
}

impl PartialEq for PlayerData {
    fn eq(&self, other: &Self) -> bool {
        match (&self.text_end, &other.text_end) {
            (None, None) => true,
            (Some(v1), Some(v2)) => v1 == v2,
            _ => false,
        }
    }
}

impl Eq for PlayerData {}

impl Ord for PlayerData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.text_end, &other.text_end) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (Some(v1), Some(v2)) => v1.1.cmp(&v2.1),
        }
    }
}

impl PartialOrd for PlayerData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PlayerData {
    fn finish(&mut self) {
        let duration = self.time_start.elapsed();
        let seconds = duration.as_secs() % 60;
        let minutes = duration.as_secs() / 60;

        self.text_end = Some((
            format!("{} loaded in {:02}:{:02}", self.player_name, minutes, seconds),
            duration
        ))
    }

    fn render(&self, ui: &mut egui::Ui) {
        match self.text_end.as_ref() {
            Some(text) => {
                ui.colored_label(Color32::GREEN, &text.0);
            },
            None => {
                ui.colored_label(Color32::YELLOW, &self.text_start);
            },
        }
    }
}

impl From<String> for PlayerData {
    fn from(value: String) -> Self {
        Self {
            text_start: format!("{} loading", value),
            player_name: value,
            time_start: Instant::now(),
            text_end: None
        }
    }
}

pub struct PlayerLoading {
    show: bool,
    player_states: Vec<PlayerData>,
    owned_player: PlayerData,
    
    continous_parser: ContinousParser<Token>,
}

impl Default for PlayerLoading {
    fn default() -> Self {
        Self {
            show: false,
            player_states: Default::default(),
            continous_parser: ContinousParser::new(SubscribeCode::Tokenizer),
            owned_player: "".to_owned().into()
        }
    }
}

impl Render for PlayerLoading {
    type Response = usize;

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        while let Some(r) = self.continous_parser.try_recv() {
            match r {
                Token::GameStateManagerChange(_, GameState::Generating) => {
                    self.show = true;
                    self.owned_player = "You".to_owned().into();
                },
                Token::GameStateManagerChange(_, GameState::ReadyToStopElevatorRide) => {
                    self.owned_player.finish();
                },
                Token::GameStateManagerChange(_, GameState::ReadyToStartLevel) => {
                    self.show = false;
                    self.player_states.clear();
                },
                Token::PlayerStateChange(name, GameState::Generating) => {
                    if self.player_states.iter().find(|e| e.player_name == name).is_none() {
                        self.player_states.push(name.to_owned().into());
                        self.player_states.sort();
                    }
                },
                Token::PlayerStateChange(name, GameState::ReadyToStopElevatorRide) => {
                    self.player_states.iter_mut()
                        .find(|p| p.player_name == name)
                        .map(|ps| ps.finish());
                }
                Token::UserExitLobby | Token::GameEndAbort | Token::LogFileEnd | Token::GameStateManagerChange(_, Lobby) => {
                    self.show = false;
                    self.player_states.clear();
                }
                _ => {}
            }
        }

        if self.show {
            
            self.owned_player.render(ui);
            for data in &self.player_states {
                data.render(ui);
            }

            ui.separator();

            return 28 + self.player_states.len() * 22
        }

        0
    }
}