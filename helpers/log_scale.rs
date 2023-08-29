/// LOG SCALE TO 65535 BY 24 STEPS, KEEP HIGHEST 16

fn main() {
    const MAX_VALUE: f64 = 65535.0;
    const TOTAL_VALUES: usize = 32;
    const NEEDED_VALUES: usize = 16;
    let r = MAX_VALUE.powf(1.0 / (TOTAL_VALUES as f64 - 1.0));
    let mut values = Vec::with_capacity(TOTAL_VALUES);
    for i in 0..TOTAL_VALUES {
        values.push((r.powf(i as f64) + 0.5) as u32);
    }
    let values: Vec<_> = values.into_iter().rev().take(NEEDED_VALUES).collect();
    println!("{:?}", values);
}
