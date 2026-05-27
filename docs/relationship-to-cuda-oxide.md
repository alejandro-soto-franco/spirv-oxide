# Relationship to cuda-oxide

spirv-oxide is an independent project with its own roadmap. The only
formal coupling to
[cuda-oxide](https://github.com/NVlabs/cuda-oxide) (NVIDIA Labs) is that
both projects depend on the same revision of
[Pliron](https://github.com/vaivaswatha/pliron). Sharing the rev lets a
SPIR-V dialect here use the same IR primitives that NVIDIA's PTX dialect
uses there.

## Pinned Pliron rev

Both `Cargo.toml` files reference the same `pliron` git rev. Keeping
them in sync is a manual choice; bump together if at all. The current
rev lives in `[workspace.dependencies]`.

## Cherry-pick across local checkouts

During development it can be useful to move a single commit between
local checkouts of the two projects:

```bash
# Pick a commit from a local cuda-oxide clone into spirv-oxide:
cd ~/spirv-oxide
git remote add cuda-oxide ~/cuda-oxide   # local filesystem remote
git fetch cuda-oxide
git cherry-pick <sha>
```

The local-filesystem remote keeps the cross-pollination out of either
GitHub origin. Drop the remote with `git remote remove cuda-oxide` when
the work is done.

## Scope clarifications

- Git history started fresh in this repo. There is no shared commit
  lineage with cuda-oxide.
- Contributions to cuda-oxide go through the contributor's own cuda-oxide
  fork, on its own branches. That is an unrelated workstream.
- Both projects target Pliron directly. spirv-oxide does not consume
  cuda-oxide's published crates.

## Naming

The naming pattern `<api>-oxide` follows cuda-oxide's lead, where
`oxide` signals Rust and `<api>` names the surface that the host runtime
targets. spirv-oxide's host runtime targets Vulkan compute via SPIR-V;
the IR (SPIR-V) is named directly because it can also be consumed by
OpenCL drivers, OpenGL compute, and (via naga) WebGPU pipelines.
