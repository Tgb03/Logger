use egui::Ui;
use strum::IntoEnumIterator;

use crate::{run::{objectives::{game_run_objective::GameRunObjective, game_run_rundown::GameRunRundown, run_objective::RunObjective}, run_enum::RunEnum, timed_run::LevelRun, traits::Run}, save_run::SaveManager};

use super::{sorter_window::add_sorter_buttons, traits::RenderRun};

pub struct LogParserWindow {

  timed_runs: Vec<LevelRun>,

  set_all_secondary: bool,
  set_all_overload: bool,
  set_all_glitched: bool,
  set_all_early_drop: bool,

  game_rundown: GameRunRundown,
  game_obj: GameRunObjective,
  player_count_input: String,
  player_count: u8,

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
      player_count: Default::default(),
      game_rundown: GameRunRundown::Rundown1, 
      game_obj: GameRunObjective::AnyPercent,
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
      if ui.button(super::create_text("Save ALL runs")).clicked() {
        save_manager.save_multiple(
          self.timed_runs
            .iter()
            .map(|f| RunEnum::Level(f.clone()))
            .collect()
        );
        self.timed_runs = Vec::new();
      }
    
      if secondary_checkbox.clicked() {
        for timed_run in &mut self.timed_runs {
          timed_run.set_objective(&timed_run.get_objective::<RunObjective>().unwrap().with_glitched(self.set_all_secondary));
        }
      }
      
      if overload_checkbox.clicked() {
        for timed_run in &mut self.timed_runs {
          timed_run.set_objective(&timed_run.get_objective::<RunObjective>().unwrap().with_glitched(self.set_all_overload));
        }
      }

      if glitched_checkbox.clicked() {
        for timed_run in &mut self.timed_runs {
          timed_run.set_objective(&timed_run.get_objective::<RunObjective>().unwrap().with_glitched(self.set_all_glitched));
        }
      }

      if early_drop_checkbox.clicked() {
        for timed_run in &mut self.timed_runs {
          timed_run.set_objective(&timed_run.get_objective::<RunObjective>().unwrap().with_glitched(self.set_all_early_drop));
        }
      }
    });

    ui.horizontal(|ui| {
      egui::ComboBox::from_label("")
      .selected_text(super::create_text(format!("{}", self.game_rundown)))
      .height(256.0)
      .show_ui(ui, |ui| {
        
        for key in GameRunRundown::iter() {
          ui.selectable_value(&mut self.game_rundown, key.clone(), super::create_text(key.to_string()));
        }

      });
      
      egui::ComboBox::from_label(" ")
      .selected_text(super::create_text(format!("{}", self.game_obj)))
      .height(256.0)
      .show_ui(ui, |ui| {
        
        for key in GameRunObjective::iter() {
          ui.selectable_value(&mut self.game_obj, key.clone(), super::create_text(key.to_string()));
        }

      });

      if ui.add(egui::TextEdit::singleline(&mut self.player_count_input)
          .desired_width(100.0)
        ).changed() {
        
        let player_count = self.player_count_input.parse::<u8>().ok();
        
        if let Some(player_count) = player_count {
          self.player_count = player_count;
        }
      }
    });
    
    egui::ScrollArea::vertical().show_rows(ui, ui.text_style_height(&egui::TextStyle::Body), self.timed_runs.len(), |ui, row_range| {
      let mut for_removal = Vec::new();

      for row in row_range {
        let timed_run = &mut self.timed_runs[row];

        let result = timed_run.show_editable(ui);

        if result.delete { for_removal.push(row); }
      }

      for id in for_removal.iter().rev() {
        self.timed_runs.remove(*id);
      }
    });
  }

}