#[cfg(test)]
mod tests {
    use std::fs;

    type Pixel = u8;
    #[derive(Debug)]
    struct Layer {
        pixels: Vec<Vec<Pixel>>,
    }

    #[derive(Debug)]
    struct Image {
        layers: Vec<Layer>,
        height: usize,
        width: usize,
    }

    impl Layer {
        fn new(width: usize, _height: usize, data: &[u8]) -> Self {
            let mut new_layer = Self { pixels: Vec::new() };
            let rows = data.chunks_exact(width);
            for row in rows {
                new_layer.pixels.push(row.into());
            }

            new_layer
        }

        fn count_char(&self, input: &str) -> usize {
            let digit = input.as_bytes()[0];

            let val = self.pixels.iter().flatten().fold(0, |mut acc, &v| {
                if v == digit {
                    acc += 1;
                }
                acc
            });

            val
        }
    }

    impl Image {
        fn new(input: &str, width: usize, height: usize) -> Self {
            let mut new_image = Image {
                layers: Vec::new(),
                height,
                width,
            };
            let chars = input.as_bytes();
            let chunks = chars.chunks_exact(width * height);

            for chunk in chunks {
                let new_layer = Layer::new(width, height, chunk);

                new_image.layers.push(new_layer);
            }
            println!("Found {} layers", new_image.layers.len());
            new_image
        }

        fn get_pixel(&self, w: usize, h: usize) -> char {
            for l in self.layers.iter() {
                let v = l.pixels[h][w];
                match v {
                    x if x == '0' as u8 => return ' ',
                    x if x == '1' as u8 => return '.',
                    _ => (),
                }
            }
            ' '
        }

        fn render(&self) {
            //To do
            for h in 0..self.height {
                for w in 0..self.width {
                    print!("{}", self.get_pixel(w, h));
                }
                println!();
            }
            println!();
        }
    }

    #[test]
    fn example2() {
        let image = Image::new("0222112222120000", 2, 2);
        image.render();
    }

    #[test]
    fn part1_2() {
        let data = fs::read_to_string("input.txt").expect("Unable to read file");
        let image = Image::new(&data, 25, 6);

        let mut min = 999;
        let mut min_layer = 0;
        for (i, l) in image.layers.iter().enumerate() {
            let count = l.count_char("0");
            println!("Layer:{} Count:{}", i, count);
            if count < min {
                min = count;
                min_layer = i;
            }
        }

        println!("Min layer is {}", min_layer);

        let ones = image.layers[min_layer].count_char("1");
        let twos = image.layers[min_layer].count_char("2");

        println!("{}*{}={}", ones, twos, ones * twos);

        assert_eq!(ones, 24);
        assert_eq!(twos, 121);

        image.render();
    }

    #[test]
    fn example1() {
        let image = Image::new("123456789012", 3, 2);

        assert_eq!(image.layers.len(), 2);
        assert_eq!(image.layers[0].pixels.len(), 2);
        assert_eq!(image.layers[0].pixels[0].len(), 3);
        assert_eq!(image.layers[0].pixels[1].len(), 3);
        assert_eq!(image.layers[1].pixels.len(), 2);
        assert_eq!(image.layers[1].pixels[0].len(), 3);
        assert_eq!(image.layers[1].pixels[1].len(), 3);

        for (i, l) in image.layers.iter().enumerate() {
            println!("Layer:{} Count:{}", i, l.count_char("0"));
        }
    }
}
