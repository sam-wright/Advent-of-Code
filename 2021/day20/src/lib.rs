use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input(filename: &str) -> (String, HashMap<(i32, i32), char>) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut decoder = String::new();
    let mut pixels = HashMap::new();

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if y == 0 {
            decoder = line.clone();
        } else if y == 1 {
            continue;
        } else {
            for (x, c) in line.chars().enumerate() {
                pixels.insert((x as i32, y as i32 - 2), c);
            }
        }
    }

    (decoder, pixels)
}

pub fn get_limits(pixels: &HashMap<(i32, i32), char>) -> (i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;

    for (x, y) in pixels.keys() {
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }

    (max_x, max_y)
}

fn get_pixel(pixels: &HashMap<(i32, i32), char>, x: i32, y: i32) -> char {
    if pixels.contains_key(&(x, y)) {
        *pixels.get(&(x, y)).unwrap()
    } else {
        '.'
    }
}

fn get_decoder_word(pixels: &HashMap<(i32, i32), char>, x: i32, y: i32) -> String {
    let mut word = String::new();
    word.push(get_pixel(pixels, x - 1, y - 1));
    word.push(get_pixel(pixels, x, y - 1));
    word.push(get_pixel(pixels, x + 1, y - 1));
    word.push(get_pixel(pixels, x - 1, y));
    word.push(get_pixel(pixels, x, y));
    word.push(get_pixel(pixels, x + 1, y));
    word.push(get_pixel(pixels, x - 1, y + 1));
    word.push(get_pixel(pixels, x, y + 1));
    word.push(get_pixel(pixels, x + 1, y + 1));
    word
}

pub fn decode(decoder_word: &str, decoder: &str) -> char {
    let index = decoder_word
        .chars()
        .rev()
        .enumerate()
        .fold(0, |index, (i, c)| {
            if c == '#' {
                index + 2u32.pow(i as u32)
            } else {
                index
            }
        });

    decoder.chars().nth(index as usize).unwrap()
}

pub fn enhance(
    pixels: &HashMap<(i32, i32), char>,
    decoder: &str,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) -> HashMap<(i32, i32), char> {
    let mut new_pixels = HashMap::new();
    for x in min_x..max_x {
        for y in min_y..max_y {
            let decoder_word = get_decoder_word(pixels, x, y);
            let decoded_pixel = decode(&decoder_word, decoder);
            new_pixels.insert((x, y), decoded_pixel);
        }
    }
    new_pixels
}

pub fn print(pixels: &HashMap<(i32, i32), char>) {
    for y in -55..155 {
        for x in -55..155 {
            print!("{}", get_pixel(pixels, x, y));
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example1() {
        let (decoder, pixels) = read_input("example.txt");

        let (x, y) = get_limits(&pixels);

        let mut iter = 2;
        let new_pixels = enhance(&pixels, &decoder, -iter, x + iter, -iter, y + iter);
        assert_eq!(24, new_pixels.iter().filter(|(_, &x)| { x == '#' }).count());

        iter = 4;
        let new_pixels = enhance(&new_pixels, &decoder, -iter, x + iter, -iter, y + iter);
        assert_eq!(35, new_pixels.iter().filter(|(_, &x)| { x == '#' }).count());
    }

    #[test]
    fn part1() {
        let (decoder, pixels) = read_input("input.txt");

        let (x, y) = get_limits(&pixels);

        let mut iter = 10; // need to handle the stupid "flashing" problem, solve far out, trim back on the next flash
        let new_pixels = enhance(&pixels, &decoder, -iter, x + iter, -iter, y + iter);

        iter = 3;
        let new_pixels = enhance(&new_pixels, &decoder, -iter, x + iter, -iter, y + iter);

        assert_eq!(
            5479,
            new_pixels.iter().filter(|(_, &x)| { x == '#' }).count()
        )
    }

    #[test]
    fn example2() {
        let (decoder, mut pixels) = read_input("example.txt");

        let (x, y) = get_limits(&pixels);

        for i in 0..49 {
            let buffer = 5;
            pixels = enhance(
                &pixels,
                &decoder,
                -i - buffer,
                x + i + buffer,
                -i - buffer,
                y + i + buffer,
            );
        }
        let i = 51; // trim to proper space

        pixels = enhance(&pixels, &decoder, -i, x + i, -i, y + i);

        assert_eq!(3351, pixels.iter().filter(|(_, &x)| { x == '#' }).count());
    }

    #[test]
    fn part2() {
        let (decoder, mut pixels) = read_input("input.txt");

        let (x, y) = get_limits(&pixels);

        for _ in 0..49 {
            let i = 0;
            // run 49 times with "buffer"
            let buffer = 110;
            pixels = enhance(
                &pixels,
                &decoder,
                -i - buffer,
                x + i + buffer,
                -i - buffer,
                y + i + buffer,
            );
        }
        let i = 51; // trim to proper space

        pixels = enhance(&pixels, &decoder, -i, x + i, -i, y + i);

        assert_eq!(19012, pixels.iter().filter(|(_, &x)| { x == '#' }).count());
    }
}
