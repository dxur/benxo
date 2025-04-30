pub fn is_subpath(base: &str, current: &str) -> bool {
    let base = base.trim_end_matches('/');
    let current = if current.is_empty() { "/" } else { current };

    if base.is_empty() {
        return current == "/" || current.is_empty();
    }

    if current == base {
        return true;
    }

    if let Some(stripped) = current.strip_prefix(base) {
        return stripped == "/" || stripped.starts_with('/');
    }

    false
}
