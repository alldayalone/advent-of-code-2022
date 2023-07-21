use std::fs;
use std::ops::Div;
use std::ops::Rem;

struct SnafuDigit(i64);

impl SnafuDigit {
  fn new(ch: char) -> SnafuDigit {
    match ch {
      '2' => SnafuDigit(2),
      '1' => SnafuDigit(1),
      '0' => SnafuDigit(0),
      '-' => SnafuDigit(-1),
      '=' => SnafuDigit(-2),
      _ => panic!("Invalid input")
    }
  }

  fn display(&self) {
    print!("{}", match self {
       SnafuDigit(2) => '2',
       SnafuDigit(1) => '1',
       SnafuDigit(0) => '0',
       SnafuDigit(-1) => '-',
       SnafuDigit(-2) => '=',
      _ => panic!("Invalid input")
    });
  }
}

struct Snafu {
  digits: Vec<SnafuDigit>,
}

impl Snafu {
  fn new(str: &str) -> Snafu {
    Snafu {
      digits: str.chars().map(|ch| SnafuDigit::new(ch)).collect(),
    }
  }

  fn from(numb: i64) -> Snafu {
    let base: i64 = 5;
    let mut digits = Vec::new();

    let numb = Number::from(numb, base);
    let mut carry = 0;
    for d in numb.digits.iter().rev() {
      let dig = d.value + carry;
      carry = (dig + 2) / base;
      let rem = dig % base;

      match rem {
        0 => digits.push(SnafuDigit(0)),
        1 => digits.push(SnafuDigit(1)),
        2 => digits.push(SnafuDigit(2)),
        3 => digits.push(SnafuDigit(-2)),
        4 => digits.push(SnafuDigit(-1)),
        _ => panic!("Invalid input")
      }
    }

    if carry > 0 {
      digits.push(SnafuDigit(carry));
    }

    digits.reverse();

    Snafu { digits }
  }

  fn display(&self) {
    self.digits.iter().for_each(|d| d.display());
    println!();
  }

  fn to_string(&self) -> String {
    let mut result = String::new();
    for d in self.digits.iter() {
      result.push(match d {
        SnafuDigit(2) => '2',
        SnafuDigit(1) => '1',
        SnafuDigit(0) => '0',
        SnafuDigit(-1) => '-',
        SnafuDigit(-2) => '=',
        _ => panic!("Invalid input")
      });
    }

    result
  }

  fn to(&self) -> i64 {
    let mut result = 0;
    let mut power = 0;

    for digit in self.digits.iter().rev() {
      result += digit.0 as i64 * 5i64.pow(power);
      power += 1;
    }

    result
  }
}

struct Digit {
  value: i64,
}

impl Digit {
  fn new(ch: char) -> Digit {
    ch.to_string().parse::<i64>().map(|n| Digit { value: n }).unwrap()
  }

  fn display(&self) {
    print!("{}", self.value);
  }
}

struct Number {
  base: i64,
  digits: Vec<Digit>,
}

impl Number {
  fn new(str: &str, base: i64) -> Number {
    Number {
      digits: str.chars().map(|ch| Digit::new(ch)).collect(),
      base
    }
  }

  fn display(&self) {
    self.digits.iter().for_each(|d| d.display());
    println!();
  }

  fn from(numb: i64, base: i64) -> Number {
    let mut digits = Vec::new();
    let mut div = numb;

    if numb == 0 {
      return Number {
        digits: vec![Digit { value: 0 }],
        base
      }
    }

    while div > 0 {
      let rem = div.rem(base as i64);
      digits.push(Digit { value: rem as i64 });
      div = div.div(base as i64);
    }

    digits.reverse();

    Number {
      digits: digits,
      base
    }    
  }

  fn to(&self) -> i64 {
    let mut result = 0;
    let mut power = 0;

    for digit in self.digits.iter().rev() {
      result += digit.value as i64 * self.base.pow(power);
      power += 1;
    }

    result
  }
}


fn main() {
  for i in 0..10 {
    Number::from(i, 2).display()
  }

  for i in 0..26 {
    Number::from(i, 5).display()
  }

  for i in 0..26 {
    Snafu::from(i).display()
  }

  assert_eq!(Snafu::from(1).to_string(),               "1");
  assert_eq!(Snafu::from(2).to_string(),               "2");
  assert_eq!(Snafu::from(3).to_string(),              "1=");
  assert_eq!(Snafu::from(4).to_string(),              "1-");
  assert_eq!(Snafu::from(5).to_string(),              "10");
  assert_eq!(Snafu::from(6).to_string(),              "11");
  assert_eq!(Snafu::from(7).to_string(),              "12");
  assert_eq!(Snafu::from(8).to_string(),              "2=");
  assert_eq!(Snafu::from(9).to_string(),              "2-");
  assert_eq!(Snafu::from(10).to_string(),              "20");
  assert_eq!(Snafu::from(15).to_string(),             "1=0");
  assert_eq!(Snafu::from(20).to_string(),             "1-0");
  assert_eq!(Snafu::from(2022).to_string(),          "1=11-2");
  assert_eq!(Snafu::from(12345).to_string(),         "1-0---0");
  assert_eq!(Snafu::from(314159265).to_string(),   "1121-1110-1=0");

  assert_eq!(Snafu::new("1=-0-2").to(), 1747);
  assert_eq!(Snafu::new("12111").to(),       906);
  assert_eq!(Snafu::new("2=0=").to(),       198);
  assert_eq!(Snafu::new("21").to(),        11);
  assert_eq!(Snafu::new("2=01").to(),       201);
  assert_eq!(Snafu::new("111").to(),        31);
  assert_eq!(Snafu::new("20012").to(),      1257);
  assert_eq!(Snafu::new("112").to(),        32);
  assert_eq!(Snafu::new("1=-1=").to(),       353);
  assert_eq!(Snafu::new("1-12").to(),       107);
  assert_eq!(Snafu::new("12").to(),         7);
  assert_eq!(Snafu::new("1=").to(),         3);
  assert_eq!(Snafu::new("122").to(),        37);


  let input = fs::read_to_string("src/input25.txt").expect("Something went wrong reading the file");

  let sum: i64 = input.lines().map(|line| Snafu::new(line).to()).sum();

  println!("Sum: {}", sum);

  println!("SNAFU: {}", Snafu::from(sum).to_string());
}