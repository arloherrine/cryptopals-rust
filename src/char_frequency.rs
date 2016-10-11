fn char_score(c: &u8) -> f32 {
    match *c as char {
        '\n' =>  0.0228016604012,
        ' ' => 0.237062444957,
        '"' => 8.61089894304e-05,
        '&' => 3.847422932e-06,
        '(' => 0.000115056266728,
        '*' => 1.1542268796e-05,
        ',' => 0.0152383597593,
        '.' => 0.0142950082985,
        '0' => 5.4779974127e-05,
        '2' => 6.70550853862e-05,
        '4' => 1.70385872703e-05,
        '6' => 1.1542268796e-05,
        '8' => 7.32842463237e-06,
        ':' => 0.000334725795084,
        '<' => 8.57425681988e-05,
        '>' => 8.07958815719e-05,
        '@' => 1.46568492647e-06,
        'B' => 0.00282382522147,
        'D' => 0.00287329208774,
        'F' => 0.00214594594297,
        'H' => 0.00338243438907,
        'J' => 0.000378696342878,
        'L' => 0.00437103887198,
        'N' => 0.005008611815,
        'P' => 0.00218735154215,
        'R' => 0.00530761154,
        'T' => 0.00729178250921,
        'V' => 0.000655894004597,
        'X' => 0.00011102563318,
        'Z' => 9.74680476106e-05,
        '`' => 1.83210615809e-07,
        'b' => 0.00852717169161,
        'd' => 0.0245097329724,
        'f' => 0.0126054399995,
        'h' => 0.0400142977565,
        'j' => 0.000496867190075,
        'l' => 0.0267782468173,
        'n' => 0.039559569008,
        'p' => 0.00852387390053,
        'r' => 0.0382715983789,
        't' => 0.0531264983193,
        'v' => 0.00622714562074,
        'x' => 0.000858891366914,
        'z' => 0.000201348466774,
        '|' => 6.04595032171e-06,
        '~' => 1.83210615809e-07,
        '!' => 0.00162031468622,
        '#' => 1.83210615809e-07,
        '%' => 1.83210615809e-07,
        '\'' => 0.00569217062258,
        ')' => 0.000115239477344,
        '-' => 0.00147924251204,
        '/' => 9.16053079047e-07,
        '1' => 0.000170019451471,
        '3' => 6.04595032171e-05,
        '5' => 1.50232704964e-05,
        '7' => 7.51163524818e-06,
        '9' => 0.000173683663787,
        ';' => 0.00315103938131,
        '=' => 1.83210615809e-07,
        '?' => 0.00191931441122,
        'A' => 0.00815030745489,
        'C' => 0.00393847860805,
        'E' => 0.00780165765301,
        'G' => 0.0020453633149,
        'I' => 0.0102242516259,
        'K' => 0.00113517297555,
        'M' => 0.00290791889413,
        'O' => 0.00608424134041,
        'Q' => 0.000215822105423,
        'S' => 0.00623117625429,
        'U' => 0.00258858279077,
        'W' => 0.00302224231839,
        'Y' => 0.00166703339325,
        '[' => 0.000381994133963,
        ']' => 0.000380528449036,
        '_' => 1.30079537225e-05,
        'a' => 0.0448250421064,
        'c' => 0.0122179495471,
        'e' => 0.0741308625794,
        'g' => 0.0104494174727,
        'i' => 0.0363094126836,
        'k' => 0.00535194850902,
        'm' => 0.0175112706591,
        'o' => 0.0515538183932,
        'q' => 0.000440438320406,
        's' => 0.0393862517655,
        'u' => 0.021035876486,
        'w' => 0.0133549546288,
        'y' => 0.0156225524207,
        '}' => 3.66421231619e-07,
        _   => 0.0,
    }
}

pub fn score_string(s: &[u8]) -> f32 {
    s.iter().fold(0.0, |acc, x| acc + char_score(x))
}