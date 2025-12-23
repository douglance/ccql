# ccql

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
ccql --data-dir ~/.claude <command>
```

## Commands

### prompts

Extract user prompts from history.

```bash
ccql prompts
ccql prompts --project myproject --limit 10
ccql prompts --since 2024-01-01 --until 2024-12-31
```

### sessions

List and browse sessions.

```bash
ccql sessions
ccql sessions --detailed --sort-by size
```

### stats

Display usage statistics.

```bash
ccql stats
ccql stats --group-by date
```

### search

Full-text search across all data.

```bash
ccql search "error handling"
ccql search "TODO" --scope prompts
ccql search "function\s+\w+" --regex --case-sensitive
ccql search "bug" -B 2 -A 2  # with context lines
```

### todos

List todos and their status.

```bash
ccql todos
ccql todos --status pending
ccql todos --agent main
```

### duplicates

Find repeated/similar prompts using fuzzy matching.

```bash
ccql duplicates
ccql duplicates --threshold 0.9 --min-count 3
ccql duplicates --show-variants
```

### query

Execute jq-style queries on data sources.

```bash
ccql query '.[] | select(.type == "human")' history
ccql query '.[] | .content' transcripts --file-pattern abc123
```

Data sources: `history`, `transcripts`, `stats`, `todos`

### sql

Execute SQL queries on Claude Code data using GlueSQL.

```bash
# Basic queries
ccql sql "SELECT * FROM history LIMIT 10"
ccql sql "SELECT display, project FROM history WHERE project LIKE '%myproject%'"
ccql sql "SELECT COUNT(*) as total FROM history"

# Aggregations
ccql sql "SELECT project, COUNT(*) as count FROM history GROUP BY project"

# Output formats (note: -f flag goes before subcommand)
ccql -f json sql "SELECT * FROM history LIMIT 5"
ccql -f jsonl sql "SELECT * FROM history LIMIT 5"
```

#### Write Operations

Write operations (INSERT, UPDATE, DELETE) require explicit flags for safety:

```bash
# Preview changes without modifying (dry run)
ccql sql --dry-run "DELETE FROM history WHERE timestamp < 1704067200000"

# Execute write operation (requires --write flag)
ccql sql --write "UPDATE history SET project = 'archived' WHERE project = 'old-project'"
ccql sql --write "DELETE FROM history WHERE timestamp < 1704067200000"
```

#### Available Tables

| Table | Source | Description |
|-------|--------|-------------|
| `history` | `~/.claude/history.jsonl` | User prompts and commands |
| `stats` | `~/.claude/stats-cache.json` | Usage statistics |
| `transcripts` | `~/.claude/transcripts/*.jsonl` | All conversation messages (virtual table) |
| `todos` | `~/.claude/todos/*.json` | Task lists from all sessions (virtual table) |

#### Virtual Table Metadata

The `transcripts` and `todos` tables merge multiple files and include metadata columns:

**transcripts columns:** `_source_file`, `_session_id`, `type`, `timestamp`, `content`, `tool_name`, `tool_input`, `tool_output`

**todos columns:** `_source_file`, `_workspace_id`, `_agent_id`, `content`, `status`, `activeForm`

#### Practical Query Examples

**Productivity Insights:**
```bash
# Prompt breakdown: how much is commands vs real work?
ccql sql "SELECT COUNT(*) as total,
         SUM(CASE WHEN display LIKE '/%' THEN 1 ELSE 0 END) as commands,
         SUM(CASE WHEN display LIKE '[Pasted%' THEN 1 ELSE 0 END) as pastes
         FROM history"

# Find wasted effort: undo/revert patterns
ccql sql "SELECT display FROM history
         WHERE display LIKE '%undo%' OR display LIKE '%revert%'"

# Low-value prompts: repeated short confirmations
ccql sql "SELECT display, COUNT(*) as cnt FROM history
         WHERE LENGTH(display) < 30 AND display NOT LIKE '/%'
         GROUP BY display ORDER BY cnt DESC"

# Top projects by activity
ccql sql "SELECT project, COUNT(*) as prompts FROM history
         GROUP BY project ORDER BY prompts DESC"
```

**Todo Completion:**
```bash
# Completion rate with percentages
ccql sql "SELECT status, COUNT(*) as count,
         ROUND(COUNT(*) * 100.0 / (SELECT COUNT(*) FROM todos)) as pct
         FROM todos GROUP BY status"

# Pending todos by workspace (find stale work)
ccql sql "SELECT _workspace_id, COUNT(*) as pending FROM todos
         WHERE status = 'pending' GROUP BY _workspace_id ORDER BY pending DESC"
```

**Tool Usage:**
```bash
# Most used tools
ccql sql "SELECT tool_name, COUNT(*) as uses FROM transcripts
         WHERE tool_name IS NOT NULL GROUP BY tool_name ORDER BY uses DESC"

# Read vs Edit vs Write ratio
ccql sql "SELECT tool_name, COUNT(*) as cnt FROM transcripts
         WHERE tool_name IN ('read', 'edit', 'write') GROUP BY tool_name"

# Largest sessions by tool calls
ccql sql "SELECT _session_id, COUNT(*) as tool_calls FROM transcripts
         WHERE type='tool_use' GROUP BY _session_id ORDER BY tool_calls DESC"
```

**Command Usage:**
```bash
# Most used slash commands
ccql sql "SELECT display, COUNT(*) as cnt FROM history
         WHERE display LIKE '/%' GROUP BY display ORDER BY cnt DESC"

# Git operations
ccql sql "SELECT display FROM history
         WHERE display LIKE '%commit%' OR display LIKE '%push%'"
```

**Search & Discovery:**
```bash
# Find work on a topic
ccql sql "SELECT _session_id, timestamp, content FROM transcripts
         WHERE type='user' AND content LIKE '%authentication%' LIMIT 10"

# Bug/fix related work
ccql sql "SELECT project, display FROM history
         WHERE display LIKE '%error%' OR display LIKE '%bug%' OR display LIKE '%fix%'"
```

## Output Formats

```bash
ccql prompts -f table   # default, human-readable tables
ccql prompts -f json    # JSON array
ccql prompts -f jsonl   # JSON lines (one per line)
ccql prompts -f raw     # raw output
```

## License

MIT
