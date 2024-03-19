use crate::prelude::*;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;

use cursive::view::Nameable;
use cursive::views::Button;
use cursive::views::LinearLayout;
use cursive::views::TextView;

pub fn action_bar(
    speed_multiplier: Arc<Mutex<u8>>,
    play_state: Arc<Mutex<AtomicBool>>,
) -> LinearLayout {
    let decrease_multiplier = Arc::clone(&speed_multiplier);
    let decrease_button = Button::new("+", move |s| {
        let mut multiplier = decrease_multiplier.lock().unwrap();
        if *multiplier > 1 {
            *multiplier -= 1;
            s.call_on_name("speed_label", |view: &mut TextView| {
                let npm = NOTES_PER_MINUTE / *multiplier as u32;
                view.set_content(format!(" {:<4} ", npm));
            });
        }
    });

    let increase_multiplier = Arc::clone(&speed_multiplier);
    let increase_button = Button::new("-", move |s| {
        let mut multiplier = increase_multiplier.lock().unwrap();
        if *multiplier < 21 {
            *multiplier += 1;
            s.call_on_name("speed_label", |view: &mut TextView| {
                let npm = NOTES_PER_MINUTE / *multiplier as u32;
                view.set_content(format!(" {:<4} ", npm));
            });
        }
    });

    let initial_multiplier = speed_multiplier.lock().unwrap();
    let npm = NOTES_PER_MINUTE / *initial_multiplier as u32;
    let speed_label = TextView::new(format!(" {:<4} ", npm)).with_name("speed_label");

    let play_button = {
        let play_state_clone = Arc::clone(&play_state);
        let speed_multiplier_clone = Arc::clone(&speed_multiplier);

        Button::new("PLAY", move |s| {
            let state = play_state.lock().unwrap();
            if state.load(Ordering::SeqCst) {
                state.store(false, Ordering::SeqCst);
                s.call_on_name("play_button", |button: &mut Button| {
                    button.set_label("PLAY");
                });
            } else {
                state.store(true, Ordering::SeqCst);
                crate::audio::play_preview(
                    Arc::clone(&play_state_clone),
                    Arc::clone(&speed_multiplier_clone),
                );
                s.call_on_name("play_button", |button: &mut Button| {
                    button.set_label("STOP");
                });
            }
        })
    };
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
