#[cfg(test)]
mod tests {

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    fn calculate(input: &[i32]) -> Vec<i32> {
        let mut output = input.clone().to_vec();
        let mut pos = 0;
        loop {
            match output[pos] {
                1 => {
                    let output_index = output[pos + 3] as usize;
                    let a_index = output[pos + 1] as usize;
                    let b_index = output[pos + 2] as usize;

                    output[output_index] = add(output[a_index], output[b_index])
                }
                2 => {
                    let output_index = output[pos + 3] as usize;
                    let a_index = output[pos + 1] as usize;
                    let b_index = output[pos + 2] as usize;

                    output[output_index] = multiply(output[a_index], output[b_index])
                }
                99 => break,
                _ => panic!("Invalid opcode"),
            }
            pos += 4;
        }

        output
    }
    #[test]
    fn examples1() {
        assert_eq!(
            calculate(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );

        assert_eq!(calculate(&[1, 0, 0, 0, 99]), [2, 0, 0, 0, 99]);

        assert_eq!(calculate(&[2, 3, 0, 3, 99]), [2, 3, 0, 6, 99]);
        assert_eq!(calculate(&[2, 4, 4, 5, 99, 0]), [2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            calculate(&[1, 1, 1, 4, 99, 5, 6, 0, 99]),
            [30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }

    #[test]
    fn part_1() {
        let mut program = [
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 9, 19, 1, 5, 19, 23, 1, 6, 23,
            27, 1, 27, 10, 31, 1, 31, 5, 35, 2, 10, 35, 39, 1, 9, 39, 43, 1, 43, 5, 47, 1, 47, 6,
            51, 2, 51, 6, 55, 1, 13, 55, 59, 2, 6, 59, 63, 1, 63, 5, 67, 2, 10, 67, 71, 1, 9, 71,
            75, 1, 75, 13, 79, 1, 10, 79, 83, 2, 83, 13, 87, 1, 87, 6, 91, 1, 5, 91, 95, 2, 95, 9,
            99, 1, 5, 99, 103, 1, 103, 6, 107, 2, 107, 13, 111, 1, 111, 10, 115, 2, 10, 115, 119,
            1, 9, 119, 123, 1, 123, 9, 127, 1, 13, 127, 131, 2, 10, 131, 135, 1, 135, 5, 139, 1, 2,
            139, 143, 1, 143, 5, 0, 99, 2, 0, 14, 0,
        ];
        program[1] = 12;
        program[2] = 2;

        let result = calculate(&program);
        println!("Position 0:{}", result[0]);
        assert_eq!(result[0], 3562672);
    }

    #[test]
    fn part_2() {
        let program = [
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 9, 19, 1, 5, 19, 23, 1, 6, 23,
            27, 1, 27, 10, 31, 1, 31, 5, 35, 2, 10, 35, 39, 1, 9, 39, 43, 1, 43, 5, 47, 1, 47, 6,
            51, 2, 51, 6, 55, 1, 13, 55, 59, 2, 6, 59, 63, 1, 63, 5, 67, 2, 10, 67, 71, 1, 9, 71,
            75, 1, 75, 13, 79, 1, 10, 79, 83, 2, 83, 13, 87, 1, 87, 6, 91, 1, 5, 91, 95, 2, 95, 9,
            99, 1, 5, 99, 103, 1, 103, 6, 107, 2, 107, 13, 111, 1, 111, 10, 115, 2, 10, 115, 119,
            1, 9, 119, 123, 1, 123, 9, 127, 1, 13, 127, 131, 2, 10, 131, 135, 1, 135, 5, 139, 1, 2,
            139, 143, 1, 143, 5, 0, 99, 2, 0, 14, 0,
        ];

        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut test_program = program.clone();
                test_program[1] = noun;
                test_program[2] = verb;
                let result = calculate(&test_program);

                if result[0] == 19690720 {
                    println!("Noun:{}\nVerb:{}", noun, verb);
                    println!("100*noun + verb == {}", 100 * noun + verb);
                    return;
                }
            }
        }
    }
}
