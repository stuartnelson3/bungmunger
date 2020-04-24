#[derive(Debug)]
struct Track {
    particles: Vec<Particle>,
}

#[derive(Debug)]
struct Particle {
    detections: Vec<Detection>,
}

#[derive(Debug)]
struct Detection {
    t: i64,
    x: f64,
    y: f64,
    z: f64,
}

fn main() {
    use calamine::{open_workbook, Reader, Xlsx};

    println!("opening");
    let mut excel: Xlsx<_> = open_workbook("example.xlsx").unwrap();
    println!("opened");
    let range = match excel.worksheet_range_at(0) {
        Some(Ok(r)) => {
            println!("worksheet");
            match r.rows().enumerate().find(|&row| 
                match row.1[0].get_string() {
                    Some(s) => s.trim() == "frame",
                    None => false
                }
            
            ) 
            {
                Some((idx, _)) => {
                    let start = ((idx+1) as u32,0);
                    let end = r.end().unwrap();
                    r.range(start,end)
                }
                None => panic!("not found!")
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
            particles.push(Particle{
                detections: vec![]
            });
        }
        // append to the last particle in our vec
        let len = particles.len();
        let particle = &mut particles[len-1];

        particle.detections.push(Detection {
            t: t,
            x: row[1].get_float().unwrap(),
            y: row[2].get_float().unwrap(),
            z: 0.0,
        });
    });

    println!("{:?}",particles);
}
