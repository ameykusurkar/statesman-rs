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
```

Then use the state machine:

```rust
use statesman::adapters::InMemory;
use statesman::machine::{Machine, Transition};

fn main() {
  let mut machine = InMemory::new(Order::Pending);

  machine.transition_to(Order::CheckingOut);

  let result = machine.transition_to(Order::Failed);

  assert_eq!(result, false);
  assert_eq!(machine.current_state(), Order::CheckingOut);

  machine.transition_to(Order::Purchased);

  assert_eq!(
      machine.last_transition(),
      Transition::new(Order::Purchased, 30),
  )
}
