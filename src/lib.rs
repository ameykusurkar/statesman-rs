pub mod adapters;
pub mod machine;

pub mod macros {
    pub use machine_derive::InMemoryMachine;
    pub use state_derive::State;
}
