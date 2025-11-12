use core::{run::{objectives::game_objective::GameObjective, timed_run::LevelRun}, save_manager::{SaveManager}};

use egui::Ui;
use glr_core::run_gen_result::RunGeneratorResult;
use glr_lib::dll_exports::enums::SubscribeCode;

use crate::{dll::parse_continously::ContinousParser, render::Render, windows::{live_window::run_renderer::RunRender, settings_window::SettingsWindow}};


pub struct GameRunRenderer {
    render: RunRender<LevelRun>,
    run_buffer: Vec<LevelRun>,
    objective: GameObjective,

    continous_parser: ContinousParser<RunGeneratorResult>,
    no_save_for_frames: usize,
}

impl GameRunRenderer {
    pub fn new(settings: &SettingsWindow, save_manager: &SaveManager, objective: GameObjective) -> Self {
        if let Some(result) = Self::load_data(
            settings,
            save_manager,
            &objective
        ) {
            return result
        }
        
        Self {
            render: RunRender::new(objective.to_string(), settings),
            run_buffer: Vec::new(),
            objective,
            continous_parser: ContinousParser::new(SubscribeCode::RunInfo),
            no_save_for_frames: 5,
        }
    }

    fn load_data(
        settings: &SettingsWindow, 
        save_manager: &SaveManager, 
        objective: &GameObjective
    ) -> Option<Self> {
        let path = SaveManager::get_config_directory()?
            .join("game_run_buffer.json");
        let text_buf = std::fs::read_to_string(path)
            .ok()?;
        let inner = serde_json::from_str::<Vec<LevelRun>>(&text_buf)
            .ok()?;
        let mut run_render = RunRender::new(objective.to_string(), settings);

        for item in &inner {
            run_render.add_split(item, save_manager);
        }

        Some(
            Self {
                render: run_render,
                continous_parser: ContinousParser::new(SubscribeCode::RunInfo),
                no_save_for_frames: 5,
                run_buffer: inner,
                objective: objective.clone(),
            }
        )
    }

    pub fn save_data(&self) -> Option<()> {
        let path = SaveManager::get_config_directory()?
            .join("game_run_buffer.json");
        let text = serde_json::to_string_pretty(&self.run_buffer)
            .ok()?;

        let _ = std::fs::create_dir_all(&path);
        let _ = std::fs::write(&path, text);

        Some(())
    }

    pub fn render(
        &mut self, 
        save_manager: &mut SaveManager,
        settings: &SettingsWindow,
        ui: &mut Ui,
    ) -> usize {
        self.no_save_for_frames = self.no_save_for_frames.saturating_sub(1);

        while let Some(r) = self.continous_parser.try_recv() {
            match r {
                RunGeneratorResult::LevelRun(timed_run) => {
                    let run = timed_run.into();
                    
                    self.render.add_split(&run, save_manager);
                    self.run_buffer.push(run);
                }
                _ => {}
            }
        }

        self.render.render(ui)
    }
}

