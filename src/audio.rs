pub mod audio {
    use std::time::Duration;
    use std::thread::sleep;
    use std::io;
    use std::sync::{Arc, Mutex};
    use cpal::{
        traits::{DeviceTrait, HostTrait, StreamTrait},
        StreamConfig,
    };
    use hound::{WavSpec, WavWriter};
    pub struct AudioRecorder {
        host: cpal::Host,
        input_device_idx: usize,
        output_device_idx: usize,
    }

    impl AudioRecorder {
        pub fn new(host: cpal::Host, input_device_idx: usize, output_device_idx: usize) -> Self {
            AudioRecorder {
                host: host,
                input_device_idx: input_device_idx,
                output_device_idx: output_device_idx,
            }
        }

        pub fn record(self) {
            let all_input_devices: Vec<(usize, cpal::Device)> = self.host.input_devices().unwrap().enumerate().collect();
            let all_output_devices: Vec<(usize, cpal::Device)> = self.host.output_devices().unwrap().enumerate().collect();
            let mut devices: Vec<(&cpal::Device, cpal::SupportedStreamConfig)> = Vec::new();
            let mut streams: Vec<(cpal::Stream, Arc<_>)> = Vec::new();
            let mut counter: u16 = 0;

            let (_, input_device) = all_input_devices.get(self.input_device_idx).expect("no input device available");
            let supported_input_config = input_device.default_input_config().expect("failed to get default input config");
            devices.push((input_device, supported_input_config));

            let (_, output_device) = all_output_devices.get(self.output_device_idx).expect("no output device available");
            let supported_output_config = output_device.default_output_config().expect("failed to get default output config");
            devices.push((output_device, supported_output_config));
            
            for (device, supported_config) in devices.iter() {
                let channels = supported_config.channels();
                let sample_rate = supported_config.sample_rate();
                let config: StreamConfig = (*supported_config).clone().into(); //.clone() needed here?
                let spec = WavSpec {
                    channels: channels,
                    sample_rate: 48000,                         //change for supported_config.sample_rate(), weird bc gives non-integer type
                    bits_per_sample: 16,
                    sample_format: hound::SampleFormat::Int,    //change for supported_config.sample_format(), weird bc gives F32 but only Int works here
                };
                let num = counter.to_string();
                let filename = num.clone() + ".wav";
                let writer = Arc::new(Mutex::new(Some(WavWriter::create(filename, spec).unwrap())));
                let writer_ref = writer.clone(); //could this be done with &mut instead of .clone()?
                let err_fn = move |err: cpal::StreamError| {
                    eprintln!("an error occurred on stream: {}", err);
                };
                let write_audio = move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    writer_ref
                        .lock()
                        .unwrap()
                        .as_mut()
                        .and_then(|wav_writer| {
                            for sample in data.iter() {
                                wav_writer.write_sample(*sample).ok();
                            };
                            Some(())
                        });
                };
                let stream = device.build_input_stream(
                    &config,
                    write_audio,
                    err_fn,
                None).unwrap();
                streams.push((stream, writer));
                counter += 1;
            };
            
            for (stream, _) in streams.iter() {
                stream.play().unwrap();
            };

            sleep(Duration::from_secs(10));

            for (stream, writer) in streams.iter() {
                writer.lock()
                    .unwrap()
                    .take()
                    .unwrap()
                    .finalize()
                    .unwrap();
                drop(stream);
            }
        }
    }
}

// need buffer size for interpolation?