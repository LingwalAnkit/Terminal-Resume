//! Word wrapping for section text. The first line and the following lines can use different indents.

/// Wraps `text` to `width` columns. The first line starts with `first_indent`,
/// and every line after it starts with `next_indent`.
/// A word longer than the width gets a line to itself.
pub fn wrap(text: &str, width: usize, first_indent: &str, next_indent: &str) -> Vec<String> {
    let width = width.max(1);
    let mut lines = Vec::new();
    let mut line = first_indent.to_string();
    let mut has_word = false;

    for word in text.split_whitespace() {
        let word_len = word.chars().count();
        let line_len = line.chars().count();
        let gap = usize::from(has_word);

        if has_word && line_len + gap + word_len > width {
            lines.push(line);
            line = next_indent.to_string();
            has_word = false;
        }
        if has_word {
            line.push(' ');
        }
        line.push_str(word);
        has_word = true;
    }

    if has_word || lines.is_empty() {
        lines.push(line);
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::wrap;

    #[test]
    fn wraps_at_word_boundaries() {
        assert_eq!(wrap("aaa bbb ccc", 7, "", ""), vec!["aaa bbb", "ccc"]);
    }

    #[test]
    fn uses_different_first_and_following_indents() {
        assert_eq!(
            wrap("aaa bbb ccc", 8, "• ", "  "),
            vec!["• aaa", "  bbb", "  ccc"]
        );
    }

    #[test]
    fn long_word_gets_its_own_line() {
        assert_eq!(
            wrap("supercalifragilistic x", 5, "", ""),
            vec!["supercalifragilistic", "x"]
        );
    }

    #[test]
    fn exact_fit_stays_on_one_line() {
        assert_eq!(wrap("aa bb", 5, "", ""), vec!["aa bb"]);
    }

    #[test]
    fn empty_text_keeps_the_indent() {
        assert_eq!(wrap("", 10, "• ", "  "), vec!["• "]);
    }
}
