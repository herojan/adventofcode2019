use std::fs::File;
use std::io::Read;

fn main() -> () {
    let mut input = String::new();
    File::open("input.txt")
        .expect("Could not find input file")
        .read_to_string(&mut input)
        .expect("could not read input file");
    let masses: Vec<u32> = input
        .lines()
        .map(|line| {
            line.parse()
                .expect(format!("input line {} is not a u32", line).as_str())
        })
        .collect();

    part1(&masses);
    part2(&masses);
}

fn part1(masses: &Vec<u32>) {
    let sum: u32 = masses.iter().map(|&mass| fuel_for_mass(mass)).sum();
    println!("{}", sum);
}

fn part2(masses: &Vec<u32>) {
    let sum: u32 = masses
        .iter()
        .map(|&mass| {
            let mass_fuel = fuel_for_mass(mass);
            mass_fuel + fuel_for_fuel(mass_fuel)
        })
        .sum();
    println!("{}", sum);
}

fn fuel_for_mass(mass: u32) -> u32 {
    let fuel = mass as i32 / 3 - 2;

    return if fuel < 0 { 0 } else { fuel as u32 };
}

fn fuel_for_fuel(fuel_mass: u32) -> u32 {
    let mut sum = 0;
    let mut fuel = fuel_for_mass(fuel_mass);

    while fuel > 0 {
        sum += fuel;
        fuel = fuel_for_mass(fuel);
    }

    return sum;
}
