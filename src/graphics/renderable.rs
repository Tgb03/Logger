use egui::Ui;


pub trait Renderable {

    fn render(&self, ui: &mut Ui);

}

pub trait BufferedRenderable {
    type RenderObj: Renderable;
    type Data;

    fn update(&mut self, data: &Self::Data);
    fn get_obj(&self) -> &Self::RenderObj;
}

impl<T> Renderable for T
where
    T: BufferedRenderable {
    
    fn render(&self, ui: &mut Ui) {
        self.get_obj().render(ui);
    }
}

#[cfg(test)]
mod test {

    #[test]
    pub fn test_basic() {
        
    }

}
