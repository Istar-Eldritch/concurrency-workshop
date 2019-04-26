// Exercises:
// 1. Create two threads with infinite loops, one of them using the builder API. Name your thread.
// 2. Run your program and see if you can see it in your task manager. `eg. htop`
// You can check the stack allocated with valgrind
//
// !#/bin/bash
// valgrind --tool=massif  --massif-out-file=massif.out --stacks=yes ./target/debug/examples/01_create_thread && \
// grep mem_stacks_B massif.out | sed -e 's/mem_stacks_B=\(.*\)/\1/' | sort -g | tail -n 1 && \
// rm massif.out
//
use std::thread;

fn main() {
  let t = thread::spawn(move || {
      println!("Hello world");
      5
  });

  dbg!(t.join().unwrap());

}