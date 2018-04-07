#![feature(duration_extras)]
#![feature(asm)]

// TODO: take into account existing load of other processes using feedback loop
// TODO: multiple threds
// TODO: audio input lol

use std::thread;
use std::time;
extern crate nix;

fn compute() { 
    unsafe { asm!("PAUSE") } 
}

fn main() {
    const NUM_CORES: usize = 24;  // TODO: get dynamically

    let mut cpuset = nix::sched::CpuSet::new();

    let pid = nix::unistd::getpid();
    println!("Hello, world!");

    for cpuid in 0..(NUM_CORES-1) {
      for n in 0..(NUM_CORES-1) {
        match cpuset.unset(n) {
          Ok(_) => (),
          Err(_) => panic!("can't unset")
        }
      }

      match cpuset.set(cpuid) {
        Ok(_) => (),
        Err(_) => panic!("can't set")
      }

      match nix::sched::sched_setaffinity(pid, &cpuset) {
        Ok(_) => (),
        Err(_) => panic!("can't set affinity")
      }

      let period = time::Instant::now();
      while period.elapsed().as_secs() < 1 {
        let work = time::Instant::now();
        while work.elapsed().subsec_millis() < 10 {
            compute();
        }

        thread::sleep(std::time::Duration::new(0, 20_000_000));
      }
    }
}
