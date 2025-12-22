use std::collections::HashMap;
use strsim::normalized_levenshtein;

#[derive(Debug, Clone)]
pub struct PromptCluster {
    pub canonical: String,
    pub variants: Vec<String>,
    pub count: usize,
}

pub struct FuzzyDeduper {
    threshold: f64,
}

impl FuzzyDeduper {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    /// Cluster similar prompts together using fuzzy matching
    pub fn cluster(&self, prompts: Vec<String>) -> Vec<PromptCluster> {
        // Count occurrences first
        let mut counts: HashMap<String, usize> = HashMap::new();
        for prompt in &prompts {
            let normalized = self.normalize(prompt);
            if !normalized.is_empty() && normalized.len() > 3 {
                *counts.entry(normalized).or_insert(0) += 1;
            }
        }

        // Sort by count descending
        let mut items: Vec<(String, usize)> = counts.into_iter().collect();
        items.sort_by(|a, b| b.1.cmp(&a.1));

        // Cluster similar items
        let mut clusters: Vec<PromptCluster> = Vec::new();

        for (prompt, count) in items {
            // Check if this prompt belongs to an existing cluster
            let mut found_cluster = false;

            for cluster in &mut clusters {
                if self.is_similar(&prompt, &cluster.canonical) {
                    cluster.variants.push(prompt.clone());
                    cluster.count += count;
                    found_cluster = true;
                    break;
                }
            }

            if !found_cluster {
                clusters.push(PromptCluster {
                    canonical: prompt.clone(),
                    variants: vec![prompt],
                    count,
                });
            }
        }

        // Sort clusters by total count
        clusters.sort_by(|a, b| b.count.cmp(&a.count));
        clusters
    }

    fn normalize(&self, s: &str) -> String {
        let s = s.trim().to_lowercase();

        // Filter out code-like content
        if s.contains("import ")
            || s.contains("export ")
            || s.contains("const ")
            || s.contains("function ")
            || s.contains("interface ")
            || s.starts_with("//")
            || s.starts_with("/*")
            || s.starts_with("```")
            || s.contains(".js:")
            || s.contains(".ts:")
            || s.contains(".tsx:")
            || s.contains("chunk-")
            || s.contains("requestanimationframe")
            || s.contains("installhook")
            || s.starts_with('[')
            || s.starts_with('{')
            || s.starts_with('<')
        {
            return String::new();
        }

        s
    }

    fn is_similar(&self, a: &str, b: &str) -> bool {
        // Quick check for exact match
        if a == b {
            return true;
        }

        // Length check - very different lengths are unlikely to be similar
        let len_ratio = a.len().min(b.len()) as f64 / a.len().max(b.len()) as f64;
        if len_ratio < 0.5 {
            return false;
        }

        // Use normalized Levenshtein distance
        let similarity = normalized_levenshtein(a, b);
        similarity >= self.threshold
    }
}

impl Default for FuzzyDeduper {
    fn default() -> Self {
        Self::new(0.8) // 80% similarity threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similar_prompts() {
        let deduper = FuzzyDeduper::default();
        assert!(deduper.is_similar("continue", "continue"));
        assert!(deduper.is_similar("continue", "cotninue"));
        assert!(deduper.is_similar("continue", "contnue"));
        assert!(deduper.is_similar("commit this", "commit that"));
    }

    #[test]
    fn test_different_prompts() {
        let deduper = FuzzyDeduper::default();
        assert!(!deduper.is_similar("continue", "fix issues"));
        assert!(!deduper.is_similar("commit this", "run tests"));
    }

    #[test]
    fn test_clustering() {
        let deduper = FuzzyDeduper::default();
        let prompts = vec![
            "continue".to_string(),
            "continue".to_string(),
            "cotninue".to_string(),
            "contnue".to_string(),
            "fix it".to_string(),
            "fix this".to_string(),
        ];

        let clusters = deduper.cluster(prompts);
        assert!(!clusters.is_empty());
    }
}
