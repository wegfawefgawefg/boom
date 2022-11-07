use alto::{Alto, AltoResult, Mono, Source};
use std::{thread, time};

fn main() {
    use std::process::exit;

    if let Err(e) = run() {
        println!("Failed to run basic example: {}", e);
        exit(1);
    }
}

fn run() -> AltoResult<()> {
    use std::sync::Arc;

    let alto = Alto::load_default()?;

    for s in alto.enumerate_outputs() {
        println!("Found device: {}", s.to_str().unwrap());
    }

    let device = alto.open(None)?; // Opens the default audio device
    let context = device.new_context(None)?; // Creates a default context

    // Configure listener
    context.set_position([1.0, 4.0, 5.0])?;
    context.set_velocity([2.5, 0.0, 0.0])?;
    context.set_orientation(([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]))?;

    let mut _source = context.new_static_source()?;

    // Now you can load your samples and store them in a buffer with
    // `context.new_buffer(samples, frequency)`;

    let pi = std::f32::consts::PI;
    let data: Vec<_> = (0..88200u32)
        .map(|i| ((i16::MAX as f32) * f32::sin(2.0 * pi * (i as f32) * 220.0 / 44100.0)) as i16)
        .collect();
    let buffer = context.new_buffer::<Mono<i16>, _>(data, 44_100);
    let buf = Arc::new(buffer.unwrap());

    let good_result = _source.set_buffer(buf);
    assert!(good_result.is_ok() && !good_result.is_err());

    _source.play();

    thread::sleep(time::Duration::from_millis(2000));
    Ok(())
}
