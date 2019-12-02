use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .expect("Could not find input file")
        .read_to_string(&mut input)
        .expect("could not read input file");
    let program: IntCodeProgram = input.parse().unwrap();

    part1(&program);
    part2(&program);
}

fn part1(program: &IntCodeProgram) {
    let result = program.compute(12, 2);
    println!("{}", result);
}

fn part2(program: &IntCodeProgram) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let result = program.compute(noun, verb);
            if result == 19690720 {
                println!(
                    "noun: {}, verb: {}, answer: {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
                return;
            }
        }
    }

    println!("Found no answers :(");
}

struct IntCodeProgram {
    initial_state: Vec<u32>,
}

impl IntCodeProgram {
    fn compute(&self, noun: u32, verb: u32) -> u32 {
        let memory = &mut self.initial_state.clone();
        memory[1] = noun;
        memory[2] = verb;

        let mut instruction_pointer = 0;
        let mut op_code = memory.get(instruction_pointer).unwrap();
        while (*op_code == 1 || *op_code == 2) && (*op_code != 99) {
            let param1 = *memory.get(instruction_pointer + 1).unwrap() as usize;
            let param2 = *memory.get(instruction_pointer + 2).unwrap() as usize;
            let param3 = *memory.get(instruction_pointer + 3).unwrap() as usize;

            let num1 = *memory.get(param1).unwrap();
            let num2 = *memory.get(param2).unwrap();

            if *op_code == 1 {
                memory[param3] = num1 + num2;
            } else if *op_code == 2 {
                memory[param3] = num1 * num2;
            }

            instruction_pointer += 4;
            op_code = memory.get(instruction_pointer).unwrap();
        }

        return memory[0];
    }
}

impl FromStr for IntCodeProgram {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let initial_state: Vec<u32> = s
            .lines()
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();

        return Ok(IntCodeProgram { initial_state });
    }
}
