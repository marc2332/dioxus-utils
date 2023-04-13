use std::{future::Future, time::Duration};

use dioxus::prelude::{to_owned, use_effect, use_state, ScopeState, UseState};
use tokio::time::{interval, Interval};

pub struct UseInterval {
    interval_state: UseState<IntervalState>,
}

impl UseInterval {
    /// Stop the interval
    pub fn clear(&self) {
        self.interval_state.set(IntervalState::Cleared);
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum IntervalState {
    Running,
    Cleared,
}

pub fn use_interval<Handler>(
    cx: &ScopeState,
    duration: Duration,
    action: impl Fn(&Interval) -> Handler + 'static,
) -> UseInterval
where
    Handler: Future<Output = ()> + 'static,
{
    let interval_state = use_state(cx, || IntervalState::Running);

    use_effect(cx, (interval_state,), move |(interval_state,)| {
        to_owned![interval_state];
        async move {
            if *interval_state.current() == IntervalState::Cleared {
                return;
            }
            let action = Box::new(action);
            let mut interval = interval(duration);
            while *interval_state.current() == IntervalState::Running {
                interval.tick().await;
                action(&interval).await;
            }
        }
    });

    UseInterval {
        interval_state: interval_state.clone(),
    }
}
