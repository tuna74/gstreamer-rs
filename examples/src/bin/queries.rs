extern crate gstreamer as gst;
use gst::*;

extern crate glib;
use glib::*;

use std::env;

fn main() {
    let pipeline_str = env::args().collect::<Vec<String>>()[1..].join(" ");

    gst::init().unwrap();

    let main_loop = MainLoop::new(None, false);

    let pipeline = gst::parse_launch(&pipeline_str).unwrap();
    let bus = pipeline.get_bus().unwrap();

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    let main_loop_clone = main_loop.clone();

    let pipeline_clone = pipeline.clone();
    glib::timeout_add_seconds(1, move || {
        let pipeline = &pipeline_clone;

        //let pos = pipeline.query_position(Format::Time).unwrap_or(-1);
        //let dur = pipeline.query_duration(Format::Time).unwrap_or(-1);
        let pos = {
            let mut q = Query::new_position(Format::Time);
            pipeline.query(q.get_mut().unwrap());
            match q.view() {
                QueryView::Position(ref p) => p.get().1,
                _ => unreachable!(),
            }
        };

        let dur = {
            let mut q = Query::new_duration(Format::Time);
            pipeline.query(q.get_mut().unwrap());
            match q.view() {
                QueryView::Duration(ref p) => p.get().1,
                _ => unreachable!(),
            }
        };

        println!("{} / {}", pos, dur);

        glib::Continue(true)
    });

    //bus.add_signal_watch();
    //bus.connect_message(move |_, msg| {
    bus.add_watch(move |_, msg| {
        let main_loop = &main_loop_clone;
        match msg.view() {
            MessageView::Eos(..) => main_loop.quit(),
            MessageView::Error(err) => {
                println!(
                    "Error from {}: {} ({:?})",
                    msg.get_src().get_path_string(),
                    err.get_error(),
                    err.get_debug()
                );
                main_loop.quit();
            }
            _ => (),
        };

        glib::Continue(true)
    });

    main_loop.run();

    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}
