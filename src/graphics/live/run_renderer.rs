use std::collections::VecDeque;

use egui::{Color32, Ui};

use crate::{graphics::create_text, run::{run_enum::RunEnum, time::Time, traits::{Run, Timed}}, save_run::SaveManager};

pub struct RunRenderer {
  
  labels: VecDeque<String>,
  best_run_labels: VecDeque<(String, Color32)>,
  best_split_labels: VecDeque<(String, Color32)>,

  best_run: Option<RunEnum>,
  best_run_total_time: Time,

  size: usize,

  compare_theoretical: bool,
  max_length: usize,

}

impl RunRenderer {

  pub fn new<T: Run>(run: &T, compare_best: bool, compare_theoretical: bool, max_length: usize, save_manager: &SaveManager) -> Self {
    let best_run = match compare_best {
      true => save_manager.get_best_run(run.get_objective_str()),
      false => None,
    }.cloned();

    let mut result = Self {
      labels: VecDeque::new(),
      best_run_labels: VecDeque::new(),
      best_split_labels: VecDeque::new(),

      best_run,
      best_run_total_time: Time::default(),

      size: 0,

      compare_theoretical,
      max_length,
    };

    result.update(run, save_manager);

    result
  }

  pub fn update<T: Run>(&mut self, run: &T, save_manager: &SaveManager) {

    let times_to_be_added: Vec<&dyn Timed> = run.get_splits()
      .skip(self.size)
      .filter(|v| v.get_name() != "LOSS")
      .collect();

    if times_to_be_added.is_empty() { return }

    // println!("!!! Got len: {}", times_to_be_added.len());
    // println!("!!! Best Run: {:?}", self.best_run.is_some());
    // println!("!!! Theoretical: {:?}", self.compare_theoretical);

    if let Some(best_run) = &self.best_run {
      self.best_run_labels.extend(
        best_run.get_splits()
          .skip(self.size)
          .zip(times_to_be_added.iter())
          .map(|(s, time): (&dyn Timed, &&dyn Timed)| {
            let run_time = time.get_time();
            let compared_time = s.get_time();
            self.best_run_total_time = self.best_run_total_time.add(&compared_time);
            
            match run_time.is_smaller_or_equal_than(&compared_time) {
              true => (compared_time.sub(&run_time).to_string_no_hours(), Color32::GREEN),
              false => (run_time.sub(&compared_time).to_string_no_hours(), Color32::RED),
            }
          })
      );
    }

    if self.compare_theoretical {
      self.best_split_labels.extend(
        times_to_be_added.iter()
          .map(|run_split| {
            save_manager
              .get_best_split(run.get_objective_str(), run_split.get_name())
              .map(|best_split| {
                let run_split = run_split.get_time();
                // println!("!!! Looked up: {}", best_split.to_string());
                
                match run_split.is_smaller_or_equal_than(best_split) {
                  true => (best_split.sub(&run_split).to_string_no_hours(), Color32::GREEN),
                  false => (run_split.sub(best_split).to_string_no_hours(), Color32::RED),
                }
              })
              .unwrap_or(("            ".to_string(), Color32::BLACK))
          })
      );
    }

    self.labels.extend(
      times_to_be_added.iter().map(|v| v.get_time().to_string())
    );

    self.size += times_to_be_added.len();

    while self.labels.len() > self.max_length {
      self.labels.pop_front();
      self.best_run_labels.pop_front();
      self.best_split_labels.pop_front();
    }
  }

  pub fn render(&self, ui: &mut Ui) -> usize {
    let size = self.labels.len();
    
    for i in 0..size {
      ui.horizontal(|ui| {
        match self.labels.get(i) {
          Some(label) => ui.label(create_text(label)),
          None => ui.label(create_text("            ")),
        };

        match self.best_run_labels.get(i) {
          Some((label, color)) => ui.colored_label(*color, create_text(label)),
          None => ui.label(create_text("            ")),
        };

        match self.best_split_labels.get(i) {
          Some((label, color)) => ui.colored_label(*color, create_text(label)),
          None => ui.label(create_text("            ")),
        }
      });
    }

    (size + 1) * 22
  }

  pub fn render_run<T: Run>(ui: &mut Ui, run: &T, objective: Option<&String>, compare_best: bool, compare_theoretical: bool, mut max_length: usize, save_manager: &SaveManager) -> usize {
    
    let mut size = run.len();
    let objective = match objective {
      None => &run.get_objective_str(),
      Some(obj) => obj,
    };
    
    let best_run = save_manager.get_best_run(&objective);
    
    let mut current_time = Time::default();
    let mut compared_run_time = Time::default();

    max_length += match run.is_win() {
      true => 1,
      false => 0,
    };
    let skip_count = size.saturating_sub(max_length);
    let mut splits = run
      .get_splits();

    for _ in 0..skip_count {
      if let Some(split) = splits.next() {
        current_time = current_time.add(&split.get_time());

        if let Some(split) = best_run.map(|v| v.get_time_for_split(split.get_name())).flatten() {
          compared_run_time = compared_run_time.add(&split);
        }
      }
    }

    ui.vertical(|ui| {

      for split in splits {

        if split.get_name() == "LOSS" {
          size -= 1;
          
          continue;
        }
        
        ui.horizontal(|ui| {
          
          let split_time = split.get_time();
          //ui.label(create_text(split.get_name()));

          current_time = current_time.add(&split_time);
          ui.label(create_text(current_time.to_string()));
          
          if compare_best {
            if best_run.is_some() {
              compared_run_time = compared_run_time.add(
                &best_run.map(|r| r.get_time_for_split(split.get_name()))
                  .flatten()
                  .unwrap_or_default()
              );

              let (time, color) = match current_time.is_smaller_or_equal_than(&compared_run_time) {
                true => (compared_run_time.sub(&current_time), Color32::GREEN),
                false => (current_time.sub(&compared_run_time), Color32::RED),
              };

              ui.colored_label(color, create_text(time.to_string_no_hours()));
            } else {
              ui.label(create_text("         "));
            }
          }

          if compare_theoretical {
            if let Some(best_split) = save_manager.get_best_split(&objective, split.get_name()) {
              let (time, color) = match split_time.is_smaller_or_equal_than(best_split) {
                true => (best_split.sub(&split_time), Color32::GREEN),
                false => (split_time.sub(best_split), Color32::RED),
              };

              ui.colored_label(color, create_text(time.to_string_no_hours()));
            }
          }
        
        });
      }

    });

    ui.label(create_text(format!("Rendering: {}", objective)));

    (size + 1) * 22
  }

}
