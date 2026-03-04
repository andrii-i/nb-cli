# Jupyter CLI

A command-line interface tool for working with Jupyter notebooks (`.ipynb` files). Built with Rust for performance and designed for programmatic interaction with notebooks, especially by AI agents.

## Features

- **Read notebook content** - View cells, outputs, and metadata
- **Cell access by index or ID** - Reference cells by stable IDs or positional index
- **Filter by cell type** - Extract only code or markdown cells
- **Multiple output formats** - JSON (default) for agents, text for humans
- **Negative indexing** - Use `-1` to access the last cell, like Python
- **Short flag aliases** - Efficient CLI with `-f`, `-c`, `-i`, `-o` shortcuts
- **Fast and reliable** - Built with Rust using the `nbformat` crate

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/jupyter-cli`.

## Quick Start

```bash
# View notebook overview
jupyter-cli read notebook.ipynb

# Get specific cell (by index or ID)
jupyter-cli read notebook.ipynb -c 5
jupyter-cli read notebook.ipynb -i "cell-id"

# Extract all code cells
jupyter-cli read notebook.ipynb --only-code

# View cell output
jupyter-cli read notebook.ipynb -c 3 -o

# Human-readable text format
jupyter-cli read notebook.ipynb -f text
```

## Usage

### View Notebook Structure

Get an overview of the notebook with cell types, IDs, and execution status:

```bash
jupyter-cli read notebook.ipynb
```

**Output (JSON):**
```json
{
  "cell_count": 7,
  "code_cells": 4,
  "markdown_cells": 3,
  "kernel": "python3",
  "cells": [
    {
      "index": 0,
      "id": "intro-cell",
      "type": "markdown",
      "preview": "# Data Analysis Example..."
    },
    {
      "index": 1,
      "id": "imports-cell",
      "type": "code",
      "preview": "import pandas as pd...",
      "executed": true
    }
  ]
}
```

### Get Specific Cell

By index (0-based):
```bash
jupyter-cli read notebook.ipynb --cell 0
# or use short form
jupyter-cli read notebook.ipynb -c 0
```

By cell ID (more stable - IDs don't change when cells are added/removed):
```bash
jupyter-cli read notebook.ipynb --cell-id "intro-cell"
# or use short form
jupyter-cli read notebook.ipynb -i "intro-cell"
```

Negative indexing (last cell):
```bash
jupyter-cli read notebook.ipynb -c -1
```

### Get Cell Output

View the execution output of a code cell:

```bash
jupyter-cli read notebook.ipynb -c 3 --with-output
# or use short form
jupyter-cli read notebook.ipynb -c 3 -o
```

### Extract All Code Cells

Get all code cells for analysis:

```bash
jupyter-cli read notebook.ipynb --only-code
# backward compatible alias
jupyter-cli read notebook.ipynb --code
```

**Output:**
```json
{
  "cells": [
    {
      "index": 1,
      "id": "imports-cell",
      "source": "import pandas as pd\nimport numpy as np",
      "execution_count": 1
    },
    ...
  ]
}
```

### Extract All Markdown Cells

Get all documentation from the notebook:

```bash
jupyter-cli read notebook.ipynb --only-markdown
# backward compatible alias
jupyter-cli read notebook.ipynb --markdown
```

### Get All Outputs

Extract all execution outputs:

```bash
jupyter-cli read notebook.ipynb --all-outputs
```

### Text Format

Use `-f text` or `--format text` for human-readable output:

```bash
jupyter-cli read notebook.ipynb -f text
jupyter-cli read notebook.ipynb -c 0 -f text
jupyter-cli read notebook.ipynb --only-code -f text
```

## Command Reference

```
jupyter-cli read [OPTIONS] <FILE>

Arguments:
  <FILE>  Path to notebook file

Options:
  -f, --format <FORMAT>   Output format: json (default) or text
  -c, --cell <INDEX>      Get cell by index (supports negative like -1)
  -i, --cell-id <ID>      Get cell by ID (more stable than index)
  -o, --with-output       Show cell execution output (requires --cell or --cell-id)
      --only-code         Show only code cells (alias: --code)
      --only-markdown     Show only markdown cells (alias: --markdown)
      --all-outputs       Show all cell outputs
  -h, --help              Print help
  -V, --version           Print version
```

## Cell IDs vs Indexes

Jupyter notebooks support two ways to reference cells:

- **Index** (`--cell`): Position-based (0, 1, 2, ..., -1 for last). Simple but changes when cells are reordered.
- **ID** (`--cell-id`): Stable identifier (e.g., "intro-cell", "abc123"). Doesn't change when cells are moved.

**Recommendation:** Use `--cell-id` when you need stable references across notebook edits. Use `--cell` for quick interactive access.

## Agent Workflows

Common patterns for AI agents working with notebooks:

### Analyze Code
```bash
# Get all code for analysis
jupyter-cli read notebook.ipynb --only-code

# Check specific cell
jupyter-cli read notebook.ipynb -i "data-processing"
```

### Debug Outputs
```bash
# See what a cell produced
jupyter-cli read notebook.ipynb -c 5 -o

# Get all outputs for debugging
jupyter-cli read notebook.ipynb --all-outputs
```

### Extract Documentation
```bash
# Get markdown content
jupyter-cli read notebook.ipynb --only-markdown
```

### Quick Inspection
```bash
# Overview
jupyter-cli read notebook.ipynb

# Last cell
jupyter-cli read notebook.ipynb -c -1
```

## Examples

See `examples/sample.ipynb` for a test notebook demonstrating various cell types and outputs.

## Architecture

- **`src/main.rs`** - CLI entry point with `clap`
- **`src/notebook.rs`** - Notebook I/O using `nbformat` crate
- **`src/commands/read.rs`** - Read command implementation
- **`examples/`** - Sample notebooks for testing

## Dependencies

- **nbformat** - Jupyter notebook parsing (nbformat v4 specification)
- **jupyter-protocol** - Output data structures
- **clap** - CLI argument parsing
- **serde/serde_json** - JSON serialization
- **anyhow** - Error handling

## Roadmap

Future commands (not yet implemented):

- `search` - Find patterns in notebooks
- `edit` - Modify cell content
- `insert` - Add new cells
- `delete` - Remove cells
- `clear` - Clear outputs
- `execute` - Run cells (requires kernel protocol)
- `convert` - Export to .py, .md, etc.

## License

MIT
