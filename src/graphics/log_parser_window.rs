use std::fs::File;

use egui::{Color32, Ui};
use strum::IntoEnumIterator;

use crate::{export::Export, run::{objectives::{game_objective::GameObjective, game_run_objective::GameRunObjective, game_run_rundown::GameRunRundown, run_objective::RunObjective, Objective}, run_enum::RunEnum, timed_run::{GameRun, LevelRun}, traits::{Run, Timed}}, save_run::SaveManager};

use super::{sorter_window::add_sorter_buttons, traits::RenderResult};

pub struct LogParserWindow {

  timed_runs: Vec<LevelRun>,

  set_all_secondary: bool,
  set_all_overload: bool,
  set_all_glitched: bool,
  set_all_early_drop: bool,

  game_obj: GameObjective,
  player_count_input: String,

}

impl Default for LogParserWindow {
  fn default() -> Self {
    Self { 
      timed_runs: Default::default(), 
      set_all_secondary: Default::default(), 
      set_all_overload: Default::default(), 
      set_all_glitched: Default::default(), 
      set_all_early_drop: Default::default(), 
      player_count_input: Default::default(),
      game_obj: GameObjective::default(),
    }
  }
}

impl LogParserWindow {

  pub fn set_times(&mut self, times: Vec<LevelRun>) {
    self.timed_runs = times;
  }

  pub fn show(&mut self, ui: &mut Ui, save_manager: &mut SaveManager) {
    // handles all sorters
    add_sorter_buttons(ui, &mut self.timed_runs);

    // handles all the set all buttons.
    ui.horizontal(|ui| {
      let secondary_checkbox = ui.checkbox(&mut self.set_all_secondary, super::create_text("Set ALL secondary"));
      let overload_checkbox = ui.checkbox(&mut self.set_all_overload, super::create_text("Set ALL overload"));
      let glitched_checkbox = ui.checkbox(&mut self.set_all_glitched, super::create_text("Set ALL glitched"));
      let early_drop_checkbox = ui.checkbox(&mut self.set_all_early_drop, super::create_text("Set ALL early drop"));
    
      if secondary_checkbox.clicked() {
        for timed_run in &mut self.timed_runs {
          timed_run.set_objective(&timed_run.get_objective::<RunObjective>().unwrap().with_secondary(self.set_all_secondary));
        }
      }
      
      if overload_checkbox.clicked() {
        for timed_run in &mut self.timed_runs {
          timed_run.set_objective(&timed_run.get_objective::<RunObjective>().unwrap().with_overload(self.set_all_overload));
        }
      }

      if glitched_checkbox.clicked() {
        for timed_run in &mut self.timed_runs {
          timed_run.set_objective(&timed_run.get_objective::<RunObjective>().unwrap().with_glitched(self.set_all_glitched));
        }
      }

      if early_drop_checkbox.clicked() {
        for timed_run in &mut self.timed_runs {
          timed_run.set_objective(&timed_run.get_objective::<RunObjective>().unwrap().with_early_drop(self.set_all_early_drop));
        }
      }
    });

    ui.horizontal(|ui| {
      if ui.button(super::create_text("Save ALL runs")).clicked() {
        save_manager.save_multiple(
          self.timed_runs
            .iter()
            .map(|f| RunEnum::Level(f.clone()))
            .collect()
        );
        self.timed_runs = Vec::new();
      }
      if ui.button(super::create_text("Save ALL as FULL GAME RUN")).clicked() {
        let mut game_run = GameRun::new(self.game_obj.clone());

        self.timed_runs.sort_by(|a, b| a.get_objective_str().cmp(b.get_objective_str()));
        for run in self.timed_runs.drain(0..self.timed_runs.len()) {
          game_run.add_split(run);
        }

        game_run.validate();

        save_manager.save(RunEnum::Game(game_run));
      }
      if ui.button(super::create_text("Export File")).clicked() {
        if let Some(path) = rfd::FileDialog::new()
          .set_title("Export to file")
          .set_file_name("new_file.csv")
          .save_file() {
          
          match File::create(path) {
            Ok(file) => {
              Export::export_times(self.timed_runs.iter(), file);
            },
            Err(_) => {},
          }

        }
      }
    });

    ui.horizontal(|ui| {
      egui::ComboBox::from_label("")
      .selected_text(super::create_text(format!("{}", self.game_obj.get_rundown())))
      .height(256.0)
      .show_ui(ui, |ui| {
        
        for key in GameRunRundown::iter() {
          ui.selectable_value(self.game_obj.get_mut_rundown(), key.clone(), super::create_text(key.to_string()));
        }

      });
      
      egui::ComboBox::from_label(" ")
      .selected_text(super::create_text(format!("{}", self.game_obj.get_objectives())))
      .height(256.0)
      .show_ui(ui, |ui| {
        
        for key in GameRunObjective::iter() {
          ui.selectable_value(self.game_obj.get_mut_objectives(), key.clone(), super::create_text(key.to_string()));
        }

      });

      if ui.add(egui::TextEdit::singleline(&mut self.player_count_input)
          .desired_width(100.0)
        ).changed() {
        
        let player_count = self.player_count_input.parse::<u8>().ok();
        
        if let Some(player_count) = player_count {
          self.game_obj = self.game_obj.clone().with_player_count(player_count);
        }
      }
    });
    
    egui::ScrollArea::vertical().show_rows(ui, ui.text_style_height(&egui::TextStyle::Body), self.timed_runs.len(), |ui, row_range| {
      let mut for_removal = Vec::new();
      let mut for_saving = Vec::new();

      for row in row_range {
        let timed_run = &mut self.timed_runs[row];
        let mut result = RenderResult::default();

        let time = timed_run.get_time();
        let color = match timed_run.is_win() {
          true => Color32::GREEN,
          false => Color32::RED,
        };
        let mut objective = timed_run.get_objective::<RunObjective>().unwrap();

        ui.horizontal(|ui| {
          
          ui.label(super::create_text(&objective.level_name));
          ui.colored_label(Color32::WHITE, super::create_text(format!("{}p", objective.get_player_count().to_string())));
          
          ui.colored_label(color, super::create_text(time.to_string()));

          ui.colored_label(Color32::WHITE, super::create_text(format!("{:03} stamps", timed_run.len())));

          ui.checkbox(&mut objective.secondary, super::create_text("Secondary"));
          ui.checkbox(&mut objective.overload, super::create_text("Overload"));
          ui.checkbox(&mut objective.glitched, super::create_text("Glitched"));
          ui.checkbox(&mut objective.early_drop, super::create_text("Early Drop"));

          timed_run.set_objective(&objective);

          if ui.button(super::create_text("SAVE RUN")).clicked() {
            result.save = true;
          }

          if ui.button(super::create_text("DELETE")).clicked() {
            result.delete = true;
          }

        });

        if result.delete { for_removal.push(row); }
        if result.save { for_saving.push(row); }
      }

      for id in for_removal.iter().rev() {
        self.timed_runs.remove(*id);
      }

      for id in for_saving.iter().rev() {
        let run = self.timed_runs.remove(*id);
        save_manager.save(RunEnum::Level(run));
      }
    });
  }

}