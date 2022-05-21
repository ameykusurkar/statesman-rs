use state_derive::State;
use statesman::machine::State;

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
        TrafficLight::Red.can_transition_to(TrafficLight::Red),
        false,
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
    assert_eq!(
        TrafficLight::Green.can_transition_to(TrafficLight::Green),
        false,
    );
}
