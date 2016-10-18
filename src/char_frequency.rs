use std::ascii::AsciiExt;

fn char_score(c: &u8) -> f32 {
    match *c as char {
        'a' => 65.1738,
        'b' => 12.4248,
        'c' => 21.7339,
        'd' => 34.9835,
        'e' => 104.1442,
        'f' => 19.7881,
        'g' => 15.861,
        'h' => 49.2888,
        'i' => 55.8094,
        'j' => 0.9033,
        'k' => 5.0529,
        'l' => 33.149,
        'm' => 20.2124,
        'n' => 56.4513,
        'o' => 59.6302,
        'p' => 13.7645,
        'q' => 0.8606,
        'r' => 49.7563,
        's' => 51.576,
        't' => 72.9357,
        'u' => 22.5134,
        'v' => 8.2903,
        'w' => 17.1272,
        'x' => 1.3692,
        'y' => 14.5984,
        'z' => 0.7836,
        'A' => 13.03476,
        'B' => 2.48496,
        'C' => 4.34678,
        'D' => 6.9967,
        'E' => 20.82884,
        'F' => 3.95762,
        'G' => 3.1722,
        'H' => 9.85776,
        'I' => 11.16188,
        'J' => 0.18066,
        'K' => 1.01058,
        'L' => 6.6298,
        'M' => 4.04248,
        'N' => 11.29026,
        'O' => 11.92604,
        'P' => 2.7529,
        'Q' => 0.17212,
        'R' => 9.95126,
        'S' => 10.3152,
        'T' => 14.58714,
        'U' => 4.50268,
        'V' => 1.65806,
        'W' => 3.42544,
        'X' => 0.27384,
        'Y' => 2.91968,
        'Z' => 0.15672,
        ' ' => 191.8182,
        x if x.is_ascii() => -100.0,
        _ => -200.0,
    }
}

pub fn score_string(s: &[u8]) -> f32 {
    s.iter().fold(0.0, |acc, x| acc + char_score(x))
}