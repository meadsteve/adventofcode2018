extern crate core;

use std::io;
use std::io::Read;
use std::str::Lines;
use std::io::Write;
use std::iter::Map;
use std::collections::HashSet;
use core::iter;

type ExecutionResult<T> = Result<T, Box<::std::error::Error>>;

fn main() -> ExecutionResult<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(input.clone().lines())?;
    part2(input.clone().lines())?;
    Ok(())
}

fn part1(input: Lines) -> ExecutionResult<()> {
    let result = sum_list(input);
    writeln!(io::stdout(), "Part One: {}", result)?;
    Ok(())
}

fn part2(input: Lines) -> ExecutionResult<()> {
    let result = first_repeated(input);
    match result {
        Ok(i) => writeln!(io::stdout(), "Part Two: {}", i)?,
        Err(e) => writeln!(io::stdout(), "Part Two went wrong: {}", e)?
    }
    Ok(())
}

fn sum_list(lines: Lines) -> i32 {
    return cast_lines_to_numbers(lines)
        .fold(0, |total, x| total + x);
}

fn first_repeated(lines: Lines) -> Result<i32, &'static str>{
    let mut totals = HashSet::new();
    let mut total = 0;
    totals.insert(total);

    for set_of_lines in iter::repeat(lines) {
        for number in cast_lines_to_numbers(set_of_lines) {
            total = total + number;
            if totals.contains(&total) {
                return Ok(total);
            }
            totals.insert(total);
        }
    }
    return Err("no total repeated");
}

fn cast_lines_to_numbers(lines: Lines) -> Map<Lines, fn(&str) -> i32> {
    return lines
        .map(|num_string| num_string.parse::<i32>().unwrap());
}

#[test]
fn it_sums_a_list() {
    let test_input = "1\n2\n1\n-1";
    assert_eq!(3, sum_list(test_input.lines()))
}

#[test]
fn it_finds_the_first_repeated_intermediate_total() {
    let test_input = "1\n2\n2\n-4\n2";
//                         1  3  5   1  3
    let result = first_repeated(test_input.lines());
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap_or_default(), 1)
}

#[test]
fn it_will_loop_over_the_input_to_find_repeats() {
    let test_input = "1\n2\n2\n-3";
//                         1  3  5   2
//                         3
    let result = first_repeated(test_input.lines());
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap_or_default(), 3)
}