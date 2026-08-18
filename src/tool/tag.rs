/// Parses: `tag1,tag2,tag3` into `vec!["tag1", "tag2", "tag3"]`
pub fn parse_tags(tags: &str) -> Option<Vec<String>> {
    if tags.trim().is_empty() {
        return None;
    }
    Some(
        tags.split(',')
            .map(|tag| tag.trim().to_string())
            .filter(|tag| !tag.is_empty())
            .collect(),
    )
}
