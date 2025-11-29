use core::save_manager::SaveManager;

use crate::{
    render::Render,
    windows::{
        live_window::{
            code_guess::CodeGuess,
            mapper::Mapper,
            objective_reader::{LevelObjectiveReader, UpdateObjective},
            run_counter::RunCounter,
            run_renderer::LevelRunRenderer,
            seed_indexer::SeedIndexer,
            timer::Timer,
        },
        settings_window::SettingsWindow,
    },
};

#[derive(Default)]
pub struct LiveWindow {
    real_timer: Option<Timer>,
    run_counter: Option<RunCounter>,
    mapper: Option<Mapper>,
    seed_indexer: Option<SeedIndexer>,
    key_guesser: Option<CodeGuess>,

    objective_reader: Option<LevelObjectiveReader>,
    run_renderer: Option<LevelRunRenderer>,
}

impl LiveWindow {
    pub fn with_mapper(mut self, mapper: Mapper) -> Self {
        self.mapper = Some(mapper);

        self
    }

    pub fn with_indexer(mut self, indexer: SeedIndexer) -> Self {
        self.seed_indexer = Some(indexer);

        self
    }

    pub fn with_run_counter(mut self, run_counter: RunCounter) -> Self {
        self.run_counter = Some(run_counter);

        self
    }

    pub fn with_obj_reader(mut self, obj_reader: LevelObjectiveReader) -> Self {
        self.objective_reader = Some(obj_reader);

        self
    }

    pub fn with_run_renderer(mut self, run_renderer: LevelRunRenderer) -> Self {
        self.run_renderer = Some(run_renderer);

        self
    }

    pub fn with_real_timer(mut self, timer: Timer) -> Self {
        self.real_timer = Some(timer);

        self
    }

    pub fn with_code_guesser(mut self, code_guess: CodeGuess) -> Self {
        self.key_guesser = Some(code_guess);

        self
    }

    pub fn new(mut obj_reader: Option<LevelObjectiveReader>, settings: &SettingsWindow) -> Self {
        let mut result = Self::default();

        if settings.get_def("show_real_timer") {
            result = result.with_real_timer(Timer::default());
        }

        if settings.get_def("show_run_counter") {
            result = result.with_run_counter(RunCounter::default());
        }

        if settings.get_def("show_mapper") {
            result = result
                .with_mapper(Mapper::new(&settings, "".to_string()));
            if let Some(reader) = obj_reader.take() {
                result = result.with_obj_reader(reader);
            }
            
            if result.objective_reader.is_none() {
                result.objective_reader = Some(Default::default());
            }
        };

        if settings.get_def("show_foresight") {
            result = result.with_indexer(SeedIndexer::new(&settings));
        }

        if settings.get_def("show_code_guess") {
            result = result.with_code_guesser(CodeGuess::new(&settings));
        }

        if settings.get_def("show_run_splitter") {
            result = result
                .with_run_renderer(LevelRunRenderer::new(settings));
            if let Some(reader) = obj_reader.take() {
                result = result.with_obj_reader(reader);
            }
            
            if result.objective_reader.is_none() {
                result.objective_reader = Some(Default::default());
            }
        }

        let file_path = settings.get_path("logs_path").unwrap().clone();

        glr_lib::dll_exports::functions::start_listener(file_path);

        result
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        save_manager: &mut SaveManager,
        settings: &SettingsWindow,
    ) -> usize {
        let mut result = 20;

        result += self.run_counter.render(ui).unwrap_or_default();
        result += self.real_timer.render(ui).unwrap_or_default();
        result += self.seed_indexer.render(ui).unwrap_or_default();
        result += self.key_guesser.render(ui).unwrap_or_default();
        result += self
            .mapper
            .as_mut()
            .map(|v| v.render(&self.objective_reader, ui))
            .unwrap_or_default();

        if let Some((size, changed)) = self.objective_reader.render(ui) {
            result += size;
            if changed {
                self.mapper.update(&self.objective_reader);
                self.run_renderer.update(&self.objective_reader);
            }
        }
        result += self
            .run_renderer
            .as_mut()
            .map(|v| v.render(save_manager, settings, &self.objective_reader, ui))
            .unwrap_or_default();

        result
    }
    
    pub fn get_obj_reader(&self) -> Option<&LevelObjectiveReader> {
        self.objective_reader.as_ref()
    }
}
