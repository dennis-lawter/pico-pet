mod freq;
mod model;

fn main() {
    let model = model::Track::from_args();
    model.write();
}
