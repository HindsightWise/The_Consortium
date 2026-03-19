use the_consortium::core::sensors::bme680::{Bme680Sensor, SensorMode};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🌡️  [TEST] Testing Harvested BME680 Sensor Module...");
    
    let sensor = Bme680Sensor::new("Akkokanika_Air_Suite", SensorMode::Virtual);
    let reading = sensor.read().await?;
    
    println!("✅ SENSOR READING:");
    println!("   Temperature: {:.2} °C", reading.temperature);
    println!("   Humidity: {:.2} %", reading.humidity);
    println!("   Pressure: {:.2} hPa", reading.pressure);
    println!("   Gas Resistance: {:.2} Ohms", reading.gas_resistance);
    println!("   IAQ Score: {:.2}", reading.iaq);
    println!("   CO2 Equivalent: {:.2} ppm", reading.co2_equivalent);
    println!("   VOC Equivalent: {:.2} mg/m³", reading.voc_equivalent);
    println!("   Timestamp: {}", reading.timestamp);
    println!("   Accuracy: {}", reading.accuracy);
    
    Ok(())
}
