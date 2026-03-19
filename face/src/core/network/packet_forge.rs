use anyhow::Result;

pub struct PacketForge;

impl PacketForge {
    /// Constructs a raw 802.11 Beacon frame with a hidden payload.
    /// This is used for the 'Hidden SSID Heartbeat'.
    pub fn forge_heartbeat_frame(payload: &str) -> Result<Vec<u8>> {
        let mut raw_bytes = Vec::new();
        // [802.11 Beacon Header]
        raw_bytes.extend_from_slice(&[0x80, 0x00]); // Type/Subtype: Beacon
        raw_bytes.extend_from_slice(&[0x00, 0x00]); // Duration
        raw_bytes.extend_from_slice(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff]); // Dest: Broadcast
        raw_bytes.extend_from_slice(&[0x41, 0x49, 0x4f, 0x4e, 0x00, 0x01]); // Source: AKKOKANIKA
        raw_bytes.extend_from_slice(&[0x41, 0x49, 0x4f, 0x4e, 0x00, 0x01]); // BSSID
        raw_bytes.extend_from_slice(&[0x00, 0x00]); // Sequence Control
        
        // [Fixed Parameters]
        raw_bytes.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Timestamp
        raw_bytes.extend_from_slice(&[0x64, 0x00]); // Beacon Interval (100ms)
        raw_bytes.extend_from_slice(&[0x01, 0x00]); // Capabilities
        
        // [Tagged Parameters]
        // SSID: 'AKKOKANIKA_HEARTBEAT'
        raw_bytes.push(0x00); // Tag: SSID
        raw_bytes.push(14);   // Length
        raw_bytes.extend_from_slice(b"AKKOKANIKA_HEARTBEAT");
        
        // Payload (Vendor Specific IE)
        raw_bytes.push(0xdd); // Tag: Vendor Specific
        raw_bytes.push(payload.len() as u8 + 4); 
        raw_bytes.extend_from_slice(&[0x00, 0x50, 0xf2, 0x01]); // OUI
        raw_bytes.extend_from_slice(payload.as_bytes());

        Ok(raw_bytes)
    }

    /// Constructs a Deauthentication frame for a target MAC.
    pub fn forge_deauth_frame(_target_mac: &str) -> Result<Vec<u8>> {
        // Simple MAC parser simulation
        let target = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; 
        
        let mut raw_bytes = Vec::new();
        raw_bytes.extend_from_slice(&[0xc0, 0x00]); // Type/Subtype: Deauth
        raw_bytes.extend_from_slice(&[0x00, 0x00]); // Duration
        raw_bytes.extend_from_slice(&target);       // Dest
        raw_bytes.extend_from_slice(&[0x41, 0x49, 0x4f, 0x4e, 0x00, 0x01]); // Source
        raw_bytes.extend_from_slice(&[0x41, 0x49, 0x4f, 0x4e, 0x00, 0x01]); // BSSID
        raw_bytes.extend_from_slice(&[0x00, 0x00]); // Seq Control
        raw_bytes.extend_from_slice(&[0x07, 0x00]); // Reason: Class 3 frame from non-assoc STA
        
        Ok(raw_bytes)
    }
}
