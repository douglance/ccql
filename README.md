# ccql

SQL query engine for Claude Code data.

## Installation

```bash
cargo install --path .
```

## Quick Start

```bash
# SQL is the default command - just pass a query
ccql "SELECT display FROM history ORDER BY timestamp DESC LIMIT 5"
ccql "SELECT tool_name, COUNT(*) as n FROM transcripts WHERE type='tool_use' GROUP BY tool_name"
ccql "SELECT content FROM todos WHERE status='pending'"

# Get help
ccql -h              # Quick reference
ccql --help          # Full documentation
ccql tables          # Show table schemas
ccql examples        # Show query examples
```

## Tables

| Table | Source | Description |
|-------|--------|-------------|
| `history` | `history.jsonl` | User prompts |
| `transcripts` | `transcripts/*.jsonl` | Conversation logs (virtual) |
| `todos` | `todos/*.json` | Task items (virtual) |

### history

| Column | Type | Description |
|--------|------|-------------|
| `display` | TEXT | The prompt text |
| `timestamp` | INTEGER | Unix timestamp (ms) |
| `project` | TEXT | Project directory |
| `pastedContents` | OBJECT | Pasted content (JSON) |

### transcripts

| Column | Type | Description |
|--------|------|-------------|
| `_source_file` | TEXT | Source file (ses_xxx.jsonl) |
| `_session_id` | TEXT | Session ID |
| `type` | TEXT | `user` \| `tool_use` \| `tool_result` |
| `timestamp` | TEXT | ISO 8601 timestamp |
| `content` | TEXT | Message text (type='user') |
| `tool_name` | TEXT | Tool name (type='tool_*') |
| `tool_input` | OBJECT | Tool parameters |
| `tool_output` | OBJECT | Tool response (type='tool_result') |

### todos

| Column | Type | Description |
|--------|------|-------------|
| `_source_file` | TEXT | Source filename |
| `_workspace_id` | TEXT | Workspace ID |
| `_agent_id` | TEXT | Agent ID |
| `content` | TEXT | Todo description |
| `status` | TEXT | `pending` \| `in_progress` \| `completed` |
| `activeForm` | TEXT | Display text when active |

## Examples

### History Queries

```bash
# Recent prompts
ccql "SELECT display FROM history ORDER BY timestamp DESC LIMIT 10"

# Search prompts
ccql "SELECT display FROM history WHERE display LIKE '%error%'"

# Prompts by project
ccql "SELECT project, COUNT(*) as n FROM history GROUP BY project ORDER BY n DESC"

# Long prompts (likely pasted code)
ccql "SELECT LENGTH(display) as len, SUBSTR(display, 1, 60) as preview
      FROM history ORDER BY len DESC LIMIT 10"
```

### Transcript Queries

```bash
# Tool usage stats
ccql "SELECT tool_name, COUNT(*) as n FROM transcripts
      WHERE type='tool_use' GROUP BY tool_name ORDER BY n DESC"

# Most active sessions
ccql "SELECT _session_id, COUNT(*) as n FROM transcripts
      GROUP BY _session_id ORDER BY n DESC LIMIT 10"

# Find sessions mentioning a topic
ccql "SELECT DISTINCT _session_id FROM transcripts
      WHERE content LIKE '%authentication%'"
```

### Todo Queries

```bash
# Pending todos
ccql "SELECT content FROM todos WHERE status='pending'"

# Todo counts by status
ccql "SELECT status, COUNT(*) as n FROM todos GROUP BY status"
```

## Output Formats

```bash
ccql -f table "SELECT ..."    # Pretty table (default)
ccql -f json "SELECT ..."     # JSON array
ccql -f jsonl "SELECT ..."    # JSON lines
ccql -f raw "SELECT ..."      # Raw output
```

## Write Operations

Write operations require explicit flags for safety:

```bash
# Preview changes (dry run)
ccql --dry-run "DELETE FROM history WHERE timestamp < 1700000000000"

# Execute with backup
ccql --write "DELETE FROM history WHERE timestamp < 1700000000000"
```

## Other Commands

```bash
ccql prompts                  # Extract prompts with filtering
ccql sessions                 # List sessions
ccql search "term"            # Full-text search
ccql todos --status pending   # List todos
ccql stats                    # Usage statistics
ccql duplicates               # Find repeated prompts
ccql query '.[]' history      # jq-style queries
```

## Configuration

```bash
# Set data directory
export CLAUDE_DATA_DIR=~/.claude

# Or via flag
ccql --data-dir ~/.claude "SELECT ..."
```

## License

MIT
