use std::collections::VecDeque;

use egui::{Color32, Ui};

use crate::{graphics::create_text, run::{run_enum::RunEnum, time::Time, traits::{Run, Timed}}, save_run::SaveManager};

pub struct RunRenderer {
  
  labels: VecDeque<String>,
  best_run_labels: VecDeque<(String, Color32)>,
  best_split_labels: VecDeque<(String, Color32)>,

  current_run_total_time: Time,

  best_run: Option<RunEnum>,
  best_run_total_time: Time,

  size: usize,

  compare_theoretical: bool,
  max_length: usize,

  objective_str: String,

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

      current_run_total_time: Time::default(),
      best_run_total_time: Time::default(),

      size: 0,

      compare_theoretical,
      max_length,
      objective_str: run.get_objective_str().clone(),
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
      let mut copy = self.current_run_total_time;

      self.best_run_labels.extend(
        times_to_be_added.iter()
          .map(|v| {
            copy = copy.add(&v.get_time());
            self.best_run_total_time = self.best_run_total_time.add(
              &best_run.get_time_for_split(v.get_name()).unwrap_or_default()
            );

            match copy.is_smaller_or_equal_than(&self.best_run_total_time) {
              true => (self.best_run_total_time.sub(&copy).to_string_no_hours(), Color32::GREEN),
              false => (copy.sub(&self.best_run_total_time).to_string_no_hours(), Color32::RED),
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
              .unwrap_or(("         ".to_string(), Color32::BLACK))
          })
      );
    }

    self.labels.extend(
      times_to_be_added
        .iter()
        .map(|v| {
          self.current_run_total_time = self.current_run_total_time.add(&v.get_time());

          self.current_run_total_time.to_string()
        })
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
          None => ui.label(create_text("         ")),
        };

        match self.best_split_labels.get(i) {
          Some((label, color)) => ui.colored_label(*color, create_text(label)),
          None => ui.label(create_text("         ")),
        }
      });
    }

    (size + 1) * 22
  }

  pub fn get_objective_str(&self) -> &String {
    &self.objective_str
  }

}
