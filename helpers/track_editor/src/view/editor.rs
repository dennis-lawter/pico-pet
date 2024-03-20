use cursive::view::Nameable;
use cursive::view::Resizable;
use cursive::view::Scrollable;
use cursive::views::LinearLayout;
use cursive::views::ScrollView;
use cursive::views::TextArea;
use cursive::views::TextView;

pub fn editor_row() -> ScrollView<LinearLayout> {
    let track = crate::model::TRACK_INSTANCE.lock().unwrap();
    let editor = TextArea::new()
        .content(track.text.clone())
        .with_name("editor")
        .full_height()
        .full_width();
    let track_follower = TextView::new("▶️").with_name("track_follower");
    LinearLayout::horizontal()
        .child(track_follower)
        .child(editor)
        .scrollable()
}
