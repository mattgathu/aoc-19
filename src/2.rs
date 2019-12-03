// --- Day 2: 1202 Program Alarm ---
//
// https://adventofcode.com/2019/day/2
//

fn run_intcode(mut memory: Vec<usize>) -> Vec<usize> {
    let mut ip = 0;
    loop {
        if memory[ip] == 1 {
            let lreg = memory[ip + 1];
            let rreg = memory[ip + 2];
            let dst = memory[ip + 3];
            memory[dst] = memory[lreg] + memory[rreg];
        } else if memory[ip] == 2 {
            let lreg = memory[ip + 1];
            let rreg = memory[ip + 2];
            let dst = memory[ip + 3];
            memory[dst] = memory[lreg] * memory[rreg];
        } else {
            break;
        }

        ip += 4;
    }

    memory
}

fn main() {
    let mut input = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 9, 19, 23, 1, 13, 23, 27,
        1, 5, 27, 31, 2, 31, 6, 35, 1, 35, 5, 39, 1, 9, 39, 43, 1, 43, 5, 47, 1, 47, 5, 51, 2, 10,
        51, 55, 1, 5, 55, 59, 1, 59, 5, 63, 2, 63, 9, 67, 1, 67, 5, 71, 2, 9, 71, 75, 1, 75, 5, 79,
        1, 10, 79, 83, 1, 83, 10, 87, 1, 10, 87, 91, 1, 6, 91, 95, 2, 95, 6, 99, 2, 99, 9, 103, 1,
        103, 6, 107, 1, 13, 107, 111, 1, 13, 111, 115, 2, 115, 9, 119, 1, 119, 6, 123, 2, 9, 123,
        127, 1, 127, 5, 131, 1, 131, 5, 135, 1, 135, 5, 139, 2, 10, 139, 143, 2, 143, 10, 147, 1,
        147, 5, 151, 1, 151, 2, 155, 1, 155, 13, 0, 99, 2, 14, 0, 0,
    ];

    // restore gravity assist program
    input[1] = 12;
    input[2] = 2;

    println!("Part One {:?}", run_intcode(input.clone())[0]);

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut mem = input.clone();
            mem[1] = noun;
            mem[2] = verb;
            if run_intcode(mem)[0] == 19690720 {
                println!("Part Two {:?}", 100 * noun + verb);
                break;
            }
        }
    }
}

#[test]
fn test_run_intcode() {
    assert_eq!(run_intcode(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
    assert_eq!(run_intcode(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
    assert_eq!(
        run_intcode(vec![2, 4, 4, 5, 99, 0]),
        vec![2, 4, 4, 5, 99, 9801]
    );
    assert_eq!(
        run_intcode(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    );
}
