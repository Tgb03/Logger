
use egui::Color32;

use crate::{graphics::{create_text, traits::{RenderResult, RenderRun}}, run::{objectives::{game_objective::GameObjective, run_objective::RunObjective}, run_enum::RunEnum, timed_run::{GameRun, LevelRun}, traits::{Run, Timed}}};


impl RenderRun for LevelRun {
  fn show(&self, ui: &mut egui::Ui) -> RenderResult {
    
    let mut result = RenderResult::default();

    ui.horizontal(|ui| {

      let objective = self.get_objective::<RunObjective>().unwrap();
      let time = self.get_time();

      ui.label(create_text(objective.level_name));
      
      let color = match self.is_win() {
        true => Color32::GREEN,
        false => Color32::RED,
      };

      ui.colored_label(color, create_text(time.to_string()));
      ui.label(create_text(format!("{:03}", self.len())));
      
      if ui.button(create_text(format!("DELETE RUN"))).clicked() {
        result.delete = true;
      }

      for stamp in self.get_splits() {
        ui.colored_label(Color32::GRAY, create_text(stamp.to_string_no_hours()));
      }
    });

    result

  }

  fn show_editable(&mut self, ui: &mut egui::Ui) -> RenderResult {
    
    let mut result = RenderResult::default();

    ui.horizontal(|ui| {

      let mut objective = self.get_objective::<RunObjective>().unwrap();
      let time = self.get_time();

      ui.label(create_text(&objective.level_name));
      
      let color = match self.is_win() {
        true => Color32::GREEN,
        false => Color32::RED,
      };
      
      ui.colored_label(color, create_text(time.to_string()));
      ui.label(create_text(format!("{:03}", self.len())));
      
      ui.label(create_text(format!("{} players", objective.player_count)));

      ui.checkbox(&mut objective.secondary, create_text("Secondary"));
      ui.checkbox(&mut objective.overload, create_text("Overload"));
      ui.checkbox(&mut objective.glitched, create_text("Glitched"));
      if ui.checkbox(&mut objective.early_drop, create_text("EarlyDrop")).changed() {
        objective.glitched = objective.early_drop;
      }
      
      if ui.button(create_text(format!("DELETE RUN"))).clicked() {
        result.delete = true;
      }
      
      if ui.button(create_text(format!("SAVE RUN"))).clicked() {
        result.save = true;
      }

      self.set_objective(&objective);

    });

    result

  }
}

impl RenderRun for GameRun {
  fn show(&self, ui: &mut egui::Ui) -> RenderResult {
    let mut result = RenderResult::default();

    ui.horizontal(|ui| {

      let objective = self.get_objective::<GameObjective>().unwrap();
      let time = self.get_time();

      ui.label(create_text::<String>(objective.to_string()));
      
      let color = match self.is_win() {
        true => Color32::GREEN,
        false => Color32::RED,
      };
      
      ui.colored_label(color, create_text(time.to_string()));
      if ui.button(create_text(format!("DELETE RUN"))).clicked() {
        result.delete = true;
      }
      ui.label(create_text(format!("{:03}", self.len())));
      
      let level_names: &[&str] = objective.get_rundown().clone().into();

      todo!()
    });

    result
  }

  fn show_editable(&mut self, ui: &mut egui::Ui) -> RenderResult {
    todo!()
  }
}

impl RenderRun for RunEnum {
  fn show(&self, ui: &mut egui::Ui) -> RenderResult {
    match self {
      RunEnum::Level(timed_run) => timed_run.show(ui),
      RunEnum::Game(timed_run) => timed_run.show(ui),
    }
  }

  fn show_editable(&mut self, ui: &mut egui::Ui) -> RenderResult {
    match self {
      RunEnum::Level(timed_run) => timed_run.show_editable(ui),
      RunEnum::Game(timed_run) => timed_run.show_editable(ui),
    }
  }
}
