use std::fs;

fn mix(vec: &mut Vec<(usize, i64)>, i: &usize) {
  let len = vec.len() as i64;
  let (pos, element) = vec.iter().enumerate().find(|(_, (j,_))| j == i).unwrap();
  let shift = element.1.rem_euclid(len - 1);

  // println!("pos: {}, element: {:?}, shift: {}", pos, element, shift);

  let new_pos = if pos as i64 + shift + 1 < len {
    pos + shift as usize + 1
  } else {
    (pos as i64 + shift - (len - 1)) as usize
  };

  vec.splice(new_pos..new_pos, [*element]);

  if pos < new_pos {
    vec.remove(pos);
  } else {
    vec.remove(pos + 1);
  }
}

fn main () {
  // Unit test mix fn
//   let mut vec = vec![-1,2,3,4];
//   mix(&mut vec, &-1);
//   assert_eq!(vec, vec![2,3,-1,4]);

//   vec = vec![-4,2,3,4];
//   mix(&mut vec, &-4);
//   assert_eq!(vec, vec![2,3,-4,4]);

//   vec = vec![-7,2,3,4];
//   mix(&mut vec, &-7);
//   assert_eq!(vec, vec![2,3,-7,4]);

//   vec = vec![5,2,3,4];
//   mix(&mut vec, &5);
//   assert_eq!(vec, vec![2,3,5,4]);

//   vec = vec![8,2,3,4];
//   mix(&mut vec, &8);
//   assert_eq!(vec, vec![2,3,8,4]);
//  //
//   vec = vec![2,-4,3,4];
//   mix(&mut vec, &-4);
//   assert_eq!(vec, vec![-4,2,3,4]);

//   vec = vec![2,-1,3,4];
//   mix(&mut vec, &-1);
//   assert_eq!(vec, vec![-1,2,3,4]);

//   vec = vec![2,8,3,4];
//   mix(&mut vec, &8);
//   assert_eq!(vec, vec![8,2,3,4]);

//   vec = vec![2,5,3,4];
//   mix(&mut vec, &5);
//   assert_eq!(vec, vec![5,2,3,4]);

//   vec = vec![2,-7,3,4];
//   mix(&mut vec, &-7);
//   assert_eq!(vec, vec![-7,2,3,4]);

//   vec = vec![2,3,-4,4];
//   mix(&mut vec, &-4);
//   assert_eq!(vec, vec![2,-4,3,4]);  
  
  let input = fs::read_to_string("src/input20.txt").expect("File does not exist");
  let mut mixed = input.lines().map(|l| l.parse::<i64>().expect("Line should be a number") * 811589153).enumerate().collect::<Vec<_>>();
  // println!("{:?}", mixed);

  for _ in 0..10 {
    for i in 0..mixed.len() {
      mix(&mut mixed, &i);
      // println!("{:?}: {}", mixed, i);
    }
  }

  let len = mixed.len() as i64;

  let zero_pos = mixed.iter().position(|(_,m)| m == &0).unwrap();
  let n1000 = mixed.get((zero_pos + 1000) % len as usize).unwrap();
  let n2000 = mixed.get((zero_pos + 2000) % len as usize).unwrap();
  let n3000 = mixed.get((zero_pos + 3000) % len as usize).unwrap();
  let answer = n1000.1 + n2000.1 + n3000.1;
  
  println!("n1000: {:?}, n2000: {:?}, n3000: {:?}, answer: {}", n1000, n2000, n3000, answer);
}