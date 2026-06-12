use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Frontmatter {
    pub title: Option<String>,
    pub category: Option<String>,
    pub tile_color: Option<String>,
    pub render_markdown: Option<bool>,
}

/// Parses a string that might contain a YAML frontmatter block.
/// Returns the extracted frontmatter and the remaining content (body).
pub fn parse_mint_content(content: &str) -> (Frontmatter, &str) {
    if !content.starts_with("---\n") && !content.starts_with("---\r\n") {
        return (Frontmatter::default(), content);
    }

    let end_marker_index = content[3..].find("\n---");
    if let Some(mut idx) = end_marker_index {
        idx += 3; // Offset for the initial slice
        let frontmatter_str = &content[..idx];
        let mut body = &content[idx + 4..];
        if body.starts_with('\n') {
            body = &body[1..];
        } else if body.starts_with("\r\n") {
            body = &body[2..];
        }

        let mut fm = Frontmatter::default();
        for line in frontmatter_str.lines().skip(1) {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim().trim_matches(|c| c == '"' || c == '\'');
                match key {
                    "title" => fm.title = Some(value.to_string()),
                    "category" => fm.category = Some(value.to_string()),
                    "tileColor" => fm.tile_color = Some(value.to_string()),
                    "renderMarkdown" => {
                        if value == "true" {
                            fm.render_markdown = Some(true);
                        } else if value == "false" {
                            fm.render_markdown = Some(false);
                        }
                    }
                    _ => {}
                }
            }
        }
        return (fm, body);
    }

    (Frontmatter::default(), content)
}

/// Formats the frontmatter block and prepends it to the body.
pub fn format_mint_content(body: &str, frontmatter: &Frontmatter) -> String {
    let mut out = String::from("---\n");
    if let Some(title) = &frontmatter.title {
        // Escaping simple quotes just by using JSON-like serialization is overkill, but let's avoid double quotes inside.
        // Or we can just output without quotes if safe, or with quotes if needed.
        // Using quotes is safer for YAML parsing.
        let escaped = title.replace('"', "\\\"");
        out.push_str(&format!("title: \"{}\"\n", escaped));
    }
    if let Some(category) = &frontmatter.category {
        let escaped = category.replace('"', "\\\"");
        out.push_str(&format!("category: \"{}\"\n", escaped));
    }
    if let Some(tile_color) = &frontmatter.tile_color {
        let escaped = tile_color.replace('"', "\\\"");
        out.push_str(&format!("tileColor: \"{}\"\n", escaped));
    }
    if let Some(render_markdown) = frontmatter.render_markdown {
        out.push_str(&format!("renderMarkdown: {}\n", render_markdown));
    }
    out.push_str("---\n\n");
    out.push_str(body);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let content = "---\ntitle: \"Hello\"\ncategory: 'Work'\ntileColor: blue\nrenderMarkdown: true\n---\nHello world";
        let (fm, body) = parse_mint_content(content);
        assert_eq!(fm.title.as_deref(), Some("Hello"));
        assert_eq!(fm.category.as_deref(), Some("Work"));
        assert_eq!(fm.tile_color.as_deref(), Some("blue"));
        assert_eq!(fm.render_markdown, Some(true));
        assert_eq!(body, "Hello world");
    }

    #[test]
    fn test_parse_no_frontmatter() {
        let content = "Hello world";
        let (fm, body) = parse_mint_content(content);
        assert_eq!(fm.title, None);
        assert_eq!(body, "Hello world");
    }

    #[test]
    fn test_format_frontmatter() {
        let fm = Frontmatter {
            title: Some("Hello \"World\"".to_string()),
            category: Some("Work".to_string()),
            tile_color: None,
            render_markdown: Some(true),
        };
        let out = format_mint_content("Hello body", &fm);
        assert_eq!(out, "---\ntitle: \"Hello \\\"World\\\"\"\ncategory: \"Work\"\nrenderMarkdown: true\n---\n\nHello body");
    }
}
