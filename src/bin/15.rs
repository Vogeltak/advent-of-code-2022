use std::convert::TryFrom;

use itertools::Itertools;

struct Sensor {
    x: i32,
    y: i32,
    beacon_x: i32,
    beacon_y: i32,
    manhattan: u32,
}

impl TryFrom<&str> for Sensor {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.strip_prefix("Sensor at x=").unwrap();
        let (x, value) = value.split_once(", y=").unwrap();
        let (y, value) = value.split_once(": closest beacon is at x=").unwrap();
        let (beacon_x, beacon_y) = value.split_once(", y=").unwrap();

        let (x, y) = (x.parse::<i32>()?, y.parse::<i32>()?);
        let (beacon_x, beacon_y) = (beacon_x.parse::<i32>()?, beacon_y.parse::<i32>()?);
        let manhattan = ((beacon_x - x).abs() + (beacon_y - y).abs()) as u32;

        Ok(Self {x, y, beacon_x, beacon_y, manhattan})
    }
}

impl Sensor {
    fn manhattan_distance(&self, x: i32, y: i32) -> u32 {
        ((x - self.x).abs() + (y - self.y).abs()) as u32
    }

    fn pos_cannot_contain_beacon(&self, x: i32, y: i32) -> bool {
        if (x, y) == (self.beacon_x, self.beacon_y) {
            return false;
        }

        self.manhattan_distance(x, y) <= self.manhattan
    }
}

#[aoc::main(15)]
fn main(input: &str) -> (usize, usize) {
    let sensors = input
        .lines()
        .map(|l| Sensor::try_from(l).unwrap())
        .collect_vec();

    let left = sensors
        .iter()
        .map(|s| s.x - s.manhattan as i32)
        .min()
        .unwrap();

    let right = sensors
        .iter()
        .map(|s| s.x + s.manhattan as i32)
        .max()
        .unwrap();

    let p1 = (left..=right)
        .map(|x| {
            for s in &sensors {
                if s.pos_cannot_contain_beacon(x, 2_000_000) {
                    return true;
                }
            }

            false
        })
        .filter(|&x| x)
        .count();

    (p1, 0)
}