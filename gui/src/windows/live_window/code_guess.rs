use egui::Color32;

use crate::{render::Render, windows::settings_window::SettingsWindow};

static ALL_CODES: &'static [u8] = include_bytes!("..\\..\\..\\..\\keys.txt");

pub struct CodeGuess {
    selected: String,
    valid: [bool; 733],

    max_lines: usize,
    codes_per_line: usize,
}

impl CodeGuess {
    pub fn new(settings_window: &SettingsWindow) -> Self {
        Self {
            selected: String::new(),
            valid: [true; 733],
            max_lines: settings_window
                .get::<i32>("code_guess_line_count")
                .unwrap_or(3) as usize,
            codes_per_line: settings_window
                .get::<i32>("code_guess_word_count")
                .unwrap_or(3) as usize,
        }
    }

    fn check(&mut self) {
        for it in &mut self.valid {
            *it = true;
        }

        for (id, ch) in self.selected.as_bytes().iter().enumerate() {
            let upper_ch = if (b'a'..=b'z').contains(ch) {
                ch - 32
            } else {
                *ch
            };

            if !(b'A'..=b'Z').contains(&upper_ch) {
                continue;
            }

            for checking in 0..733 {
                if ALL_CODES[checking * 4 + id] != upper_ch {
                    self.valid[checking] = false;
                }
            }
        }
    }
}

impl Render for CodeGuess {
    type Response = usize;

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        ui.horizontal(|ui| {
            ui.label("Code: ");
            if ui
                .add(
                    egui::TextEdit::singleline(&mut self.selected)
                        .desired_width(32.0)
                        .background_color(Color32::from_rgb(32, 32, 32))
                        .text_color(Color32::WHITE),
                )
                .changed()
            {
                self.check();
            }
        });

        let mut counter = 0;
        let mut found = 0;
        let mut in_line_counter = 0;

        while counter < 733 && found < self.max_lines * self.codes_per_line {
            ui.horizontal(|ui| {
                while counter < 733
                    && found < self.max_lines * self.codes_per_line
                    && in_line_counter < self.codes_per_line
                {
                    if self.valid[counter] {
                        ui.label(unsafe {
                            std::str::from_utf8_unchecked(
                                &ALL_CODES[(counter * 4)..(counter * 4 + 4)],
                            )
                        });

                        found += 1;
                        in_line_counter += 1;
                    }

                    counter += 1;
                }
            });

            in_line_counter = 0;
        }

        ui.separator();

        27 + found.saturating_sub(1) / self.codes_per_line * 22
    }
}
