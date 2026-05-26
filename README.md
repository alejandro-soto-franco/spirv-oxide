# spirv-oxide

A custom rustc backend for compiling GPU compute kernels in pure Rust to
SPIR-V. Sibling project to [cuda-oxide](https://github.com/NVlabs/cuda-oxide),
sharing the Pliron MLIR-style IR for an eventual unified
Rust→{PTX, SPIR-V} pipeline.

## Status

Pre-alpha. Workspace scaffold only. No functioning codegen yet. Tracks
the same Pliron rev as cuda-oxide so the two projects can share a future
Pliron-GPU dialect crate.

## Why

The Rust ecosystem has:

- `rust-gpu` (Embark / community fork): forked rustc backend, Rust→SPIR-V.
  Mature for shader work. Direct backend, no MLIR-style middle.
- `cuda-oxide` (NVIDIA Labs): Rust→MIR→Pliron→LLVM→PTX. NVIDIA-only.
- `CubeCL`: procedural macro DSL inside Rust, multi-backend. Not a rustc
  backend; you write a sub-language.

None of these gives you a single Rust kernel source that compiles to BOTH
PTX and SPIR-V through a shared MLIR-style middle. That is the gap
spirv-oxide aims at, by mirroring cuda-oxide's pipeline architecture
(Rust → MIR → Pliron IR → backend) but swapping the LLVM/PTX back half for
a SPIR-V back half.

## Architecture target

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

The Pliron-middle is the strategic bet. spirv-oxide is its second consumer,
proving the IR is target-portable. The two host runtimes diverge below
that line: cuda-host wraps cudarc; spirv-host wraps ash.

## Workspace

| Crate | Purpose |
|---|---|
| `crates/spirv-device` | Device-side abstractions: kernel marker, thread indexing, shared memory, atomics, barriers. Mirrors cuda-oxide's `cuda-device`. |
| `crates/spirv-host` | Host-side runtime: ash Vulkan device, module loading, kernel launch. Mirrors cuda-oxide's `cuda-host`. |
| `crates/dialect-spirv` | Pliron dialect for SPIR-V ops. Mirrors cuda-oxide's `dialect-nvvm`. |
| `crates/rustc-codegen-spirv` | (planned) Forked rustc backend that lowers Pliron-IR to SPIR-V via the SPIR-V dialect. Not in the workspace; built separately. |

## Relationship to existing projects

**cuda-oxide.** spirv-oxide is a sibling, not a fork. Both projects depend
on the same `pliron` rev. Cross-pollination happens via cherry-pick. The
near-term plan is to upstream small refactors to cuda-oxide that factor
its Pliron usage into a reusable dialect interface, then have both
projects consume that interface. See `docs/relationship-to-cuda-oxide.md`.

**rust-gpu.** spirv-oxide will likely borrow from rust-gpu's SPIR-V
emission layer where the work overlaps. rust-gpu does not use Pliron;
the novel piece in spirv-oxide is the Pliron-middle, not the SPIR-V
codegen itself.

**gpufft.** [gpufft](https://github.com/alejandro-soto-franco/gpufft)
provides a typed Rust GPU FFT trait over VkFFT (Vulkan) and cuFFT (CUDA).
A working spirv-oxide could eventually replace gpufft's vendored VkFFT
C++ blob with SPIR-V kernels emitted from Rust, removing the FFI shim
entirely. That is a v2.0 horizon, not a near-term goal.

## License

Apache-2.0. See `LICENSE-APACHE`.

This project is an independent sibling to cuda-oxide. It shares the
Pliron dependency and architectural pattern but no code under the
NVIDIA-specific portions of cuda-oxide's license.
