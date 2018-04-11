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

fn bind_to_cpu(cpuid: usize) {
    let mut cpuset = nix::sched::CpuSet::new();

    match cpuset.set(cpuid) {
      Ok(_) => (),
      Err(_) => panic!("Can't modify CpuSet")
    }

    let pid = nix::unistd::getpid();
    match nix::sched::sched_setaffinity(pid, &cpuset) {
      Ok(_) => (),
      Err(_) => panic!("Can't set affinity")
    }
}

fn main() {
    const NUM_CORES: usize = 24;  // TODO: get dynamically

    println!("🎉  let's get this party started 🎉");

    for cpuid in 0..(NUM_CORES-1) {
      bind_to_cpu(cpuid);

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
