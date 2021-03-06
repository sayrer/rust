/*
  A simple way to make sure threading works. This should use all the
  CPU cycles an any machines that we're likely to see for a while.
*/

// xfail-stage0
// xfail-stage1
// xfail-stage2
// xfail-stage3

use std;
import std::task::join;

fn loop(int n) {
  let task t1;
  let task t2;

  if(n > 0) {
    t1 = spawn loop(n - 1);
    t2 = spawn loop(n - 1);
  }

  while(true) {}
}

fn main() {
  let task t = spawn loop(5);
  join(t);
}
