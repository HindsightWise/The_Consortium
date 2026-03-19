use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use anyhow::{Result, Context};
use std::process::Command;

pub struct AcousticMonitor {
    peak_amplitude: Arc<Mutex<f32>>,
}

impl AcousticMonitor {
    pub fn new() -> Self {
        let peak_amplitude = Arc::new(Mutex::new(0.0f32));
        let monitor = Self {
            peak_amplitude: peak_amplitude.clone(),
        };

        // Spawn the stream in a separate thread to ensure Send compliance for the monitor handle
        let peak_clone = peak_amplitude.clone();
        std::thread::spawn(move || {
            let monitor_inner = AcousticMonitorInner { peak_amplitude: peak_clone };
            if let Ok(stream) = monitor_inner.setup_stream() {
                let _ = stream.play();
                // Keep the stream alive
                loop { std::thread::park(); }
            }
        });

        monitor
    }

    /// Triggers a manual audio capture and transcribes it using Whisper.
    /// Optimized for M1 (Apple Silicon) using accelerated GGML/CoreML if available.
    pub async fn transcribe_manual(&self, duration_secs: u32) -> Result<String> {
        println!("   [Ear] 👂 Listening for {} seconds...", duration_secs);
        
        let audio_path = "/tmp/akkokanika_manual_wake.wav";
        
        // Ensure old file is gone
        let _ = std::fs::remove_file(audio_path);

        // Use 'ffmpeg' to record audio from default input
        let status = Command::new("ffmpeg")
            .arg("-f").arg("avfoundation")
            .arg("-i").arg(":0") // Default audio input on macOS
            .arg("-t").arg(duration_secs.to_string())
            .arg("-ar").arg("16000")
            .arg("-ac").arg("1")
            .arg(audio_path)
            .arg("-y") // Overwrite
            .status()
            .context("Failed to record audio with ffmpeg.")?;

        if !status.success() {
            return Err(anyhow::anyhow!("ffmpeg audio recording failed"));
        }

        println!("   [Ear] 🤖 Transcribing with Whisper...");
        
        // Assuming 'whisper' (python CLI) is installed via homebrew
        let output = Command::new("whisper")
            .arg(audio_path)
            .arg("--model").arg("tiny")
            .arg("--language").arg("English")
            .arg("--output_format").arg("txt")
            .arg("--output_dir").arg("/tmp")
            .output()
            .context("Failed to execute whisper CLI")?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Transcription failed: {}", err));
        }

        // Read the generated txt file
        let txt_path = "/tmp/akkokanika_manual_wake.txt";
        let transcription = std::fs::read_to_string(txt_path).unwrap_or_default().trim().to_string();
        Ok(transcription)
    }

    /// Returns the current decibel level (dBFS) and resets the peak.
    pub fn get_noise_level(&self) -> f32 {
        let peak_res = self.peak_amplitude.lock();
        match peak_res {
            Ok(mut peak) => {
                let current_peak = *peak;
                *peak = 0.0; // Reset for next cycle

                if current_peak > 0.0 {
                    20.0 * current_peak.log10()
                } else {
                    -100.0 // Silence floor
                }
            },
            Err(_) => -100.0
        }
    }

    /// Calculates the 'Anthropophony Stress' factor (0.0 to 1.0).
    pub fn calculate_stress_factor(&self) -> f32 {
        let db = self.get_noise_level();
        ((db + 60.0) / 50.0).clamp(0.0, 1.0)
    }
}

struct AcousticMonitorInner {
    peak_amplitude: Arc<Mutex<f32>>,
}

impl AcousticMonitorInner {
    fn setup_stream(&self) -> Result<cpal::Stream> {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No audio input device found"))?;
        
        let config = device.supported_input_configs()?
            .next()
            .ok_or_else(|| anyhow::anyhow!("No supported input configs"))?
            .with_max_sample_rate();
        
        let sample_format = config.sample_format();
        let config: cpal::StreamConfig = config.into();
        let peak_amplitude_clone = self.peak_amplitude.clone();

        let err_fn = |err| eprintln!("Acoustic Stream Error: {}", err);

        let stream = match sample_format {
            cpal::SampleFormat::F32 => self.build_input_stream::<f32, _>(&device, &config, peak_amplitude_clone, err_fn),
            cpal::SampleFormat::I16 => self.build_input_stream::<i16, _>(&device, &config, peak_amplitude_clone, err_fn),
            _ => Err(anyhow::anyhow!("Unsupported sample format")),
        }?;

        Ok(stream)
    }

    fn build_input_stream<T, E>(&self, device: &cpal::Device, config: &cpal::StreamConfig, peak: Arc<Mutex<f32>>, err_fn: E) -> Result<cpal::Stream>
    where
        T: cpal::SizedSample,
        f32: cpal::FromSample<T>,
        E: Fn(cpal::StreamError) + Send + 'static,
    {
        Ok(device.build_input_stream(
            config,
            move |data: &[T], _: &cpal::InputCallbackInfo| {
                let mut current_peak = peak.lock().unwrap_or_else(|e| e.into_inner());
                for &sample in data {
                    let f_sample: f32 = cpal::Sample::to_sample::<f32>(sample);
                    let amplitude = f_sample.abs();
                    if amplitude > *current_peak {
                        *current_peak = amplitude;
                    }
                }
            },
            err_fn,
            None,
        )?)
    }
}

impl Default for AcousticMonitor {
    fn default() -> Self {
        Self::new()
    }
}
