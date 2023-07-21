use num_bigint::BigUint;
use std::env;

/*fn str_to_bigint(s: String) -> BigUint {
    s.parse::<u32>().unwrap().into()
}*/

fn pause(msg: String) {
    use std::io::{stdin, stdout, Read, Write};
    let mut stdout = stdout();
    stdout.write(msg.as_bytes()).unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn ask_for_num (q: &str) -> u32 {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("{}: ", q);
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");

    s.trim().parse().unwrap()
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    args.push(String::new());

    let n_a: BigUint = ask_for_num("First number").into();
    let n_b: BigUint = ask_for_num("Second number").into();
    let arr = ask_for_num("Number of arrows") as u8;

    println!("\n--\n{}", log_knuth(
        &n_a,
        &n_b,
        arr + 1,
        false
    ));

    let (pr, inn) = (args[1].contains("p"), args[1].contains("i"));

    println!("Print progress: {pr}\nPrint how it works: {inn}\n");

    pause(String::from("Is it OK? Press [ENTER] to continue calculation..."));

    let start = std::time::Instant::now();
    let result  = knuth_nat(&n_a, &n_b, arr, pr, inn);
    println!("Calculating...");

    let duration = start.elapsed();

    println!("Duration: {duration:?}");
    pause(String::from("\nPress [ENTER] to show the result..."));
    println!("\nResult: {result}", );

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
        } else {String::new() } )
}

fn knuth(num_a: &BigUint, num_b: &BigUint, arrows: u8, progress: bool, inners: bool) -> BigUint {
    if inners {
        println!("{} {}", String::from("   ").repeat((16 - arrows) as usize), log_knuth(num_a, num_b, arrows, true));
    }

    if arrows == 1 {
        num_a * num_b
    } else {
        let mut res = num_a.clone();
        let max = num_b.clone() - 1u32;
        for i in num_iter::range_inclusive(BigUint::from(1u32), max.clone()) {

            if progress {
                println!("{} {i} / {max}", String::from("   ").repeat((16 - arrows) as usize));
            }
            //println!("{} i: {i}, num_a: {num_a}, num_b: {num_b}, res: {res}", String::from("   ").repeat((4 - arrows) as usize));
            res = knuth(num_a, &res, arrows - 1, progress, inners);
        }
        res
    }
}
