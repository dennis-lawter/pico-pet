use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;

use crate::audio::play_preview;

use cursive::view::Nameable;
use cursive::view::Resizable;
use cursive::view::Scrollable;
use cursive::views::Button;
use cursive::views::LinearLayout;
use cursive::views::NamedView;
use cursive::views::ResizedView;
use cursive::views::ScrollView;
use cursive::views::TextArea;
use cursive::views::TextView;

use cursive::Cursive;
use cursive::CursiveExt;

fn action_bar(
    speed_multiplier: Arc<Mutex<u8>>,
    play_state: Arc<Mutex<AtomicBool>>,
) -> LinearLayout {
    let decrease_multiplier = Arc::clone(&speed_multiplier);
    let decrease_button = Button::new("-", move |s| {
        let mut multiplier = decrease_multiplier.lock().unwrap();
        if *multiplier > 1 {
            *multiplier -= 1;
            s.call_on_name("speed_label", |view: &mut TextView| {
                view.set_content(format!("{}x", multiplier));
            });
        }
    });

    let increase_multiplier = Arc::clone(&speed_multiplier);
    let increase_button = Button::new("+", move |s| {
        let mut multiplier = increase_multiplier.lock().unwrap();
        if *multiplier < 10 {
            *multiplier += 1;
            s.call_on_name("speed_label", |view: &mut TextView| {
                view.set_content(format!("{}x", multiplier));
            });
        }
    });

    let initial_multiplier = speed_multiplier.lock().unwrap();
    let speed_label = TextView::new(format!("{}x", *initial_multiplier)).with_name("speed_label");

    let play_button = Button::new("PLAY", move |s| {
        let state = play_state.lock().unwrap();
        if state.load(Ordering::SeqCst) {
            state.store(false, Ordering::SeqCst);
            s.call_on_name("play_button", |button: &mut Button| {
                button.set_label("PLAY");
            });
        } else {
            state.store(true, Ordering::SeqCst);
            play_preview(Arc::clone(&play_state));
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
        .child(TextView::new("Playback Speed: "))
        .child(decrease_button)
        .child(speed_label)
        .child(increase_button)
        .child(TextView::new("    "))
        .child(save_button)
        .child(TextView::new("    "))
        .child(quit_button)
}
fn title_bar() -> TextView {
    TextView::new("PicoPet Track Editor").center()
}
fn editor() -> NamedView<ScrollView<ResizedView<ResizedView<TextArea>>>> {
    TextArea::new()
        .content("")
        .full_height()
        .full_width()
        .scrollable()
        .with_name("editor")
}

pub fn tui() {
    let speed_multiplier = Arc::new(Mutex::new(1u8));
    let mut siv = Cursive::default();
    siv.load_toml(include_str!("../../assets/cursive.toml"))
        .unwrap();

    let play_state = Arc::new(Mutex::new(AtomicBool::new(false)));
    let speed_bar_view = action_bar(Arc::clone(&speed_multiplier), Arc::clone(&play_state));
    let layout = LinearLayout::vertical()
        .child(title_bar())
        .child(editor())
        .child(speed_bar_view)
        .with_name("root");

    siv.add_fullscreen_layer(layout);

    siv.add_global_callback(cursive::event::Key::Esc, |s| {
        s.call_on_name("root.editor", |view: &mut TextArea| {
            view.disable();
        });
        s.focus_name("root")
            .expect("Could not focus on element 'root'");
    });

    siv.run();
}
