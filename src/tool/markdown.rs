/// Check: 見出しの行
pub fn is_heading(line: &str) -> bool {
    line.trim_start().starts_with('#')
}
/// Check: 空行
pub fn is_blank(line: &str) -> bool {
    line.trim().is_empty()
}
/// Check: 箇条書き
pub fn is_list(line: &str) -> bool {
    let line = line.trim_start();
    // 箇条書き
    if matches!(line.as_bytes().first(), Some(b'-' | b'*')) {
        return line
            .as_bytes()
            .get(1)
            .is_some_and(|c| c.is_ascii_whitespace());
    }
    // 番号付きリスト
    let Some(separator) = line.find('.') else {
        return false;
    };
    separator > 0
        && line[..separator].chars().all(|c| c.is_ascii_digit())
        && line[separator + 1..]
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_whitespace())
}