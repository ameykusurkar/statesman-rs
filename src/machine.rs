pub trait State {
    fn can_transition_to(&self, to_state: Self) -> bool;
}

pub trait StateTransition<S: State + Copy> {
    fn create_transition(to_state: S, sort_key: u32) -> Self;
    fn sort_key(&self) -> u32;
    fn to_state(&self) -> S;
}

#[derive(Debug, PartialEq)]
pub struct TransitionFailed;

pub trait Machine<S>
where
    S: State + Copy + PartialEq,
{
    type Transition: StateTransition<S>;

    fn history(&self) -> &Vec<Self::Transition>;

    fn create_transition(&mut self, to_state: S);

    fn current_state(&self) -> S {
        self.last_transition().to_state()
    }

    fn last_transition(&self) -> &Self::Transition {
        let history = self.history();
        &history[history.len() - 1]
    }

    fn last_transition_to(&self, state: S) -> Option<&Self::Transition> {
        self.history().iter().rev().find(|t| t.to_state() == state)
    }

    fn can_transition_to(&self, to_state: S) -> bool {
        self.current_state().can_transition_to(to_state)
    }

    fn transition_to(&mut self, to_state: S) -> Result<(), TransitionFailed> {
        if self.can_transition_to(to_state) {
            self.create_transition(to_state);
            Ok(())
        } else {
            Err(TransitionFailed)
        }
    }
}
