fn main() {
    println!("Result: {}", knuth_nat(3, 3, 2));
}

fn knuth_nat(num_a: u128, num_b: u128, arrows: u8) -> u128 {
    knuth(num_a, num_b, arrows + 1, false)
}

fn log_knuth(num_a: u128, num_b: u128, arrows: u8) -> String {
    let arrs = match arrows {
        0 => String::from("+"),
        1 => String::from("×"),
        _ => "↑".repeat((arrows - 1)as usize)
    };
    format!("{num_a} {arrs} {num_b} = {}", knuth(num_a, num_b, arrows, true))
}

fn knuth(num_a: u128, num_b: u128, arrows: u8, mut silent: bool) -> u128 {
    if arrows == 100 {
        silent = true;
    }

    if !silent {
        println!("{} {}", String::from("   ").repeat((16 - arrows) as usize), log_knuth(num_a, num_b, arrows));
    }

    if arrows == 2 {
        num_a.pow(num_b as u32)
        //num_a + num_b
    } else {
        let mut res = num_a;
        for _ in 1..num_b {
            //println!("{} i: {i}, num_a: {num_a}, num_b: {num_b}, res: {res}", String::from("   ").repeat((4 - arrows) as usize));
            res = knuth(num_a, res, arrows - 1, silent);
            if !silent {
                //println!("{}  -> {res}", String::from("   ").repeat((4 - arrows) as usize))
            }
        }

        //println!("{}-> res: {res}", String::from("   ").repeat((4 - arrows) as usize));
        res
    }
}
