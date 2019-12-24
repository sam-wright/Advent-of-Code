fn calculate_module(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn calculate_fuel(mass: i32) -> i32 {
    let mut total_fuel = calculate_module(mass);
    let mut fuel_fuel = calculate_module(total_fuel);
    while fuel_fuel > 0 {
        total_fuel += fuel_fuel;
        fuel_fuel = calculate_module(fuel_fuel);
    }

    total_fuel
}

#[test]
fn examples1() {
    assert_eq!(calculate_module(12), 2);
    assert_eq!(calculate_module(14), 2);
    assert_eq!(calculate_module(1969), 654);
    assert_eq!(calculate_module(100756), 33583);
}

#[test]
fn examples2() {
    assert_eq!(calculate_fuel(14), 2);
    assert_eq!(calculate_fuel(1969), 966);
    assert_eq!(calculate_fuel(100756), 50346);
}

#[test]
fn part_1() -> Result<(), std::io::Error> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let collection: Vec<&str> = contents.split('\n').collect();

    let sum = collection
        .iter()
        .fold(0, |acc, c| acc + calculate_module(c.parse().expect("NAN")));

    println!("Sum: {}", sum);
    assert_eq!(sum, 3511949);

    Ok(())
}

#[test]
fn part_2() -> Result<(), std::io::Error> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let collection: Vec<&str> = contents.split('\n').collect();

    let sum = collection
        .iter()
        .fold(0, |acc, c| acc + calculate_fuel(c.parse().expect("NAN")));

    println!("Sum: {}", sum);
    assert_eq!(sum, 5265045);

    Ok(())
}
