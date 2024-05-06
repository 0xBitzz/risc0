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

use std::{env, path::Path};

use risc0_build_kernel::{KernelBuild, KernelType};

fn main() {
    build_cpu_kernels();

    if env::var("CARGO_FEATURE_CUDA").is_ok() {
        build_cuda_kernels();
    }

    if env::var("CARGO_FEATURE_METAL").is_ok() {
        build_metal_kernels();
    }
}

fn build_cpu_kernels() {
    KernelBuild::new(KernelType::Cpp)
        .files(glob::glob("cxx/*.cpp").unwrap().map(|x| x.unwrap()))
        .deps(glob::glob("cxx/*.h").unwrap().map(|x| x.unwrap()))
        .include(env::var("DEP_RISC0_SYS_CXX_ROOT").unwrap())
        .compile("circuit");
}

fn build_cuda_kernels() {
    KernelBuild::new(KernelType::CudaLink)
        .files(glob::glob("kernels/cuda/*.cu").unwrap().map(|x| x.unwrap()))
        .deps(glob::glob("kernels/cuda/*.h").unwrap().map(|x| x.unwrap()))
        .include(env::var("DEP_RISC0_SYS_CXX_ROOT").unwrap())
        .include(env::var("DEP_RISC0_SYS_CUDA_ROOT").unwrap())
        .compile("risc0_rv32im_cuda");
}

fn build_metal_kernels() {
    const SRCS: &[&str] = &[
        "eval_check.metal",
        "step_compute_accum.metal",
        "step_verify_accum.metal",
    ];

    let dir = Path::new("kernels").join("metal");
    let src_paths = SRCS.iter().map(|x| dir.join(x));

    KernelBuild::new(KernelType::Metal)
        .files(src_paths)
        .compile("metal_kernel");
}
