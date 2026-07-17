extern crate rodio; //to import the rodio crate for audio related things

use rodio::{OutputStream, Sink, Source, source::SineWave}; //importing the Sine, OutputStream and Sink from rodio crate

const SAMPLE_RATE: u32 = 44100; //setting the sample rate to 44100 Hz
const FREQUENCY_OSC1: f32 = 440.0; //setting the frequency for Oscillator 1
const FREQUENCY_OSC2: f32 = 660.0; //setting the frequency for Oscillator 2
const DURATION: u32 = 2000; //setting the duration to 2000 milliseconds which are equal to 2 seconds

fn main()
{
    let (_stream, stream_handle)= OutputStream::try_default().unwrap(); //creating an output stream and getting the stream handle
    let sink = Sink::try_new(&stream_handle).unwrap(); //creating a sink to play the actual sound

    // Creating Oscillator 1
    let osc1 = SineWave::new(FREQUENCY_OSC1).take_duration(std::time::Duration::from_millis(DURATION.into()));

    // Creating Oscillator 2
    let osc2 = SineWave::new(FREQUENCY_OSC2).take_duration(std::time::Duration::from_millis(DURATION.into()));

    // Mixing Oscillators
    let mixed_osc = osc1.mix(osc2);

    // Adding mixed oscillator to the sink
    sink.append(mixed_osc);

    std::thread::sleep(std::time::Duration::from_millis(DURATION.into())); //wait for sound to finish playing
}