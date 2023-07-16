import * as assert from 'assert';
function mix(vec, element) {
  const len = vec.length;
  const pos = vec.findIndex((m) => m === element);
  let rem = (pos + element) % len;

  if (rem < 0) {
    rem = rem + len - 1;
  }

  const new_pos = rem;

  vec.splice(pos, 1);
  vec.splice(new_pos, 0, element);
}

let vec = [1, 2, 3, 4, 5];
let element = 3;
mix(vec, element);
assert.equal(vec[3], element);