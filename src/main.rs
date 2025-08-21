#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use {
    rodio::*,
    slint::include_modules,
    std::{
        cell::RefCell,
        error::Error,
        io::Cursor,
        rc::Rc
        }
    };

const LAUGH_AUDIO: Cursor<&[u8]> = Cursor::new(include_bytes!("../assets/laugh.mp3"));
const DISAPPROVAL_AUDIO: Cursor<&[u8]> = Cursor::new(include_bytes!("../assets/sus.mp3"));

include_modules!();

fn try_decode_audio(cursor: Cursor<&'static [u8]>) -> Decoder<Cursor<&'static [u8]>> {
    Decoder::new_mp3(cursor)
        .expect("Problem occured while the decoding audio")
    }

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new()?;
    let question = Question::new()?;
    let positive = Positive::new()?;
    let negative = Negative::new()?;

    let stream_handle = OutputStreamBuilder::open_default_stream()?;
    let sink = Sink::connect_new(&stream_handle.mixer());
    
    let sink_cell = Rc::new(RefCell::new(sink));
    let sink_copy = sink_cell.clone();

    question.on_open_positive(move || {
        positive.run();
        sink_cell.borrow()
            .append(try_decode_audio(LAUGH_AUDIO));
        });

    question.on_open_negative(move || {
        negative.run();
        sink_copy.borrow()
            .append(try_decode_audio(DISAPPROVAL_AUDIO));
        });

    app.on_open_start(move || {
        question.run();
        });

    app.run();

    Ok(())
    }