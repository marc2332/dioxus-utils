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
