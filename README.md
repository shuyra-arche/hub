# hub

Public API for the Shuyra Hub Unit: messages, marks, grants, and
Hub/driver coordination encode/decode.

This repository is the API-only prerequisite for the Hub migration. It owns
wire encoding and decoding; it does not own runtime or hardware state. The Hub
keeps logical binding and admission authority, and the Driver keeps hardware
stop and recovery authority. Full Hub Unit migration remains deferred.

## Layout

- `api/` — `hub-api` 0.1.0, edition 2024, `no_std` + `alloc`.
- `tests/hub-api/` — four original API tests, four provider-local
  source-contract cases, and one public-path compatibility case.
- `tests/hub-coordination/` — ten original coordination tests.
- `tests/*/Cargo.toml` — separate test workspaces; each depends on `api/` by
  path and the required subset of public dependencies. Their dependency closure
  uses the library's same four public Git revisions.

Each ordinary type lives in its own file. `frame/mod.rs` and
`frame/dma.rs` are module/re-export surfaces; `api/src/lib.rs` keeps the
single `mold::interface` declaration as one macro input.

## Public pins

| Component | Repository | Revision |
| --- | --- | --- |
| `env` | `https://github.com/shuyra-arche/env.git` | `d3000253cd5d13d466e91fafd4dbb27a5ec5aaae` |
| `wire` | `https://github.com/shuyra-arche/wire.git` | `0ff77dc4573fa2620d2fe995a18c01949d9c7e57` |
| `mold` | `https://github.com/shuyra-arche/mold.git` | `f52501f4c91b686eded8b4bcd743b4e96084b7f9` |
| `system-api` | `https://github.com/shuyra-arche/system.git` | `9e5d63ca536a176583cf1b0054b4ecf7ddd88257` |

## Build

```sh
# Host library (explicit host target).
cargo build --locked --target x86_64-unknown-linux-gnu

# RV64 library, debug and release.
cargo build --locked --target riscv64gc-unknown-none-elf --lib
cargo build --locked --release --target riscv64gc-unknown-none-elf --lib
```

## Test

```sh
cd tests/hub-api
cargo test --locked --target x86_64-unknown-linux-gnu

cd ../hub-coordination
cargo test --locked --target x86_64-unknown-linux-gnu
```

Add `--offline` after the dependency cache is prepared.

## Provenance and license

See `PROVENANCE.md` for the frozen source commit, authors, file map, hash
proof, and the exact source-contract mapping. No license file was present in
the source tree at the frozen commit; this snapshot does not add or imply a
license or copyright permission.
