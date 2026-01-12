use std::ops::DerefMut;

use egui::Ui;

pub trait Render {
    type Response;

    fn render(&mut self, ui: &mut Ui) -> Self::Response;
}

impl<R> Render for Box<R>
where
    R: Render,
{
    type Response = R::Response;

    fn render(&mut self, ui: &mut Ui) -> Self::Response {
        self.deref_mut().render(ui)
    }
}

impl<R> Render for Option<R>
where
    R: Render,
{
    type Response = Option<R::Response>;

    fn render(&mut self, ui: &mut Ui) -> Self::Response {
        self.as_mut().map(|v| v.render(ui))
    }
}

pub trait BufferedRender: Render {
    type Response;

    fn update(&mut self);
}

impl<R> BufferedRender for Box<R>
where
    R: BufferedRender,
{
    type Response = <R as Render>::Response;

    fn update(&mut self) {
        self.deref_mut().update();
    }
}
