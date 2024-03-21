use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATION_LAYER: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("C#", "Cs");
        m.insert("Db", "Cs");
        m.insert("D#", "Ds");
        m.insert("Eb", "Ds");
        m.insert("F#", "Fs");
        m.insert("Gb", "Fs");
        m.insert("G#", "Gs");
        m.insert("Ab", "Gs");
        m.insert("A#", "As");
        m.insert("Bb", "As");
        m
    };
}

pub fn translate(name: &str) -> String {
    if name.len() <= 2 {
        return name.to_string();
    }
    let (first_two_chars, remainder) = name.split_at(2);
    let translated_name = TRANSLATION_LAYER.get(first_two_chars).copied();
    match translated_name {
        Some(translated_name) => translated_name.to_string() + remainder,
        None => name.to_string(),
    }
}

lazy_static! {
    pub static ref FREQ_TABLE: HashMap<&'static str, f32> = {
        let mut m = HashMap::new();
        m.insert("C4", 261.63);
        m.insert("Cs4", 277.18);
        m.insert("D4", 293.66);
        m.insert("Ds4", 311.13);
        m.insert("E4", 329.63);
        m.insert("F4", 349.23);
        m.insert("Fs4", 369.99);
        m.insert("G4", 392.0);
        m.insert("Gs4", 415.3);
        m.insert("A4", 440.0);
        m.insert("As4", 466.16);
        m.insert("B4", 493.88);
        m.insert("C5", 523.25);
        m.insert("Cs5", 554.37);
        m.insert("D5", 587.33);
        m.insert("Ds5", 622.25);
        m.insert("E5", 659.25);
        m.insert("F5", 698.46);
        m.insert("Fs5", 739.99);
        m.insert("G5", 783.99);
        m.insert("Gs5", 830.61);
        m.insert("A5", 880.0);
        m.insert("As5", 932.33);
        m.insert("B5", 987.77);
        m.insert("C6", 1046.5);
        m.insert("Cs6", 1108.73);
        m.insert("D6", 1174.66);
        m.insert("Ds6", 1244.51);
        m.insert("E6", 1318.51);
        m.insert("F6", 1396.91);
        m.insert("Fs6", 1479.98);
        m.insert("G6", 1567.98);
        m.insert("Gs6", 1661.22);
        m.insert("A6", 1760.0);
        m.insert("As6", 1864.66);
        m.insert("B6", 1975.53);
        m.insert("C7", 2093.0);
        m.insert("_", 0.0);
        m
    };
}

pub fn get_freq(name: &str) -> Option<f32> {
    let name = translate(name);
    FREQ_TABLE.get(name.as_str()).copied()
}
