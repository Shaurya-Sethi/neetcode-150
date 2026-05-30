# neetcode-150 (Rust)

My Rust solutions for the NeetCode 150 list using a Cargo workspace.

## Structure

- `common/`: shared helpers (e.g., linked list / tree utilities)
- `problems/<category>/<slug>/`: one library crate per problem
- `scripts/new_problem.sh`: scaffold helper for new problems

## Commands

- Run all tests: `cargo test --workspace`
- Run one problem: `cargo test -p lc0001_two_sum`
- Format: `cargo fmt --all`
- Lint: `cargo clippy --workspace --all-targets -- -D warnings`

## Add a new problem

```bash
./scripts/new_problem.sh arrays_hashing lc0002_valid_anagram
```

`new_problem.sh` scaffolds the crate, then runs `agy` in headless mode to generate
`problems/<category>/<slug>/README.md`. It tries LeetCode first, then falls back to NeetCode/other sources
if needed, and skips README generation only when exact verbatim content cannot be retrieved.

Prerequisite: install `agy` locally and ensure it's on PATH.
