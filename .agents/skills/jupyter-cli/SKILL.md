---
name: jupyter-cli
description: Use the custom Rust-based jupyter-cli for working with Jupyter notebooks instead of built-in tools. Provides programmatic access to notebook operations (read, create, edit cells, execute, search) with JSON output for AI agents. Invoke when working with .ipynb files in this project.
---

# Working with Jupyter Notebooks using jupyter-cli

Use the custom `jupyter-cli` tool (Rust-based CLI) for programmatic notebook manipulation instead of Claude Code's built-in notebook operations.

## Project Context

- **Location**: `/Users/pijain/projects/2026/jupyter-cli`
- **Binary**: `./target/debug/jupyter-cli` (build with `cargo build` if needed)
- **Output**: JSON by default (ideal for parsing), use `-f text` for human-readable format

## Command Structure

The CLI is organized by resource type:
```bash
jupyter-cli notebook <command>  # Notebook-level operations
jupyter-cli cell <command>      # Cell-level operations
jupyter-cli output <command>    # Output management
```

Available commands:
- **notebook**: `create`, `read`, `execute`, `search`
- **cell**: `add`, `update`, `delete`, `execute`
- **output**: `clear`

Use `--help` with any command for detailed options.

## Key Concepts

### Cell Referencing
Two ways to reference cells:
- **By index** (`--cell N` or `-c N`): 0-based position, supports negative indexing (-1 = last cell)
- **By ID** (`--cell-id "id"`): Stable identifier that doesn't change when cells move

### Real-Time Collaboration
Add/update cells support `--server` and `--token` options for real-time sync with open JupyterLab notebooks via Y.js:
```bash
jupyter-cli cell add <file> --source "code" --server http://localhost:8888 --token <token>
```

### Output Format
- Default: JSON (nbformat-compliant with additional `index` field)
- Human-readable: Add `--format text` or `-f text`

## Common Operations

### Reading
```bash
# Notebook overview (cell count, types, all cell summaries)
jupyter-cli notebook read <file>

# Specific cell (by index or ID)
jupyter-cli notebook read <file> --cell 0
jupyter-cli notebook read <file> --cell-id "my-cell"

# With execution outputs
jupyter-cli notebook read <file> -c 0 --with-outputs

# Filter by type
jupyter-cli notebook read <file> --only-code
jupyter-cli notebook read <file> --only-markdown
```

### Creating & Editing
```bash
# Create notebook
jupyter-cli notebook create <file> [--template basic|markdown] [--kernel python3]

# Add cell (at end, specific position, or relative to another cell)
jupyter-cli cell add <file> --source "code" --type code [--insert-at 0|--after "id"|--before "id"]

# Update cell (replace or append)
jupyter-cli cell update <file> --cell 0 --source "new content"
jupyter-cli cell update <file> --cell 0 --append "\nmore code"

# Delete cell
jupyter-cli cell delete <file> --cell 0
jupyter-cli cell delete <file> --cell-id "my-cell"
```

### Execution
```bash
# Execute single cell
jupyter-cli cell execute <file> --cell 0 [--kernel python3] [--timeout 60] [--allow-errors]

# Execute entire notebook
jupyter-cli notebook execute <file> [--kernel python3] [--timeout 60] [--start 2 --end 5]

# Remote execution (via Jupyter server)
jupyter-cli [cell|notebook] execute <file> --server http://localhost:8888 --token <token>
```

### Searching
```bash
# Search in source code
jupyter-cli notebook search <file> <pattern> [--ignore-case]

# Search in outputs or all
jupyter-cli notebook search <file> <pattern> --scope output|all

# Filter by cell type
jupyter-cli notebook search <file> <pattern> --cell-type code|markdown

# Find cells with errors
jupyter-cli notebook search <file> --with-errors
```

### Output Management
```bash
# Clear all outputs
jupyter-cli output clear <file> --all [--keep-execution-count]

# Clear specific cell
jupyter-cli output clear <file> --cell 0
```

## Typical Workflows

**Analyze**: `notebook read --only-code` → get all code cells for analysis
**Debug**: `notebook search --with-errors` → find problematic cells → `notebook read -c N --with-outputs` → inspect
**Fix**: `cell update -c N --source "fixed"` → `cell execute -c N` → verify
**Build**: `notebook create` → `cell add` (multiple times) → construct notebook programmatically

## Important Notes

- All commands output JSON following nbformat specification
- Supports negative indexing for convenience (-1 = last cell)
- Cell IDs auto-generated if not present
- Real-time updates via Y.js when using `--server`/`--token` options
- Escape sequences (`\n`, `\t`) automatically interpreted in `--source` and `--append`
