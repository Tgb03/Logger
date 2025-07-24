use enum_dispatch::enum_dispatch;

use crate::{run::{objectives::objective_enum::ObjectiveEnum, split::Split, timed_run::RunEnum}, time::Time};

#[enum_dispatch]
pub trait Run: Split {
    fn get_splits<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Split> + 'a>;
    fn get_time_for_split(&self, split_name: &str) -> Option<Time>;

    fn is_win(&self) -> bool;
    fn len(&self) -> usize;

    fn set_win(&mut self, is_win: bool);

    fn get_objective(&self) -> &ObjectiveEnum;
    fn set_objective(&mut self, objective: ObjectiveEnum);
    fn set_objective_str(&mut self, objective: &str);

    fn get_split_by_name<'a>(&'a self, split_name: &str) -> Option<&'a dyn Split> {
        self.get_splits()
            .find(|s| s.get_name() == split_name)
    }
}

impl<S: Split> Split for Vec<S> {
    fn get_name(&self) ->  &str {
        ""
    }

    fn get_time(&self) -> Time {
        self.iter()
            .map(|v| v.get_time())
            .fold(Time::new(), |a, b| a + b)
    }
}


impl<S: Split> Run for Vec<S> {
    fn get_splits<'a>(&'a self) -> Box<dyn Iterator<Item =  &'a dyn Split> +'a> {
        Box::new(self.iter().map(|v| v as &dyn Split))
    }

    fn get_time_for_split(&self, split_name: &str) -> Option<Time> {
        self.iter()
            .find(|s| s.get_name() == split_name)
            .map(|v| v.get_time())
    }

    fn is_win(&self) -> bool {
        false
    }

    fn len(&self) -> usize {
        self.len()
    }

    
    fn get_objective(&self) ->  &ObjectiveEnum {
        panic!()
    }
    
    fn set_win(&mut self, _: bool) {}
    fn set_objective(&mut self, _: ObjectiveEnum) {}
    fn set_objective_str(&mut self, _: &str) {}
}

