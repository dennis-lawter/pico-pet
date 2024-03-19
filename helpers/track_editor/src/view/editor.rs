// use cursive::theme::BaseColor;
// use cursive::theme::Color;
// use cursive::theme::ColorStyle;
// use cursive::utils::markup::StyledString;
use cursive::view::Nameable;
use cursive::view::Resizable;
use cursive::view::Scrollable;
use cursive::views::LinearLayout;
use cursive::views::ScrollView;
use cursive::views::TextArea;
use cursive::views::TextView;

pub fn editor_row(track: &crate::model::Track) -> ScrollView<LinearLayout> {
    let track_text = {
        let text_lock = track.text.lock().unwrap();
        text_lock.clone()
    };
    let editor = TextArea::new()
        .content(track_text)
        .full_height()
        .fixed_width(4)
        .with_name("editor");
    let track_follower = TextView::new("▶️");
    // let mut line_numbers_text = String::new();
    // for i in 1..2 {
    //     line_numbers_text = format!("{}{:>5} \n", line_numbers_text, i);
    // }
    // let line_numbers_styled = StyledString::styled(
    //     line_numbers_text,
    //     ColorStyle::new(
    //         Color::Dark(BaseColor::Black),
    //         Color::Light(BaseColor::White),
    //     ),
    // );
    // let line_numbers_view = TextView::new(line_numbers_text)
    //     .with_name("line_numbers")
    // .style(ColorStyle::new(
    //     Color::Dark(BaseColor::Black),
    //     Color::Light(BaseColor::White),
    // ));
    LinearLayout::horizontal()
        // .child(line_numbers_view)
        .child(track_follower)
        .child(editor)
        .scrollable()
}
