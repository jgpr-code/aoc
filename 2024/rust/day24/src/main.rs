#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
use common::{regx, Answer};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    fmt, io,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
impl fmt::Display for GateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GateType::And => "AND",
                GateType::Or => "OR",
                GateType::Xor => "XOR",
            }
        )
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
    fn get_stage(&self) -> Option<i32> {
        let name_str = &self.name[1..3];
        if let Some(n) = i32::from_str_radix(name_str, 10).ok() {
            if name_str.starts_with("c") {
                return Some(n - 1);
            }
        }
        let a_str = &self.inputs.0[1..3];
        let b_str = &self.inputs.1[1..3];
        i32::from_str_radix(a_str, 10)
            .ok()
            .or(i32::from_str_radix(b_str, 10).ok())
    }
    fn rename(&mut self, gate_from: &str, gate_to: &str) {
        let rename_fn = |field: &mut String| {
            if *field == gate_from {
                *field = String::from(gate_to);
            }
        };
        rename_fn(&mut self.name);
        rename_fn(&mut self.inputs.0);
        rename_fn(&mut self.inputs.1);
    }
    fn rename_gates(gates: &mut [Gate], gate_from: &str, gate_to: &str) {
        gates.iter_mut().for_each(|g| g.rename(gate_from, gate_to));
    }
    fn is_swap(renaming: &(String, String)) -> bool {
        i32::from_str_radix(&renaming.0[1..3], 10).is_ok() || renaming.1.starts_with("z")
    }
    fn swap_gates(gates: &mut [Gate], gate_a: &str, gate_b: &str) {
        Self::rename_gates(gates, gate_a, "TEMP");
        Self::rename_gates(gates, gate_b, gate_a);
        Self::rename_gates(gates, "TEMP", gate_b);
    }
    fn get_renaming(&self) -> Option<(String, String)> {
        // N = 00
        // x00 XOR y00 -> z00 (present in input)
        // x00 AND y00 -> c01
        // N > 00
        // xN XOR yN -> XN
        // xN AND yN -> AN
        // XN XOR cN -> zN
        // XN AND cN -> BN
        // AN OR BN -> cN+1
        let mut new_name = None;
        if let Some(stage) = self.get_stage() {
            let def_str = &self.to_string().replace(&format!("{:02}", stage), "N")[..9];
            if stage == 0 {
                if def_str == "xN AND yN" {
                    new_name = Some(String::from("c01"));
                }
            } else {
                new_name = Some(match def_str {
                    "xN XOR yN" => format!("X{:02}", stage),
                    "xN AND yN" => format!("A{:02}", stage),
                    "XN XOR cN" => format!("z{:02}", stage),
                    "XN AND cN" => format!("B{:02}", stage),
                    "AN OR BN " => format!("c{:02}", stage + 1),
                    _ => return None,
                });
            }
        }
        let old_name = self.name.clone();
        return new_name.and_then(|new_name| {
            if new_name == old_name {
                None
            } else {
                Some((old_name, new_name))
            }
        });
    }
    fn is_ripple_adder_gate(&self) -> Result<bool> {
        // assuming this is using a carry-ripple full adder
        // 00 is special (only 2 gates instead of 5)
        // xN XOR yN -> zN (present in input)
        // xN AND yN -> cN1
        // N > 00
        // xN XOR yN -> XN
        // cN XOR XN -> zN
        // xN AND yN -> AN
        // XN AND cN -> BN
        // AN OR BN -> cN1
        let gate_str = self.to_string();
        let n_str = &self.inputs.0[1..];
        let n1_str = &self.name[1..];
        let n = i32::from_str_radix(&n_str, 10)?;
        let n1 = i32::from_str_radix(&n1_str, 10)?;
        match self.gate_type {
            GateType::And | GateType::Xor if n != 0 && n != n1 => return Ok(false),
            GateType::Or if n + 1 != n1 => return Ok(false),
            _ => {}
        }
        let gate_str = gate_str.replace(n_str, "N").replace(n1_str, "N1");
        Ok(if n == 0 {
            gate_str == "xN XOR yN -> zN" || gate_str == "xN AND yN -> cN1"
        } else {
            gate_str == "xN XOR yN -> XN"
                || gate_str == "XN XOR cN -> zN"
                || gate_str == "xN AND yN -> AN"
                || gate_str == "XN AND cN -> BN"
                || gate_str == "AN OR BN -> cN1"
        })
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
impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut inputs = vec![&self.inputs.0, &self.inputs.1];
        inputs.sort();
        write!(
            f,
            "{} {} {} -> {}",
            inputs[0], self.gate_type, inputs[1], self.name
        )
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
    // assuming this is using a carry-ripple full adder
    // 00 is special (only 2 gates instead of 5)
    // x00 XOR y00 -> z00 (present in input)
    // x00 AND y00 -> c01

    // N > 00
    // xN XOR yN -> XN
    // xN AND yN -> AN
    // XN XOR cN -> zN
    // XN AND cN -> BN
    // AN OR BN -> cN+1

    // total of 222 = (5 * 45) - 3 (because of 00 case)

    let mut gates: Vec<Gate> = input.gates.values().cloned().collect();
    let mut all_swaps: Vec<(String, String)> = Vec::new();
    loop {
        let (swaps, renamings): (Vec<_>, Vec<_>) = gates
            .iter()
            .filter_map(|g| g.get_renaming())
            .partition(|r| Gate::is_swap(r));
        if swaps.len() == 0 && renamings.len() == 0 {
            break;
        }
        let mut was_swapped = HashSet::new();
        for swap in swaps {
            let rswap = (swap.1.clone(), swap.0.clone());
            if was_swapped.contains(&swap) || was_swapped.contains(&rswap) {
                continue;
            }
            was_swapped.insert(swap.clone());
            was_swapped.insert(rswap);
            let (swap_a, swap_b) = swap;
            Gate::swap_gates(&mut gates, &swap_a, &swap_b);
            all_swaps.push((swap_a, swap_b))
        }
        for (gate_from, gate_to) in renamings {
            Gate::rename_gates(&mut gates, &gate_from, &gate_to);
        }
    }
    println!("{:?}", all_swaps);
    for gate in gates
        .iter()
        .filter(|g| !g.is_ripple_adder_gate().unwrap_or(false))
    {
        println!("{}", gate);
    }
    Ok(Answer::Num(-1))
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
    // #[test]
    // fn test_two() -> Result<()> {
    //     let answer = super::part_two(&TEST)?;
    //     assert_eq!(answer, Answer::Num(0));
    //     Ok(())
    // }
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
