use serde::Serialize;

use crate::frontmatter;

/// A Codex custom agent (`~/.codex/agents/<name>.toml`). `model` is left out so the agent
/// inherits the session's model, like `model: inherit` does in Claude Code.
#[derive(Serialize)]
struct CodexAgent<'a> {
    name: &'a str,
    description: &'a str,
    developer_instructions: &'a str,
}

/// The same agent, in the TOML form Codex reads. `None` when the Markdown has no valid
/// frontmatter.
pub fn from_markdown(markdown: &str) -> Option<String> {
    let meta = frontmatter::parse(markdown)?;
    let instructions = frontmatter::body(markdown)?.trim();
    toml::to_string(&CodexAgent {
        name: &meta.name,
        description: &meta.description,
        developer_instructions: instructions,
    })
    .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(serde::Deserialize, Debug, PartialEq)]
    struct Parsed {
        name: String,
        description: String,
        developer_instructions: String,
    }

    fn convert(markdown: &str) -> Parsed {
        toml::from_str(&from_markdown(markdown).unwrap()).unwrap()
    }

    #[test]
    fn keeps_name_description_and_body() {
        let md = "---\nname: seo\ndescription: >-\n  Ranks pages. Examples: \"x\"\nmodel: inherit\n---\n\n# SEO\n\nDo it.\n";
        assert_eq!(
            convert(md),
            Parsed {
                name: "seo".into(),
                description: "Ranks pages. Examples: \"x\"".into(),
                developer_instructions: "# SEO\n\nDo it.".into(),
            }
        );
    }

    #[test]
    fn survives_toml_delimiters_in_the_body() {
        let body = "Quote ''' and \"\"\" and a \\ path\ttab";
        let md = format!("---\nname: a\ndescription: b\n---\n{body}\n");
        assert_eq!(convert(&md).developer_instructions, body);
    }

    #[test]
    fn rejects_files_without_frontmatter() {
        assert!(from_markdown("# just markdown").is_none());
    }
}
