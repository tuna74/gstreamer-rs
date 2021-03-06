extern crate gstreamer as gst;
use gst::*;

extern crate glib;
use glib::ObjectExt;

extern crate gtk;
use gtk::prelude::*;
use gtk::{Window, WindowType};

pub mod utils;

fn create_playbin() -> Result<gst::Pipeline, utils::ExampleError> {
    let playbin = utils::create_element("playbin")?;
    Ok(playbin.downcast::<gst::Pipeline>().unwrap())
}

fn main_loop() -> Result<(), utils::ExampleError> {
    gst::init().map_err(utils::ExampleError::InitFailed)?;
    gtk::init().unwrap();

    let (sink, widget) = if let Some(gtkglsink) = ElementFactory::make("gtkglsink", None) {
        let glsinkbin = utils::create_element("glsinkbin")?;
        glsinkbin
            .set_property("sink", &gtkglsink.to_value())
            .unwrap();

        let widget = gtkglsink.get_property("widget").unwrap();
        (glsinkbin, widget.get::<gtk::Widget>().unwrap())
    } else {
        let sink = utils::create_element("gtksink")?;
        let widget = sink.get_property("widget").unwrap();
        (sink, widget.get::<gtk::Widget>().unwrap())
    };

    let playbin = create_playbin()?;
    playbin.set_property("video_sink", &sink.to_value()).unwrap();

    let window = Window::new(WindowType::Toplevel);
    window.set_default_size(640, 480);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(&widget, true, true, 0);

    let scale = gtk::Scale::new_with_range(gtk::Orientation::Horizontal,
                                           0.0, 1.0, 0.0001);
    let playbin_clone = playbin.clone();
    let scaler_moved_id = scale.connect_value_changed(move |scale: &gtk::Scale| {
        let value = scale.get_value();
        println!("value: {}", value);
        if let Some(duration) = playbin_clone.query_duration(Format::Time) {
            println!("duration: {}", duration);
            let position = duration as f64 * value;
            println!("position: {}", position);
            playbin_clone.seek_simple(Format::Time, gst::SEEK_FLAG_FLUSH,
                                      position as i64);
        }
    });
    
    let draw_value = false.to_value();
    scale.set_property("draw-value", &draw_value).unwrap();
    vbox.pack_start(&scale, false, false, 0);
    
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let label = gtk::Label::new("Position: 00:00:00");

    let filechooserbutton = gtk::FileChooserButton::new("Open media file",
                                                        gtk::FileChooserAction::Open);
    
    let play_pause_button = gtk::Button::new_with_label("Pause");
    play_pause_button.set_sensitive(false);
    let playbin_clone = playbin.clone();

    let play_pause_button_clone = play_pause_button.clone();
    filechooserbutton.connect_file_set(move |button| {
        println!("button {:?}", button);
        let path = button.get_filename();
        if let Some(path) = path {
            let filesuffix = "file://";
            let filename = filesuffix.to_string() + path.to_str().unwrap();
            println!("filename {:?}", filename);
            let (_, current_state, _) = playbin_clone.get_state(1);
            
            if current_state != gst::State::Null {
                let ret = playbin_clone.set_state(gst::State::Null);
                assert_ne!(ret, gst::StateChangeReturn::Failure);
            }
            playbin_clone.set_property("uri", &Value::from(&filename)).unwrap();
//            utils::set_state(&playbin_clone, gst::State::Playing)?;
            let ret = playbin_clone.set_state(gst::State::Playing);
            play_pause_button_clone.set_sensitive(true);
            assert_ne!(ret, gst::StateChangeReturn::Failure);
        }
    });

    let playbin_clone = playbin.clone();    
    play_pause_button.connect_clicked(move |play_pause_button| {
        let (_, current_state, _) = playbin_clone.get_state(1);
        
        if current_state == gst::State::Paused {
            playbin_clone.set_state(gst::State::Playing);
            play_pause_button.set_label("Pause");            
        }
        else if current_state == gst::State::Playing {
            playbin_clone.set_state(gst::State::Paused);
            play_pause_button.set_label("Play");
        }
        println!("play_pause_button clicked");

    });
    hbox.pack_start(&label, false, false, 0);
    hbox.pack_start(&filechooserbutton, true, true, 0);
    hbox.pack_end(&play_pause_button, false, false, 0);

    vbox.add(&hbox);
    window.add(&vbox);
    window.show_all();


    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let playbin_clone = playbin.clone();
    let scale_clone = scale.clone();
    gtk::timeout_add(250, move || {
        let playbin = &playbin_clone;
        let position = playbin.query_position(Format::Time);

        if let Some(position) = position {
            if let Some(duration) = playbin.query_duration(Format::Time) {
                glib::signal::signal_handler_block(&scale_clone, scaler_moved_id);
                scale_clone.set_value(position as f64/duration as f64);
                glib::signal::signal_handler_unblock(&scale_clone, scaler_moved_id);
            }
            
            let mut seconds = position / 1_000_000_000;
            let mut minutes = seconds / 60;
            let hours = minutes / 60;

            seconds -= hours * 60 * 60 + minutes * 60;
            minutes -= hours * 60;

            label.set_text(&format!(
                "Position: {:02}:{:02}:{:02}",
                hours,
                minutes,
                seconds
            ));
        } else {
            label.set_text("Position: 00:00:00");
        }

        glib::Continue(true)
    });

    if let Some(bus) = playbin.get_bus() {
        bus.add_watch(move |_, msg| {
            match msg.view() {
                MessageView::Eos(..) => gtk::main_quit(),
                MessageView::Error(err) => {
                    println!(
                        "Error from {}: {} ({:?})",
                        msg.get_src().get_path_string(),
                        err.get_error(),
                        err.get_debug()
                    );
                    gtk::main_quit();
                }
                _ => (),
            };
            
            glib::Continue(true)
        });
    }
    
    gtk::main();

    utils::set_state(&playbin, gst::State::Null)?;
    Ok(())
}


fn main() {
    match main_loop() {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}
