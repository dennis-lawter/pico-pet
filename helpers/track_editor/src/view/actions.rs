use crate::prelude::*;

use cursive::view::Nameable;
use cursive::views::Button;
use cursive::views::LinearLayout;
use cursive::views::TextView;

pub fn action_bar() -> LinearLayout {
    let decrease_button = Button::new("+", move |s| {
        let mut model = crate::model::TRACK_INSTANCE.lock().unwrap();
        if model.speed_divisor > 1 {
            model.speed_divisor -= 1;
            s.call_on_name("speed_label", |view: &mut TextView| {
                let npm = NOTES_PER_MINUTE / model.speed_divisor as u32;
                view.set_content(format!(" {:<4} ", npm));
            });
        }
    });

    let increase_button = Button::new("-", move |s| {
        let mut model = crate::model::TRACK_INSTANCE.lock().unwrap();
        if model.speed_divisor < 21 {
            model.speed_divisor += 1;
            s.call_on_name("speed_label", |view: &mut TextView| {
                let npm = NOTES_PER_MINUTE / model.speed_divisor as u32;
                view.set_content(format!(" {:<4} ", npm));
            });
        }
    });

    let model = crate::model::TRACK_INSTANCE.lock().unwrap();
    let npm = NOTES_PER_MINUTE / (model.speed_divisor as u32);
    drop(model);
    let speed_label = TextView::new(format!(" {:<4} ", npm)).with_name("speed_label");

    let play_button = Button::new("PLAY", move |s| {
        let mut model = crate::model::TRACK_INSTANCE.lock().unwrap();
        if model.playing {
            model.playing = false;
            s.call_on_name("play_button", |button: &mut Button| {
                button.set_label("PLAY");
            });
        } else {
            let text = s
                .call_on_name("editor", |view: &mut cursive::views::TextArea| {
                    view.get_content().to_string()
                })
                .unwrap_or_default();

            model.playing = true;
            model.text = text.clone();

            drop(model);
            crate::audio::play_preview();
            s.call_on_name("play_button", |button: &mut Button| {
                button.set_label("STOP");
            });
        }
    });
    let play_button = play_button.with_name("play_button");

    let save_button = Button::new("SAVE", move |_s| {
        // TODO
        ()
    });

    let quit_button = Button::new("QUIT", move |s| {
        s.quit();
    });

    LinearLayout::horizontal()
        .child(play_button)
        .child(TextView::new("    "))
        .child(TextView::new("NPM: "))
        .child(decrease_button)
        .child(speed_label)
        .child(increase_button)
        .child(TextView::new("    "))
        .child(save_button)
        .child(TextView::new("    "))
        .child(quit_button)
}
