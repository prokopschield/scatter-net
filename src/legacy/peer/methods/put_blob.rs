use std::{
    future::Future,
    mem::replace,
    pin::Pin,
    sync::Arc,
    task::Poll::{Pending, Ready},
};

use bytes::Bytes;
use n0_future::{FutureExt, StreamExt};
use ps_promise::PromiseRejection;

use crate::{spawn_and_forget, Interaction, Packet, Peer, PutRequest, PutResponse};

impl Peer {
    pub fn put_blob(self: Arc<Self>, data: Bytes) -> PeerPutBlob {
        PeerPutBlob {
            // Start in the BeginInteraction state
            state: PeerPutBlobState::BeginInteraction {
                data,
                // Start the interaction process immediately
                future: Box::pin(self.begin_interaction()),
            },
        }
    }
}

type BeginInteractionResult = Result<Interaction, crate::PeerBeginInteractionError>;
type SendPacketResult = Result<(), crate::InteractionSendPacketError>;

enum PeerPutBlobState {
    BeginInteraction {
        data: Bytes,
        future: Pin<Box<dyn Future<Output = BeginInteractionResult> + Send + Sync>>,
    },
    SendPacket {
        interaction: Interaction,
        future: Pin<Box<dyn Future<Output = SendPacketResult> + Send + Sync>>,
    },
    AwaitResponse {
        interaction: Interaction,
    },
    Done,
    Failed,
    Processing,
}

pub struct PeerPutBlob {
    state: PeerPutBlobState,
}

impl Future for PeerPutBlob {
    type Output = Result<PutResponse, PeerPutBlobError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();

        loop {
            match replace(&mut this.state, PeerPutBlobState::Processing) {
                // --- State 1: Begin Interaction ---
                PeerPutBlobState::BeginInteraction { data, mut future } => {
                    match future.poll(cx) {
                        Ready(Ok(interaction)) => {
                            // Success! Prepare the request packet
                            let request = PutRequest { data };
                            let packet = Packet::PutRequest(request);

                            let send_future = {
                                let interaction = interaction.clone();

                                async move { interaction.send_packet(packet).await }
                            };

                            // Transition to the next state
                            this.state = PeerPutBlobState::SendPacket {
                                interaction,
                                future: Box::pin(send_future),
                            };

                            // Continue the loop to poll the SendPacket state immediately
                            continue;
                        }
                        Ready(Err(err)) => {
                            this.state = PeerPutBlobState::Failed;

                            // Drop all state and return the Error
                            return Ready(Err(PeerPutBlobError::BeginInteraction(err)));
                        }
                        Pending => {
                            this.state = PeerPutBlobState::BeginInteraction { data, future };

                            // Interaction not ready, keep waiting...
                            return Pending;
                        }
                    }
                }

                // --- State 2: Send Packet ---
                PeerPutBlobState::SendPacket {
                    interaction,
                    mut future,
                } => {
                    match future.poll(cx) {
                        Ready(Ok(())) => {
                            // Packet sent successfully!
                            this.state = PeerPutBlobState::AwaitResponse { interaction };

                            // Continue the loop to poll the AwaitResponse state immediately
                            continue;
                        }
                        Ready(Err(err)) => {
                            this.state = PeerPutBlobState::Failed;

                            // Drop state and return the Error.
                            return Ready(Err(PeerPutBlobError::SendPacket(err)));
                        }
                        Pending => {
                            this.state = PeerPutBlobState::SendPacket {
                                interaction,
                                future,
                            };

                            // Packet not delivered, keep waiting...
                            return Pending;
                        }
                    }
                }

                // --- State 3: Await Response ---
                PeerPutBlobState::AwaitResponse { mut interaction } => {
                    // Poll the interaction stream for the next packet
                    let packet = match interaction.poll_next(cx) {
                        Pending => {
                            this.state = PeerPutBlobState::AwaitResponse { interaction };

                            return Pending;
                        }
                        Ready(None) => {
                            this.state = PeerPutBlobState::Failed;

                            return Ready(Err(PeerPutBlobError::DidNotRespond));
                        }
                        Ready(Some(Ok(packet))) => packet,
                        Ready(Some(Err(err))) => {
                            this.state = PeerPutBlobState::Failed;

                            return Ready(Err(PeerPutBlobError::ReadPacket(err)));
                        }
                    };

                    // Process the received packet
                    match packet {
                        Packet::PutResponse(response) => {
                            // Got the expected response!
                            this.state = PeerPutBlobState::Done;

                            return Ready(Ok(response));
                        }
                        packet => {
                            // Got an unexpected packet type
                            let peer = interaction.get_peer().clone();

                            // Spawn a task to process the unexpected packet anyway
                            spawn_and_forget(async move {
                                packet.process(peer).await?;
                                Ok(())
                            });

                            this.state = PeerPutBlobState::Failed;

                            return Ready(Err(PeerPutBlobError::InvalidResponse));
                        }
                    }
                }
                // --- Terminal States ---
                PeerPutBlobState::Done | PeerPutBlobState::Failed => {
                    return Ready(Err(PeerPutBlobError::MultipleAwaits))
                }
                // --- Invalid State ---
                PeerPutBlobState::Processing => {
                    return Ready(Err(PeerPutBlobError::ProcessingState))
                }
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PeerPutBlobError {
    #[error(transparent)]
    BeginInteraction(#[from] crate::PeerBeginInteractionError),
    #[error("This Promise was consumed more than once.")]
    ConsumedAlready,
    #[error("Peer did not respond to the put request.")]
    DidNotRespond,
    #[error("Peer did not provide a valid response.")]
    InvalidResponse,
    #[error("This future was awaited multiple times, which isn't supported.")]
    MultipleAwaits,
    #[error("This is an internal exception which you shouldn't encounter.")]
    ProcessingState,
    #[error("Failed to read packet: {0}")]
    ReadPacket(#[from] crate::InteractionReadPacketError),
    #[error(transparent)]
    SendPacket(#[from] crate::InteractionSendPacketError),
}

impl PromiseRejection for PeerPutBlobError {
    fn already_consumed() -> Self {
        Self::ConsumedAlready
    }
}

type Result<T = PutResponse, E = PeerPutBlobError> = std::result::Result<T, E>;
