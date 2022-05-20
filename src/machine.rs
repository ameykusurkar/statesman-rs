pub trait State {
    fn can_transition_to(&self, to_state: Self) -> bool;
}

pub trait Machine<S: State + Copy + PartialEq> {
    fn history(&self) -> &Vec<Transition<S>>;

    fn create_transition(&mut self, to_state: S);

    fn current_state(&self) -> S {
        self.last_transition().to_state
    }

    fn last_transition(&self) -> &Transition<S> {
        let history = self.history();
        &history[history.len() - 1]
    }

    fn last_transition_to(&self, state: S) -> Option<&Transition<S>> {
        self.history().iter().rev().find(|t| t.to_state == state)
    }

    fn can_transition_to(&self, to_state: S) -> bool {
        self.current_state().can_transition_to(to_state)
    }

    fn transition_to(&mut self, to_state: S) -> bool {
        if self.can_transition_to(to_state) {
            self.create_transition(to_state);
            true
        } else {
            false
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Transition<S: State> {
    to_state: S,
    sort_key: u32,
}

impl<S: State + Copy> Transition<S> {
    pub fn new(to_state: S, sort_key: u32) -> Self {
        Self { to_state, sort_key }
    }

    pub fn to_state(&self) -> S {
        self.to_state
    }

    pub fn sort_key(&self) -> u32 {
        self.sort_key
    }
}
