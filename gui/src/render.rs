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
    type Renderer: Render;
    
    fn get_renderer(&mut self) -> &mut Self::Renderer;
    fn update(&mut self);

}

impl<BR, R, Resp> Render for BR
where 
    R: Render<Response = Resp>,
    BR: BufferedRender<Response = Resp, Renderer = R> {
    
    type Response = Resp;

    fn render(&mut self, ui: &mut Ui) -> Self::Response {
        self.get_renderer().render(ui)
    }
}
