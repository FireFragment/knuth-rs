use num_bigint::BigUint;
use std::env;

fn str_to_bigint(s: String) -> BigUint {
    s.parse::<u32>().unwrap().into()
}

fn pause() {
    use std::io::{stdin, stdout, Read, Write};
    let mut stdout = stdout();
    stdout.write(b"Is it OK? [press ENTER to continue]").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{}", log_knuth(
        &str_to_bigint(args[1].clone()),
        &str_to_bigint(args[2].clone()),
        args[3].parse::<u8>().unwrap() + 1,
        false
    ));

    let (pr, inn) = (args[4].contains("p"), args[4].contains("i"));

    println!("Print progress: {pr}\nPrint how it works: {inn}\n");

    pause();

    println!("\nResult: {}", knuth_nat(&str_to_bigint(args[1].clone()), &str_to_bigint(args[2].clone()), args[3].parse().unwrap(), pr, inn));
}

fn knuth_nat(num_a: &BigUint, num_b: &BigUint, arrows: u8, progress: bool, inners: bool) -> BigUint {
    knuth(num_a, num_b, arrows + 1, progress, inners)
}

fn log_knuth(num_a: &BigUint, num_b: &BigUint, arrows: u8, result: bool) -> String {
    let arrs = match arrows {
        0 => String::from("+"),
        1 => String::from("×"),
        _ => "↑".repeat((arrows - 1)as usize)
    };
    format!("{num_a} {arrs} {num_b} = {}",
        if result {
            knuth(num_a, num_b, arrows, false, false).to_string()
        } else {String::from("To be computed...") } )
}

fn knuth(num_a: &BigUint, num_b: &BigUint, arrows: u8, progress: bool, inners: bool) -> BigUint {
    if inners {
        println!("{} {}", String::from("   ").repeat((16 - arrows) as usize), log_knuth(num_a, num_b, arrows, true));
    }

    if arrows == 1 {
        num_a * num_b
    } else {
        let mut res = num_a.clone();
        for i in num_iter::range_inclusive(BigUint::from(1u32), num_b.clone() - 1u32) {

            if progress {
                println!("{} {i} / {num_b}", String::from("   ").repeat((16 - arrows) as usize));
            }
            //println!("{} i: {i}, num_a: {num_a}, num_b: {num_b}, res: {res}", String::from("   ").repeat((4 - arrows) as usize));
            res = knuth(num_a, &res, arrows - 1, progress, inners);
        }
        res
    }
}
