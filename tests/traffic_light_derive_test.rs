use statesman::adapters::InMemory;
use statesman::machine::{Machine, State, Transition};
use state_derive::State;

#[derive(Clone, Copy, PartialEq, Debug, State)]
enum TrafficLight {
    #[can_transition_to(Red)]
    #[can_transition_to(Green)]
    Amber,

    #[can_transition_to(Amber)]
    Green,

    #[can_transition_to(Amber)]
    Red,
}

#[test]
fn it_allows_the_correct_transitions() {
    assert_eq!(
        TrafficLight::Red.can_transition_to(TrafficLight::Green),
        false,
    );
    assert_eq!(
        TrafficLight::Red.can_transition_to(TrafficLight::Amber),
        true,
    );

    assert_eq!(
        TrafficLight::Amber.can_transition_to(TrafficLight::Amber),
        false,
    );
    assert_eq!(
        TrafficLight::Amber.can_transition_to(TrafficLight::Green),
        true,
    );
    assert_eq!(
        TrafficLight::Amber.can_transition_to(TrafficLight::Red),
        true,
    );

    assert_eq!(
        TrafficLight::Green.can_transition_to(TrafficLight::Red),
        false,
    );
    assert_eq!(
        TrafficLight::Green.can_transition_to(TrafficLight::Amber),
        true,
    );
}

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
