use crate::machine::{Machine, State, StateTransition};

pub struct InMemory<S: State> {
    history: Vec<InMemoryTransition<S>>,
}

impl<S: State + Copy> InMemory<S> {
    pub fn new(state: S) -> Self {
        Self {
            history: vec![InMemoryTransition::create_transition(state, 10)],
        }
    }
}

impl<S: State + Copy + PartialEq> Machine<S> for InMemory<S> {
    type Transition = InMemoryTransition<S>;

    fn history(&self) -> &Vec<Self::Transition> {
        &self.history
    }

    fn create_transition(&mut self, to_state: S) {
        let last_sort_key = self.last_transition().sort_key();
        let transition = Self::Transition::create_transition(to_state, last_sort_key + 10);

        self.history.push(transition);
    }
}

#[derive(PartialEq, Debug)]
pub struct InMemoryTransition<S: State> {
    to_state: S,
    sort_key: u32,
}

impl<S: State + Copy> InMemoryTransition<S> {
    // Convenience function for testing
    pub fn new(to_state: S, sort_key: u32) -> Self {
        Self::create_transition(to_state, sort_key)
    }
}

impl<S: State + Copy> StateTransition<S> for InMemoryTransition<S> {
    fn create_transition(to_state: S, sort_key: u32) -> Self {
        Self { to_state, sort_key }
    }

    fn to_state(&self) -> S {
        self.to_state
    }

    fn sort_key(&self) -> u32 {
        self.sort_key
    }
}
