extern crate calamine;
extern crate clap;

use calamine::{open_workbook, Reader, Xlsx};
use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
struct Track {
    particles: Vec<Particle>,
}

impl Track {
    fn xml(&self) -> String {
        // make sure this is strings joined by newlines.
        let particles = self
            .particles
            .iter()
            .map(|p| format!("  {}", p.xml()))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "<Tracks nTracks='{}' spaceUnits='pixel' frameInterval='1.0' timeUnits='frame' generationDateTime='Fr, 13 Mai 2016 21:49:25' from='TrackMate v2.8.1'>\n{}\n</Tracks>",
            self.particles.len(),
            particles
        )
    }
}

#[derive(Debug)]
struct Particle {
    detections: Vec<Detection>,
}

impl Particle {
    fn xml(&self) -> String {
        let detections = self
            .detections
            .iter()
            .map(|d| format!("    {}", d.xml()))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "<particle nSpots='{}'>\n{}\n  </particle>",
            self.detections.len(),
            detections
        )
    }
}

#[derive(Debug)]
struct Detection {
    t: i64,
    x: f64,
    y: f64,
    z: f64,
}

impl Detection {
    fn xml(&self) -> String {
        format!(
            "<detection t='{}' x='{}' y='{}' z='{}' />",
            self.t, self.x, self.y, self.z
        )
    }
}

fn main() {
    let matches = App::new("bungmunger")
        .version("0.1.0")
        .author("stuart nelson <stuartnelson3@gmail.com>")
        .about("munges xlsx trajectory files into xml")
        .arg(
            Arg::with_name("i")
                .short("i")
                .long("in-file")
                .value_name("example.xlsx")
                .help("the .xlsx input file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("o")
                .short("o")
                .long("out-file")
                .value_name("munged.xml")
                .help("the output file name")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let input = matches.value_of("i").unwrap();
    let output = matches.value_of("o").unwrap();
    let path = Path::new(output);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let mut excel: Xlsx<_> = open_workbook(input).unwrap();

    println!(
        "===> bung munging\n===> source file: {}",
        Path::new(input).display()
    );

    let range = match excel.worksheet_range_at(0) {
        Some(Ok(r)) => {
            match r
                .rows()
                .enumerate()
                .find(|&row| match row.1[0].get_string() {
                    Some(s) => s.trim() == "frame",
                    None => false,
                }) {
                Some((idx, _)) => {
                    let start = ((idx + 1) as u32, 0);
                    let end = r.end().unwrap();
                    r.range(start, end)
                }
                None => panic!("not found!"),
            }
        }
        Some(Err(e)) => {
            panic!("error: {}", e);
        }
        None => {
            panic!("none!");
        }
    };

    let mut particles: Vec<Particle> = vec![];
    range.rows().for_each(|row| {
        let t = row[0].get_float().unwrap() as i64;
        if t == 0 {
            // new particle, add to vec
            particles.push(Particle { detections: vec![] });
        }
        // append to the last particle in our vec
        let len = particles.len();
        let particle = &mut particles[len - 1];

        particle.detections.push(Detection {
            t: t,
            x: row[1].get_float().unwrap(),
            y: row[2].get_float().unwrap(),
            z: 0.0,
        });
    });

    let track = Track {
        particles: particles,
    };
    let header = "<?xml version='1.0' encoding='UTF-8'?>";
    match file.write_all(format!("{}\n{}", header, track.xml()).as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("===> bung munged\n===> output file: {}", display),
    }
}
