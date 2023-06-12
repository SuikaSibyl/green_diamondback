use std::env;

#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: u64) -> u64;
}

#[no_mangle]
#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    // TODO: print error message according to writeup
    if errcode == 1 { eprintln!("Runtime: invalid argument error"); }
    else if errcode == 2 { eprintln!("Runtime: overflow error"); }
    else { eprintln!("Runtime: unkown error with code {}", errcode); }
    std::process::exit(1);
}

#[no_mangle]
#[export_name = "\x01snek_print"]
pub extern "C" fn snek_print(val: i64) {
    if val == 3 { println!("true"); }
    else if val == 1 { println!("false"); }
    else if val % 2 == 0 { println!("{}", val >> 1); }
    else { println!("NaN, with value {}", val); }
}

fn print_value(val: i64) {
    if val == 3 { println!("true"); }
    else if val == 1 { println!("false"); }
    else if val % 2 == 0 { println!("{}", val >> 1); }
    else { println!("NaN, with value {}", val); }
}

fn parse_input(input: &str) -> u64 {
    // TODO: parse the input string into internal value representation
    if input == "true" { 3 }
    else if input == "false" { 1 }
    else if input.parse::<i64>().is_ok() {
        let n = input.parse::<i64>().unwrap();
        if n < 2i64.pow(62) && n >= -2i64.pow(62) {
            (n as u64) << 1
        } else {
            panic!("Invalid")
        }
    }
    else { panic!("Invalid") }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    let i: i64 = unsafe { our_code_starts_here(input) } as i64;
    print_value(i);
}