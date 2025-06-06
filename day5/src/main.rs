use std::fs;

fn parse_program(input: &str) -> Vec<i64> {
    input.trim().split(',').map(|s| s.parse().unwrap()).collect()
}

fn run_program(mut program: Vec<i64>, input: i64) -> (Vec<i64>, Vec<i64>) {
    fn read(program: &mut Vec<i64>, idx: usize) -> i64 {
        if idx >= program.len() {
            program.resize(idx + 1, 0);
        }
        program[idx]
    }

    fn write(program: &mut Vec<i64>, idx: usize, val: i64) {
        if idx >= program.len() {
            program.resize(idx + 1, 0);
        }
        program[idx] = val;
    }

    fn get(program: &mut Vec<i64>, mode: i64, idx: usize) -> i64 {
        if mode == 0 {
            let target = read(program, idx) as usize;
            read(program, target)
        } else {
            read(program, idx)
        }
    }

    let mut ip: usize = 0;
    let mut outputs = Vec::new();
    loop {
        let opcode = read(&mut program, ip) % 100;
        let mode1 = (read(&mut program, ip) / 100) % 10;
        let mode2 = (read(&mut program, ip) / 1000) % 10;
        match opcode {
            1 | 2 | 7 | 8 => {
                let p1 = get(&mut program, mode1, ip + 1);
                let p2 = get(&mut program, mode2, ip + 2);
                let dest = read(&mut program, ip + 3) as usize;
                let result = match opcode {
                    1 => p1 + p2,
                    2 => p1 * p2,
                    7 => if p1 < p2 { 1 } else { 0 },
                    8 => if p1 == p2 { 1 } else { 0 },
                    _ => unreachable!(),
                };
                write(&mut program, dest, result);
                ip += 4;
            }
            3 => {
                let dest = read(&mut program, ip + 1) as usize;
                write(&mut program, dest, input);
                ip += 2;
            }
            4 => {
                let val = get(&mut program, mode1, ip + 1);
                outputs.push(val);
                ip += 2;
            }
            5 | 6 => {
                let p1 = get(&mut program, mode1, ip + 1);
                let p2 = get(&mut program, mode2, ip + 2);
                if (opcode == 5 && p1 != 0) || (opcode == 6 && p1 == 0) {
                    ip = p2 as usize;
                } else {
                    ip += 3;
                }
            }
            99 => break,
            _ => panic!("Unknown opcode {} at position {}", opcode, ip),
        }
    }
    (program, outputs)
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Can't read file");
    let program = parse_program(&input);

    let (_mem, out1) = run_program(program.clone(), 1);
    println!("Part 1: {:?}", out1.last().unwrap());

    let (_mem, out2) = run_program(program, 5);
    println!("Part 2: {:?}", out2.last().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo() {
        let program = vec![3,0,4,0,99];
        let (_mem, output) = run_program(program, 42);
        assert_eq!(output, vec![42]);
    }

    #[test]
    fn test_position_and_immediate() {
        let prog = vec![1002,4,3,4,33];
        let (mem, output) = run_program(prog, 0);
        assert!(output.is_empty());
        assert_eq!(mem[4], 99);
    }

    #[test]
    fn test_equals_position() {
        let prog = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let (_, out) = run_program(prog.clone(), 8);
        assert_eq!(out, vec![1]);
        let (_, out) = run_program(prog, 5);
        assert_eq!(out, vec![0]);
    }
}
