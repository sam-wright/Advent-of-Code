use std::fs::File;
use std::io::{self, Read};

fn react(chars: Vec<char>) -> Result<Vec<char>, Vec<char>> {
    let mut new_chars: Vec<char> = Vec::with_capacity(chars.len());

    let mut iter = chars.iter().peekable();
    let mut changed = false;

    loop {
        let this = iter.next();
        let that = iter.peek();

        if this.is_some() && that.is_some() {
            let this_val = this.unwrap();
            let that_val = that.unwrap();

            // if this is uppercase
            if this_val.is_uppercase()
                && that_val.is_lowercase()
                && *this_val == that_val.to_ascii_uppercase()
            {
                // reacting pair!
                //println!("Removing reacting pair {} {}", this_val, that_val);
                changed = true;

                //consume the peeked val
                iter.next();
                continue;
            } else if this_val.is_lowercase()
                && that_val.is_uppercase()
                && *this_val == that_val.to_ascii_lowercase()
            {
                // reacting pair!
                //println!("Removing reacting pair {} {}", this_val, that_val);
                changed = true;

                //consume the peeked val
                iter.next();
                continue;
            }

            new_chars.push(*this_val);
        } else {
            new_chars.push(*this.unwrap());

            break;
        }
    }
    if changed {
        Ok(new_chars)
    } else {
        Err(new_chars)
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    //let mut file = File::open("test_input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let my_chars: Vec<_> = contents.chars().filter(|v| *v != '\n').collect();
    let letters = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    for l in &letters {
        println!("Removing \"{}\"", l);

        let mut temp_chars: Vec<char> = my_chars.clone();
        temp_chars.retain(|v| *v != l.to_ascii_lowercase());
        temp_chars.retain(|v| *v != l.to_ascii_uppercase());

        loop {
            match react(temp_chars) {
                Ok(c) => temp_chars = { c },
                Err(c) => {
                    //println!("{:?}\n{}", c, c.len());
                    println!("{}", c.len());
                    break;
                }
            }
        }
    }
    Ok(())
}
