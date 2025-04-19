use core::{
    logs::{
        collectable_mapper::CollectableMapper,
        live_parser::LiveParser,
        token_parser::TokenParserT,
        tokenizer::{GenerationTokenizer, GenericTokenizer, RunTokenizer, Tokenizer},
    },
    run::{objectives::run_objective::RunObjective, run_enum::RunEnum, timed_run::LevelRun},
    save_manager::SaveManager,
};

use crate::{
    live::{
        key_guess::KeyGuesserVisual, mapper::Mapper, objective_reader::LevelObjectiveReader,
        run_counter::RunCounterBuffer, run_renderer::RunRendererBuffer,
    },
    render::{BufferedRender, Render},
    windows::settings_window::SettingsWindow,
};

#[derive(Default)]
pub struct LiveRender<'a> {
    mapper: Option<Mapper<'a>>,
    run_counter: Option<RunCounterBuffer>,
    key_guess: Option<KeyGuesserVisual<'a>>,

    level_renderer: Option<RunRendererBuffer<LevelRun, RunObjective>>,
    level_obj_reader: Option<LevelObjectiveReader>,

    last_y_size: usize,
}

impl<'a> LiveRender<'a> {
    fn get_run_from_parser(parser: &LiveParser) -> Option<&LevelRun> {
        match parser.get_run_parser() {
            Some(run_p) => Some(run_p.into_result()),
            None => parser.into_result().get_runs().last(),
        }
    }

    pub fn add_mapper(&mut self, mapper: Mapper<'a>) {
        self.mapper = Some(mapper);
    }

    pub fn add_run_counter(&mut self, run_counter: RunCounterBuffer) {
        self.run_counter = Some(run_counter);
    }

    pub fn add_key_guess(&mut self, key_geuss: KeyGuesserVisual<'a>) {
        self.key_guess = Some(key_geuss);
    }

    pub fn add_level_obj_reader(&mut self, level_obj_reader: LevelObjectiveReader) {
        self.level_obj_reader = Some(level_obj_reader);
    }

    pub fn add_level_renderer(
        &mut self,
        level_renderer: RunRendererBuffer<LevelRun, RunObjective>,
    ) {
        self.level_renderer = Some(level_renderer);
    }

    pub fn update(&mut self, parser: &LiveParser, save_manager: &SaveManager) {
        self.mapper.as_mut().map(|v| v.update(parser));
        self.run_counter
            .as_mut()
            .map(|v| v.update(parser.into_result()));

        if let Some(run) = Self::get_run_from_parser(parser) {
            self.level_renderer.as_mut().map(|v| {
                v.update(
                    parser.into_result().get_run_counter(),
                    run,
                    save_manager,
                    &self.level_obj_reader,
                )
            });
        }
    }
}

impl<'a> Render for LiveRender<'a> {
    type Response = (usize, bool);

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        let mut result = (20, false);

        result.0 += self.run_counter.render(ui).unwrap_or_default();
        result.0 += self.mapper.render(ui).unwrap_or_default();
        result.0 += self.level_obj_reader.render(ui).unwrap_or_default();
        result.0 += self.key_guess.render(ui).unwrap_or_default();
        result.0 += self.level_renderer.render(ui).unwrap_or_default();

        if result.0 != self.last_y_size {
            result.1 = true;
            self.last_y_size = result.0;
        }

        result
    }
}

pub struct LiveWindow<'a> {
    frame_counter: u8,
    run_counter: usize,
    parser: LiveParser,
    to_be_added_runs: Vec<RunEnum>,

    render: LiveRender<'a>,

    tokenizer: GenericTokenizer,
}

impl<'a> Drop for LiveWindow<'a> {
    fn drop(&mut self) {
        self.parser.stop_watcher();
    }
}

impl<'a> LiveWindow<'a> {
    pub fn new(
        collectable_mapper: Option<&'a CollectableMapper>,
        settings: &SettingsWindow,
    ) -> Self {
        let mut live_render = LiveRender::default();

        if settings.get_show_warden_mapper() {
            live_render.add_mapper(Mapper::new(collectable_mapper, settings))
        }
        if settings.get_show_run_counter() {
            live_render.add_run_counter(RunCounterBuffer::default())
        };
        if settings.get_show_code_guess() {
            live_render.add_key_guess(KeyGuesserVisual::new(settings));
        }
        if settings.get_show_splitter() || settings.get_show_warden_mapper() {
            live_render.add_level_obj_reader(LevelObjectiveReader::default());
        }
        if settings.get_show_splitter() {
            live_render.add_level_renderer(RunRendererBuffer::new("".to_string(), settings));
        }

        let mut result = Self {
            frame_counter: 0,
            run_counter: 0,
            parser: LiveParser::default(),
            render: live_render,
            tokenizer: GenericTokenizer::default()
                .add_tokenizer(RunTokenizer)
                .add_tokenizer(GenerationTokenizer),
            to_be_added_runs: Vec::new(),
        };

        result
            .parser
            .start_watcher(settings.get_logs_folder().clone());

        result
    }

    pub fn get_vec_list(&mut self) -> &mut Vec<RunEnum> {
        &mut self.to_be_added_runs
    }
}

impl<'a> BufferedRender for LiveWindow<'a> {
    type Response = (usize, bool);
    type UpdateData = SaveManager;
    type Render = LiveRender<'a>;

    fn update(&mut self, save_manager: &Self::UpdateData) {
        self.frame_counter += 1;
        if self.frame_counter == 32 {
            self.frame_counter = 0;
            if self.parser.load_file() {
                self.run_counter = 0;
            }
            let new_lines = self.parser.load_text();

            let tokens = self.tokenizer.tokenize(&new_lines);
            self.parser.parse_continously(tokens.into_iter());

            self.render.update(&self.parser, save_manager);
            for run in self
                .parser
                .into_result()
                .get_runs()
                .iter()
                .skip(self.run_counter)
            {
                self.to_be_added_runs.push(RunEnum::Level(run.clone()));
                self.run_counter += 1;
            }
        }
    }

    fn reset(&mut self) {}

    fn get_renderer(&mut self) -> &mut Self::Render {
        &mut self.render
    }
}
