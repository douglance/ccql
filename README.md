# ccq

CLI tool for querying and analyzing Claude Code data.

## Installation

```bash
cargo install --path .
```

## Configuration

Set the Claude data directory:

```bash
# Via environment variable
export CLAUDE_DATA_DIR=~/.claude

# Or via command line flag
ccq --data-dir ~/.claude <command>
```

## Commands

### prompts

Extract user prompts from history.

```bash
ccq prompts
ccq prompts --project myproject --limit 10
ccq prompts --since 2024-01-01 --until 2024-12-31
```

### sessions

List and browse sessions.

```bash
ccq sessions
ccq sessions --detailed --sort-by size
```

### stats

Display usage statistics.

```bash
ccq stats
ccq stats --group-by date
```

### search

Full-text search across all data.

```bash
ccq search "error handling"
ccq search "TODO" --scope prompts
ccq search "function\s+\w+" --regex --case-sensitive
ccq search "bug" -B 2 -A 2  # with context lines
```

### todos

List todos and their status.

```bash
ccq todos
ccq todos --status pending
ccq todos --agent main
```

### duplicates

Find repeated/similar prompts using fuzzy matching.

```bash
ccq duplicates
ccq duplicates --threshold 0.9 --min-count 3
ccq duplicates --show-variants
```

### query

Execute jq-style queries on data sources.

```bash
ccq query '.[] | select(.type == "human")' history
ccq query '.[] | .content' transcripts --file-pattern abc123
```

Data sources: `history`, `transcripts`, `stats`, `todos`

## Output Formats

```bash
ccq prompts -f table   # default, human-readable tables
ccq prompts -f json    # JSON array
ccq prompts -f jsonl   # JSON lines (one per line)
ccq prompts -f raw     # raw output
```

## License

MIT
