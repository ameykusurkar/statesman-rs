pub trait Definition {
    fn can_transition_to(&self, to_state: Self) -> bool;
}

pub trait Machine<T: Definition + Copy> {
    fn current_state(&self) -> T;
    fn create_transition(&mut self, to_state: T);

    fn can_transition_to(&self, to_state: T) -> bool {
        self.current_state().can_transition_to(to_state)
    }

    fn transition_to(&mut self, to_state: T) -> bool {
        if self.can_transition_to(to_state) {
            self.create_transition(to_state);
            true
        } else {
            false
        }
    }
}
