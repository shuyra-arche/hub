# Provenance

## Source checkpoint

- Repository: `https://github.com/anreonyr/sqware.git`
- Public pull request: **#43**
- Frozen source HEAD: `89ecffd0f8ce2077cd757d9561be141c87b8ec3b`
- Frozen commit subject: `Share allocator mechanisms through a Kernel-owned package`
- Commit author: `anreonyr <anreonyr@gmail.com>`
- API subtree: `programs/src/service/hub/api`
- Test subtrees: `programs/tests/hub-api`, `programs/tests/hub-coordination`
- Toolchain copied from source: `nightly-2026-09-04`, edition 2024, resolver 3.

The source working tree was clean at this commit before and after this
snapshot. No source file was written by this preparation.

## Authors

Git history for the API subtree records these author identities:

- `anreonyr <anreonyr@gmail.com>`
- `anreonyr <88045926+anreonyr@users.noreply.github.com>`

The original history remains at `anreonyr/sqware`; this repository is a
fresh snapshot, not a history rewrite.

## License

No `LICENSE`, `COPYING`, or `NOTICE` file is tracked at the frozen source
commit. The source `Cargo.toml` declares neither `authors` nor `license`.
This snapshot therefore adds no license and asserts no copyright permission.
Do not infer a license from publication or from this document.

## API move

The source API is 19 Rust files. This provider has 34 Rust files: 13 source
files are byte-identical, and 6 source files were mechanically split into
one-type-per-file children with public re-exports preserving every original
public path. One item, `frame::dma::DmaSaid`, remains in its original
`dma_said.rs`.

Mechanical proof is recorded in
`evidence/pr21/provider/move-proof.json` (outside this repository):

- 13 / 13 unchanged Rust files have equal SHA-256 source and provider bytes.
- 18 / 18 moved item blocks occur byte-for-byte in their target files.
- 19 source Rust files map to 34 provider Rust files with no unaccounted file.

`api/Cargo.toml` is byte-identical to the source manifest. The two test
manifests differ only in the `hub-api` path (`../../src/service/hub/api` to
`../../api`).

### Split files

| Source file | Provider files | Source SHA-256 | Provider SHA-256 |
| --- | --- | --- | --- |
| `src/frame/mod.rs` | `frame/mod.rs`, `frame/{wire,said,deed,window}.rs`; Message impls appended to `frame/{bond,list,claim}.rs` | `382c692a6314289f31b3474641626fcfdfc31f49c513fc0d127533694272d993` | `5b2c58bacd026b470ee1bb352d25d00ce40bf8a07093285885e27f596bdf6f59` |
| `src/frame/inventory.rs` | `frame/inventory.rs`, `frame/inventory/snapshot.rs` | `fa0606319410687694f8926eea38540a0dce66b24ace0644b080b4a9ef86329c` | `84b44d862bf7f97fdffd37515c74d4e2f62ac0d3ff9ce50fdcc39aa908b6951b` |
| `src/frame/dma.rs` | `frame/dma.rs`, ten `frame/dma/*.rs` children | `2e29a5853aa906e75d23f1d7214a17d6c3e8586c5312dc29ffe3586dab3aeba2` | `89bc91e3420c6c1bce0c46df2d9380085042bf1247f16d6c12aaf198aa97cb0f` |
| `src/frame/bond.rs` | `frame/bond.rs` + `impl Message for Bond` | `772bc351be52f3777c56ee82252d0884fafe5edaa89017ad465d062928a76d79` | `8c134d0f028a1fe189501039ee045f27f4b5c9dd3442e5167cff72311fbe6fe8` |
| `src/frame/list.rs` | `frame/list.rs` + `impl Message for ListReq` | `ef568d67a1b8b8cddbc1f9ed4f54e7611fb2a33b3595e298c3ccb6e77056ab8a` | `761fc266a00c2b98c200a5845ff496a907d93ca4b06c7c4745a9d96c2d334371` |
| `src/frame/claim.rs` | `frame/claim.rs` + `impl Message for Claim` | `c5743dbcddef38be94991db20cb1f6f1fdd327a5b7e463ef4f27e68ea2fbbae9` | `2b05a8420ec3f7631fc8c95ac81ad332da0432a223bab55b1f7757fb2938c34f` |

Item-level moves inside the split files:

| Source item / range | Provider target |
| --- | --- |
| `frame/mod.rs:33-47` `impl Message for Bond` | `frame/bond.rs` |
| `frame/mod.rs:49-63` `impl Message for ListReq` | `frame/list.rs` |
| `frame/mod.rs:65-81` `impl Message for Claim` | `frame/claim.rs` |
| `frame/mod.rs:83-124` `Wire` + impl | `frame/wire.rs` |
| `frame/mod.rs:126-155` `Said` + impl | `frame/said.rs` |
| `frame/mod.rs:157-213` `Deed` + impl | `frame/deed.rs` |
| `frame/mod.rs:215-275` `Window`, `WINDOW_LEN`, impl | `frame/window.rs` |
| `frame/inventory.rs:35-135` `Inventory` + impl | `frame/inventory/snapshot.rs` |
| `frame/dma.rs:21-27` `BindingId` | `frame/dma/binding_id.rs` |
| `frame/dma.rs:29-61` `BindingState` + impl | `frame/dma/binding_state.rs` |
| `frame/dma.rs:63-85` `Close` + impl | `frame/dma/close.rs` |
| `frame/dma.rs:87-93` `Failure` | `frame/dma/failure.rs` |
| `frame/dma.rs:102-112` `Enroll` | `frame/dma/enroll.rs` |
| `frame/dma.rs:117-129` `DmaClaim` | `frame/dma/dma_claim.rs` |
| `frame/dma.rs:131-139` `CloseBinding` | `frame/dma/close_binding.rs` |
| `frame/dma.rs:141-148` `Inspect` | `frame/dma/inspect.rs` |
| `frame/dma.rs:150-156` `Settle` | `frame/dma/settle.rs` |
| `frame/dma.rs:158-221` `DmaWire` + impl | `frame/dma/dma_wire.rs` |

The original `frame::dma.rs` keeps `Call`, the operation and party constants,
`DMA_BACK`, and the private `reply_to`, so the `mold::contract` expansion scope
and `frame::dma::Call` path are unchanged. `frame/mod.rs` re-exports
`frame::dma::Call as DmaCall`.

`frame/inventory.rs` keeps `InventoryItem`, `INVENTORY_MAX`, the private
constants, and `inventory_slot`. `InventoryItem::EMPTY` stays private inside
the parent module; the child `snapshot` module can read it without widening
visibility.

### Unchanged files and hashes

| File | SHA-256 (source = provider) |
| --- | --- |
| `src/coord.rs` | `d79ef21ddda4ac7bd9a717a376cd67e14c912a7fae66086379763dd30cc37133` |
| `src/coordination/call.rs` | `df99f28c1dbbbf79ea985389f8052ac1e753072188610c1d7107bc4a9973aa7c` |
| `src/coordination/mod.rs` | `8e306132963ca900b90c4d6305c9fc14c812eb2fa82d7d1b90a70e3e51b8a71f` |
| `src/coordination/prepare.rs` | `58734cf3b19064f68add50a8965a722f3663abfd1ceac3ac564782bc201e37e7` |
| `src/coordination/release.rs` | `2dcfe379b7d1cbb747b4ef896a51a441ae1ae74cab1ae2516f894986d451f793` |
| `src/coordination/wire.rs` | `05487d3ab5062abe4af04463005a3bff42b9d9cc4f2578a54479afa3fad6d116` |
| `src/declaration.rs` | `c556301963d5ab9b0d3ac2c58c3ba323b14ae3a7aee56af551d39cb303eb5c62` |
| `src/device_id.rs` | `3b5a727903524587571c5841b3cef3c34887ab92a16547f019e8ec47fa6215d5` |
| `src/frame/dma_said.rs` | `f2eb9dcb49f107700ba9adc476aee1562f13ce80bdb9f8e14332e9e0ad568f71` |
| `src/frame/vocab.rs` | `6442dcf76751fd0e04d24d6b33c7521ffce2a1acd4009457735c3e116598bc15` |
| `src/grant.rs` | `47eca29278fed3775531e942f659fb2f5069ba0810b0b6f2b93651d05bff377a` |
| `src/lib.rs` | `0f88b19e5ad71f08b9009fb50972a1614674e2e826e2d5d0d036ad54211f12ad` |
| `src/marks.rs` | `0623d4445617bf007b8bc4c6714529d7ba259f9e442aa8f9669f0e2886a9d820` |

`api/src/lib.rs` remains byte-identical, including the single inline
`mold::interface` declaration for `Channel` and `Grant`. The generated
`code_to_fail` / `fail_to_code` in `frame/vocab.rs` also remain unchanged.

## Test provenance

The 14 original test functions are copied without body changes:

| Original file | Provider file | SHA-256 |
| --- | --- | --- |
| `programs/tests/hub-api/lib.rs` | `tests/hub-api/lib.rs` | `d7b1d2b46222d36ec0837cab404240568001f923be9717bd65f94c446006378f` |
| `programs/tests/hub-coordination/lib.rs` | `tests/hub-coordination/lib.rs` | `808f6705c8ce099827824995370e20fc38575fc8cd805d1d10a22e0cf5512dd4` |
| `programs/tests/hub-coordination/tests.rs` | `tests/hub-coordination/tests.rs` | `df3d8594f8018d065afb06cb327057229b1db096250ca6b5d18745590fedf304` |
| `programs/tests/hub-coordination/device_key.rs` | `tests/hub-coordination/device_key.rs` | `538fa04ff9bdb315a158cc9fb5f40dc31c4f75eb0743635a40e06ce558fd7c89` |

Four source-owned system-shape assertions were migrated into
`tests/hub-api/tests/source_contract.rs`. The remaining system-shape
composition/server assertions stay in the source tree and are not part of
this provider.

| Source assertion | Provider case | Intended check |
| --- | --- | --- |
| `system-shape/lib.rs:209` | `coordination_api_has_no_iova_ownership` | `api/src/coord.rs` contains no `IOVA`. |
| `system-shape/lib.rs:255` | `dma_claim_has_no_requester_trust_shape` | `DmaClaim` (now `frame/dma/dma_claim.rs`) and the whole `frame/dma` module have no `pub requester: u32,\n    pub sensor` shape. |
| `system-shape/lib.rs:575` | `hub_api_dependencies_stay_env_wire_mold_system_api` | `api/Cargo.toml` dependencies are only `env`, `wire`, `mold`, `system-api`. |
| `system-shape/lib.rs:710` | `hub_api_generates_interface_metadata` | `api/src/lib.rs` contains `mold::interface`. |

`tests/hub-api/tests/public_api.rs` is an additional compile-only
compatibility case. It names the original root, `frame`, `frame::dma`, and
`frame::inventory` paths, including `DmaCall`, `frame::dma::Call`,
`WINDOW_LEN`, `Call`, and the inventory exports, without constructing any
runtime or hardware state.

Test count: 14 legacy + 4 migrated source-contract cases + 1 compatibility
case = 19. The API test workspace runs 9 and the coordination test workspace runs 10.

## Lockfiles and verification

All three `Cargo.lock` files (`Cargo.lock`, `tests/hub-api/Cargo.lock`,
`tests/hub-coordination/Cargo.lock`) were generated by Cargo from the public
Git pins and then exercised with `--locked` and `--locked --offline`. No
source override, vendoring, path replacement, or hand-written lock entry was
used.

Verification artifacts are stored outside this repository at
`evidence/pr21/provider/`, including command lines, working directories,
exit codes, logs, metadata, and `move-proof.json`. This checkpoint is not a
commit, publication, Kernel-independence result, or Stage 3/4 completion at the worker
handoff. Root performs reviewed publication after sealing that checkpoint.
