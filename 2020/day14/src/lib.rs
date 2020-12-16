use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct OperationSet {
    pub mask_string: String,
    pub addrs: Vec<usize>,
    pub values: Vec<usize>,
}

impl OperationSet {
    pub fn new(mask_string: String, addrs: Vec<usize>, values: Vec<usize>) -> Self {
        OperationSet {
            mask_string,
            addrs,
            values,
        }
    }

    pub fn get_mask(&self, target: char) -> usize {
        let mut output = 0;
        for (i, v) in self.mask_string.chars().rev().enumerate() {
            if v == target {
                output |= 2_usize.pow(i.try_into().unwrap());
            }
        }
        output
    }
}

// Evaluates the commands using the V1 protocol
pub fn evaluate_v1(opset: Vec<OperationSet>) -> HashMap<usize, usize> {
    let mut data = HashMap::new();
    for (num, ops) in opset.iter().enumerate() {
        println!("Solving set: {}", num);

        assert_eq!(ops.addrs.len(), ops.values.len());

        let on_mask = ops.get_mask('1');
        let off_mask = ops.get_mask('0');
        for i in 0..ops.addrs.len() {
            let addr = ops.addrs[i];
            let mut value = ops.values[i];
            value |= on_mask;
            value &= !off_mask;

            let e = data.entry(addr).or_insert(value);
            *e = value;
        }
    }
    data
}

// Evaluates the commands using the V2 protocol
pub fn evaluate_v2(opset: Vec<OperationSet>) -> HashMap<usize, usize> {
    let mut data = HashMap::with_capacity(opset.len());
    for (num, ops) in opset.iter().enumerate() {
        assert_eq!(ops.addrs.len(), ops.values.len());
        println!("Solving set: {}", num);

        let on_mask = ops.get_mask('1');
        let floating_mask = ops.get_mask('X');
        // let masks = get_floating_combinations(floating_mask);
        let masks = get_floating_combinations_v2(&ops.mask_string);
        for i in 0..ops.addrs.len() {
            let addr = ops.addrs[i] | on_mask;
            let value = ops.values[i];

            // for mask in &masks {
            //     println!("using the floating_mask: {}", floating_mask);
            //     let local_addr = addr + mask - floating_mask + 1;
            //     data.insert(local_addr, value);
            // }
        }
    }
    data
}

pub fn get_floating_combinations_v2(input: &str) -> Vec<usize> {
    let mut masks = vec![0];

    for (i, c) in input.chars().rev().enumerate() {
        if c == 'X' {
            let mut new_set = Vec::new();
            for m in &masks {
                new_set.push(m | 2_usize.pow(i.try_into().unwrap()));
            }
            masks.append(&mut new_set);
        }
    }
    masks
}

pub fn read_input(filename: &str) -> Vec<String> {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();

    contents
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

// VEERRY ugly parser!
pub fn input_to_opset(input: &Vec<String>) -> Vec<OperationSet> {
    let mut collection = Vec::new();

    let mut tmp_mask = String::new();
    let mut tmp_addrs = Vec::new();
    let mut tmp_vars = Vec::new();

    let mut first_run = true;
    for line in input {
        let line = line.replace("mask = ", "");
        let line = line.replace("mem[", "");
        let line = line.replace("] =", "");

        if line.len() == 36 {
            // Handle a new mask
            if !first_run {
                collection.push(OperationSet::new(
                    tmp_mask.to_string(),
                    tmp_addrs.clone(),
                    tmp_vars.clone(),
                ));
            }
            tmp_mask = line;
            tmp_vars = Vec::new();
            tmp_addrs = Vec::new();

            first_run = false;
        } else {
            // And subsequent operations with that mask
            let vars = line.split(" ").collect::<Vec<&str>>();
            tmp_vars.push(vars[1].parse().unwrap());
            tmp_addrs.push(vars[0].parse().unwrap());
        }
    }
    //Special handling for final element
    collection.push(OperationSet::new(
        tmp_mask.to_string(),
        tmp_addrs.clone(),
        tmp_vars.clone(),
    ));

    collection
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = read_input("example1.txt");

        let opset = input_to_opset(&input);
        let data = evaluate_v1(opset);

        let ans: usize = data.iter().fold(0, |acc, (_k, v)| acc + v);
        assert_eq!(ans, 165)
    }

    #[test]
    fn part_1_solution() {
        let input = read_input("input.txt");

        let opset = input_to_opset(&input);
        let data = evaluate_v1(opset);

        let ans: usize = data.iter().fold(0, |acc, (_k, v)| acc + v);
        assert_eq!(ans, 15172047086292)
    }

    #[test]
    fn part_2_example() {
        let input = read_input("example2.txt");

        let opset = input_to_opset(&input);
        let data = evaluate_v2(opset);
        dbg!(&data);
        let ans: usize = data.iter().fold(0, |acc, (_k, v)| acc + v);
        assert_eq!(ans, 208)
    }

    #[test]
    fn part_2_solution() {
        let input = read_input("input.txt");

        let opset = input_to_opset(&input);
        let data = evaluate_v2(opset);

        let ans: usize = data.iter().fold(0, |acc, (_k, v)| acc + v);
        assert_eq!(ans, 4197941339968)
        // Guessed 4222502079458 (too high)
    }

    #[test]
    fn get_floating_test() {
        // let vals =get_floating_combinations_v2("X0X0X0");
        // for v in &vals{
        //     println!("{}: ({:b})", v,v);
        // }

        let vals = get_floating_combinations_v2("0X0010X11100000010011011111010101XX1");
        for v in &vals {
            println!("{}: ({:b})", v, v);
        }
    }
}
