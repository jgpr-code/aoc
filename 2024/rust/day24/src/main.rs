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

#[derive(Clone, Copy)]
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

#[derive(Clone)]
struct Gate {
    name: String,
    inputs: GateInputs,
    gate_type: GateType,
    value: Option<bool>,
}
impl Gate {
    fn try_compute(&mut self) -> Result<()> {
        let mut iter = self.inputs.0.values();
        let a = iter
            .next()
            .ok_or(anyhow!("inputs must be of size 2"))?
            .ok_or(anyhow!("value must be set"))?;
        let b = iter
            .next()
            .ok_or(anyhow!("inputs must be of size 2"))?
            .ok_or(anyhow!("value must be set"))?;
        self.value = Some(match self.gate_type {
            GateType::And => a && b,
            GateType::Or => a || b,
            GateType::Xor => a ^ b,
        });
        Ok(())
    }
}

#[derive(Clone)]
struct GateInputs(HashMap<String, Option<bool>>);
impl GateInputs {
    fn fully_initialized(&self) -> bool {
        assert!(self.0.len() == 2);
        self.0.values().all(|v| v.is_some())
    }
    fn get_input_mut(&mut self, name: &str) -> Result<&mut Option<bool>> {
        Ok(self
            .0
            .get_mut(name)
            .ok_or(anyhow!("gate input {} must be present", name))?)
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
        let inputs = HashMap::from([
            (String::from(&caps[1]), None),
            (String::from(&caps[3]), None),
        ]);
        let gate_type = GateType::try_from(&caps[2])?;
        Ok(Gate {
            name,
            inputs: GateInputs(inputs),
            gate_type,
            value: None,
        })
    }
}

#[derive(Clone)]
struct Circuit {
    gate_values: HashMap<String, Option<bool>>,
    inputs: HashMap<String, bool>,
    gates: HashMap<String, Gate>,             // foo -> Gate foo
    gate_graph: HashMap<String, Vec<String>>, // foo -> all gates with foo as input
}

impl Circuit {
    fn get_gate_mut<'a>(
        gates: &'a mut HashMap<String, Gate>,
        gate_name: &str,
    ) -> Result<&'a mut Gate> {
        Ok(gates
            .get_mut(gate_name)
            .ok_or(anyhow!("gate {} must be present", gate_name))?)
    }
    fn simulate(&mut self) -> Result<()> {
        let mut fully_initialized_gates = VecDeque::new();
        for (input_name, input_value) in self.inputs.iter() {
            if let Some(has_input) = self.gate_graph.get(input_name) {
                for gate_name in has_input.iter() {
                    let gate = Self::get_gate_mut(&mut self.gates, gate_name)?;
                    *gate.inputs.get_input_mut(input_name)? = Some(*input_value);
                    if gate.inputs.fully_initialized() {
                        gate.try_compute()?;
                        fully_initialized_gates.push_back((
                            gate_name.clone(),
                            gate.value.ok_or(anyhow!("gate must be initialized"))?,
                        ));
                    }
                }
            }
        }
        while let Some((initialized_gate_name, initialized_value)) =
            fully_initialized_gates.pop_front()
        {
            if let Some(has_input) = self.gate_graph.get(&initialized_gate_name) {
                for gate_name in has_input.iter() {
                    let gate = Self::get_gate_mut(&mut self.gates, gate_name)?;
                    *gate.inputs.get_input_mut(&gate_name)? = Some(initialized_value);
                    if gate.inputs.fully_initialized() {
                        gate.try_compute()?;
                        fully_initialized_gates.push_back(gate_name.clone());
                    }
                }
            }
        }
        Ok(())
    }
    fn get(pre: char) -> i128 {}
}

type Input = Circuit;

fn parse_input(input: &str) -> Result<Input> {
    let (inputs_str, gates_str) = input
        .trim()
        .split_once("\n\n")
        .ok_or(anyhow!("there should be two section"))?;

    let mut inputs = HashMap::new();
    let mut gates = HashMap::new();
    let mut gate_graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in inputs_str.lines() {
        let (name, value) = line
            .split_once(": ")
            .ok_or(anyhow!("invalid line {}", line))?;
        let value = parse_bool(&value)?;
        inputs.insert(String::from(name), value);
    }
    for line in gates_str.lines() {
        let gate = Gate::try_from(line)?;
        for gate_input in gate.inputs.0.keys().cloned() {
            gate_graph
                .entry(gate_input)
                .and_modify(|v| v.push(gate.name.clone()))
                .or_insert(vec![gate.name.clone()]);
        }
        gates.insert(gate.name.clone(), gate);
    }
    Ok(Input {
        inputs,
        gates,
        gate_graph,
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
    let x = circuit.get('x')?;
    let y = circuit.get('y')?;
    let z = circuit.get('z')?;
    Ok(Answer::Num(-1))
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
        assert_eq!(answer, Answer::Num(0));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(0));
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
