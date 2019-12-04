#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() -> () {
    let mut input = String::new();
    File::open("input.txt")
        .expect("Could not find input file")
        .read_to_string(&mut input)
        .expect("could not read input file");

    let mut lines = input.lines();
    let line1 = lines.next().unwrap();
    let line2 = lines.next().unwrap();

    let points1 = find_points(line1);
    let points2 = find_points(line2);

    let common: Vec<&Point> = points1.iter().filter(|&p| points2.contains(p)).collect();

    part1(&common);
    part2(&points1, &points2, &common);
}

fn part1(common: &Vec<&Point>) {
    let min = common
        .iter()
        .map(|&p| manhattan_distance(&Point { x: 0, y: 0 }, p))
        .min()
        .unwrap();
    println!("{:?}", min);
}

fn part2(points1: &Vec<Point>, points2: &Vec<Point>, common: &Vec<&Point>) {
    let steps1 = steps_count(points1, common);
    let steps2 = steps_count(points2, common);
    let min_combined_steps = steps1
        .iter()
        .map(|(s1, s2)| steps2.get(s1).unwrap() + s2)
        .min();
    println!("{:?}", min_combined_steps);
}

fn steps_count(points: &Vec<Point>, common: &Vec<&Point>) -> HashMap<Point, u32> {
    let mut result: HashMap<Point, u32> = HashMap::new();
    let mut count: u32 = 0;
    for p in points {
        count += 1;
        if common.contains(&p) {
            result.entry(p.clone()).or_insert(count);
        }
    }

    return result;
}

fn manhattan_distance(point1: &Point, point2: &Point) -> u32 {
    return ((point1.x - point2.x).abs() + (point1.y - point2.y).abs()) as u32;
}

fn find_points(line: &str) -> Vec<Point> {
    let vectors: Vec<Vector> = line.split(",").map(|part| part.parse().unwrap()).collect();

    let mut prev = Point { x: 0, y: 0 };
    let mut points: Vec<Point> = vec![];
    for vector in vectors {
        for _ in 1..=vector.magnitude as i32 {
            let next = prev.step(&vector.direction);
            points.push(next);
            prev = next.clone();
        }
    }
    return points;
}
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn step(&self, direction: &Direction) -> Point {
        return match direction {
            Direction::U => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::D => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::L => Point {
                x: self.x - 1,
                y: self.y,
            },
            Direction::R => Point {
                x: self.x + 1,
                y: self.y,
            },
        };
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Vector {
    direction: Direction,
    magnitude: u32,
}
#[derive(Hash, Eq, PartialEq, Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl FromStr for Direction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            "L" => Ok(Direction::L),
            "R" => Ok(Direction::R),
            _ => Err(Box::<dyn Error>::from("invalid direction")),
        }
    }
}

impl FromStr for Vector {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(\s)*(?P<direction>[U,D,L,R])(?P<magnitude>\d+)$").unwrap();
        }

        let captures = match RE.captures(s) {
            None => return Err(Box::<dyn Error>::from("invalid point")),
            Some(captures) => captures,
        };

        return Ok(Vector {
            direction: captures["direction"].parse().unwrap(),
            magnitude: captures["magnitude"].parse().unwrap(),
        });
    }
}
