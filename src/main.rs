struct Track {
    particles: Vec<Particle>
}

struct Particle {
    detection: Vec<Detection>
}

#[derive(Debug)]
struct Detection {
    t: i64,
    x: f64,
    y: f64,
    z: f64
}

fn main() {
    use calamine::{open_workbook, Reader, Xlsx};

    let mut i = 0;
    println!("hello world");

    let mut excel: Xlsx<_> = open_workbook("example.xlsx").unwrap();
    match excel.worksheet_range_at(0) {
        Some(Ok(r)) => {
            let mut data_found = false;
        for row in r.rows() {
            // something is up with scoping, not finding this data_found
            // business.
            if i > 10 {
                return;
            }
            match row[0].get_string() {
                Some(s) => {
                    if s == "frame" {
                        data_found = true;
                        continue
                    }
                }
                None => continue
            }
            if !data_found {
                continue
            }
            if i > 10 {
                return
            }
            let d = Detection {
                t : row[0].get_int().unwrap(),
                x: row[1].get_float().unwrap(),
                y: row[2].get_float().unwrap(),
                z: 0.0
            };
            println!("{:?}", d);
            i += 1
        }
        }
        Some(Err(e)) => {
            println!("{}",e);
        }
        None => {
            println!("none!");
        }
    }
}
