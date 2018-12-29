use std::io;
use std::io::Read;
use std::str::Lines;
use std::io::Write;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(input.lines())?;
    Ok(())
}

fn part1(input: Lines) -> Result<()> {
    let result = sum_list(input);
    writeln!(io::stdout(), "Part One: {}", result)?;
    Ok(())
}

fn sum_list(numbers: Lines) -> i32{
    return numbers
        .map(|num_string| num_string.parse::<i32>().unwrap())
        .fold(0, |total, x| total + x);
}


#[test]
fn it_sums_a_list() {
    let test_input = "1\n2\n1\n-1";
    assert_eq!(3, sum_list(test_input.lines()))
}