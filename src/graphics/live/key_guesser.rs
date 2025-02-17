use egui::{Color32, FontId, Ui};

use crate::{graphics::{create_text, settings_window::SettingsWindow}, key_guess::KeyGuess};

pub struct KeyGuesser<'a> {

  key_guesser: KeyGuess<'a>,
  key_guess_input_string: String,

}

impl<'a> Default for KeyGuesser<'a> {
  fn default() -> Self {
    Self { 
      key_guesser: Default::default(), 
      key_guess_input_string: "----".to_string(), 
    }
  }
}

impl<'a> KeyGuesser<'a> {
  
  pub fn render_key_guesser(&mut self, ui: &mut Ui, settings: &SettingsWindow) -> usize {

    let line_count = settings.get_code_guess_line_count();
    let line_width = settings.get_code_guess_line_width();

    ui.horizontal(|ui| {
      
      ui.label(create_text("Code: "));
      if ui.add(egui::TextEdit::singleline(&mut self.key_guess_input_string)
        .desired_width(32.0)
        .font(FontId::new(12.0, egui::FontFamily::Name("jetbrains_mono".into())))
        .background_color(Color32::from_rgb(32, 32, 32))
        .text_color(Color32::WHITE))
        .changed() {
          self.key_guesser = KeyGuess::default();

          for (id, key) in self.key_guess_input_string.bytes().enumerate() {
            if 97 <= key && key <= 122 {
              self.key_guesser.add_key(id as u8, key);
            } 

            if 65 <= key && key <= 90 {
              self.key_guesser.add_key(id as u8, key + 32);
            }
          }
        };
      ui.label(create_text(format!("Count: {}", self.key_guesser.len())));

    });

    let list = self.key_guesser.get_list();
    let len = self.key_guesser.len();

    for line in 0..line_count {
      if len <= line * line_width {
        ui.separator();
        return 27 + line * 22
      }

      ui.horizontal(|ui| {

        for i in 0..line_width {
          if len == i + line * line_width {
            break
          }

          ui.label(create_text(format!("{}", std::str::from_utf8(&list[i + line * line_width][0..4]).unwrap().to_ascii_uppercase())));
        }

      });
    }

    ui.separator();

    27 + line_count * 22
  }

}
