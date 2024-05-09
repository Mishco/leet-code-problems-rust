use leet_code_problems_rust::easy::s0013_roman_to_integer::roman_to_int;
use leet_code_problems_rust::medium::s3075_maximize_happiness_selected_children::maximum_happiness_sum;

mod easy;
mod medium;

fn main() {
    println!("{}", roman_to_int(String::from("MMXXIV")));
    println!("{}", maximum_happiness_sum(Vec::from([2,3,4,5]), 1));
}
