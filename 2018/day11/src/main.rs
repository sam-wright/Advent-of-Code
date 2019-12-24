//Correct
fn compute_coord(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial;
    power_level *= rack_id;
    (power_level / 100) as i32 % 10 - 5
}

fn value_3_3(grid: &Box<[[i32; 301]; 301]>, x: usize, y: usize) -> i32 {
    let mut sum = 0;
    for dy in 0..3 {
        for dx in 0..3 {
            sum += grid[x + dx][y + dy];
        }
    }
    sum
}

fn value_n_n(grid: &Box<[[i32; 301]; 301]>, x: usize, y: usize, n: usize) -> i32 {
    let mut sum = 0;
    for dy in 0..n {
        for dx in 0..n {
            sum += grid[x + dx][y + dy];
        }
    }
    sum
}

fn main() {
    println!(
        "Fuel cell at  3,5, grid serial number 8: power level {} =?= 4",
        compute_coord(3, 5, 8)
    );
    println!(
        "Fuel cell at  122,79, grid serial number 57: power level {} =?= -5",
        compute_coord(122, 79, 57)
    );
    println!(
        "Fuel cell at 217,196, grid serial number 39: power level {} =?= 0",
        compute_coord(217, 196, 39)
    );
    println!(
        "Fuel cell at 101,153, grid serial number 71: power level {} =?= 4",
        compute_coord(101, 153, 71)
    );

    let serial = 9110;
    //let serial = 18;
    //let serial = 42;

    // Create grid
    let mut grid = Box::new([[0; 301]; 301]);
    for y in 0i32..=300 {
        for x in 0i32..=300 {
            assert!(y >= 0);
            assert!(x >= 0);
            grid[x as usize][y as usize] = compute_coord(x, y, serial);
            //print!("{} ", grid[x as usize][y as usize]);
        }
        //print!("\n");
    }

    //println!(
    //    "combined value of 33,45: {} =?= 29",
    //    value_3_3(&grid, 33, 45)
    //);

    // println!(
    //     "combined value of 21,61: {} =?= 30",
    //     value_3_3(&grid, 21, 61)
    // );

    // Search grid for max (top-left)
    let mut max = 0;
    let mut max_index = (500, 500);

    for y in 0usize..=295 {
        for x in 0usize..=295 {
            let value = value_3_3(&grid, x, y);
            if value > max {
                max = value;
                max_index = (x, y);
            }
        }
    }
    println!("Max index: {:?}\nMax value:{}", max_index, max);

    let mut max = 0;
    let mut max_index = (500, 500);
    let mut grid_size = 500;

    for n in 1usize..=299 {
        for y in 0usize..=300 - n {
            for x in 0usize..=300 - n {
                let value = value_n_n(&grid, x, y, n);
                if value > max {
                    max = value;
                    max_index = (x, y);
                    grid_size = n;
                }
            }
        }
    }
    println!(
        "Max index: {:?}\nMax value:{} Using grid_size: {}",
        max_index, max, grid_size
    );
}
