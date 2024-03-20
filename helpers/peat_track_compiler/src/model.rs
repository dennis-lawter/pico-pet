use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::freq::get_freq;

#[repr(C)]
pub struct Track {
    pub speed_divisor: u8,
    pub notes: Vec<u8>,
}
impl Track {
    pub fn from_filename(filename: &str) -> Self {
        let text = std::fs::read_to_string(filename).expect("Could not read file");

        let mut model = Self {
            speed_divisor: 1,
            notes: vec![],
        };
        model.load_text(&text);

        model
    }
    pub fn write(&self, filename: &str) {
        let mut file = File::create(filename).expect("Could not create file");
        let mut writer = BufWriter::new(&mut file);
        writer
            .write_all(&[self.speed_divisor])
            .expect("Could not write to file");
        writer
            .write_all(&self.notes)
            .expect("Could not write to file");
        writer.flush().expect("Could not write to file");
    }
    pub fn load_text(&mut self, text: &str) {
        let lines: Vec<&str> = text.lines().collect();
        assert!(lines.len() >= 5, "Not enough lines");
        assert_eq!(lines[0], "PEAT 1", "Invalid header: PEAT version");
        assert!(lines[1].starts_with("NPMD "), "Invalid header: NPMD value");
        assert_eq!(
            lines[2], "",
            "Invalid header, expected empty line on line 3"
        );
        assert_eq!(
            lines[4], "",
            "Invalid header, expected empty line on line 5"
        );

        self.notes.clear();

        self.speed_divisor = lines[1]
            .strip_prefix("NPMD ")
            .expect("Invalid header")
            .parse::<u8>()
            .expect("Invalid speed divisor");

        let mut previous_frequency: Option<u8> = None;
        for line in lines.iter().skip(5) {
            let line = *line;
            let freq = match line {
                "." => match previous_frequency {
                    Some(freq) => freq,
                    None => panic!("Sustain without prior note"),
                },
                "" => continue,
                other_text => match get_freq(other_text) {
                    Some(freq) => freq,
                    None => {
                        panic!("Invalid note: {}", other_text);
                    }
                },
            };
            previous_frequency = Some(freq);
            self.notes.push(freq);
        }
    }
}
