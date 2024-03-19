mod audio;
mod freq;
mod model;
mod prelude;
mod view;

use cursive::Cursive;

use view::setup_ui;

fn main() {
    let mut track = model::Track::new();

    let mut siv = Cursive::default();
    setup_ui(&mut siv, &mut track);
}
