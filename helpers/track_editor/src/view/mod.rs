mod actions;
mod editor;
mod title;

// use std::sync::atomic::AtomicBool;
// use std::sync::atomic::Ordering;
use std::sync::Arc;
// use std::sync::Mutex;

// use crate::audio::play_preview;

// use cursive::theme::BaseColor;
// use cursive::theme::Color;
// use cursive::theme::ColorStyle;
// use cursive::theme::Style;
// use cursive::utils::markup::StyledString;
// use cursive::view::Nameable;
// use cursive::view::Resizable;
// use cursive::view::Scrollable;
// use cursive::views::Button;
use cursive::views::LinearLayout;
// use cursive::views::NamedView;
// use cursive::views::ResizedView;
// use cursive::views::ScrollView;
// use cursive::views::TextArea;
// use cursive::views::TextView;

use cursive::Cursive;
use cursive::CursiveExt;
// use cursive::View;

pub fn setup_ui(siv: &mut Cursive, track: &mut crate::model::Track) {
    let speed_multiplier = Arc::clone(&track.speed_multiplier);
    let play_state = Arc::clone(&track.playing);
    siv.load_toml(include_str!("../../assets/cursive.toml"))
        .unwrap();

    let speed_bar_view =
        actions::action_bar(Arc::clone(&speed_multiplier), Arc::clone(&play_state));
    let layout = LinearLayout::vertical()
        .child(title::title_bar())
        .child(editor::editor_row(&track))
        .child(speed_bar_view);

    siv.add_fullscreen_layer(layout);

    // siv.add_global_callback(cursive::event::Key::Esc, |s| {
    //     s.call_on_name("root.editor", |view: &mut TextArea| {
    //         view.disable();
    //     });
    //     s.focus_name("root")
    //         .expect("Could not focus on element 'root'");
    // });

    siv.run();
}
