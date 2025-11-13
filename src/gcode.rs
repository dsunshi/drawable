

use std::io::prelude::*;
use std::fs::File;

const FEED_RATE: f32 = 1000.0;

const Z0: f32 = 5.0;

pub struct Printer {
    min: (f32, f32),
    max: (f32, f32),
    pub width:  f32,
    pub height: f32,
    mode : PrintMode,
    commands:   Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PrintMode {
    LINES,
    DOTS,
}

impl Printer {
    pub fn new( (minx, miny): (f32, f32),
                (maxx, maxy): (f32, f32),
                mode: PrintMode) -> Self {
        Printer {
            min:     (minx, miny),
            max:     (maxx, maxy),
            width:    maxx - minx,
            height:   maxy - miny,
            mode:     mode,
            commands: Vec::new(),
        }.init()
    }

    pub fn goto(&mut self, x: f32, y: f32) {
        if self.mode == PrintMode::DOTS {
            self.commands.push(format!("G1 X{:.1} Y{:.1}", x, y));
            // Pen down for the dot
            self.commands.push(format!("G1 Z-1.0"));
            self.commands.push(format!("G1 Z0.0"));
        } else if self.mode == PrintMode::LINES {
            self.commands.push(format!("G1 X{:.1} Y{:.1}", x, y));
        }
        else {
        }
    }

    pub fn save(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        for cmd in &self.commands {
            _ = file.write_all(cmd.as_bytes());
            _ = file.write_all("\n".as_bytes());
        }
        _ = file.write_all("M84 ; disable motors\n".as_bytes());
    }

    fn init(mut self) -> Self {
        let (minx, miny) = self.min;

        self.commands.clear();

        self.commands.push("M862.3 P \"MK3S\" ; printer model check".to_owned());
        self.commands.push("G21   ; set units to millimeters".to_owned());
        self.commands.push("G90   ; use absolute coordinates".to_owned());
        self.commands.push("G28 W ; home all without mesh bed level".to_owned());
        
        self.commands.push(format!("G1 X{:.1} Y{:.1}, Z{:.1}, F{:.1}",
                minx,
                miny,
                Z0,
                FEED_RATE));
        self.commands.push("G92 X0 Y0 Z0 ; set current position to origin".to_owned());
        self
    }
}
