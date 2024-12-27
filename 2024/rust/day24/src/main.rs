#![feature(test)]
extern crate test;

use anyhow::{anyhow, Result};
use common::{regx, Answer};
use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
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
    fn gate_stage(&self) -> Option<i32> {
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

/// applies new_names and returns (old_name, new_name) required additional renaming
fn rename_gate(gate: &mut Gate, new_names: &HashMap<String, String>) -> Option<(String, String)> {
    if let Some(new_name) = new_names.get(&gate.inputs.0) {
        gate.inputs.0 = new_name.clone();
    }
    if let Some(new_name) = new_names.get(&gate.inputs.1) {
        gate.inputs.1 = new_name.clone();
    }
    if let Some(new_name) = new_names.get(&gate.name) {
        gate.name = new_name.clone();
    }
    if gate.name.starts_with("z") {
        if gate.inputs.0.starts_with("X") {
            let n = i32::from_str_radix(&gate.inputs.0[1..3], 10).expect("must be XN");
            let new_name = format!("c{:02}", n);
            if new_name != gate.inputs.1 {
                return Some((gate.inputs.1.clone(), new_name));
            }
        }
        if gate.inputs.1.starts_with("X") {
            let n = i32::from_str_radix(&gate.inputs.1[1..3], 10).expect("must be XN");
            let new_name = format!("c{:02}", n);
            if new_name != gate.inputs.0 {
                return Some((gate.inputs.0.clone(), new_name));
            }
        }
    }
    if gate.name.starts_with("B") {
        if gate.inputs.0.starts_with("X") {
            let n = i32::from_str_radix(&gate.inputs.0[1..3], 10).expect("must be XN");
            let new_name = format!("c{:02}", n);
            if new_name != gate.inputs.1 {
                return Some((gate.inputs.1.clone(), new_name));
            }
        }
        if gate.inputs.1.starts_with("X") {
            let n = i32::from_str_radix(&gate.inputs.1[1..3], 10).expect("must be XN");
            let new_name = format!("c{:02}", n);
            if new_name != gate.inputs.0 {
                return Some((gate.inputs.0.clone(), new_name));
            }
        }
    }
    let mut gate_str = gate.to_string();
    let n_str = &gate_str[1..3];
    if let Some(n) = i32::from_str_radix(&n_str, 10).ok() {
        gate_str = gate_str.replace(n_str, "N");
        let def_str = &gate_str[..9];
        if n == 0 {
            if def_str == "xN AND yN" {
                return Some((gate.name.clone(), String::from("c01")));
            } else {
                return None;
            }
        } else {
            let new_name = match def_str {
                "xN XOR yN" => format!("X{:02}", n),
                "xN AND yN" => format!("A{:02}", n),
                // "XN XOR cN" => format!("z{:02}", n),
                "XN AND cN" => format!("B{:02}", n),
                "AN OR BN " => format!("c{:02}", n + 1),
                _ => return None,
            };
            return Some((gate.name.clone(), new_name));
        }
    } else {
        return None;
    }
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

    // algo outline: rename things until all makes sense, then check for inconsistent gates
    let mut old_names = HashMap::new();
    let mut new_names = HashMap::new();
    let mut gates: Vec<Gate> = input.gates.values().cloned().collect();
    loop {
        println!("processing");
        let renamings: Vec<(String, String)> = gates
            .iter_mut()
            .map(|gate| rename_gate(gate, &new_names))
            .filter(|mapping| mapping.is_some())
            .map(|mapping| mapping.unwrap())
            .collect();
        if renamings.iter().filter(|(a, b)| a != b).count() == 0 {
            break;
        }
        for (old_name, new_name) in renamings {
            println!("{} -> {}", old_name, new_name);
            if old_name == new_name {
                continue;
            }
            old_names.insert(new_name.clone(), old_name.clone());
            new_names.insert(old_name, new_name);
        }
    }

    gates.sort_by(|a, b| {
        let a_str = a.to_string();
        let an = i32::from_str_radix(&a_str[1..3], 10).unwrap_or(100);
        let b_str = b.to_string();
        let bn = i32::from_str_radix(&b_str[1..3], 10).unwrap_or(100);
        let cmp = an.cmp(&bn);
        if cmp == Ordering::Equal {
            b_str.cmp(&a_str)
        } else {
            cmp
        }
    });
    for gate in gates.iter() {
        println!("{}", gate);
    }
    let mut answer = Vec::new();
    for gate in gates
        .iter()
        .filter(|gate| !gate.is_ripple_adder_gate().unwrap_or(false))
        .map(|g| {
            let mut oldest_name = g.name.clone();
            while let Some(older_name) = old_names.get(&oldest_name) {
                oldest_name = older_name.clone();
                let c = &oldest_name[0..1];
                // match c {
                //     "A" | "B" | "X" => {}
                //     "c" if i32::from_str_radix(&oldest_name[1..3], 10).is_ok() => {}
                //     _ => break,
                // }
                println!("oldest_name {}", oldest_name);
            }
            return oldest_name;
        })
    {
        answer.push(gate);
    }
    answer.sort();
    Ok(Answer::Str(answer.join(",")))
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
