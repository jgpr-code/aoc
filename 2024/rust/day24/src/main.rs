#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
use common::{regx, Answer};
use std::{
    collections::{HashMap, VecDeque},
    io,
};

pub fn main() -> Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    println!("part1: {}", part_one(&stdin)?);
    println!("part2: {}", part_two(&stdin)?);
    Ok(())
}

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

#[derive(Debug, Clone, Copy)]
enum GateType {
    And,
    Or,
    Xor,
}

impl TryFrom<&str> for GateType {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => Err(anyhow!("not a valid gate_type: {}", value))?,
        })
    }
}

#[derive(Debug, Clone)]
struct Gate {
    name: String,
    inputs: (String, String),
    gate_type: GateType,
    value: Option<bool>,
}
impl Gate {
    fn compute(&mut self, inputs: (bool, bool)) {
        let (lhs, rhs) = inputs;
        self.value = Some(match self.gate_type {
            GateType::And => lhs && rhs,
            GateType::Or => lhs || rhs,
            GateType::Xor => lhs ^ rhs,
        });
    }
    // fn try_compute(&mut self, inputs: &HashMap<String, bool>, gates: &HashMap<String, Gate>) -> Result<()> {
    //     let inputs = self.get_inputs(inputs, gates).ok_or(anyhow!("both inputs must be initialized"))?;
    //     self.compute(inputs);
    //     Ok(())
    // }
    fn get_inputs(
        &self,
        inputs: &HashMap<String, bool>,
        gates: &HashMap<String, Gate>,
    ) -> Option<(bool, bool)> {
        let lhs = inputs
            .get(&self.inputs.0)
            .cloned()
            .or(gates.get(&self.inputs.0).and_then(|g| g.value));
        let rhs = inputs
            .get(&self.inputs.1)
            .cloned()
            .or(gates.get(&self.inputs.1).and_then(|g| g.value));
        if lhs.is_some() && rhs.is_some() {
            Some((lhs.unwrap(), rhs.unwrap()))
        } else {
            None
        }
    }
}

impl TryFrom<&str> for Gate {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let gate_regex = regx!(r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)");
        let caps = gate_regex
            .captures(value)
            .ok_or(anyhow!("not a valid gate_str {}", value))?;
        let name = String::from(&caps[4]);
        let inputs = (String::from(&caps[1]), String::from(&caps[3]));
        let gate_type = GateType::try_from(&caps[2])?;
        Ok(Gate {
            name,
            inputs,
            gate_type,
            value: None,
        })
    }
}

#[derive(Clone)]
struct Circuit {
    inputs: HashMap<String, bool>,
    gates: HashMap<String, Gate>,
    consumers: HashMap<String, Vec<String>>,
}

impl Circuit {
    fn get_gate<'a>(gates: &'a HashMap<String, Gate>, gate_name: &str) -> Result<&'a Gate> {
        Ok(gates
            .get(gate_name)
            .ok_or(anyhow!("gate {} must be present", gate_name))?)
    }
    fn get_gate_mut<'a>(
        gates: &'a mut HashMap<String, Gate>,
        gate_name: &str,
    ) -> Result<&'a mut Gate> {
        Ok(gates
            .get_mut(gate_name)
            .ok_or(anyhow!("gate {} must be present", gate_name))?)
    }
    fn update_consumers(
        &mut self,
        input_name: &str,
        initialized: &mut VecDeque<String>,
    ) -> Result<()> {
        if let Some(consumer_names) = self.consumers.get(input_name) {
            for consumer_name in consumer_names.iter() {
                let consumer = Self::get_gate(&self.gates, consumer_name)?;
                if let Some(inputs) = consumer.get_inputs(&self.inputs, &self.gates) {
                    let consumer = Self::get_gate_mut(&mut self.gates, consumer_name)?;
                    consumer.compute(inputs);
                    initialized.push_back(consumer_name.clone());
                }
            }
        }
        Ok(())
    }
    fn simulate(&mut self) -> Result<()> {
        let mut initialized = VecDeque::new();
        let input_keys: Vec<String> = self.inputs.keys().cloned().collect();
        for input_name in input_keys {
            self.update_consumers(&input_name, &mut initialized)?;
        }
        while let Some(gate_name) = initialized.pop_front() {
            self.update_consumers(&gate_name, &mut initialized)?;
        }
        Ok(())
    }
    fn get_z(&self) -> Result<i128> {
        let mut z_gates: Vec<Gate> = self
            .gates
            .values()
            .filter(|&g| g.name.starts_with("z"))
            .cloned()
            .collect();
        z_gates.sort_by(|a, b| b.name.cmp(&a.name));
        let mut z_str = String::new();
        for z in z_gates.iter().map(|g| g.value) {
            let z = z.ok_or(anyhow!("all z gates must be initialized"))?;
            if z {
                z_str.push('1');
            } else {
                z_str.push('0');
            }
        }
        let z = i128::from_str_radix(&z_str, 2)?;
        Ok(z)
    }
}

type Input = Circuit;

fn parse_input(input: &str) -> Result<Input> {
    let (inputs_str, gates_str) = input
        .trim()
        .split_once("\n\n")
        .ok_or(anyhow!("there should be two section"))?;

    let mut inputs = HashMap::new();
    let mut gates = HashMap::new();
    let mut consumers: HashMap<String, Vec<String>> = HashMap::new();

    for line in inputs_str.lines() {
        let (name, value) = line
            .split_once(": ")
            .ok_or(anyhow!("invalid line {}", line))?;
        let value = parse_bool(&value)?;
        inputs.insert(String::from(name), value);
    }
    for line in gates_str.lines() {
        let gate = Gate::try_from(line)?;
        for gate_input in vec![gate.inputs.0.clone(), gate.inputs.1.clone()].into_iter() {
            consumers
                .entry(gate_input)
                .and_modify(|v| v.push(gate.name.clone()))
                .or_insert(vec![gate.name.clone()]);
        }
        gates.insert(gate.name.clone(), gate);
    }
    Ok(Circuit {
        inputs,
        gates,
        consumers,
    })
}

fn parse_bool(bool_str: &str) -> Result<bool> {
    if bool_str == "0" {
        Ok(false)
    } else if bool_str == "1" {
        Ok(true)
    } else {
        Err(anyhow!("Only 0 or 1 are parsed as bool (not {})", bool_str))
    }
}

fn solve_one(input: &Input) -> Result<Answer> {
    let mut circuit = input.clone();
    circuit.simulate()?;
    Ok(Answer::Num(circuit.get_z()?))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let _unused = input;
    Ok(Answer::Num(0))
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day24_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(2024));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(49430469426918));
        Ok(())
    }
    #[bench]
    fn part_one(b: &mut Bencher) {
        part_one_impl().expect("Error");
        b.iter(|| part_one_impl())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(0));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(0));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}
