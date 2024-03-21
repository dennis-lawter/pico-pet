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
    pub static ref FREQ_TABLE: HashMap<&'static str, u8> = {
        let mut m = HashMap::new();
        m.insert("C4", 0x77);
        m.insert("Cs4", 0x78);
        m.insert("D4", 0x79);
        m.insert("Ds4", 0x7A);
        m.insert("E4", 0x7B);
        m.insert("F4", 0x7C);
        m.insert("Fs4", 0x7D);
        m.insert("G4", 0x7E);
        m.insert("Gs4", 0x7F);
        m.insert("A4", 0x80);
        m.insert("As4", 0x81);
        m.insert("B4", 0x82);
        m.insert("C5", 0x83);
        m.insert("Cs5", 0x84);
        m.insert("D5", 0x85);
        m.insert("Ds5", 0x86);
        m.insert("E5", 0x87);
        m.insert("F5", 0x88);
        m.insert("Fs5", 0x89);
        m.insert("G5", 0x8A);
        m.insert("Gs5", 0x8B);
        m.insert("A5", 0x8C);
        m.insert("As5", 0x8D);
        m.insert("B5", 0x8E);
        m.insert("C6", 0x8F);
        m.insert("Cs6", 0x90);
        m.insert("D6", 0x91);
        m.insert("Ds6", 0x92);
        m.insert("E6", 0x93);
        m.insert("F6", 0x94);
        m.insert("Fs6", 0x95);
        m.insert("G6", 0x96);
        m.insert("Gs6", 0x97);
        m.insert("A6", 0x98);
        m.insert("As6", 0x99);
        m.insert("B6", 0x9A);
        m.insert("C7", 0x9B);
        m.insert("_", 0x00);
        m
    };
}

pub fn get_freq(name: &str) -> Option<u8> {
    let name = translate(name);
    FREQ_TABLE.get(name.as_str()).copied()
}
