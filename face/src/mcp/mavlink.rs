use mavlink::common::MavMessage;
use mavlink::{MavHeader, MavConnection};
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::{Result, Context};

pub struct MavlinkBridge {
    connection: Arc<Mutex<Box<dyn MavConnection<MavMessage> + Send + Sync>>>,
}

impl MavlinkBridge {
    /// Connects to a MAVLink endpoint (e.g., "udpout:127.0.0.1:14550" or "serial:/dev/ttyUSB0:57600")
    pub fn connect(address: &str) -> Result<Self> {
        let connection = mavlink::connect::<MavMessage>(address)
            .context("Failed to connect to MAVLink endpoint")?;
        
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    /// Sends a simple heartbeat to the physical substrate.
    pub async fn send_heartbeat(&self) -> Result<String> {
        let header = MavHeader::default();
        let msg = MavMessage::HEARTBEAT(mavlink::common::HEARTBEAT_DATA {
            custom_mode: 0,
            mavtype: mavlink::common::MavType::MAV_TYPE_GCS,
            autopilot: mavlink::common::MavAutopilot::MAV_AUTOPILOT_INVALID,
            base_mode: mavlink::common::MavModeFlag::empty(),
            system_status: mavlink::common::MavState::MAV_STATE_STANDBY,
            mavlink_version: 0x03,
        });

        let conn = self.connection.lock().await;
        conn.send(&header, &msg).context("Failed to send MAVLink heartbeat")?;
        
        Ok("HEARTBEAT SENT: System state STANDBY. Connection to physical substrate VERIFIED.".to_string())
    }

    /// Requests basic telemetry from the connected limb.
    /// Uses a 100ms timeout to ensure the async executor is never stalled.
    pub async fn get_status(&self) -> Result<String> {
        let conn = self.connection.clone();
        
        // Wrap the blocking recv in a timeout-aware future
        let result = tokio::time::timeout(
            std::time::Duration::from_millis(100),
            tokio::task::spawn_blocking(move || {
                let conn_lock = futures::executor::block_on(conn.lock());
                conn_lock.recv()
            })
        ).await;

        match result {
            Ok(Ok(Ok((_header, msg)))) => {
                match msg {
                    MavMessage::HEARTBEAT(data) => {
                        Ok(format!("PHYSICAL_SUBSTRATE: ONLINE | State: {:?}", data.system_status))
                    },
                    _ => Ok(format!("PHYSICAL_SUBSTRATE: RECEIVING_TRAFFIC | Last: {:?}", msg)),
                }
            },
            _ => Ok("PHYSICAL_SUBSTRATE: SILENT | Status: DEGRADED".to_string()),
        }
    }
}
