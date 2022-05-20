use state_derive::State;
use statesman::adapters::InMemory;
use statesman::machine::{Machine, State, Transition};

#[derive(Clone, Copy, PartialEq, Debug, State)]
enum Order {
    #[can_transition_to(CheckingOut)]
    #[can_transition_to(Cancelled)]
    Pending,

    #[can_transition_to(Purchased)]
    #[can_transition_to(Cancelled)]
    CheckingOut,

    #[can_transition_to(Shipped)]
    #[can_transition_to(Failed)]
    Purchased,

    #[can_transition_to(Refunded)]
    Shipped,

    Cancelled,
    Failed,
    Refunded,
}

#[test]
fn it_works() {
    let mut machine = InMemory::new(Order::Pending);

    machine.transition_to(Order::CheckingOut);

    assert_eq!(machine.current_state(), Order::CheckingOut);

    let result = machine.transition_to(Order::Failed);

    assert_eq!(result, false);
    assert_eq!(machine.current_state(), Order::CheckingOut);

    machine.transition_to(Order::Purchased);
    machine.transition_to(Order::Failed);

    assert_eq!(machine.current_state(), Order::Failed);
    assert_eq!(
        machine.history(),
        &vec![
            Transition::new(Order::Pending, 10),
            Transition::new(Order::CheckingOut, 20),
            Transition::new(Order::Purchased, 30),
            Transition::new(Order::Failed, 40),
        ],
    );
    assert_eq!(
        machine.last_transition_to(Order::CheckingOut),
        Some(&Transition::new(Order::CheckingOut, 20)),
    );
    assert_eq!(machine.last_transition_to(Order::Shipped), None);
    assert_eq!(machine.last_transition_to(Order::Cancelled), None);
    assert_eq!(machine.last_transition_to(Order::Refunded), None);
}
