use egui::Ui;

pub trait Render {
    type Response;

    fn render(&mut self, ui: &mut Ui) -> Self::Response;
}

impl<R, Resp> Render for Option<R>
where
    R: Render<Response = Resp>,
{
    type Response = Option<Resp>;

    fn render(&mut self, ui: &mut Ui) -> Self::Response {
        self.as_mut().map(|v| v.render(ui))
    }
}

pub trait BufferedRender {
    type Response;
    type UpdateData;
    type Render: Render<Response = Self::Response>;

    fn update(&mut self, update_data: &Self::UpdateData);
    fn reset(&mut self);

    fn get_renderer(&mut self) -> &mut Self::Render;
}

impl<BR, Resp> Render for BR
where
    BR: BufferedRender<Response = Resp>,
{
    type Response = Resp;

    fn render(&mut self, ui: &mut Ui) -> Resp {
        self.get_renderer().render(ui)
    }
}
