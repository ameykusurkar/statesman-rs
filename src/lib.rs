trait StateMachineDefinition {
    fn can_transition_to(&self, to_state: Self) -> bool;
}

trait StateMachine<T: StateMachineDefinition + Copy> {
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

struct SimpleStateMachine<T> {
    state: T,
}

impl<T> SimpleStateMachine<T> {
    fn new(state: T) -> Self {
        Self { state }
    }
}

impl<T: StateMachineDefinition + Copy> StateMachine<T> for SimpleStateMachine<T> {
    fn current_state(&self) -> T {
        self.state
    }

    fn create_transition(&mut self, to_state: T) {
        self.state = to_state;
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum TrafficLight {
    Amber,
    Green,
    Red,
}

impl StateMachineDefinition for TrafficLight {
    fn can_transition_to(&self, to_state: TrafficLight) -> bool {
        match (self, to_state) {
            (TrafficLight::Amber, TrafficLight::Red | TrafficLight::Green) => true,
            (TrafficLight::Green, TrafficLight::Amber) => true,
            (TrafficLight::Red, TrafficLight::Amber) => true,
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut machine = SimpleStateMachine::new(TrafficLight::Red);

        let result = machine.transition_to(TrafficLight::Green);
        assert_eq!(result, false);
        assert_eq!(machine.current_state(), TrafficLight::Red);

        let result = machine.transition_to(TrafficLight::Amber);
        assert_eq!(result, true);
        assert_eq!(machine.current_state(), TrafficLight::Amber);
    }
}
