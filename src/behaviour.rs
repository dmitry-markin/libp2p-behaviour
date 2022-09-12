use core::pin::Pin;
use futures::{
    io::{AsyncRead, AsyncWrite},
    Future,
};
use libp2p::{
    core::{
        connection::ConnectionId, upgrade, InboundUpgrade, Multiaddr, OutboundUpgrade, PeerId,
        UpgradeInfo,
    },
    swarm::{NetworkBehaviour, NetworkBehaviourAction, OneShotHandler, PollParameters},
};
use std::{
    io,
    task::{Context, Poll},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct CustomConfig {}

impl UpgradeInfo for CustomConfig {
    type Info = &'static [u8];
    type InfoIter = std::iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        std::iter::once(b"protocol")
    }
}

impl<TSocket> InboundUpgrade<TSocket> for CustomConfig
where
    TSocket: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = CustomMessage;
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_inbound(self, mut _socket: TSocket, _info: Self::Info) -> Self::Future {
        Box::pin(async move { Ok(CustomMessage::default()) })
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CustomMessage {}

impl UpgradeInfo for CustomMessage {
    type Info = &'static [u8];
    type InfoIter = std::iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        std::iter::once(b"protocol")
    }
}

impl<TSocket> OutboundUpgrade<TSocket> for CustomMessage
where
    TSocket: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = ();
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_outbound(self, mut socket: TSocket, _info: Self::Info) -> Self::Future {
        Box::pin(async move { upgrade::write_length_prefixed(&mut socket, b"data").await })
    }
}

#[derive(Debug, Default)]
pub struct HandlerEvent {}

impl From<CustomMessage> for HandlerEvent {
    fn from(_m: CustomMessage) -> Self {
        Self::default()
    }
}

impl From<()> for HandlerEvent {
    fn from(_m: ()) -> Self {
        Self::default()
    }
}

pub struct CustomBehaviour {
    _data: i32,
}

impl NetworkBehaviour for CustomBehaviour {
    type ConnectionHandler = OneShotHandler<CustomConfig, CustomMessage, HandlerEvent>;
    type OutEvent = void::Void;

    fn new_handler(&mut self) -> Self::ConnectionHandler {
        Default::default()
    }

    fn addresses_of_peer(&mut self, _peer: &PeerId) -> Vec<Multiaddr> {
        Vec::new()
    }

    fn inject_event(&mut self, _peer: PeerId, _connection: ConnectionId, _message: HandlerEvent) {}

    fn poll(
        &mut self,
        _ctx: &mut Context,
        _: &mut impl PollParameters,
    ) -> Poll<NetworkBehaviourAction<Self::OutEvent, Self::ConnectionHandler>> {
        Poll::Pending
    }
}
