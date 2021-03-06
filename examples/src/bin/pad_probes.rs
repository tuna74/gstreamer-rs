extern crate gstreamer as gst;
use gst::*;

extern crate gstreamer_audio as gst_audio;

use std::u64;
use std::i16;

fn main() {
    gst::init().unwrap();

    let pipeline = gst::parse_launch(&format!(
        "audiotestsrc name=src ! audio/x-raw,format={},channels=1 ! fakesink",
        gst_audio::AUDIO_FORMAT_S16.to_string()
    )).unwrap();
    let bus = pipeline.get_bus().unwrap();

    let src = pipeline
        .clone()
        .dynamic_cast::<Bin>()
        .unwrap()
        .get_by_name("src")
        .unwrap();
    let src_pad = src.get_static_pad("src").unwrap();
    src_pad.add_probe(PAD_PROBE_TYPE_BUFFER, |_, probe_info| {
        if let Some(PadProbeData::Buffer(ref buffer)) = probe_info.data {
            let map = buffer.map_readable().unwrap();

            let data =
                gst_audio::AudioData::new(map.as_slice(), gst_audio::AUDIO_FORMAT_S16).unwrap();
            let samples = if let gst_audio::AudioData::S16(samples) = data {
                samples
            } else {
                return PadProbeReturn::Ok;
            };

            let sum: f64 = samples
                .iter()
                .map(|sample| {
                    let f = (*sample as f64) / (i16::MAX as f64);
                    f * f
                })
                .sum();
            let rms = (sum / (samples.len() as f64)).sqrt();
            println!("rms: {}", rms);
        }

        PadProbeReturn::Ok
    });

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    loop {
        let msg = match bus.timed_pop(u64::MAX) {
            None => break,
            Some(msg) => msg,
        };

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {}: {} ({:?})",
                    msg.get_src().get_path_string(),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            _ => (),
        }
    }

    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}
