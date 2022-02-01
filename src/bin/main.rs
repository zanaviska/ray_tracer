use std::fs::File;
use std::io::{BufRead, BufReader};

use ray_tracer::Color;

#[derive(Copy, Clone, Debug)]
struct Coor {
    x: f64,
    y: f64,
    z: f64,
}

type Triangle = [Coor; 3];

fn main() {
    let c = Color {
        red: 15u8,
        green: 24u8,
        blue: 48u8,
    };
    println!("{:?}", c);

    let f = File::open("cow.obj").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut vertexes: Vec<Coor> = Vec::new();
    let mut shape: Vec<Triangle> = Vec::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let mut it = line.split(' ');
        match it.next() {
            Some("f") => {
                let c1 = vertexes[it
                    .next()
                    .unwrap()
                    .split('/')
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
                    - 1];
                let c2 = vertexes[it
                    .next()
                    .unwrap()
                    .split('/')
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
                    - 1];
                while let Some(ind) = it.next() {
                    let c3 = vertexes[ind.split('/').next().unwrap().parse::<usize>().unwrap() - 1];
                    shape.push([c1, c2, c3]);
                }
            }
            Some("v") => {
                let coor = Coor {
                    x: it.next().unwrap().parse::<f64>().unwrap(),
                    y: it.next().unwrap().parse::<f64>().unwrap(),
                    z: it.next().unwrap().parse::<f64>().unwrap(),
                };
                vertexes.push(coor);
            }
            None => {}
            Some(_) => {}
        }
    }
}
