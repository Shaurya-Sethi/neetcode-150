# neetcode-150 (Rust)

My Rust solutions for the NeetCode 150 list using a Cargo workspace.

## Structure

- `common/`: shared helpers (e.g., linked list / tree utilities)
- `problems/<category>/<slug>/`: one library crate per problem
- `scripts/new_problem.sh`: scaffold helper for new problems

## Useful Commands

- Run all tests: `cargo test --workspace`
- Run one problem: `cargo test -p lc0001_two_sum`
- Format: `cargo fmt --all`
- Lint: `cargo clippy --workspace --all-targets -- -D warnings`

## Add a new problem

```bash
./scripts/new_problem.sh arrays_hashing lc0002_valid_anagram
```

I use `new_problem.sh`  to scaffold the crate, which runs an agent (`agy`) in headless mode to retrieve the exact problem description and generate
`problems/<category>/<slug>/README.md`.

## Why not use leetcode directly?

I like to start in my local code-editor so I can take advantage of IDE autocomplete and inline LSP diagnostics - function signatures and documentation right where I'm typing - leetcode doesn't have this.

Additionally, writing my own tests helps me understand the problem and solution function behaviour more and helps me get into the habit of TDD.

Once tests pass locally, I type the solution on leetcode and submit there as normal, so it isn't like I don't use lc at all.

I use helix to write code so that AI isn't as easily accessible and so I don't get bothered by inline AI autocomplete suggestions too.

Maintaining a git repo also helps me track improvements in Time and Space complexities as I optimize my solutions if I don't arrive at the best solution the first time - I can always revert/review my old solutions for a problem which is nice.

Lastly, this helps me keep the github contribution graph green (if i stay consistent), lol.
