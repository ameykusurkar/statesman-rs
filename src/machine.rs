pub trait State {
    fn can_transition_to(&self, to_state: Self) -> bool;
}

pub trait Machine<S: State + Copy> {
    fn history(&self) -> &Vec<S>;

    fn current_state(&self) -> S {
        let history = self.history();
        history[history.len() - 1]
    }
    fn create_transition(&mut self, to_state: S);

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
