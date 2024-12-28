pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    let digits_end = s
        .char_indices()
        .find_map(|(idx, c)| if c.is_ascii_digit() { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let digits = &s[..digits_end];
    let remainder = &s[digits_end..];
    (remainder, digits)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("bad operator"),
    }

    (&s[1..], &s[0..1])
}
