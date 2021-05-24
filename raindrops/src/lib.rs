pub fn raindrops(n: u32) -> String {
    //raindrops_with_some_closure(n)
    raindrops_with_map_and_filter(n)
}

// The solution i came up with initially
pub fn raindrops_initial(n: u32) -> String {
    match (n % 3, n % 5, n % 7) {
        (0, 0, 0) => "PlingPlangPlong".to_string(),
        (0, 0, _) => "PlingPlang".to_string(),
        (0, _, 0) => "PlingPlong".to_string(),
        (_, 0, 0) => "PlangPlong".to_string(),
        (0, _, _) => "Pling".to_string(),
        (_, 0, _) => "Plang".to_string(),
        (_, _, 0) => "Plong".to_string(),
        (_, _, _) => n.to_string(),
    }
}

// A boring solution
pub fn raindrops_with_simple_ifs(n: u32) -> String {
    let mut result: String = String::new();

    if n % 3 == 0 {
        result.push_str("Pling");
    }
    if n % 5 == 0 {
        result.push_str("Plang");
    }
    if n % 7 == 0 {
        result.push_str("Plong");
    }

    if result.is_empty() {
        result.push_str(&n.to_string());
    }

    result
}

// Source: https://exercism.io/tracks/rust/exercises/raindrops/solutions/6218001325d246309ca9bda239ae9108
pub fn raindrops_with_some_closure(n: u32) -> String {
    let is_divisible_by = |x| n % x == 0;
    let mut result: String = String::new();

    if is_divisible_by(3) {
        result.push_str("Pling");
    }
    if is_divisible_by(5) {
        result.push_str("Plang");
    }
    if is_divisible_by(7) {
        result.push_str("Plong");
    }

    if result.is_empty() {
        result.push_str(&n.to_string())
    }

    result
}

/*
Based on my Python solution:
  * https://exercism.io/my/solutions/1731c4bae0ac4b208024baf6aae74154
  * https://github.com/ederst/exercism-python/blob/e84d123995f0d62855710687df5d47076d7290bf/raindrops/raindrops.py
and:
  * https://exercism.io/tracks/rust/exercises/raindrops/solutions/f4a2a248431041fd97647c19d9a9476c
  * https://exercism.io/tracks/rust/exercises/raindrops/solutions/ad56bd6ff61c47f9944ce883b7110528
  * https://exercism.io/tracks/rust/exercises/raindrops/solutions/156b36dbabd14ab1bbb2b3ae5b96d60f
*/
pub fn raindrops_with_map_and_filter(n: u32) -> String {
    let sound_mapping = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

    match sound_mapping
        .iter()
        .map(|&(factor, sound)| if n % factor == 0 { sound } else { "" })
        .collect::<String>()
    {
        ref res if !res.is_empty() => res.to_owned(),
        _ => n.to_string(),
    }
}
