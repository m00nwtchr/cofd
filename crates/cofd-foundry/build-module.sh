#!/usr/bin/env bash
#
# Build the Foundry module: compile cofd-foundry for wasm32-unknown-unknown,
# stage the wasm and the static module files under dist/cofd-foundry/, and zip
# the staged module with the Python standard library.

set -euo pipefail

CRATE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_DIR="$(cd "$CRATE_DIR/../.." && pwd)"
MODULE_ID="cofd-foundry"
TARGET="wasm32-unknown-unknown"
WASM_NAME="cofd_foundry.wasm"
DIST_DIR="$CRATE_DIR/dist"
STAGE_DIR="$DIST_DIR/$MODULE_ID"

find_python() {
	if command -v python3 >/dev/null 2>&1; then
		command -v python3
		return 0
	fi
	if command -v python >/dev/null 2>&1; then
		command -v python
		return 0
	fi
	for candidate in /nix/store/*-python3-*/bin/python3; do
		if [ -x "$candidate" ]; then
			printf '%s\n' "$candidate"
			return 0
		fi
	done
	return 1
}

cd "$WORKSPACE_DIR"
cargo build -p "$MODULE_ID" --target "$TARGET" --release --locked

rm -rf "$STAGE_DIR"
mkdir -p "$STAGE_DIR/scripts"

cp "target/$TARGET/release/$WASM_NAME" "$STAGE_DIR/$WASM_NAME"
cp "$CRATE_DIR/module/module.json" "$STAGE_DIR/module.json"
cp "$CRATE_DIR/module/README.md" "$STAGE_DIR/README.md"
cp "$CRATE_DIR/module/scripts/$MODULE_ID.js" "$STAGE_DIR/scripts/$MODULE_ID.js"

if ! PYTHON="$(find_python)"; then
	echo "build-module.sh: python3 is required to package the module" >&2
	exit 1
fi

"$PYTHON" - "$STAGE_DIR" "$STAGE_DIR/$MODULE_ID.zip" <<'PY'
import pathlib
import sys
import zipfile

stage = pathlib.Path(sys.argv[1]).resolve()
archive = pathlib.Path(sys.argv[2]).resolve()

with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as bundle:
    for path in sorted(stage.rglob("*")):
        if not path.is_file() or path.resolve() == archive:
            continue
        bundle.write(path, path.relative_to(stage.parent))

print(f"created {archive}")
PY

echo "cofd-foundry: module staged in $STAGE_DIR"
