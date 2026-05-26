//! Pliron IR dialect for SPIR-V operations.
//!
//! Pre-alpha scaffold. The eventual surface will define:
//!
//! - SPIR-V types: `Float`, `Int`, `Vector`, `Array`, `RuntimeArray`,
//!   `Pointer<StorageClass>`, `Image`, `SampledImage`.
//! - Storage classes: `Workgroup`, `StorageBuffer`, `Uniform`,
//!   `PushConstant`, `Private`, `Function`.
//! - Capability declarations + extension imports.
//! - Memory + barrier ops: `OpControlBarrier`, `OpMemoryBarrier`,
//!   `OpAtomic*`.
//! - Compute ops: `OpExecutionMode LocalSize ...`, work-item builtins
//!   (`GlobalInvocationId`, `LocalInvocationId`, `WorkgroupId`).
//!
//! Tracks the same `pliron` rev as cuda-oxide's `dialect-nvvm` so a
//! future refactor can factor the GPU-common subset (work-item ops,
//! barriers, atomics) into a shared `dialect-gpu` crate consumed by
//! both backends.

#![no_std]
