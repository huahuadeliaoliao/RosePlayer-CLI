use log::error;
use tokio::sync::watch;
use zbus::{fdo, interface, ConnectionBuilder};

#[derive(Clone)]
pub struct TempDBus {
    stop_signal: watch::Sender<()>,
}

#[interface(name = "org.rosesong.Player")]
impl TempDBus {
    async fn test_connection(&self) -> fdo::Result<()> {
        Ok(())
    }

    async fn playlist_change(&self) -> fdo::Result<()> {
        if let Err(e) = self.stop_signal.send(()) {
            error!("TempDBus: Failed to send stop signal: {}", e);
        }
        Ok(())
    }

    async fn stop(&self) -> fdo::Result<()> {
        if let Err(e) = self.stop_signal.send(()) {
            error!("TempDBus: Failed to send stop signal: {}", e);
        }
        Ok(())
    }
}

pub async fn run_temp_dbus_server(
    stop_signal: watch::Sender<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let temp_dbus = TempDBus {
        stop_signal: stop_signal.clone(),
    };

    let _connection = ConnectionBuilder::session()?
        .name("org.rosesong.Player")?
        .serve_at("/org/rosesong/Player", temp_dbus)?
        .build()
        .await?;

    let mut stop_receiver = stop_signal.subscribe();

    // Wait for the stop signal
    tokio::select! {
        _ = stop_receiver.changed() => {
        }
    }

    Ok(())
}
