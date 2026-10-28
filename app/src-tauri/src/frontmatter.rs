// Known limit: hand-rolled reader for the YAML subset agents use (flat `key: value`, quoted
// values, folded `>`/`|` blocks). Swap for a YAML crate if agents ever nest keys.

#[derive(Debug, Clone, PartialEq)]
pub struct Frontmatter {
    pub name: String,
    pub description: String,
}

pub fn parse(markdown: &str) -> Option<Frontmatter> {
    let block = block(markdown)?;
    Some(Frontmatter {
        name: field(block, "name")?,
        description: field(block, "description")?,
    })
}

/// Lowercase kebab-case ASCII. The name becomes a file name, so this is also the
/// path-traversal guard for saving and installing.
pub fn is_valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 64
        && !name.starts_with('-')
        && !name.ends_with('-')
        && name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// Models sometimes wrap the file in a code fence or add a preamble; keep only the agent.
pub fn extract_agent(raw: &str) -> Option<String> {
    let start = line_offset(raw, "---")?;
    let body = raw[start..].trim_end();
    let body = body.strip_suffix("```").unwrap_or(body).trim_end();
    let meta = parse(body)?;
    is_valid_name(&meta.name).then(|| format!("{body}\n"))
}

pub fn rename(markdown: &str, new_name: &str) -> String {
    let mut renamed = false;
    markdown
        .split_inclusive('\n')
        .map(|line| {
            if renamed || !line.starts_with("name:") {
                return line.to_string();
            }
            renamed = true;
            let eol = if line.ends_with('\n') { "\n" } else { "" };
            format!("name: {new_name}{eol}")
        })
        .collect()
}

fn block(markdown: &str) -> Option<&str> {
    let rest = markdown.trim_start_matches('\u{feff}').strip_prefix("---")?;
    let rest = rest.strip_prefix("\r\n").or_else(|| rest.strip_prefix('\n'))?;
    let end = rest.find("\n---")?;
    Some(&rest[..end])
}

fn field(block: &str, key: &str) -> Option<String> {
    let mut lines = block.lines();
    while let Some(line) = lines.next() {
        let Some(value) = line.strip_prefix(key).and_then(|r| r.strip_prefix(':')) else {
            continue;
        };
        let value = value.trim();
        let text = if value.starts_with('>') || value.starts_with('|') {
            folded(&mut lines)
        } else {
            unquote(value).to_string()
        };
        return (!text.is_empty()).then_some(text);
    }
    None
}

fn folded<'a>(lines: &mut impl Iterator<Item = &'a str>) -> String {
    lines
        .take_while(|l| l.starts_with(' ') || l.starts_with('\t'))
        .map(str::trim)
        .collect::<Vec<_>>()
        .join(" ")
}

fn unquote(value: &str) -> &str {
    for quote in ['"', '\''] {
        if let Some(inner) = value.strip_prefix(quote).and_then(|v| v.strip_suffix(quote)) {
            return inner;
        }
    }
    value
}

fn line_offset(text: &str, marker: &str) -> Option<usize> {
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        if line.trim_end() == marker {
            return Some(offset);
        }
        offset += line.len();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const AGENT: &str = "---\nname: traffic-manager\ndescription: Runs ads. Examples: \"x\"\ntools: Read\n---\n\n# Body\n";

    #[test]
    fn parses_flat_fields() {
        let meta = parse(AGENT).unwrap();
        assert_eq!(meta.name, "traffic-manager");
        assert_eq!(meta.description, "Runs ads. Examples: \"x\"");
    }

    #[test]
    fn parses_quoted_and_folded_values() {
        let md = "---\nname: 'seo'\ndescription: >\n  Ranks pages\n  on Google.\nmodel: inherit\n---\n";
        let meta = parse(md).unwrap();
        assert_eq!(meta.name, "seo");
        assert_eq!(meta.description, "Ranks pages on Google.");
    }

    #[test]
    fn rejects_missing_frontmatter_or_fields() {
        assert!(parse("# just markdown").is_none());
        assert!(parse("---\nname: a\n---\n").is_none());
        assert!(parse("---\nnames: a\ndescription: b\n---\n").is_none());
    }

    #[test]
    fn validates_names() {
        assert!(is_valid_name("meta-ads-clinic-2"));
        for bad in ["", "-a", "a-", "Upper", "naïve", "../x", "a/b", &"a".repeat(65)] {
            assert!(!is_valid_name(bad), "{bad} should be invalid");
        }
    }

    #[test]
    fn extracts_agent_from_fenced_output_with_preamble() {
        let raw = format!("Here is your agent:\n```markdown\n{AGENT}```\n");
        assert_eq!(extract_agent(&raw).unwrap(), AGENT);
        assert_eq!(extract_agent(AGENT).unwrap(), AGENT);
    }

    #[test]
    fn extract_rejects_invalid_names() {
        let raw = AGENT.replace("traffic-manager", "Traffic Manager");
        assert!(extract_agent(&raw).is_none());
    }

    #[test]
    fn renames_only_the_frontmatter_name() {
        let renamed = rename(AGENT, "traffic-manager-2");
        assert_eq!(parse(&renamed).unwrap().name, "traffic-manager-2");
        assert!(renamed.ends_with("# Body\n"));
    }
}
