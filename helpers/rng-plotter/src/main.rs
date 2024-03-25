// const SEED_SAMPLE_SIZE: usize = 86400;

use plotters::prelude::*;

/// Profiles the Linear Congruential Generator
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = Lcg::new(0);

    let mut data: Vec<(f32,f32)> = vec![(0.0,0.0);1<<16];
    for i in 0..1<<16 {
        data[i] = (i as f32, rng.next() as f32);
    }

    plot_data(data)
}

fn plot_data(data: Vec<(f32,f32)>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("output.png", (2048, 256+200-58-40)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("zx81_lcg() & 0xff", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(40)
        .y_label_area_size(80)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..(1<<16) as f32, 0f32..255f32)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        .disable_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(17)
        .x_desc("i th random element")
        .y_labels(33)
        .y_desc("rand() result")
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // And we can draw something in the drawing area
    chart.draw_series(PointSeries::of_element(
        data,
        1,
        &RED,
        &|c, _s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Pixel::new((0,0),st) // At this point, the new pixel coordinate is established
        },
    ))?;
    root.present()?;
    Ok(())
}

struct Lcg {
    state: u32,
}

impl Lcg {
    const M: u32 = (1 << 16) + 1;
    const A: u32 = 75;
    const C: u32 = 74;

    pub fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    pub fn next(&mut self) -> u8 {
        let next = self.state * Self::A + Self::C;
        let next = next % Self::M;
        self.state = next;

        // isolate bits
        let output = (next >> 0) & 0xFF;

        output as u8
    }
}
