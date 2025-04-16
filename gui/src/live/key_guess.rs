use core::key_guess::KeyGuess;

use egui::Color32;

use crate::{render::Render, windows::settings_window::SettingsWindow};

pub struct KeyGuesserVisual<'a> {
    key_guess: KeyGuess<'a>,
    codes_per_line: usize,
    max_lines: usize,

    key_guess_input_string: String,
}

impl<'a> KeyGuesserVisual<'a> {
    pub fn new(settings: &SettingsWindow) -> Self {
        Self {
            key_guess: KeyGuess::default(),
            codes_per_line: settings.get_code_guess_line_width(),
            max_lines: settings.get_code_guess_line_count(),
            key_guess_input_string: "----".to_string(),
        }
    }
}

impl<'a> Render for KeyGuesserVisual<'a> {
    type Response = usize;

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        ui.horizontal(|ui| {
            ui.label("Code: ");
            match ui
                .add(
                    egui::TextEdit::singleline(&mut self.key_guess_input_string)
                        .desired_width(32.0)
                        .background_color(Color32::from_rgb(32, 32, 32))
                        .text_color(Color32::WHITE),
                )
                .changed()
            {
                true => {
                    self.key_guess = KeyGuess::default();
                    for (id, key) in self.key_guess_input_string.bytes().enumerate() {
                        if 97 <= key && key <= 122 {
                            self.key_guess.add_key(id as u8, key);
                        }

                        if 65 <= key && key <= 90 {
                            self.key_guess.add_key(id as u8, key + 32);
                        }
                    }
                }
                false => (),
            };
            ui.label(format!("Count: {}", self.key_guess.len()));
        });

        let list = self.key_guess.get_list();
        let len = self.key_guess.len();

        for line in 0..self.max_lines {
            if len <= line * self.codes_per_line {
                ui.separator();
                return 27 + line * 22;
            }

            ui.horizontal(|ui| {
                for i in 0..self.codes_per_line {
                    if len == i + line * self.codes_per_line {
                        break;
                    }

                    ui.label(format!(
                        "{}",
                        std::str::from_utf8(&list[i + line * self.codes_per_line][0..4])
                            .unwrap()
                            .to_ascii_uppercase()
                    ));
                }
            });
        }

        ui.separator();

        27 + self.max_lines * 22
    }
}
