use iroh::endpoint::VarInt;

use crate::{spawn_and_forget, Interaction, Terminate};

impl<E, R> Terminate<E, R> for Interaction
where
    E: Into<VarInt> + Send,
    R: AsRef<[u8]> + Send,
{
    fn terminate(self: &std::sync::Arc<Self>, error_code: E, _reason: &R) {
        let interaction = self.clone();
        let error_code = error_code.into();

        spawn_and_forget(async move {
            if let Some(send_stream) = &interaction.send_stream {
                send_stream.lock().await.finish()?;
            }

            interaction.recv_stream.lock().await.stop(error_code)?;

            Ok(())
        });
    }
}
