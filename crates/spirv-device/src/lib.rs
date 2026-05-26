//! Device-side abstractions for spirv-oxide GPU kernels.
//!
//! Pre-alpha scaffold. The eventual surface will mirror cuda-oxide's
//! `cuda-device` crate:
//!
//! - `#[kernel]` attribute marking a function as a GPU entry point.
//! - `thread::index_1d`, `index_2d`, `index_3d` for invocation IDs.
//! - `DisjointSlice<T>` for safe partitioned access to a buffer.
//! - Shared memory (`workgroup` storage class), atomics, barriers.
//!
//! The actual lowering to SPIR-V lives in the (planned)
//! `rustc-codegen-spirv` backend, which is not a workspace member because
//! it requires `rustc-dev` and a custom rustc toolchain.

#![no_std]
