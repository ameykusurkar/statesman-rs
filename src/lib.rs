#[derive(Clone, Copy, PartialEq, Debug)]
enum Light {
    Amber,
    Green,
    Red,
}

struct StateMachine {
    state: Light,
}

impl StateMachine {
    fn new(state: Light) -> Self {
        StateMachine { state }
    }

    fn current_state(&self) -> Light {
        self.state
    }

    fn can_transition_to(&self, to_state: Light) -> bool {
        match (self.state, to_state) {
            (Light::Amber, Light::Red | Light::Green) => true,
            (Light::Green, Light::Amber) => true,
            (Light::Red, Light::Amber) => true,
            (_, _) => false,
        }
    }

    fn transition_to(&mut self, to_state: Light) -> bool {
        if !self.can_transition_to(to_state) {
            return false;
        }

        self.state = to_state;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut machine = StateMachine::new(Light::Red);

        let result = machine.transition_to(Light::Green);
        assert_eq!(result, false);
        assert_eq!(machine.current_state(), Light::Red);

        let result = machine.transition_to(Light::Amber);
        assert_eq!(result, true);
        assert_eq!(machine.current_state(), Light::Amber);
    }
}
