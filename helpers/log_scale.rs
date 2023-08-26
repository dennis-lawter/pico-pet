/// LOG SCALE TO 65535 BY 24 STEPS, DROP LOWEST 8

fn main() {
    const M: f64 = 65535.0;
    const TOTAL_VALUES: usize = 24;
    const NEEDED_VALUES: usize = 16;
    let r = M.powf(1.0 / (TOTAL_VALUES as f64 - 1.0));
    let mut values = Vec::with_capacity(TOTAL_VALUES);
    for i in 0..TOTAL_VALUES {
        values.push((r.powf(i as f64) + 0.5) as u32);
    }
    let values: Vec<_> = values.into_iter().rev().take(NEEDED_VALUES).collect();
    println!("{:?}", values);
}
