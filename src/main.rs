#![feature(duration_extras)]
#![feature(asm)]

// TODO: take into account existing load of other processes using feedback loop
// TODO: audio input lol

use std::thread;
use std::time;
use std::f32::consts::PI;
extern crate nix;

const SCHED_SLICE_MS: usize = 40;

fn compute() { 
    unsafe { asm!("PAUSE") } 
}

fn bind_to_cpu(cpuid: usize) {
    let mut cpuset = nix::sched::CpuSet::new();

    match cpuset.set(cpuid) {
      Ok(_) => (),
      Err(_) => panic!("Can't modify CpuSet")
    }

    let tid = nix::unistd::gettid();
    match nix::sched::sched_setaffinity(tid, &cpuset) {
      Ok(_) => (),
      Err(_) => panic!("Can't set affinity")
    }
}

fn worker(cpuid: usize) {
    bind_to_cpu(cpuid);

    let duty: f32 = (PI * (cpuid as f32)/24.0).sin();

    let lifetime = time::Instant::now();
    while lifetime.elapsed().as_secs() < 4 {
      let work_time_ms = (duty * (SCHED_SLICE_MS as f32)) as u32;
      let sleep_time_us = ((1.0 - duty) * (SCHED_SLICE_MS as f32) * 1e6).floor() as u32;

      let work = time::Instant::now();
      while work.elapsed().subsec_millis() < work_time_ms {
          compute();
      }

      thread::sleep(std::time::Duration::new(0, sleep_time_us)); 
    }
}

fn main() {
    const NUM_CORES: usize = 24;  // TODO: get dynamically
    let mut workers = Vec::with_capacity(NUM_CORES);

    println!("🎉  let's get this party started 🎉");

    for cpuid in 0..NUM_CORES {
      match thread::Builder::new().name(format!("worker for core{}", cpuid))
                  .spawn(move || { worker(cpuid) }) {
        Ok(child) => workers.push(child),
        Err(_) => panic!("Can't spawn thread")
      }
    }

  workers.into_iter().for_each(|w| { 
    match w.join() {
      Ok(_) => (),
      Err(_) => panic!("Can't join")
    }
  });
}
