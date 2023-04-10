use std::{future::Future, time::Duration};

use dioxus::prelude::{use_effect, ScopeState};
use tokio::time::{interval, Interval};

pub fn use_interval<Handler>(
    cx: &ScopeState,
    duration: Duration,
    action: impl FnOnce(Interval) -> Handler,
) where
    Handler: Future<Output = ()> + 'static,
{
    use_effect(cx, (), move |_| {
        let interval = interval(duration);
        let action = action(interval);
        async move {
            action.await;
        }
    });
}
