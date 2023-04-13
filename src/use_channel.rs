use std::future::Future;

use dioxus::prelude::{to_owned, use_effect, use_state, UseState};
use freya::prelude::ScopeState;
use tokio::sync::broadcast::{
    self,
    error::{RecvError, SendError},
    Receiver, Sender,
};

pub struct UseChannel<MessageType: Clone> {
    sender: Sender<MessageType>,
    receiver: Receiver<MessageType>,
}

impl<T: Clone> PartialEq for UseChannel<T> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<MessageType: Clone> Clone for UseChannel<MessageType> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: self.sender.subscribe(),
        }
    }
}

impl<MessageType: Clone> UseChannel<MessageType> {
    pub fn send(&self, msg: impl Into<MessageType>) -> Result<(), SendError<MessageType>> {
        self.sender.send(msg.into()).map(|_| ())
    }

    pub async fn recv(&mut self) -> Result<MessageType, RecvError> {
        self.receiver.recv().await
    }
}

pub fn use_channel<MessageType: Clone + 'static>(
    cx: &ScopeState,
    size: usize,
) -> UseChannel<MessageType> {
    let sender = cx.use_hook(|| broadcast::channel::<MessageType>(size).0);

    let state = UseChannel {
        sender: sender.clone(),
        receiver: sender.subscribe(),
    };

    state
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum ChannelListenerState {
    Running,
    Stopped,
}

pub struct UseListenChannel {
    listener_state: UseState<ChannelListenerState>,
}

impl UseListenChannel {
    /// Stop the listener
    pub fn stop(&self) {
        self.listener_state.set(ChannelListenerState::Stopped);
    }
}

pub fn use_listen_channel<MessageType: Clone + 'static, Handler>(
    cx: &ScopeState,
    channel: &UseChannel<MessageType>,
    action: impl Fn(MessageType) -> Handler + 'static,
) -> UseListenChannel
where
    Handler: Future<Output = ()> + 'static,
{
    let listener_state = use_state(cx, || ChannelListenerState::Running);

    use_effect(cx, (listener_state,), move |(listener_state,)| {
        to_owned![listener_state, channel];
        async move {
            if *listener_state.current() == ChannelListenerState::Stopped {
                return;
            }
            let action = Box::new(action);
            while let Ok(msg) = channel.recv().await {
                if *listener_state.current() == ChannelListenerState::Running {
                    action(msg).await;
                }
            }
        }
    });

    UseListenChannel {
        listener_state: listener_state.clone(),
    }
}
