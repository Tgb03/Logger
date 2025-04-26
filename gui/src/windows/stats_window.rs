use core::{
    logs::parser::ParserResult, run::{
        timed_run::LevelRun,
        traits::{Run, Timed},
    }, time::Time
};
use std::{collections::{HashMap, HashSet}, fmt::Display, usize};

use egui::{Color32, Ui};
use itertools::Itertools;

use crate::render::Render;

pub struct LevelStat {
    run_count: usize,
    win_count: usize,

    fastest_time: Option<Time>,
    slowest_time : Option<Time>,
    
    total_time: Time,
    percent: f64,
}

impl Default for LevelStat {
    fn default() -> Self {
        Self { 
            run_count: Default::default(), 
            win_count: Default::default(), 
            fastest_time: None, 
            slowest_time: None, 
            total_time: Default::default(), 
            percent: Default::default() 
        }
    }
}

impl LevelStat {
    pub fn add_run(&mut self, run: &LevelRun) {
        self.run_count += 1;
        if run.is_win() {
            self.win_count += 1;

            self.slowest_time = match self.slowest_time {
                Some(st) => Some(st.max(run.get_time())),
                None => Some(run.get_time()),
            };

            self.fastest_time = match self.fastest_time {
                Some(st) => Some(st.min(run.get_time())),
                None => Some(run.get_time()),
            };
        }
        self.total_time += run.get_time();
    }

    pub fn calculate_percent(&mut self, total_time: Time) {
        self.percent = self.total_time.get_stamp() as f64 / total_time.get_stamp() as f64 * 100.0;
    }
}

impl From<&LevelRun> for LevelStat {
    fn from(value: &LevelRun) -> Self {
        let mut s = LevelStat::default();
        s.add_run(value);
        s
    }
}

impl Display for LevelStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  {: >3}    {: >7.3}%    {: >13}    {: >7.3}%    {: >13}   {: >13}", 
            self.run_count, 
            self.win_count as f32 * 100.0 / self.run_count as f32, 
            self.total_time.to_string(),
            self.percent,
            self.fastest_time.map(|v| v.to_string()).unwrap_or("None".to_owned()),
            self.slowest_time.map(|v| v.to_string()).unwrap_or("None".to_owned()),
        )
    }
}

#[derive(Default)]
pub struct Stats {
    text_winrate: String,        //            f32
    text_number_of_runs: String, //     usize
    text_total_time: String,

    text_split_times: Vec<String>,
}

impl Stats {
    pub fn build<'a, I>(runs_iter: I) -> Self
    where
        I: Iterator<Item = &'a LevelRun>,
    {
        let mut number_of_runs = 0usize;
        let mut win_counter = 0usize;
        let mut time_total = Time::default();

        let mut splits: HashMap<String, LevelStat> = HashMap::new();

        for run in runs_iter {
            if run.is_win() {
                win_counter += 1;
            }
            number_of_runs += 1;
            time_total += run.get_time();

            // should never fail but just in case it is an if statement
            if let Some(name) = run.get_name().split('_').next() {
                match splits.get_mut(name) {
                    Some(level_stat) => level_stat.add_run(run),
                    None => {
                        splits.insert(name.to_owned(), run.into());
                    }
                }
            }
        }

        for (_, s) in splits.iter_mut() {
            s.calculate_percent(time_total);
        }

        Self {
            text_total_time: format!("   Total time {}", time_total.to_string()),
            text_winrate: format!("   Winrate: {:.3}%", win_counter as f32 * 100.0 / number_of_runs as f32),
            text_number_of_runs: format!("   Number of runs: {number_of_runs}"),
            text_split_times: splits
                .iter()
                .sorted_by(|(a, _), (b, _)| a.cmp(b))
                .map(|(name, stat)| format!("   {: <8}: {}", name, stat))
                .collect(),
        }
    }
}

impl Render for Stats {
    type Response = ();

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        ui.separator();

        ui.label(&self.text_total_time);
        ui.label(&self.text_winrate);
        ui.label(&self.text_number_of_runs);

        ui.label("    Level     Runs       Win%         Time           Time%      FastestTime      SlowestTime");

        ui.separator();

        egui::ScrollArea::vertical().show_rows(
            ui,
            ui.text_style_height(&egui::TextStyle::Body),
            self.text_split_times.len(),
            |ui, row_range| {
            
            for s in row_range {
                ui.label(&self.text_split_times[s]);
            }
        });
    }
}

pub struct StatsWindow {
    stats_shown: Stats,
    run_vec: Vec<LevelRun>,

    name_filter: String,
    negative_name_filter: String,
    min_time_filter: Time,
    max_time_filter: Time,
    min_stamp_filter: usize,
    max_stamp_filter: usize,

    win_filter: Option<bool>,

    secondary_filter: Option<bool>,
    overload_filter: Option<bool>,

    string_inputs: [String; 4],
}

impl StatsWindow {
    fn make_combo_box(bool: &mut Option<bool>, name: &str, ui: &mut Ui) -> bool {
        let mut changed = false;

        egui::ComboBox::from_label(name)
            .selected_text(format!("{:?}", bool))
            .height(300.0)
            .show_ui(ui, |ui| {

                changed = changed || ui.selectable_value(bool, None, "None").clicked();
                changed = changed || ui.selectable_value(bool, Some(false), "False").clicked();
                changed = changed || ui.selectable_value(bool, Some(true), "True").clicked();
            
            });

        changed
    }

    pub fn new(run_vec: Vec<LevelRun>) -> Self {
        let mut s = Self {
            stats_shown: Stats::default(),
            run_vec,
            name_filter: "".to_owned(),
            negative_name_filter: "".to_owned(),
            min_time_filter: Time::default(),
            max_time_filter: Time::from("99:59:59.999").unwrap(),
            min_stamp_filter: 0,
            max_stamp_filter: usize::MAX,
            win_filter: None,
            secondary_filter: None,
            overload_filter: None,
            string_inputs: [
                "00:00:00.000".to_owned(),
                "99:59:59.999".to_owned(),
                "0".to_owned(),
                "999999".to_owned(),
            ],
        };

        s.update();

        s
    }

    pub fn update(&mut self) {

        self.stats_shown = Stats::build(
            self.run_vec
                .iter()
                .filter(|r| 
                    self.name_filter
                        .split('|')
                        .any(|filter| 
                            r
                                .get_name()
                                .contains(filter)
                        )
                )
                .filter(|r|
                    self.negative_name_filter
                        .split('|')
                        .all(|filter| {
                            if filter == "" { return true }

                            !r
                                .get_name()
                                .contains(filter)
                        })
                )
                .filter(|r| {
                    self.min_time_filter <= r.get_time() && r.get_time() <= self.max_time_filter
                })
                .filter(|r| self.min_stamp_filter <= r.len() && r.len() <= self.max_stamp_filter)
                .filter(|r| self.win_filter.is_none_or(|f| f == r.is_win()))
                .filter(|r| self.secondary_filter.is_none_or(|f| f == r.get_objective_str().contains("_sec")))
                .filter(|r| self.overload_filter.is_none_or(|f| f == r.get_objective_str().contains("_ovrl")))
        )
    }
}

impl Render for StatsWindow {
    type Response = ();

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        let mut changed = false;

        ui.horizontal(|ui| {
            ui.add_space(5.0);
            ui.monospace("Name filter: ");
            if ui
                .add(
                    egui::TextEdit::singleline(&mut self.name_filter)
                        .desired_width(128.0)
                        .background_color(Color32::from_rgb(32, 32, 32))
                        .text_color(Color32::WHITE),
                )
                .changed()
            {
                self.name_filter = self.name_filter.to_uppercase();
                changed = true;
            }
        });

        ui.horizontal(|ui| {
            ui.add_space(5.0);
            ui.monospace("Negative name filter: ");
            if ui
                .add(
                    egui::TextEdit::singleline(&mut self.negative_name_filter)
                        .desired_width(128.0)
                        .background_color(Color32::from_rgb(32, 32, 32))
                        .text_color(Color32::WHITE),
                )
                .changed()
            {
                self.negative_name_filter = self.negative_name_filter.to_uppercase();
                changed = true;
            }
        });

        ui.horizontal(|ui| {
            ui.add_space(5.0);
            ui.monospace("Minimum time filter: ");
            if ui
                .add(
                    egui::TextEdit::singleline(&mut self.string_inputs[0])
                        .desired_width(96.0)
                        .background_color(Color32::from_rgb(32, 32, 32))
                        .text_color(Color32::WHITE),
                )
                .changed()
            {
                changed = true;
                if let Ok(time) = Time::from(&self.string_inputs[0]) {
                    self.min_time_filter = time;
                }
            }
        });

        ui.horizontal(|ui| {
            ui.add_space(5.0);
            ui.monospace("Maximum time filter: ");
            if ui
                .add(
                    egui::TextEdit::singleline(&mut self.string_inputs[1])
                        .desired_width(96.0)
                        .background_color(Color32::from_rgb(32, 32, 32))
                        .text_color(Color32::WHITE),
                )
                .changed()
            {
                changed = true;
                if let Ok(time) = Time::from(&self.string_inputs[1]) {
                    self.max_time_filter = time;
                }
            }
        });

        ui.horizontal(|ui| {
            ui.add_space(5.0);
            ui.monospace("Minimum stamp filter: ");
            if ui
                .add(
                    egui::TextEdit::singleline(&mut self.string_inputs[2])
                        .desired_width(64.0)
                        .background_color(Color32::from_rgb(32, 32, 32))
                        .text_color(Color32::WHITE),
                )
                .changed()
            {
                changed = true;
                if let Ok(stamp) = self.string_inputs[2].parse::<usize>() {
                    self.min_stamp_filter = stamp;
                }
            }
        });

        ui.horizontal(|ui| {
            ui.add_space(5.0);
            ui.monospace("Maximum stamp filter: ");
            if ui
                .add(
                    egui::TextEdit::singleline(&mut self.string_inputs[3])
                        .desired_width(64.0)
                        .background_color(Color32::from_rgb(32, 32, 32))
                        .text_color(Color32::WHITE),
                )
                .changed()
            {
                changed = true;
                if let Ok(stamp) = self.string_inputs[3].parse::<usize>() {
                    self.max_stamp_filter = stamp;
                }
            }
        });

        if Self::make_combo_box(&mut self.win_filter, "Win filter", ui) {
            changed = true;
        }

        if Self::make_combo_box(&mut self.secondary_filter, "Secondary filter", ui) {
            changed = true;
        }

        if Self::make_combo_box(&mut self.overload_filter, "Overload filter", ui) {
            changed = true;
        }

        if changed {
            self.update();
        }

        self.stats_shown.render(ui);
    }
}

impl From<ParserResult> for StatsWindow {
    fn from(value: ParserResult) -> Self {
        let hash: HashSet<LevelRun> =
            HashSet::from_iter(Into::<Vec<LevelRun>>::into(value));
        let runs = hash.into_iter().collect();
        Self::new(runs)
    }
}
