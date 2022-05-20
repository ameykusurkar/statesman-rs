use crate::machine::{Machine, State, Transition};

pub struct InMemory<S: State> {
    history: Vec<Transition<S>>,
}

impl<S: State + Copy> InMemory<S> {
    pub fn new(state: S) -> Self {
        Self {
            history: vec![Transition::new(state, 10)],
        }
    }
}

impl<S: State + Copy + PartialEq> Machine<S> for InMemory<S> {
    fn history(&self) -> &Vec<Transition<S>> {
        &self.history
    }

    fn create_transition(&mut self, to_state: S) {
        let last_sort_key = self.last_transition().sort_key();
        let transition = Transition::new(to_state, last_sort_key + 10);

        self.history.push(transition);
    }
}
