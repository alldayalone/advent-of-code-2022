use std::fs;

#[derive(Clone)]
enum MonkeyOp<'monkey_op> {
  Add(&'monkey_op str, &'monkey_op str),
  Sub(&'monkey_op str, &'monkey_op str),
  Div(&'monkey_op str, &'monkey_op str),
  Mul(&'monkey_op str, &'monkey_op str),
  Yell(i64)
}

#[derive(Clone)]
struct Monkey<'monkey> {
  name: &'monkey str,
  op: MonkeyOp<'monkey>,
  result: Option<i64>
}

fn get_result(monkeys: &mut Vec<Monkey>, monkey_name: &str) -> i64 {
  let monkey = monkeys.iter_mut().find(|m| m.name == monkey_name).expect(format!("Monkey {} exists", monkey_name).as_str()).clone();

  match monkey.result {
    Some(result) => {
      result
    },
    None => {
      match monkey.op {
        MonkeyOp::Add(lhs, rhs) => {
          let lhs_result = get_result(monkeys, lhs);
          let rhs_result = get_result(monkeys, rhs);
          let result = lhs_result + rhs_result;

          let mut monkey_mut = monkeys.iter_mut().find(|m| m.name == monkey_name).unwrap();

          monkey_mut.result = Some(result);
          result
        },
        MonkeyOp::Sub(lhs, rhs) => {
          let lhs_result = get_result(monkeys, lhs);
          let rhs_result = get_result(monkeys, rhs);
          let result = lhs_result - rhs_result;

          let mut monkey_mut = monkeys.iter_mut().find(|m| m.name == monkey_name).unwrap();


          monkey_mut.result = Some(result);
          result
        },
        MonkeyOp::Div(lhs, rhs) => {
          let lhs_result = get_result(monkeys, lhs);
          let rhs_result = get_result(monkeys, rhs);
          let result = lhs_result / rhs_result;

          let mut monkey_mut = monkeys.iter_mut().find(|m| m.name == monkey_name).unwrap();


          monkey_mut.result = Some(result);
          result
        },
        MonkeyOp::Mul(lhs, rhs) => {
          let lhs_result = get_result(monkeys, lhs);
          let rhs_result = get_result(monkeys, rhs);
          let result = lhs_result * rhs_result;

          let mut monkey_mut = monkeys.iter_mut().find(|m| m.name == monkey_name).unwrap();

          monkey_mut.result = Some(result);
          result
        },
        MonkeyOp::Yell(result) => {
          result
        }
      }
    }
  }

}

fn main() {
  let input = fs::read_to_string("src/input21.txt").unwrap();
  let mut monkeys = input.lines().map(|line| {
    let (name, op) = line.split_once(": ").unwrap();

    if op.contains("+") {
      let (lhs, rhs) = op.split_once(" + ").unwrap();
      Monkey { name, op: MonkeyOp::Add(lhs, rhs), result: None }
    } else if op.contains("-") {
      let (lhs, rhs) = op.split_once(" - ").unwrap();
      Monkey { name, op: MonkeyOp::Sub(lhs, rhs), result: None }
    } else if op.contains("/") {
      let (lhs, rhs) = op.split_once(" / ").unwrap();
      Monkey { name, op: MonkeyOp::Div(lhs, rhs), result: None }
    } else if op.contains("*") {
      let (lhs, rhs) = op.split_once(" * ").unwrap();
      Monkey { name, op: MonkeyOp::Mul(lhs, rhs), result: None }
    } else {
      let lhs = op.parse::<i64>().unwrap();
      Monkey { name, op: MonkeyOp::Yell(lhs), result: Some(lhs) }
    }
  }).collect::<Vec<_>>();

  let result = get_result(&mut monkeys, "root");

  println!("Root yells: {}", result);
}