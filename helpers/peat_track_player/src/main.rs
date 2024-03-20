mod audio;
mod freq;
mod model;
mod prelude;

use cursive::theme::BaseColor;
use cursive::theme::Color;
use cursive::theme::ColorStyle;
use cursive::utils::markup::StyledString;
use cursive::view::Nameable;
use cursive::view::Resizable;
use cursive::views::LinearLayout;
use cursive::views::TextView;
use cursive::Cursive;
use cursive::CursiveExt;

use prelude::*;

fn load_model_from_args() {
    let args = std::env::args().collect::<Vec<String>>();
    assert_eq!(args.len(), 2, "Usage: {} <file>", args[0]);

    let text = std::fs::read_to_string(&args[1]).unwrap();

    let mut model = crate::model::TRACK_INSTANCE.lock().unwrap();
    model.load_text(&text);
    drop(model);
}

fn main() {
    load_model_from_args();

    let mut siv = Cursive::default();

    build_ui(&mut siv);

    siv.run();
}

fn build_ui(siv: &mut cursive::Cursive) {
    siv.load_toml(include_str!("../assets/cursive.toml"))
        .unwrap();

    let split_size = SAMPLE_RATE as usize * 60 / crate::prelude::NOTES_PER_MINUTE as usize;
    let (track_title, sample_length) = {
        let model = crate::model::TRACK_INSTANCE.lock().unwrap();
        (model.title.clone(), model.samples.len() / split_size)
    };

    let progress_bar_view = cursive::views::ProgressBar::new()
        .range(0, sample_length)
        .with_label(|_, (_, _)| "".to_owned())
        .with_name("progress_bar")
        .full_width();

    let track_title_styled = StyledString::styled(
        track_title.clone(),
        ColorStyle::new(
            Color::Dark(BaseColor::White),
            Color::Light(BaseColor::Black),
        ),
    );

    let track_title_view = TextView::new(track_title_styled).center();

    let help_view = TextView::new("[SPACE] to toggle play/pause    [Q] to quit");

    let layout = LinearLayout::vertical()
        .child(track_title_view)
        .child(progress_bar_view)
        .child(help_view);

    let dialog = cursive::views::Dialog::around(layout).title("PEAT AUDIO PLAYER");
    siv.add_layer(dialog);

    siv.add_global_callback('q', |s| {
        s.quit();
    });
    siv.add_global_callback(' ', toggle_handler);
}

fn play_handler(siv: &mut cursive::Cursive) {
    let mut model = crate::model::TRACK_INSTANCE.lock().unwrap();
    if !model.playing {
        model.playing = true;
        drop(model);
        crate::audio::play_preview(siv);
        siv.call_on_name("play_button", |button: &mut cursive::views::Button| {
            button.set_label("STOP");
        });
    }
}

fn stop_handler(siv: &mut cursive::Cursive) {
    let mut model = crate::model::TRACK_INSTANCE.lock().unwrap();
    model.playing = false;
    siv.call_on_name("play_button", |button: &mut cursive::views::Button| {
        button.set_label("PLAY");
    });
}

fn toggle_handler(siv: &mut cursive::Cursive) {
    let playing = {
        let model = crate::model::TRACK_INSTANCE.lock().unwrap();
        model.playing
    };
    if playing {
        stop_handler(siv);
    } else {
        play_handler(siv);
    }
}
