# spirv-oxide

**One Rust kernel source, compiled to SPIR-V via a Pliron MLIR-style IR.**
Sibling to [cuda-oxide](https://github.com/NVlabs/cuda-oxide). Both
projects pin the same Pliron rev so a single kernel definition can
eventually lower to both PTX (cuda-oxide) and SPIR-V (spirv-oxide) from a
shared dialect.

SPIR-V is the portable IR for Vulkan compute, OpenCL, OpenGL compute, and
(via `naga`) WebGPU. spirv-oxide lets you write the kernel in regular
Rust and target all of them.

## Status

Pre-alpha. Workspace scaffold only. No functioning codegen yet.

## The bet

The Rust GPU ecosystem today fragments along three axes:

| Tool | Approach | Target |
|---|---|---|
| `rust-gpu` (Embark / community) | forked rustc backend, direct emission | SPIR-V |
| `cuda-oxide` (NVIDIA Labs) | Rust → MIR → Pliron → LLVM → PTX | NVIDIA only |
| `CubeCL` | procedural macro DSL inside Rust | wgpu / CUDA / HIP |

Nothing today lets you write one Rust kernel and lower it to both PTX and
SPIR-V through a shared IR. spirv-oxide closes that gap by reusing
cuda-oxide's MIR → Pliron front half and adding a SPIR-V back half. The
strategic claim is that Pliron is the right canonical IR for Rust GPU
codegen, and that two consumers (PTX and SPIR-V) prove the IR is
target-portable.

## Architecture

```
                              ┌──────────────────┐
                              │   user kernel    │ (regular Rust)
                              └────────┬─────────┘
                                       ▼
                              ┌──────────────────┐
                              │      MIR         │ (rustc)
                              └────────┬─────────┘
                                       ▼
                              ┌──────────────────┐
                              │   Pliron IR      │ (shared with cuda-oxide)
                              └────┬─────────┬───┘
                                   ▼         ▼
                            ┌──────────┐ ┌─────────────┐
                            │ LLVM→PTX │ │   SPIR-V    │
                            │ (cuda-   │ │ (spirv-     │
                            │  oxide)  │ │  oxide)     │
                            └──────────┘ └─────────────┘
                                   │         │
                                   ▼         ▼
                            ┌──────────┐ ┌─────────────┐
                            │  cuFFT,  │ │  VkFFT,     │
                            │  cudarc, │ │  ash,       │
                            │  cuda-   │ │  wgpu, ...  │
                            │  host    │ │             │
                            └──────────┘ └─────────────┘
```

cuda-oxide owns the upper half (Rust → MIR → Pliron). spirv-oxide adds a
SPIR-V back half. The two host runtimes diverge below that line:
`cuda-host` wraps cudarc; `spirv-host` wraps ash.

## Workspace

| Crate | Purpose |
|---|---|
| `crates/spirv-device` | Device-side abstractions: kernel marker, thread indexing, shared memory, atomics, barriers. Mirrors cuda-oxide's `cuda-device`. |
| `crates/spirv-host` | Host-side runtime: ash Vulkan device, module loading, kernel launch. Mirrors cuda-oxide's `cuda-host`. |
| `crates/dialect-spirv` | Pliron dialect for SPIR-V ops. Mirrors cuda-oxide's `dialect-nvvm`. |
| `crates/rustc-codegen-spirv` | (planned) Forked rustc backend that lowers Pliron IR to SPIR-V via the SPIR-V dialect. Built separately because it needs `rustc-dev` and a custom toolchain. |

## Relationship to existing projects

**cuda-oxide.** Independent sibling project. Both depend on the same
`pliron` rev. Cross-pollination via cherry-pick. The near-term plan is
to upstream small refactors that factor cuda-oxide's Pliron usage into a
reusable dialect interface, so both backends can later consume a shared
`dialect-gpu` crate. See `docs/relationship-to-cuda-oxide.md`.

**rust-gpu.** Likely to borrow from its SPIR-V emission layer where the
work overlaps. rust-gpu uses a direct rustc backend with no MLIR-style
middle; the novel piece in spirv-oxide is the Pliron-middle, so the
codegen layer itself is the boring part.

**gpufft.** [gpufft](https://github.com/alejandro-soto-franco/gpufft)
ships a typed Rust GPU FFT trait over VkFFT (Vulkan) and cuFFT (CUDA).
A working spirv-oxide could eventually replace gpufft's vendored VkFFT
C++ blob with SPIR-V kernels emitted from Rust, removing the FFI shim
entirely. v2.0 horizon, far from a near-term goal.

## License

Apache-2.0. See `LICENSE-APACHE`.

This project is an independent sibling to cuda-oxide. It shares the
Pliron dependency and architectural pattern. It carries no code from
the NVIDIA-specific portions of cuda-oxide.
