mod audio;
mod freq;
mod model;
mod prelude;
mod view;

use cursive::Cursive;

use view::setup_ui;

fn main() {
    let mut siv = Cursive::default();
    setup_ui(&mut siv);
    siv.set_fps(30);
    siv.add_global_callback(cursive::event::Event::Refresh, |s| {
        // let notes_played = {
        //     let track = model::TRACK_INSTANCE.lock().unwrap();
        //     track.notes_played
        // };
        let notes_played = 5;
        s.call_on_name("track_follower", |view: &mut cursive::views::TextView| {
            let mut content = "\n".repeat(notes_played as usize);
            content.push_str("▶️");
            view.set_content(content);
        })
        .unwrap();
    });
}
