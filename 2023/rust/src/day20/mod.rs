use std::collections::{HashMap, VecDeque};

use super::common::*;
use anyhow::Result;

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

#[derive(Debug, Clone)]
enum Module {
    Flip(FlipFlop),
    Conj(Conjunction),
    Broad(Broadcast),
}
impl Module {
    fn handle_pulse(&mut self, pulse: Pulse) -> Vec<(String, Pulse)> {
        match self {
            Module::Flip(flipflop) => flipflop.handle_pulse(pulse),
            Module::Conj(conjunction) => conjunction.handle_pulse(pulse),
            Module::Broad(broadcast) => broadcast.handle_pulse(pulse),
        }
    }
    fn handle_pulse_iter(&mut self, pulse: Pulse, i: i128) -> Vec<(String, Pulse)> {
        match self {
            Module::Flip(flipflop) => flipflop.handle_pulse(pulse),
            Module::Conj(conjunction) => conjunction.handle_pulse_iter(pulse, i),
            Module::Broad(broadcast) => broadcast.handle_pulse(pulse),
        }
    }
    fn get_outputs(&self) -> Vec<String> {
        match self {
            Module::Flip(flipflop) => flipflop.get_outputs(),
            Module::Conj(conjunction) => conjunction.get_outputs(),
            Module::Broad(broadcast) => broadcast.get_outputs(),
        }
    }
}

#[derive(Debug, Clone)]
enum Pulse {
    Low(String),
    High(String),
}

// % flip flop module (on/off start off) low -> flips and sends pulse, high -> ignore
#[derive(Debug, Clone)]
struct FlipFlop {
    name: String,
    on: bool,
    outputs: Vec<String>,
}
impl FlipFlop {
    fn new(name: String, outputs: Vec<String>) -> Self {
        FlipFlop {
            name,
            on: false,
            outputs,
        }
    }
    fn handle_pulse(&mut self, pulse: Pulse) -> Vec<(String, Pulse)> {
        match pulse {
            Pulse::High(_) => vec![],
            Pulse::Low(_) => {
                self.on = !self.on;
                if self.on {
                    self.outputs
                        .iter()
                        .map(|s| (s.clone(), Pulse::High(self.name.clone())))
                        .collect()
                } else {
                    self.outputs
                        .iter()
                        .map(|s| (s.clone(), Pulse::Low(self.name.clone())))
                        .collect()
                }
            }
        }
    }
    fn get_outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }
}

// & conjunction remember most recent pulse for each connected (init low) if all high -> send low else send high
#[derive(Debug, Clone)]
struct Conjunction {
    name: String,
    memory: HashMap<String, Pulse>,
    outputs: Vec<String>,
}
impl Conjunction {
    fn new(name: String, outputs: Vec<String>) -> Self {
        Conjunction {
            name,
            memory: HashMap::new(),
            outputs,
        }
    }
    fn handle_pulse(&mut self, pulse: Pulse) -> Vec<(String, Pulse)> {
        match &pulse {
            Pulse::Low(sender) => {
                let mem = self.memory.get_mut(sender).unwrap();
                *mem = pulse.clone();
            }
            Pulse::High(sender) => {
                let mem = self.memory.get_mut(sender).unwrap();
                *mem = pulse.clone();
            }
        }
        let mut send_pulse = Pulse::Low(self.name.clone());
        for v in self.memory.values() {
            match v {
                &Pulse::Low(_) => {
                    send_pulse = Pulse::High(self.name.clone());
                    break;
                }
                _ => {}
            }
        }
        // let of_interest = vec!["pn", "jg", "qx", "zt"]; // specific to my input
        // if of_interest.contains(&self.name.as_str()) {
        //     match &send_pulse {
        //         Pulse::Low(name) => {
        //             println!("{} send low", name)
        //         }
        //         _ => {}
        //     }
        // }
        self.outputs
            .iter()
            .map(|s| (s.clone(), send_pulse.clone()))
            .collect()
    }
    fn handle_pulse_iter(&mut self, pulse: Pulse, i: i128) -> Vec<(String, Pulse)> {
        match &pulse {
            Pulse::Low(sender) => {
                let mem = self.memory.get_mut(sender).unwrap();
                *mem = pulse.clone();
            }
            Pulse::High(sender) => {
                let mem = self.memory.get_mut(sender).unwrap();
                *mem = pulse.clone();
            }
        }
        let mut send_pulse = Pulse::Low(self.name.clone());
        for v in self.memory.values() {
            match v {
                &Pulse::Low(_) => {
                    send_pulse = Pulse::High(self.name.clone());
                    break;
                }
                _ => {}
            }
        }
        self.check_send(i);
        // let of_interest = vec!["pn", "jg", "qx", "zt"]; // specific to my input
        // if of_interest.contains(&self.name.as_str()) {
        //     match &send_pulse {
        //         Pulse::Low(name) => {
        //             println!("{} send low", name)
        //         }
        //         _ => {}
        //     }
        // }
        self.outputs
            .iter()
            .map(|s| (s.clone(), send_pulse.clone()))
            .collect()
    }
    fn get_outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }
    fn add_memory(&mut self, from: String) {
        self.memory.insert(from.clone(), Pulse::Low(from.clone()));
    }
    fn print_memory(&self) {
        let of_interest = vec!["pn", "jg", "qx", "zt"]; // specific to my input
        if !of_interest.contains(&self.name.as_str()) {
            return;
        }
        print!("&{}: ", self.name);
        for (name, pulse) in self.memory.iter() {
            let pulse_val = match pulse {
                Pulse::Low(_) => 0,
                Pulse::High(_) => 1,
            };
            print!("{}:{}; ", name, pulse_val);
        }
        println!();
    }
    fn check_send(&self, i: i128) {
        let of_interest = vec!["pn", "jg", "qx", "zt"]; // specific to my input
        if !of_interest.contains(&self.name.as_str()) {
            return;
        }
        for v in self.memory.values() {
            match v {
                &Pulse::Low(_) => {
                    return;
                }
                _ => {}
            }
        }
        println!("{} all set on iter {}", self.name, i);
    }
}

// broadcaster sends received pulse to all modules
#[derive(Debug, Clone)]
struct Broadcast {
    name: String,
    outputs: Vec<String>,
}
impl Broadcast {
    fn new(name: String, outputs: Vec<String>) -> Self {
        Broadcast { name, outputs }
    }
    fn handle_pulse(&mut self, pulse: Pulse) -> Vec<(String, Pulse)> {
        match pulse {
            Pulse::Low(_) => self
                .outputs
                .iter()
                .map(|s| (s.clone(), Pulse::Low(self.name.clone())))
                .collect(),
            Pulse::High(_) => self
                .outputs
                .iter()
                .map(|s| (s.clone(), Pulse::High(self.name.clone())))
                .collect(),
        }
    }
    fn get_outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }
}

struct Input {
    modules: HashMap<String, Module>,
}

fn parse_input(input: &str) -> Result<Input> {
    let mut modules = HashMap::new();
    let mut conjunctions = Vec::new();
    for line in input.lines() {
        let (a, b) = line.split_once("->").unwrap();
        let mut targets = Vec::new();
        for t in b.trim().split(",") {
            targets.push(String::from(t.trim()));
        }
        let a = a.trim();
        if a == "broadcaster" {
            let broadcaster = Broadcast::new(String::from("broadcaster"), targets.clone());
            modules.insert(String::from("broadcaster"), Module::Broad(broadcaster));
        } else {
            let (c, name) = a.split_at(1);
            let c = c.chars().next().unwrap();
            if c == '%' {
                let flipflop = FlipFlop::new(String::from(name), targets.clone());
                modules.insert(String::from(name), Module::Flip(flipflop));
            } else if c == '&' {
                let conjunction = Conjunction::new(String::from(name), targets.clone());
                conjunctions.push(String::from(name));
                modules.insert(String::from(name), Module::Conj(conjunction));
            } else {
                panic!("unexpected module type");
            }
        }
    }
    let modules_clone = modules.clone();
    for (k, v) in modules_clone {
        let outputs = v.get_outputs();
        for output in outputs {
            if let Some(module) = modules.get_mut(&output) {
                match module {
                    Module::Conj(conjunction) => conjunction.add_memory(k.clone()),
                    _ => {}
                }
            } else {
                println!("untyped module {}", output);
            }
        }
    }
    Ok(Input { modules })
}

// button module send low pulse to broadcaster (implicit)
fn push_button(modules: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut pulses_to_handle = VecDeque::new();
    let mut low = 0;
    let mut high = 0;
    pulses_to_handle.push_back((
        String::from("broadcaster"),
        Pulse::Low(String::from("button")),
    ));
    while let Some((receiver, pulse)) = pulses_to_handle.pop_front() {
        match pulse {
            Pulse::Low(_) => low += 1,
            Pulse::High(_) => high += 1,
        };
        if let Some(receiving_module) = modules.get_mut(&receiver) {
            let next_pulses = receiving_module.handle_pulse(pulse);
            for np in next_pulses {
                pulses_to_handle.push_back(np);
            }
        } else {
            // println!("untyped receiver {}", receiver);
        }
    }
    (low, high)
}

fn push_button_rx(modules: &mut HashMap<String, Module>, i: i128) -> bool {
    let mut pulses_to_handle = VecDeque::new();
    pulses_to_handle.push_back((
        String::from("broadcaster"),
        Pulse::Low(String::from("button")),
    ));
    while let Some((receiver, pulse)) = pulses_to_handle.pop_front() {
        match pulse {
            Pulse::Low(_) => {
                if receiver == "rx" {
                    return true;
                }
            }
            _ => {}
        }
        if let Some(receiving_module) = modules.get_mut(&receiver) {
            let next_pulses = receiving_module.handle_pulse_iter(pulse, i);
            for np in next_pulses {
                pulses_to_handle.push_back(np);
            }
        } else {
            // println!("untyped receiver {}", receiver);
        }
    }
    return false;
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { modules } = input;
    let mut modules = modules.clone();
    let mut total_low = 0;
    let mut total_high = 0;
    let (low, high) = push_button(&mut modules);
    total_low += low;
    total_high += high;
    println!("low={}, high={}", low, high);
    for _ in 0..999 {
        let (low, high) = push_button(&mut modules);
        total_low += low;
        total_high += high;
    }
    println!("tlow={}, thigh={}", total_low, total_high);
    Ok(Answer::Num((total_low * total_high) as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { modules } = input;
    let mut modules = modules.clone();
    let mut i = 0;
    loop {
        i += 1;
        // println!("{}", i);
        if push_button_rx(&mut modules, i) {
            break;
        }
        for v in modules.values() {
            match &v {
                Module::Conj(conjunction) => {
                    //conjunction.print_memory();
                    conjunction.check_send(i); // shows that lcm of 3847, 3923, 4001, 4091 = 247023644760071 is right!
                }
                _ => {}
            }
        }
        if i % 1000000 == 0 {
            println!("{}", i);
        }
        // println!("------------------------------------------------------");
    }
    Ok(Answer::Num(i))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test.txt");
    static INPUT: LazyLock<String> = local_file!("input.txt");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(32000000));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(861743850));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(-1));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(-1));
        Ok(())
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        b.iter(|| part_one())
    }
    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        b.iter(|| part_two())
    }
}
