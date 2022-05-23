use machine_derive::InMemoryMachine;
use state_derive::State;

use statesman::adapters::{InMemory, InMemoryTransition};
use statesman::machine::{Machine, State};

#[derive(Clone, Copy, PartialEq, Debug, State)]
enum OrderState {
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

#[derive(InMemoryMachine)]
struct Order {
    state_machine: InMemory<OrderState>,
}

impl Order {
    fn new(to_state: OrderState) -> Self {
        Self {
            state_machine: InMemory::new(to_state),
        }
    }
}

#[test]
fn it_works() {
    let mut order = Order::new(OrderState::Pending);

    order.transition_to(OrderState::CheckingOut);

    assert_eq!(order.current_state(), OrderState::CheckingOut);

    let result = order.transition_to(OrderState::Failed);

    assert_eq!(result, false);
    assert_eq!(order.current_state(), OrderState::CheckingOut);

    order.transition_to(OrderState::Purchased);
    order.transition_to(OrderState::Failed);

    assert_eq!(order.current_state(), OrderState::Failed);
    assert_eq!(
        order.history(),
        &vec![
            InMemoryTransition::new(OrderState::Pending, 10),
            InMemoryTransition::new(OrderState::CheckingOut, 20),
            InMemoryTransition::new(OrderState::Purchased, 30),
            InMemoryTransition::new(OrderState::Failed, 40),
        ],
    );
    assert_eq!(
        order.last_transition_to(OrderState::CheckingOut),
        Some(&InMemoryTransition::new(OrderState::CheckingOut, 20)),
    );
    assert_eq!(order.last_transition_to(OrderState::Shipped), None);
    assert_eq!(order.last_transition_to(OrderState::Cancelled), None);
    assert_eq!(order.last_transition_to(OrderState::Refunded), None);
}
