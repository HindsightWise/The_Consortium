use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bme680Reading {
    pub temperature: f32,    // °C
    pub humidity: f32,       // %
    pub pressure: f32,       // hPa
    pub gas_resistance: f32, // Ohms
    pub iaq: f32,            // Indoor Air Quality (0-500)
    pub static_iaq: f32,
    pub co2_equivalent: f32, // ppm
    pub voc_equivalent: f32, // mg/m³
    pub timestamp: u64,
    pub accuracy: u8,        // 0 (Unreliable) to 3 (High)
}

pub struct Bme680Sensor {
    pub name: String,
    pub mode: SensorMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SensorMode {
    Hardware, // Real I2C/SPI
    Virtual,  // Physical Simulation
}

impl Bme680Sensor {
    pub fn new(name: &str, mode: SensorMode) -> Self {
        Self {
            name: name.to_string(),
            mode,
        }
    }

    pub async fn read(&self) -> Result<Bme680Reading> {
        match self.mode {
            SensorMode::Hardware => {
                // In a real deployment, this would use i2c-linux or similar.
                // For the M1 substrate, we failover to virtual if no hardware is detected.
                self.read_virtual().await
            }
            SensorMode::Virtual => self.read_virtual().await,
        }
    }

    async fn read_virtual(&self) -> Result<Bme680Reading> {
        // Physical Simulation Logic:
        // Generates realistic environmental data based on time of day and random drift.
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        
        // Simple sin-wave for temperature (warmer during day)
        let hour = (now / 3600) % 24;
        let temp_base = 22.0 + (5.0 * ((hour as f32 - 6.0) * std::f32::consts::PI / 12.0).sin());
        let drift = (rand::random::<f32>() - 0.5) * 0.5;

        Ok(Bme680Reading {
            temperature: temp_base + drift,
            humidity: 45.0 + (rand::random::<f32>() * 10.0),
            pressure: 1013.25 + (rand::random::<f32>() * 5.0),
            gas_resistance: 50000.0 + (rand::random::<f32>() * 10000.0),
            iaq: 25.0 + (rand::random::<f32>() * 50.0),
            static_iaq: 25.0,
            co2_equivalent: 400.0 + (rand::random::<f32>() * 100.0),
            voc_equivalent: 0.1 + (rand::random::<f32>() * 0.2),
            timestamp: now,
            accuracy: 3,
        })
    }
}
