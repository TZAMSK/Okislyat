pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    take_while(|c| c.is_ascii_digit(), s)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("bad operator"),
    }

    (&s[1..], &s[0..1])
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| c == ' ', s)
}

pub(crate) fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];

    (remainder, extracted)
}

pub(crate) fn extract_ident(s: &str) -> (&str, &str) {
    let input_starts_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);

    if input_starts_with_alphabetic {
        take_while(|c| c.is_ascii_alphanumeric(), s)
    } else {
        (s, "")
    }
}

pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> &'b str {
    if s.starts_with(starting_text) {
        &s[starting_text.len()..]
    } else {
        panic!("expected {}", starting_text);
    }
}
