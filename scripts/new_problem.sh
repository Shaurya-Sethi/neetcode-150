#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 2 ]; then
  echo "Usage: $0 <category> <slug>"
  echo "Example: $0 arrays_hashing lc0001_two_sum"
  exit 1
fi

CATEGORY="$1"
SLUG="$2"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
REL_DIR="problems/${CATEGORY}/${SLUG}"
DIR="${REPO_ROOT}/${REL_DIR}"
COMMON_DIR="${REPO_ROOT}/common"
README_PATH="${DIR}/README.md"
README_UNAVAILABLE_SENTINEL="__README_UNAVAILABLE__"

infer_problem_query() {
  local slug="$1"
  if [[ "$slug" =~ ^lc([0-9]+)_(.+)$ ]]; then
    local raw_id="${BASH_REMATCH[1]}"
    local title_part="${BASH_REMATCH[2]}"
    local numeric_id=$((10#${raw_id}))
    local spaced_title="${title_part//_/ }"
    echo "LeetCode ${numeric_id} ${spaced_title}"
    return
  fi

  echo "${slug//_/ }"
}

if [ -e "$DIR" ]; then
  echo "Error: ${REL_DIR} already exists"
  exit 1
fi

mkdir -p "${REPO_ROOT}/problems/${CATEGORY}"
cargo new --lib "$DIR" --vcs none
cargo add common --path "${COMMON_DIR}" --manifest-path "${DIR}/Cargo.toml"

cat > "${DIR}/src/lib.rs" <<'RS'
pub fn solve() {
    // TODO: implement solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder() {
        solve();
    }
}
RS

echo "Created ${REL_DIR}"
echo "Run: cargo test -p ${SLUG}"

if ! command -v agy >/dev/null 2>&1; then
  echo "agy not found on PATH. Install it first." >&2
  echo "Scaffolding completed, but README generation failed." >&2
  exit 127
fi

QUERY="$(infer_problem_query "$SLUG")"
PROMPT=$(cat <<EOF
Create README.md content for the coding problem crate "${SLUG}".

Use web search and follow this retrieval policy strictly:
1. Try to locate the official LeetCode problem page first for: ${QUERY}
2. If LeetCode is unavailable or inaccessible, fallback to NeetCode.
3. If NeetCode is unavailable, use another reputable source discovered via web search.

Output requirements:
- The problem statement, constraints, and examples must be verbatim from the source used.
- Do not paraphrase, summarize, or reword any statement/constraint/example text.
- If exact verbatim retrieval is not possible from LeetCode or any fallback source, output exactly ${README_UNAVAILABLE_SENTINEL} and nothing else.
- Otherwise output only Markdown suitable for README.md with these sections:
  # ${SLUG}
  ## Problem Statement
  ## Constraints
  ## Examples
  ## Source
  - URL: <exact URL used>
  - Source type: <LeetCode|NeetCode|Other>
EOF
)

AGY_PRINT_TIMEOUT="${AGY_PRINT_TIMEOUT:-5m}"
AGY_STDERR_TMP="$(mktemp)"
trap 'rm -f "$AGY_STDERR_TMP"' EXIT

echo "Generating ${REL_DIR}/README.md via agy (timeout: ${AGY_PRINT_TIMEOUT})..." >&2

set +e
GEMINI_OUTPUT="$(
  agy -p "$PROMPT" \
    --dangerously-skip-permissions \
    --print-timeout "$AGY_PRINT_TIMEOUT" \
    2> >(tee "$AGY_STDERR_TMP" >&2) \
    | tee /dev/stderr
)"
GEMINI_EXIT_CODE=$?
set -e

if [ "$GEMINI_EXIT_CODE" -ne 0 ]; then
  echo "agy failed while generating ${REL_DIR}/README.md" >&2
  # stdout (if any)
  if [ -n "$GEMINI_OUTPUT" ]; then
    printf '%s\n' "$GEMINI_OUTPUT" >&2
  fi
  exit "$GEMINI_EXIT_CODE"
fi

# agy currently exits 0 for some errors (notably print-mode timeouts).
case "$GEMINI_OUTPUT" in
  "Error:"*)
    echo "agy reported an error while generating ${REL_DIR}/README.md" >&2
    printf '%s\n' "$GEMINI_OUTPUT" >&2
    exit 1
    ;;
esac

LAST_NONEMPTY_LINE="$(printf '%s\n' "$GEMINI_OUTPUT" | awk 'NF { line=$0 } END { print line }')"
if [ "$(printf '%s' "$LAST_NONEMPTY_LINE" | tr -d '[:space:]')" = "${README_UNAVAILABLE_SENTINEL}" ]; then
  echo "Skipped README generation: exact content unavailable from LeetCode and fallback sources."
  exit 0
fi

# agy sometimes prints progress / tool-plans to stdout before the final answer.
# Extract the final README payload starting at the required header.
README_HEADER="# ${SLUG}"
CLEAN_OUTPUT="$(printf '%s\n' "$GEMINI_OUTPUT" | awk -v h="$README_HEADER" 'found || $0==h { found=1 } found { print }')"

if [ -z "$CLEAN_OUTPUT" ]; then
  echo "agy succeeded but output did not contain expected README header: ${README_HEADER}" >&2
  exit 1
fi

printf '%s\n' "$CLEAN_OUTPUT" > "$README_PATH"
echo "Generated ${REL_DIR}/README.md"
