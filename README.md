# statesman-rs

_A state machine library that is inspired by and is intended to match the functionality of [Ruby's statesman gem](https://github.com/gocardless/statesman)._

This is really just a proof-of-concept at the moment, and not intended for production use.

## Usage

Usage for the example state machine defined in the [original README](https://github.com/gocardless/statesman#usage) would look like this.

First define the states and transitions:

```rust
use state_derive::State;
use statesman::machine::State;

#[derive(Clone, Copy, State)]
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
```

Then use the state machine:

```rust
use machine_derive::InMemoryMachine;
use statesman::adapters::InMemory;
use statesman::machine::{Machine, Transition};

#[derive(InMemoryMachine)]
struct Order {
    state_machine: InMemory<OrderState>,
}

fn main() {
  let mut order = Order { state_machine: InMemory::new(OrderState::Pending) };

  order.transition_to(OrderState::CheckingOut).unwrap();

  let result = order.transition_to(OrderState::Failed);

  assert_eq!(result.is_err(), true);
  assert_eq!(order.current_state(), OrderState::CheckingOut);

  order.transition_to(OrderState::Purchased).unwrap();

  assert_eq!(
      order.last_transition(),
      Transition::new(OrderState::Purchased, 30),
  )
}
