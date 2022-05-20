#![allow(dead_code)]

use std::collections::HashMap;

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
   let result = name_shuffler("AAA BBB");
   println!("{}", result);
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
}