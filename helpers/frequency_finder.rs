/// Finds the best RP2040 PWM register settings to approximate A440 notes
use indexmap::IndexMap;

fn main() {
    let mut freq_table: IndexMap<&str, f32> = IndexMap::new();

    freq_table.insert("C4", 261.63);
    freq_table.insert("Cs4", 277.18);
    freq_table.insert("D4", 293.66);
    freq_table.insert("Ds4", 311.13);
    freq_table.insert("E4", 329.63);
    freq_table.insert("F4", 349.23);
    freq_table.insert("Fs4", 369.99);
    freq_table.insert("G4", 392.0);
    freq_table.insert("Gs4", 415.3);
    freq_table.insert("A4", 440.0);
    freq_table.insert("As4", 466.16);
    freq_table.insert("B4", 493.88);
    freq_table.insert("C5", 523.25);
    freq_table.insert("Cs5", 554.37);
    freq_table.insert("D5", 587.33);
    freq_table.insert("Ds5", 622.25);
    freq_table.insert("E5", 659.25);
    freq_table.insert("F5", 698.46);
    freq_table.insert("Fs5", 739.99);
    freq_table.insert("G5", 783.99);
    freq_table.insert("Gs5", 830.61);
    freq_table.insert("A5", 880.0);
    freq_table.insert("As5", 932.33);
    freq_table.insert("B5", 987.77);
    freq_table.insert("C6", 1046.5);
    freq_table.insert("Cs6", 1108.73);
    freq_table.insert("D6", 1174.66);
    freq_table.insert("Ds6", 1244.51);
    freq_table.insert("E6", 1318.51);
    freq_table.insert("F6", 1396.91);
    freq_table.insert("Fs6", 1479.98);
    freq_table.insert("G6", 1567.98);
    freq_table.insert("Gs6", 1661.22);
    freq_table.insert("A6", 1760.0);
    freq_table.insert("As6", 1864.66);
    freq_table.insert("B6", 1975.53);
    freq_table.insert("C7", 2093.0);

    for (note, freq) in freq_table {
        let best_combo = get_closest_setting(freq);
        println!("            Self::{} => {:?},", note, best_combo);
    }
}

fn get_closest_setting(target_frequency: f32) -> (u16, u8, u8) {
    let mut closest_frequency = std::f32::INFINITY;
    let mut best_combo = (0, 0, 0);

    for div_int in 0..=255 {
        for div_frac in 0..=15 {
            let top = (125_000_000.0
                / (target_frequency * (div_int as f32 + div_frac as f32 / 16.0)))
                - 1.0;
            if top > 65535.0 {
                continue;
            }
            let frequency = 125_000_000.0 / (top + 1.0) * (div_int as f32 + div_frac as f32 / 16.0);
            if (frequency - target_frequency).abs() < (closest_frequency - target_frequency).abs() {
                closest_frequency = frequency;
                best_combo = (top as u16, div_int, div_frac);
            }
        }
    }
    best_combo
}
