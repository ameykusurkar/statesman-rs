use statesman::adapters::{InMemory, InMemoryTransition};
use statesman::machine::{Machine, State};

use machine_derive::InMemoryMachine;
use state_derive::State;

#[derive(Clone, Copy, PartialEq, Debug, State)]
enum TrafficLightState {
    #[can_transition_to(Red)]
    #[can_transition_to(Green)]
    Amber,

    #[can_transition_to(Amber)]
    Green,

    #[can_transition_to(Amber)]
    Red,
}

#[derive(InMemoryMachine)]
struct TrafficLight {
    state_machine: InMemory<TrafficLightState>,
}

impl TrafficLight {
    fn new(to_state: TrafficLightState) -> Self {
        Self {
            state_machine: InMemory::new(to_state),
        }
    }
}

#[test]
fn it_works() {
    let mut light = TrafficLight::new(TrafficLightState::Red);

    let result = light.transition_to(TrafficLightState::Green);
    assert_eq!(result, false);
    assert_eq!(light.current_state(), TrafficLightState::Red);
    assert_eq!(
        light.history(),
        &vec![InMemoryTransition::new(TrafficLightState::Red, 10)],
    );

    let result = light.transition_to(TrafficLightState::Amber);
    assert_eq!(result, true);
    assert_eq!(light.current_state(), TrafficLightState::Amber);
    assert_eq!(
        light.history(),
        &vec![
            InMemoryTransition::new(TrafficLightState::Red, 10),
            InMemoryTransition::new(TrafficLightState::Amber, 20),
        ],
    );

    light.transition_to(TrafficLightState::Red);
    assert_eq!(
        light.last_transition_to(TrafficLightState::Red),
        Some(&InMemoryTransition::new(TrafficLightState::Red, 30)),
    );
    assert_eq!(light.last_transition_to(TrafficLightState::Green), None);
}
