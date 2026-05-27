# Relationship to cuda-oxide

spirv-oxide is an independent sibling project to
[cuda-oxide](https://github.com/NVlabs/cuda-oxide) (NVIDIA Labs). The
two projects share a strategic bet: that
[Pliron](https://github.com/vaivaswatha/pliron) is the right MLIR-style
intermediate representation for compiling pure Rust to GPU code.

## Shared dependencies

Both projects pin to the same `pliron` git rev. This is intentional. It
means a GPU op defined in one project's dialect uses the same Pliron
type system, attribute system, and IR walking machinery as the other.
If the rev diverges, the two projects can no longer share dialect code
without a manual port.

The current shared rev lives in `[workspace.dependencies]` of both
projects' `Cargo.toml`. Keep them in sync when bumping.

## Long-term plan

The goal is a Pliron-GPU dialect crate consumed by both backends. The
work decomposes into three upstream contributions to cuda-oxide,
proposed in order:

1. **Factor the Pliron usage in `dialect-nvvm` into a target-agnostic
   trait.** Today the dialect mixes NVPTX-specific ops (warp shuffles,
   tensor memory accelerator, cluster operations) with generic GPU ops
   (work-item ID, barriers, atomics). The first PR pulls the generic
   subset behind an interface.

2. **Refactor `mir-lower` to consume the dialect interface.** Once
   `dialect-nvvm` implements the interface from (1), `mir-lower` can
   target it polymorphically. Behavior preserved; the abstraction is
   the deliverable.

3. **Propose extracting the dialect interface into a shared crate.**
   At this point spirv-oxide's `dialect-spirv` implements the same
   interface as the refactored `dialect-nvvm`, and `mir-lower` already
   consumes it polymorphically. The shared crate (working name:
   `dialect-gpu` or `pliron-gpu`) lives in cuda-oxide's tree, with
   spirv-oxide consuming it via path-require or git dep.

Each PR is in scope for cuda-oxide because it strengthens the existing
project without expanding to non-NVIDIA targets. spirv-oxide picks up
the shared crate as soon as it lands.

## Cross-pollination

Local development can cherry-pick across the two repos freely. The
typical operations:

```bash
# In ~/spirv-oxide, pick a single commit from cuda-oxide:
git remote add cuda-oxide ~/cuda-oxide   # local filesystem remote
git fetch cuda-oxide
git cherry-pick <sha>

# In ~/cuda-oxide, pick a single commit from spirv-oxide:
git remote add spirv-oxide ~/spirv-oxide
git fetch spirv-oxide
git cherry-pick <sha>
```

When a cherry-picked change touches the Pliron dialect, file an upstream
PR to cuda-oxide that lands the change in canonical form. Then drop the
local cherry-pick from spirv-oxide once it's consuming the shared crate.

## Naming

The naming pattern `<api>-oxide` follows cuda-oxide's lead, where
`oxide` signals Rust and `<api>` names the surface that the host runtime
targets. spirv-oxide's host runtime targets Vulkan compute via SPIR-V;
the IR (SPIR-V) is named directly because it can also be consumed by
OpenCL drivers, OpenGL compute, and (via naga) WebGPU pipelines.
