use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, FromSample, SizedSample, StreamConfig,
};
use fundsp::hacker::{sine_hz, AudioUnit};
use std::{
    io::{Result, Write},
    sync::Arc,
};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

struct AudioController {
    playing: bool,
}

fn audio_test(ac: &mut Arc<AudioController>) -> Result<()> {
    if !ac.playing {
        println!("Playing: 440.0hz sine");

        let audio_sample = Box::new(sine_hz(440.0));

        run_sample(audio_sample)?;
    } else {
    }

    Ok(())
}

fn run_sample(sample: Box<dyn AudioUnit>) -> Result<()> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to get default audio device");

    let config = device
        .default_output_config()
        .expect("failed to get default output config");

    match config.sample_format() {
        cpal::SampleFormat::F64 => run_synth::<f64>(sample, device, config.into()),
        cpal::SampleFormat::F32 => run_synth::<f32>(sample, device, config.into()),
        cpal::SampleFormat::U8 => run_synth::<u8>(sample, device, config.into()),
        cpal::SampleFormat::I8 => run_synth::<i8>(sample, device, config.into()),

        _ => panic!("Sample format not implemented"),
    }

    Ok(())
}

fn run_synth<T: SizedSample + FromSample<f64>>(
    mut audio_sample: Box<dyn AudioUnit>,
    device: Device,
    config: StreamConfig,
) {
    std::thread::spawn(move || {
        let sample_rate = config.sample_rate.0 as f64;
        audio_sample.set_sample_rate(sample_rate);

        let mut next_value = move || audio_sample.get_stereo();
        let channels = config.channels as usize;
        let error_callback = |err| eprintln!("stream error: {err}");
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    for frame in data.chunks_mut(channels) {
                        let sample = next_value();
                        let left: T = T::from_sample(sample.0.into());
                        let right: T = T::from_sample(sample.1.into());

                        for (channel, sample) in frame.iter_mut().enumerate() {
                            *sample = if channel & 1 == 0 { left } else { right };
                        }
                    }
                },
                error_callback,
                None,
            )
            .unwrap();

        stream.play().unwrap();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().into_raw_mode()?;

    write!(
        stdout,
        r#"{}{}ctrl + q to exit, ? to help, h to print "hello world", t to test audio"#,
        termion::cursor::Goto(1, 1),
        termion::clear::All
    )?;
    stdout.flush()?;

    let mut audio_controller = Arc::new(AudioController { playing: false });

    'run: for input in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        )?;

        let input = input?;

        match input {
            Key::Char('q') | Key::Esc => break 'run,
            Key::Char('h') => println!("hello world"),
            Key::Char('t') => audio_test(&mut audio_controller)?,
            Key::Char('?') => println!("show help"),
            _ => println!("key not yet impl"),
        }
        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(4000));
    }
    Ok(())
}
