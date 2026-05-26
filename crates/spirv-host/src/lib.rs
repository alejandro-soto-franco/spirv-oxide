//! Host-side runtime for spirv-oxide.
//!
//! Pre-alpha scaffold. The eventual surface will mirror cuda-oxide's
//! `cuda-host`:
//!
//! - `VulkanContext` (ash-based device + queue + descriptor pool).
//! - `DeviceBuffer<T>` (typed device-local storage).
//! - Module loading from SPIR-V bytecode produced by the codegen backend.
//! - Kernel launch with workgroup count + push constants + descriptor sets.
//! - Pinned host transfers + stream-ordered copies.
//!
//! The handoff with [`spirv-device`](spirv_device) is the
//! `#[kernel] fn name(...)` declaration: spirv-host loads the compiled
//! SPIR-V module and dispatches it.
