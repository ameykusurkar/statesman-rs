mod adapters;
mod machine;

use crate::machine::State;

#[derive(Clone, Copy, PartialEq, Debug)]
enum TrafficLight {
    Amber,
    Green,
    Red,
}

impl State for TrafficLight {
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
    use crate::adapters::InMemory;
    use crate::machine::{Machine, Transition};

    #[test]
    fn it_works() {
        let mut machine = InMemory::new(TrafficLight::Red);

        let result = machine.transition_to(TrafficLight::Green);
        assert_eq!(result, false);
        assert_eq!(machine.current_state(), TrafficLight::Red);
        assert_eq!(
            machine.history(),
            &vec![Transition::new(TrafficLight::Red, 10)],
        );

        let result = machine.transition_to(TrafficLight::Amber);
        assert_eq!(result, true);
        assert_eq!(machine.current_state(), TrafficLight::Amber);
        assert_eq!(
            machine.history(),
            &vec![
                Transition::new(TrafficLight::Red, 10),
                Transition::new(TrafficLight::Amber, 20),
            ],
        );

        machine.transition_to(TrafficLight::Red);
        assert_eq!(
            machine.last_transition_to(TrafficLight::Red),
            Some(&Transition::new(TrafficLight::Red, 30)),
        );
        assert_eq!(machine.last_transition_to(TrafficLight::Green), None);
    }
}
