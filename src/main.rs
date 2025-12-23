use clap::{Parser, Subcommand};
use std::path::PathBuf;

use ccql::cli::commands;
use ccql::cli::OutputFormat;
use ccql::config::Config;
use ccql::error::Result;
use ccql::models::TodoStatus;

#[derive(Parser)]
#[command(name = "ccql")]
#[command(author = "Claude Code Query")]
#[command(version = "0.1.0")]
#[command(about = "Query and analyze Claude Code data", long_about = None)]
struct Cli {
    /// Path to Claude data directory
    #[arg(long, env = "CLAUDE_DATA_DIR")]
    data_dir: Option<PathBuf>,

    /// Output format
    #[arg(short, long, value_enum, default_value = "table")]
    format: OutputFormat,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract all user prompts from history
    Prompts {
        /// Filter by session ID
        #[arg(long)]
        session: Option<String>,

        /// Filter by project
        #[arg(long)]
        project: Option<String>,

        /// Filter by date range (YYYY-MM-DD)
        #[arg(long)]
        since: Option<String>,

        #[arg(long)]
        until: Option<String>,

        /// Limit number of results
        #[arg(short, long)]
        limit: Option<usize>,
    },

    /// Execute arbitrary jq query on data
    Query {
        /// jq-style query expression
        query: String,

        /// Data source to query (history, transcripts, stats, todos)
        source: String,

        /// Filter by file pattern (for transcripts)
        #[arg(long)]
        file_pattern: Option<String>,
    },

    /// List and browse sessions
    Sessions {
        /// Show session details
        #[arg(short, long)]
        detailed: bool,

        /// Filter by project
        #[arg(long)]
        project: Option<String>,

        /// Sort by: time, size
        #[arg(long, default_value = "time")]
        sort_by: String,
    },

    /// Display usage statistics
    Stats {
        /// Group by: model, date
        #[arg(long, default_value = "model")]
        group_by: String,

        /// Show statistics for date range
        #[arg(long)]
        since: Option<String>,

        #[arg(long)]
        until: Option<String>,
    },

    /// Full-text search across all data
    Search {
        /// Search term or pattern
        term: String,

        /// Search scope (all, prompts, transcripts)
        #[arg(long, default_value = "all")]
        scope: String,

        /// Case-sensitive search
        #[arg(short, long)]
        case_sensitive: bool,

        /// Use regex pattern
        #[arg(short, long)]
        regex: bool,

        /// Context lines before match
        #[arg(short = 'B', long, default_value = "0")]
        before_context: usize,

        /// Context lines after match
        #[arg(short = 'A', long, default_value = "0")]
        after_context: usize,
    },

    /// List all todos and their status
    Todos {
        /// Filter by status (pending, in_progress, completed)
        #[arg(long)]
        status: Option<String>,

        /// Filter by agent ID
        #[arg(long)]
        agent: Option<String>,
    },

    /// Find repeated/similar prompts using fuzzy matching
    Duplicates {
        /// Similarity threshold (0.0-1.0, default 0.8)
        #[arg(short, long, default_value = "0.8")]
        threshold: f64,

        /// Minimum count to show
        #[arg(short, long, default_value = "2")]
        min_count: usize,

        /// Maximum clusters to show
        #[arg(short, long, default_value = "50")]
        limit: usize,

        /// Show variants in each cluster
        #[arg(long)]
        show_variants: bool,

        /// Sort by: count (default) or latest
        #[arg(short, long, default_value = "count")]
        sort: String,

        /// Minimum prompt length in characters
        #[arg(long, default_value = "4")]
        min_length: usize,
    },

    /// Execute SQL queries on Claude Code data
    Sql {
        /// SQL query to execute (e.g., "SELECT * FROM history LIMIT 10")
        query: String,

        /// Enable write operations (INSERT, UPDATE, DELETE)
        #[arg(long)]
        write: bool,

        /// Preview what would be modified without making changes
        #[arg(long)]
        dry_run: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        tracing_subscriber::fmt()
            .with_env_filter("ccql=debug")
            .init();
    }

    let data_dir = cli
        .data_dir
        .or_else(|| {
            std::env::var("CLAUDE_DATA_DIR")
                .ok()
                .map(PathBuf::from)
        })
        .unwrap_or_else(Config::default_data_dir);

    let config = Config::new(data_dir)?;

    match cli.command {
        Commands::Prompts {
            session,
            project,
            since,
            until,
            limit,
        } => {
            commands::prompts(&config, session, project, since, until, limit, cli.format).await?;
        }
        Commands::Query {
            query,
            source,
            file_pattern,
        } => {
            commands::query(&config, &query, &source, file_pattern, cli.format).await?;
        }
        Commands::Sessions {
            detailed,
            project,
            sort_by,
        } => {
            commands::sessions(&config, detailed, project, &sort_by, cli.format).await?;
        }
        Commands::Stats {
            group_by,
            since,
            until,
        } => {
            commands::stats(&config, &group_by, since, until, cli.format).await?;
        }
        Commands::Search {
            term,
            scope,
            case_sensitive,
            regex,
            before_context,
            after_context,
        } => {
            commands::search(
                &config,
                &term,
                &scope,
                case_sensitive,
                regex,
                before_context,
                after_context,
                cli.format,
            )
            .await?;
        }
        Commands::Todos { status, agent } => {
            let status = status.and_then(|s| match s.as_str() {
                "pending" => Some(TodoStatus::Pending),
                "in_progress" => Some(TodoStatus::InProgress),
                "completed" => Some(TodoStatus::Completed),
                _ => None,
            });
            commands::todos(&config, status, agent, cli.format).await?;
        }
        Commands::Duplicates {
            threshold,
            min_count,
            limit,
            show_variants,
            sort,
            min_length,
        } => {
            commands::duplicates(&config, threshold, min_count, limit, show_variants, &sort, min_length, cli.format)
                .await?;
        }
        Commands::Sql {
            query,
            write,
            dry_run,
        } => {
            commands::sql(&config, &query, write, dry_run, cli.format).await?;
        }
    }

    Ok(())
}
