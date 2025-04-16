use crate::{save_manager::SaveMessage, sort::SortMessage};

pub enum Message {
    SaveManager(SaveMessage),
    Sort(SortMessage),
}

pub trait MessageAcceptor {
    fn accept_message(&mut self, message: &Message);
}
