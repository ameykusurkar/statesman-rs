use crate::machine::{State, Machine};

pub struct InMemory<S> {
    history: Vec<S>,
}

impl<S> InMemory<S> {
    pub fn new(state: S) -> Self {
        Self {
            history: vec![state],
        }
    }
}

impl<S: State + Copy> Machine<S> for InMemory<S> {
    fn history(&self) -> &Vec<S> {
        &self.history
    }

    fn create_transition(&mut self, to_state: S) {
        self.history.push(to_state);
    }
}
