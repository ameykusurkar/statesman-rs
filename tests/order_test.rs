use statesman::adapters::InMemory;
use statesman::machine::{Machine, State, Transition};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Order {
    Pending,
    CheckingOut,
    Purchased,
    Shipped,
    Cancelled,
    Failed,
    Refunded,
}

impl State for Order {
    fn can_transition_to(&self, to_state: Self) -> bool {
        match (self, to_state) {
            (Order::Pending, Order::CheckingOut | Order::Cancelled) => true,
            (Order::CheckingOut, Order::Purchased | Order::Cancelled) => true,
            (Order::Purchased, Order::Shipped | Order::Failed) => true,
            (Order::Shipped, Order::Refunded) => true,
            (_, _) => false,
        }
    }
}

#[test]
fn it_simulates_orders() {
    let mut machine = InMemory::new(Order::Pending);

    assert_eq!(machine.current_state(), Order::Pending);
    assert_eq!(machine.can_transition_to(Order::CheckingOut), true);
    assert_eq!(machine.can_transition_to(Order::Cancelled), true);
    assert_eq!(machine.can_transition_to(Order::Failed), false);

    machine.transition_to(Order::Cancelled);

    let result = machine.transition_to(Order::Cancelled);

    assert_eq!(result, false);
    assert_eq!(
        machine.last_transition(),
        &Transition::new(Order::Cancelled, 20)
    );
    assert_eq!(
        machine.last_transition_to(Order::Pending),
        Some(&Transition::new(Order::Pending, 10))
    );
    assert_eq!(machine.last_transition_to(Order::Shipped), None);
    assert_eq!(machine.last_transition_to(Order::Refunded), None);
    assert_eq!(machine.last_transition_to(Order::Purchased), None);
}
