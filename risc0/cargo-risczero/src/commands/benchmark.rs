// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::time::{Duration, Instant};

use risc0_zkvm_methods::{
    bench::{BenchmarkSpec, SpecWithIters},
    BENCH_ELF,
};

use risc0_zkvm::{
    get_prover_server,
    recursion::{join, lift},
    ExecutorEnv, ExecutorImpl, ProverOpts, VerifierContext,
};

use anyhow::Result;
use clap::Parser;

/// `cargo risczero benchmark`
#[derive(Parser)]
pub struct BenchmarkCommand {
    /// Number of iterations.
    #[arg(short, long)]
    pub iterations: Option<u64>,

    /// Which hash function to use.
    #[arg(short = 'f', long, default_value_t = String::from("poseidon"), value_parser = ["poseidon", "sha-256"])]
    pub hashfn: String,

    /// Specify the segment po2.
    #[arg(short, long, default_value_t = 20)]
    po2: u32,
}

impl BenchmarkCommand {
    /// Execute this command.
    pub fn run(&self) -> Result<()> {
        // TODO: Handle the case where the user does not specify the number of iterations
        let iterations = SpecWithIters(BenchmarkSpec::SimpleLoop, self.iterations.unwrap_or(4 * 1024));
        let env = ExecutorEnv::builder()
            .write(&iterations)?
            .segment_limit_po2(self.po2)
            .build()?;
        let mut exec = ExecutorImpl::from_elf(env, BENCH_ELF)?;

        // Execute
        let (session, exec_duration) = with_duration(|| exec.run())?;

        let cycles = session.get_cycles()?;
        let segments = session.resolve()?;

        let opts = ProverOpts::default();
        let ctx = VerifierContext::default();
        let prover = get_prover_server(&opts)?;

        let mut lifts = vec![];
        let mut prove_durations = vec![];
        let mut lift_durations = vec![];

        // Prove and Lift
        for segment in segments.iter() {
            let (receipt, receipt_duration) = with_duration(|| prover.prove_segment(&ctx, segment))?;
            prove_durations.push(receipt_duration);

            let (lift, lift_duration) = with_duration(|| lift(&receipt))?;
            lifts.push(lift);
            lift_durations.push(lift_duration);
        }

        let mut join_durations = vec![];
        // Optional Join
        if segments.len() > 1 {
            let (_final, duration) = with_duration(|| join(&lifts[0], &lifts[1]))?;
            join_durations.push(duration);
        }

        println!("\nSTATS:");
        println!("cycles:     {}", cycles.1);
        println!("segments:   {}", segments.len());
        println!("exec:       {exec_duration:?}");
        println!("prove:      {prove_durations:?}");
        println!("lift:       {lift_durations:?}");
        println!("prove+lift: {:?}", prove_durations[0] + lift_durations[0]);
        println!("join:       {join_durations:?}");

        Ok(())
    }
}

fn with_duration<T, F: FnOnce() -> Result<T>>(f: F) -> Result<(T, Duration)> {
    let start = Instant::now();
    let result = f()?;
    let duration = start.elapsed();
    Ok((result, duration))
}
