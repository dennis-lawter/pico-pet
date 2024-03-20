mod actions;
mod editor;
mod title;

use cursive::views::LinearLayout;
use cursive::Cursive;
use cursive::CursiveExt;

pub fn setup_ui(siv: &mut Cursive) {
    siv.load_toml(include_str!("../../assets/cursive.toml"))
        .unwrap();

    let speed_bar_view = actions::action_bar();
    let layout = LinearLayout::vertical()
        .child(title::title_bar())
        .child(editor::editor_row())
        .child(speed_bar_view);

    siv.add_fullscreen_layer(layout);

    siv.run();
}
