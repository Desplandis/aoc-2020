use std::fs::File;
use std::io::{BufReader, BufRead};

fn dirty1(values: &Vec<isize>) -> Option<(isize, isize)> {
    for x in values {
        for y in values {
            if x + y == 2020 { return Some((*x, *y)) }
        }
    }
    return None
}

fn dirty2(values: &Vec<isize>) -> Option<(isize, isize, isize)> {
    for x in values {
        for y in values {
            for z in values {
                if x + y + z == 2020 { return Some((*x, *y, *z)) }
            }
        }
    }
    return None
}

#[cfg(unused)]
fn pechu(l: &Vec<isize>) -> Option<(isize, isize)> {
    unimplemented!("A faire dans une autre vie")
}

fn main() {
    let f = File::open("input").unwrap();
    let reader = BufReader::new(f);
    let values = {
        let mut v = Vec::new();
        for l in reader.lines() {
            let i = l.unwrap().parse::<isize>().unwrap();
            v.push(i);
        }
        v
    };

    if let Some((x, y)) = dirty1(&values) {
        println!("[1] x:{}, y:{}, {}", x, y, x * y);
    } else {
        println!("[1] Not found")
    }

    if let Some((x, y, z)) = dirty2(&values) {
        println!("[2] x:{}, y:{}, z:{}, {}", x, y, z, x * y * z);
    } else {
        println!("[2] Not found")
    }
}
