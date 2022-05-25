#![allow(dead_code)]

use std::collections::HashMap;

fn multi_table(n: u64) -> String {
    let mut multi_table_string = String::new();
    for i in 1..=10 {
        let m = format!("{} * {} = {}\n", i, n, i*n);
        multi_table_string.push_str(&m);
    }
    multi_table_string.trim_end().to_string()

}

fn bin_to_decimal(inp: &str) -> i32 {
    i32::from_str_radix(inp, 2).unwrap()
}

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.len() < 2 {
        return Vec::new();
    }
    let mut positive = 0;
    let mut negative = 0;
    for n in input {
        match n {
            n if n > 0 => positive += 1, 
            _  => negative += n,
        }
    }
    vec![positive, negative]
}

fn duplicate_encode(word:&str)->String {
    let mut encode = String::new();
    for w in word.to_lowercase().chars() {
        if word.to_lowercase().matches(w).count() > 1 {
            encode.push(')');
        } else {
            encode.push('(');
        }
    }
    encode
}

fn number(bus_stops:&[(i32,i32)]) -> i32 {

    bus_stops.into_iter().fold(0, |acc,x| acc + x.0 - x.1)
    // let mut get_in = 0;
    // let mut get_off = 0;
    // for stop in bus_stops {
    //    get_in += stop.0;
    //    get_off += stop.1;
    // }
    // get_in - get_off
}

fn validate_pin(pin: &str) -> bool {

    // pin.chars().all(|c| c.is_digit(10)) && (pin.len() == 4 || pin.len() == 6)


    let is_pin_length_ok = pin.len() == 4 || pin.len() == 6;
    for c in pin.chars().into_iter() {
        if !c.is_digit(10) {
            return false;
        }
    }
    is_pin_length_ok
}


fn dna_to_rna(dna: &str) -> String {
    dna.replace("T", "U")
}

fn switch_it_up(n: usize) -> &'static str {
    match n {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        _ => "Zero",
    }
}

fn no_space(x : String) -> String{
    x.split(" ").collect::<String>()
}

fn find_average(slice: &[f64]) -> f64 {
    match slice.len() {
        0 => 0.,
        n => slice.iter().sum::<f64>() / n as f64
    }
    // my solution
    // let total: f64 = slice.iter().sum();
    // total / if slice.len() == 0 { 1.0 as f64 } else { slice.len() as f64 }
}
fn printer_error(s: &str) -> String {

    format!("{}/{}", s.chars().filter(|&c| c > 'm').count(), s.len())
    

    // my solution
    // let correct_vector: Vec<char> = ('a'..='m').collect();
    // let mut correct_count = 0;
    // for item in s.chars().into_iter() {
    //     if !correct_vector.contains(&item) {
    //         correct_count += 1;
    //     } else {
    //          continue;
    //   }
    // }
    // format!("{}/{}", correct_count, s.len())
    
}

fn find_short(s: &str) -> u32 {
    let list: Vec<&str>  = s.split(" ").collect();
    let mut min_length = list[0].len();
    for item in list {
        if item.len() < min_length {
            min_length = item.len();
        }
    }
    min_length as u32
}

fn positive_sum(slice: &[i32]) -> i32 {
    slice.into_iter().filter(|x| **x > 0).sum()
}

fn summation(n: i32) -> i32 {
    (1..=n).sum()
}

fn name_shuffler(s: &str) -> String {
    let mut result: Vec<&str> = s.split(" ").collect();
    result.reverse();
    result.join(" ")
}

fn is_divide_by(number: i32, a: i32, b: i32) -> bool {
    number.checked_rem(a).unwrap() == 0 && number.checked_rem(b).unwrap() == 0
}

fn bmi(weight: u32, height: f32) -> &'static str {
    let bmi = weight as f64 / ( height * height) as f64;
    if bmi <= 18.5 {
        return "Underweight";
    } else if bmi <= 25.0 {
        return "Normal";
    } else if bmi <= 30.0  { 
        return "Overweight";
    } else {
        return "Obese";
    }

}
fn remove_char(s: &str) -> String {
    let mut chars = s.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}


fn get_char(c: i32) -> char {
    c as u8 as char
}


fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    let result = on + wait - cap;
    if result > 0 { result } else { 0 }
}

fn abbrev_name(_name: &str) -> String {
    String::new()
}

fn repeat_str(src: &str, count: usize) -> String {
    src.repeat(count)
}
fn greet(language: &str) -> &str {
    let dictionary = HashMap::from([
        ("english", "Welcome"),
        ("czech", "Vitejte"),
        ("danish", "Velkomst"),
        ("dutch", "Welkom"),
        ("estonian", "Tere tulemast"),
        ("finnish", "Tervetuloa"),
        ("flemish", "Welgekomen"),
        ("french", "Bienvenue"),
        ("german", "Willkommen"),
        ("irish", "Failte"),
        ("italian", "Benvenuto"),
        ("latvian", "Gaidits"),
        ("lithuanian", "Laukiamas"),
        ("polish", "Witamy"),
        ("spanish", "Bienvenido"),
        ("swedish", "Valkommen"),
        ("welsh", "Croeso")
    ]);
    match dictionary.get(language) {
        Some(&value) => value,
        _ => "Welcome",
    }
}

fn xo(string: &'static str) -> bool {
    let mut x = 0;
    let mut o = 0;
    for c in string.chars().into_iter() {
        if c == 'X' || c == 'x' {
            x += 1;
        } else if c == 'O' || c == 'o' {
            o += 1;
        } else {
            continue;
        }
    }
    x == o
}
fn hero(bullets: u16, dragons: u16) -> bool {
    bullets >= dragons * 2
}

fn litres(time: f64) -> i32 {
    (time * 0.5) as i32
}
fn longest(_a1: &str, _a2: &str) -> String {
    String::new()
}

fn reverse_string() {
    let test = "world";
    let another_test = test.chars().rev().collect::<String>();
    println!("{}", another_test);

}

fn db_dig(n: i32, d: i32) -> i32 {
    let mut result: i32 = 0;
    let n: Vec<String> = (0..=n).map(|x| (x * x).to_string()).collect();
    for i in n {
        let c = i.as_bytes().windows(d.to_string().len()).filter(|&w| w == d.to_string().as_bytes()).count();
        result += c as i32;
    }
    result

}

fn to_alternating_case(s: &str) -> String {
    s.chars()
        .into_iter()
        .map(|v| {
            if v.is_lowercase() {
                v.to_uppercase().to_string()
            } else {
                v.to_lowercase().to_string()
            }
        })
        .collect()
}

fn check_for_factor(base: i32, factor: i32) -> bool {
    base % factor == 0
}

fn reverse_number() {
    let n = 321412;
    let mut v: Vec<char> = n.to_string().chars().collect();
    v.sort_by(|a,b| b.cmp(a));
    let n: i32 = String::from_iter(v).parse().unwrap();

    println!("{}", n);

}

fn count_sheep(n: u32) -> String {
    let mut  s  = String::new();
    for i in 1..=n {
        let mut result = format!("{} sheep...", i);
        s.push_str(&mut result[..]);
    }
    s
}
fn count_sheep_array(sheep: &[bool]) -> u8 {
    let mut result: u8 = 0;
    for s in sheep {
        if *s {
            result += 1 as u8;
        } else {
            continue;
        }
    }
    result
}
fn main() {
    println!("Hello world");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn litres_test() {
        assert_eq!(litres(2.), 1);
        assert_eq!(litres(1.4), 0);
        assert_eq!(litres(12.3), 6);
        assert_eq!(litres(0.82), 0);
        assert_eq!(litres(11.8), 5);
        assert_eq!(litres(1787.), 893);
        assert_eq!(litres(0.), 0);
    }

    #[test]
    fn hero_test() {
        assert_eq!(hero(10, 5), true);
        assert_eq!(hero(7, 4), false);
        assert_eq!(hero(4, 5), false);
        assert_eq!(hero(100, 40), true);
        assert_eq!(hero(1500, 751), false);
        assert_eq!(hero(0, 1), false);
    }

    #[test]
    fn xo_test() {
      assert_eq!(xo("xo"), true);
      assert_eq!(xo("Xo"), true);
      assert_eq!(xo("xxOo"), true);
      assert_eq!(xo("xxxm"), false);
      assert_eq!(xo("Oo"), false);
      assert_eq!(xo("ooom"), false);
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("english"), "Welcome");
        assert_eq!(greet("dutch"), "Welkom");
        assert_eq!(greet("IP_ADDRESS_INVALID"), "Welcome");
        assert_eq!(greet(""), "Welcome");
        assert_eq!(greet("swelsh"), "Welcome");
    }

    #[test]
    fn test_repeat_str() {
      assert_eq!(repeat_str("a", 4), "aaaa");
      assert_eq!(repeat_str("hello ", 3), "hello hello hello ");
      assert_eq!(repeat_str("abc", 2), "abcabc");
    }

    #[test]
    fn test_enough() {
        assert_eq!(enough(10, 5, 5), 0, "enough(10, 5, 5) should return 0");
        assert_eq!(enough(100, 60, 50), 10, "enough(100, 60, 50) should return 10");
        assert_eq!(enough(20, 5, 5), 0, "enough(20, 5, 5) should return 0");
    }

    #[test]
    fn test_get_char() {
        assert_eq!(get_char(55), '7');
        assert_eq!(get_char(56), '8');
        assert_eq!(get_char(57), '9');
        assert_eq!(get_char(58), ':');
        assert_eq!(get_char(59), ';');
        assert_eq!(get_char(60), '<');
        assert_eq!(get_char(61), '=');
        assert_eq!(get_char(62), '>');
        assert_eq!(get_char(63), '?');
        assert_eq!(get_char(64), '@');
        assert_eq!(get_char(65), 'A');
    }
    #[test]
    fn test_remove_char() {
        assert_eq!(remove_char("eloquent"), "loquen");
        assert_eq!(remove_char("country"), "ountr");
        assert_eq!(remove_char("person"), "erso");
        assert_eq!(remove_char("place"), "lac");
        assert_eq!(remove_char("ok"), "");
        assert_eq!(remove_char("ooopsss"), "oopss");
    }
    #[test]
    fn test_bmi() {
        assert_eq!(bmi(50, 1.80), "Underweight");
        assert_eq!(bmi(80, 1.80), "Normal");
        assert_eq!(bmi(90, 1.80), "Overweight");
        assert_eq!(bmi(110, 1.80), "Obese");
    }

    
    #[test]
    fn test_divide_by() {
        assert_eq!(is_divide_by(8, 2, 4), true);
        assert_eq!(is_divide_by(12, -3, 4), true);
        assert_eq!(is_divide_by(8, 3, 4), false);
        assert_eq!(is_divide_by(48, 2, -5), false);
        assert_eq!(is_divide_by(-100, -25, 10), true);
        assert_eq!(is_divide_by(10000, 5, -3), false);
        assert_eq!(is_divide_by(4, 4, 2), true);
        assert_eq!(is_divide_by(5, 2, 3), false);
        assert_eq!(is_divide_by(-96, 25, 17), false);
        assert_eq!(is_divide_by(33, 1, 33), true);
    }

    #[test]
    fn test_name_shuffler() {
        assert_eq!(name_shuffler("john McClane"), "McClane john");
        assert_eq!(name_shuffler("Mary jeggins"), "jeggins Mary");
        assert_eq!(name_shuffler("tom jerry"), "jerry tom");
    }

    #[test]
    fn test_summation() {
        assert_eq!(summation(1), 1);
        assert_eq!(summation(8), 36);
        assert_eq!(summation(22), 253);
        assert_eq!(summation(100), 5050);
        assert_eq!(summation(213), 22791);
    }

    #[test]
    fn test_positive_sum() {
        assert_eq!(positive_sum(&[1,2,3,4,5]), 15);
        assert_eq!(positive_sum(&[1,-2,3,4,5]), 13);
        assert_eq!(positive_sum(&[-1,2,3,4,-5]), 9);
    }

    #[test]
    fn test_find_short() {
      assert_eq!(find_short("bitcoin take over the world maybe who knows perhaps"), 3);
      assert_eq!(find_short("turns out random test cases are easier than writing out basic ones"), 3);
      assert_eq!(find_short("lets talk about javascript the best language"), 3);
      assert_eq!(find_short("i want to travel the world writing code one day"), 1);
      assert_eq!(find_short("Lets all go on holiday somewhere very cold"), 2);
      assert_eq!(find_short("Let's travel abroad shall we"), 2);
    }

    #[test]
    fn test_printer_error() {
        assert_eq!(&printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"), "3/56");
        assert_eq!(&printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"), "6/60");
        assert_eq!(&printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyzuuuuu"), "11/65");
    }

    #[test]
    fn test_average() {
        let input = [
            17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0,
        ];
        assert_eq!(find_average(&input), 200.0 / 13.0);
        assert_eq!(find_average(&[]), 0.0);
    }

    #[test]
    fn test_no_space() {
      assert_eq!("8j8mBliB8gimjB8B8jlB", no_space("8 j 8   mBliB8g  imjB8B8  jl  B".to_string()));
      assert_eq!("88Bifk8hB8BB8BBBB888chl8BhBfd", no_space("8 8 Bi fk8h B 8 BB8B B B  B888 c hl8 BhB fd".to_string()));
      assert_eq!("8aaaaaddddr", no_space("8aaaaa dddd r     ".to_string()));
      assert_eq!("jfBmgklf8hg88lbe8", no_space("jfBm  gk lf8hg  88lbe8 ".to_string()));
      assert_eq!("8jaam", no_space("8j aam".to_string()));        
    }

    #[test]
    fn test_switch_it_up() {
        assert_eq!(switch_it_up(1), "One");
        assert_eq!(switch_it_up(2), "Two");
        assert_eq!(switch_it_up(3), "Three");
    }
    
    #[test]
    fn test_dna_to_rna() {
        assert_eq!(dna_to_rna("TTTT"), String::from("UUUU"));
    }

    #[test]
    fn test_validate_pin() {
        assert_eq!(validate_pin("1234"), true);
        assert_eq!(validate_pin("1234a"), false);
        assert_eq!(validate_pin("-123"), false);
        assert_eq!(validate_pin("+123"), false);
    }

        #[test]
    fn test_validate_pin_invalid_length_tests() {
        assert_eq!(validate_pin("1"), false);
        assert_eq!(validate_pin("12"), false);
        assert_eq!(validate_pin("123"), false);
        assert_eq!(validate_pin("12345"), false);
        assert_eq!(validate_pin("1234567"), false);
        assert_eq!(validate_pin("-1234"), false);
        assert_eq!(validate_pin("1.234"), false);
        assert_eq!(validate_pin("-1.234"), false);
        assert_eq!(validate_pin("00000000"), false);
    }
    
    #[test]
    fn test_validate_pin_non_digit_chars_tests() {
        assert_eq!(validate_pin("a234"), false);
        assert_eq!(validate_pin(".234"), false);
    }
    
    #[test]
    fn test_validate_pin_valid_pin_tests() {
        assert_eq!(validate_pin("1234"), true);
        assert_eq!(validate_pin("0000"), true);
        assert_eq!(validate_pin("1111"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("098765"), true);
        assert_eq!(validate_pin("000000"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("090909"), true);
    }

    #[test]
    fn test_bus_stop() {
      assert_eq!(number(&[(10,0),(3,5),(5,8)]), 5);
      assert_eq!(number(&[(3,0),(9,1),(4,10),(12,2),(6,1),(7,10)]), 17);
      assert_eq!(number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,8)]), 21);
    }

    #[test]
    fn test_duplicate_encode() {
      assert_eq!(duplicate_encode("din"),"(((");
      assert_eq!(duplicate_encode("recede"),"()()()");
      assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
      assert_eq!(duplicate_encode("(( @"),"))((");
    }

    #[test]
    fn test_bin_to_decimal() {
        assert_eq!(bin_to_decimal("0"), 0);
        assert_eq!(bin_to_decimal("1"), 1);
        assert_eq!(bin_to_decimal("1001001"), 73);
    }
    #[test]
    fn test_multi_table() {
        assert_eq!(multi_table(5), "1 * 5 = 5\n2 * 5 = 10\n3 * 5 = 15\n4 * 5 = 20\n5 * 5 = 25\n6 * 5 = 30\n7 * 5 = 35\n8 * 5 = 40\n9 * 5 = 45\n10 * 5 = 50");
        assert_eq!(multi_table(1), "1 * 1 = 1\n2 * 1 = 2\n3 * 1 = 3\n4 * 1 = 4\n5 * 1 = 5\n6 * 1 = 6\n7 * 1 = 7\n8 * 1 = 8\n9 * 1 = 9\n10 * 1 = 10");
    }

}
