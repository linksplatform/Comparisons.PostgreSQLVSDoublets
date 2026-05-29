# Case Study: Upgrade to Latest `doublets-rs` and Switch to Stable Rust (Issue #23)

## Summary

The `Comparisons.PostgreSQLVSDoublets` benchmark project depended on an old,
pre-release, **git-pinned** build of [`doublets-rs`](https://github.com/linksplatform/doublets-rs)
(`0.1.0-pre+beta.15`) and required the **nightly** Rust toolchain
(`nightly-2022-08-22`). The nightly pin was needed because that version of
`doublets` relied on unstable language features (`allocator_api`,
`generic_associated_types`, and the `Try` trait for `?` on a custom control-flow
type).

Meanwhile, upstream `doublets-rs` released **v0.3.0** (2026-04-18), which is
published on crates.io and — most importantly — **migrated from nightly to
stable Rust (1.85+)**. This issue tracks adopting that release and removing the
nightly requirement from this repository.

- **Issue**: https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/issues/23
- **Pull Request**: https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/pull/24
- **Upstream library**: https://github.com/linksplatform/doublets-rs
- **Labels**: `documentation`, `enhancement`, `dependencies`

## Requirements (extracted from the issue)

The issue body contains both explicit migration requirements and process
requirements. Each is listed below and tracked to its resolution.

| # | Requirement | Source (issue text) | Status |
|---|-------------|---------------------|--------|
| R1 | Use the **latest version of `doublets-rs`** | "Make sure to use latest version of doublets-rs" (title) + link to repo | ✅ `doublets = "0.3.0"` (crates.io, latest release) |
| R2 | Switch the project to **stable Rust** | "make sure to switch to stable rust" (title) | ✅ `rust-toolchain.toml` + CI use `stable`; no `#![feature(...)]` remains |
| R3 | Collect data about the issue into `./docs/case-studies/issue-{id}` | "compile that data to `./docs/case-studies/issue-{id}` folder" | ✅ This `docs/case-studies/issue-23/` folder |
| R4 | Deep case-study analysis, **including online research** | "do deep case study analysis (also make sure to search online for additional facts and data)" | ✅ This document (upstream release notes, crates.io, tags) |
| R5 | List of **each and all requirements** from the issue | "list of each and all requirements from the issue" | ✅ This table |
| R6 | Propose **possible solutions and solution plans** for each requirement | "propose possible solutions and solution plans for each requirement" | ✅ [Proposed Solutions](#proposed-solutions) |
| R7 | Check **known existing components/libraries** that solve a similar problem | "we should also check known existing components/libraries…" | ✅ [Existing Components & Libraries](#existing-components--libraries-surveyed) |
| R8 | Execute everything in the **single existing PR #24** | "Please plan and execute everything in this single pull request" | ✅ All commits land on branch `issue-23-dff35da4182d` (PR #24) |

## Background & Online Research

Data gathered from the upstream repository, crates.io, and the locally vendored
crate sources (`~/.cargo/registry`).

### `doublets-rs` release history

| Version | Date | Toolchain | Notes |
|---------|------|-----------|-------|
| `0.1.0-pre+beta.15` | (pre-release) | nightly | The version this repo was pinned to (git dependency, no tag) |
| `v0.2.0` | 2026-04-14 | nightly-2022-08-22 | First tagged/published release; still nightly |
| `v0.3.0` | 2026-04-18 | **stable (1.85+)** | Latest release; the target of this issue |

Source: `gh release list --repo linksplatform/doublets-rs` and
`cargo search doublets` (shows `doublets = "0.3.0"` as the newest).

### Key facts from the `doublets` v0.3.0 release notes

The v0.3.0 changelog lists several **BREAKING** changes directly relevant here:

> - **BREAKING**: Migrated from nightly Rust to stable Rust (1.85+)
> - **BREAKING**: `Fuse` no longer implements `FnMut`; use `.call()` method instead
> - **BREAKING**: Removed `Handler` trait; handlers now use `FnMut(Link<T>, Link<T>) -> Flow` directly
> - Migrated `platform-mem` dependency from git submodule to crates.io v0.3.0
> - Migrated `platform-trees` dependency from git submodule to crates.io v0.3.3

The crate manifest confirms the toolchain target:

```toml
# doublets 0.3.0 Cargo.toml
edition = "2021"
rust-version = "1.85"
license = "Unlicense"
repository = "https://github.com/linksplatform/doublets-rs"
```

### Transitive dependency versions pulled by `doublets 0.3.0`

| Crate | Version | Role |
|-------|---------|------|
| `platform-data` | 2.0.0 | `Flow`, `LinkType`/`LinkReference`, `LinksConstants`, handlers |
| `platform-mem` | 0.3.0 | Memory backends (`Global`, `FileMapped`) |
| `platform-num` | 0.8.0 | `LinkReference` numeric trait |
| `allocator-api2` | 0.4.0 | Stable shim for the nightly `allocator_api` |

The critical insight: the nightly `allocator_api` is now provided on **stable**
through the `allocator-api2` crate, which is why `doublets 0.3.0` no longer needs
nightly for custom allocators.

## Root Cause Analysis (why nightly was required before)

The previous dependency (`0.1.0-pre+beta.15`) forced nightly for three reasons:

### 1. `allocator_api` (custom allocator generics)
The old in-memory backend was spelled `Alloc<T, std::alloc::Global>`, where
`std::alloc::Global` and the `Allocator` trait were nightly-only. This is the
single biggest reason the repo pinned `nightly-2022-08-22`.

### 2. `generic_associated_types` (GATs)
The `Benched` trait uses an associated type with a lifetime parameter
(`type Builder<'params>;`). GATs were unstable in 2022 but **stabilized in Rust
1.65**, so no source change is required for this on a modern stable compiler.

### 3. `Try` for `?` on `Flow`
The old `doublets::data::Flow` implemented the unstable `Try` trait so callback
code could write `handler(link)?` to short-circuit iteration. On stable, `Flow`
no longer implements `Try`, so `?` cannot be applied to it.

## Proposed Solutions

For each requirement, the solution actually adopted is marked **[chosen]**.

### R1 — Use the latest `doublets-rs`

- **Option A [chosen]: Depend on the published crate `doublets = "0.3.0"`.**
  Reproducible, versioned, no submodules, resolved through crates.io. Matches the
  upstream recommendation and the `dependencies` label intent.
- Option B: Keep a `git` dependency pinned to tag `v0.3.0`. Rejected — a
  published crate is strictly better for reproducibility and tooling (Dependabot,
  `cargo audit`).
- Option C: Track `git` `HEAD`. Rejected — non-reproducible.

### R2 — Switch to stable Rust

- **Option A [chosen]: Set the toolchain to `stable` and migrate all
  nightly-only API.** See the migration map below.
- Option B: Stay on nightly but only bump the dependency. Rejected — directly
  contradicts the issue title.

The migration map applied to this repository:

| Old (nightly, beta.15) | New (stable, 0.3.0) | Files |
|------------------------|---------------------|-------|
| `#![feature(allocator_api)]`, `#![feature(generic_associated_types)]` | *(removed)* | `rust/src/lib.rs`, `rust/benches/bench.rs` |
| `Alloc<T, std::alloc::Global>` | `doublets::mem::Global<T>`, built with `Global::new()` | `doublets_benched.rs`, all `benches/.../doublets/*.rs` |
| `doublets::data::LinkType` | `doublets::data::LinkReference` | `client.rs`, `transaction.rs`, `*_benched.rs`, `lib.rs` |
| `T::ZERO` | `T::from_byte(0)` | `client.rs`, `transaction.rs` |
| `value.as_i64()` | `as_i64(value)` helper wrapping `TryInto<i64>` | `client.rs`, `transaction.rs`, `lib.rs` |
| `handler(link)?` (relied on `Try for Flow`) | `if handler(link).is_break() { return Flow::Break; }` | `client.rs`, `transaction.rs` |
| `doublets::parts::LinkPart` | `doublets::unit::LinkPart` | `benches/.../doublets/*.rs` |
| `channel = "nightly-2022-08-22"` | `channel = "stable"` | `rust-toolchain.toml` |
| CI `env.toolchain: nightly-2022-08-22` | `toolchain: stable` | `.github/workflows/rust.yml` |

Notably, `doublets`'s own `Links<T>`/`Doublets<T>` traits keep their
`Link<T>`-based read/write handler signatures, so the PostgreSQL backend logic in
`client.rs`/`transaction.rs` only needed **mechanical symbol renames**, not a
structural rewrite. (The new `Links<T>` does add a `Send + Sync` bound, which the
`Exclusive<T>` wrapper already satisfies — `Send` is auto-derived and `Sync` is
manually `unsafe impl`'d.)

### R3 / R4 / R5 — Documentation, research, requirement list

- **[chosen]** Author this `README.md` in `docs/case-studies/issue-23/`,
  mirroring the structure of the existing `docs/case-studies/issue-15/` study,
  with an explicit requirements table (R5), online-research section (R4), and the
  data compiled into the folder (R3).

### R6 — Solution plans per requirement

- **[chosen]** This "Proposed Solutions" section enumerates options and the
  chosen approach per requirement.

### R7 — Existing components/libraries

See the dedicated survey below.

### R8 — Single PR

- **[chosen]** All work is committed to branch `issue-23-dff35da4182d` and
  surfaced through the pre-existing PR #24; no new PR is opened.

## Existing Components & Libraries Surveyed

| Component | What it solves | Decision |
|-----------|----------------|----------|
| [`doublets` 0.3.0](https://crates.io/crates/doublets) | The links data structure under benchmark; now stable | **Adopted** (R1/R2) |
| [`allocator-api2`](https://crates.io/crates/allocator-api2) | Stable polyfill for nightly `allocator_api`; lets `doublets::mem::Global` work without nightly | Used transitively via `doublets`; no direct dependency needed |
| `platform-mem` 0.3.0 | Provides `Global` and `FileMapped` memory backends | Used via `doublets::mem` re-exports |
| `platform-num` 0.8.0 | Provides the `LinkReference` numeric trait (replacing `LinkType`) | Used via `doublets::data` re-export |
| `std::ops::ControlFlow` | Standard-library control-flow enum analogous to `Flow` | Not required — `Flow` exposes `is_break()`/`is_continue()`/`into_control_flow()`, sufficient for the migration |
| [`postgres`](https://crates.io/crates/postgres) 0.19 | Synchronous PostgreSQL client (the comparison baseline) | Unchanged |
| [`criterion`](https://crates.io/crates/criterion) 0.4 | Benchmark harness | Unchanged |

## Evidence

### 1. Dependency is the latest published release
```
$ cargo search doublets
doublets = "0.3.0"   # Doublets (links) data structure implementation.

$ grep -A2 'name = "doublets"' rust/Cargo.lock
name = "doublets"
version = "0.3.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
```

### 2. Toolchain is stable, no nightly features remain
```
$ cat rust-toolchain.toml
[toolchain]
channel = "stable"

$ grep -rn '#!\[feature' rust/   # (no output — all feature gates removed)
```

### 3. Builds on stable
The exact command CI runs succeeds on stable (`rustc 1.95.0`):
```
$ cargo build --release --all-features --manifest-path rust/Cargo.toml
    Finished `release` profile [optimized] target(s)

$ cargo bench --no-run --all-features
    Finished `bench` profile [optimized] target(s)
  Executable benches/bench.rs (target/release/deps/bench-...)
```

> Note: the benchmarks connect to a live PostgreSQL instance only at **runtime**
> (`localhost:5432`), which is provided by the CI `postgres` service. Locally,
> compilation (`cargo build` / `cargo bench --no-run`) is the verifiable step.

## Verification Steps

1. `rustup show` → active toolchain resolves to `stable` (via `rust-toolchain.toml`).
2. `cargo build --release --all-features --manifest-path rust/Cargo.toml` succeeds.
3. `cargo bench --no-run --all-features` compiles all benchmark binaries.
4. `grep -rn '#!\[feature' rust/` returns nothing (no nightly gates).
5. `grep 'doublets' rust/Cargo.toml` shows `doublets = "0.3.0"`.
6. CI (`.github/workflows/rust.yml`) runs the build and `cargo bench` against the
   PostgreSQL service on the `stable` toolchain.

## Lessons Learned

1. **Pin to published, tagged releases, not git `HEAD`.** The original git
   dependency on a pre-release (`0.1.0-pre+beta.15`) coupled this repo to an
   unstable toolchain and made upgrades implicit and fragile.
2. **Nightly features migrate to stable over time.** GATs (1.65) and the
   `allocator_api` (via `allocator-api2`) are no longer reasons to stay on
   nightly; periodically re-checking lets downstreams drop the nightly pin.
3. **Track upstream release notes for BREAKING markers.** The `doublets` v0.3.0
   changelog precisely predicted the symbol renames (`LinkType` → `LinkReference`,
   `Handler` removal) needed downstream.
4. **Prefer mechanical, reviewable migrations.** Because the trait handler
   signatures were preserved upstream, the PostgreSQL backend changed only in
   naming — keeping the diff small and auditable.

## Related Resources

- Upstream library: https://github.com/linksplatform/doublets-rs
- `doublets` on crates.io: https://crates.io/crates/doublets/0.3.0
- `doublets` v0.3.0 release notes: https://github.com/linksplatform/doublets-rs/releases/tag/v0.3.0
- `allocator-api2` (stable allocator shim): https://crates.io/crates/allocator-api2
- Issue: https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/issues/23
- Pull Request: https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/pull/24
- Sibling case study (format reference): `docs/case-studies/issue-15/`
