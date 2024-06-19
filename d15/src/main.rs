use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("d15.txt")?;

    let mut part1_sum = 0u32;

    // let mut bins: [Vec<String>; 256] = Default::default();
    let mut bins = vec![];
    for _ in 0..256 {
        bins.push(vec![]);
    }

    let mut part2_lookup = HashMap::new();

    for part in text.split(",") {
        part1_sum += aoc_hash(part) as u32;

        if part.contains('-') {
            let part = part.split_once('-').unwrap().0;
            bins.iter_mut().for_each(|mut b| { b.retain(|e| e != part) });
            part2_lookup.remove(part);
        } else {
            let (p1, p2) = part.split_once('=').unwrap();
            let focal = p2.parse::<usize>()?;
            let p1 = p1.to_string();
            part2_lookup.insert(p1.clone(), focal);
            let bin = aoc_hash(&p1) as usize;

            if !bins[bin].contains(&p1) {
                bins[bin].push(p1);
            }
        }
    }

    dbg!(part1_sum);

    let part2_sum = bins.iter().enumerate().map(|(i, v)| {
        v.iter().enumerate().map(|(j, e)| {
            let s = (i + 1) * (j + 1) * part2_lookup.get(e).unwrap();
            // println!("{e} box {i}, slot {j}, f {} = {s}", part2_lookup.get(e).unwrap());
            s
        }).sum::<usize>()
    }).sum::<usize>();
    dbg!(part2_sum);

    Ok({})
}

fn aoc_hash(s: &str) -> u8 {
    let mut h = 0u32;
    for c in s.bytes() {
        h += c as u32;
        h *= 17;
        h %= 256;
    }
    h as u8
}
