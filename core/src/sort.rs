use crate::run::traits::Run;

pub trait Sortable<R>
where
    R: Run,
{
    fn get_vec(&mut self, objective: &String) -> Option<&mut Vec<R>>;

    fn sort_by_win(&mut self, objective: &String) {
        self.get_vec(objective)
            .map(|v| v.sort_by(|d, e| d.is_win().cmp(&e.is_win()).reverse()));
    }

    fn sort_by_objective(&mut self, objective: &String) {
        self.get_vec(objective)
            .map(|v| v.sort_by(|d, e| d.get_objective().to_string().cmp(&e.get_objective().to_string())));
    }

    fn sort_by_time(&mut self, objective: &String) {
        self.get_vec(objective)
            .map(|v| v.sort_by(|d, e| d.get_time().cmp(&e.get_time())));
    }

    fn sort_by_stamps(&mut self, objective: &String) {
        self.get_vec(objective)
            .map(|v| v.sort_by(|d, e| d.len().cmp(&e.len()).reverse()));
    }
}

pub enum SortMessage {
    SortByWin(String),
    SortByObjective(String),
    SortByTime(String),
    SortByStamps(String),
}
