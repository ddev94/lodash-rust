pub struct TruncateOptions<'a> {
    pub length: usize,
    pub separator: Option<&'a str>,
    pub omission: &'a str,
}

impl<'a> Default for TruncateOptions<'a> {
    fn default() -> Self {
        TruncateOptions {
            length: 30,
            separator: None,
            omission: "...",
        }
    }
}

pub fn truncate(s: &str, options: Option<TruncateOptions>) -> String {
    let opts = options.unwrap_or_default();
    let length = opts.length;
    let omission = opts.omission;
    
    // If string is shorter than target length, return it as is
    if s.len() <= length {
        return s.to_string();
    }

    match opts.separator {
        Some(sep) => truncate_by_separator(s, length, sep, omission),
        None => truncate_simple(s, length, omission),
    }
}

pub fn truncate_simple(s: &str, length: usize, omission: &str) -> String {
    let available_length = if length > omission.len() {
        length - omission.len()
    } else {
        0
    };

    // Find the char boundary
    let mut chars = s.char_indices();
    let mut last_valid_idx = 0;
    
    while let Some((idx, _)) = chars.next() {
        if idx > available_length {
            break;
        }
        last_valid_idx = idx;
    }

    let mut result = s[..last_valid_idx].to_string();
    result.push_str(omission);
    result
}

pub fn truncate_by_separator(s: &str, length: usize, separator: &str, omission: &str) -> String {
    let available_length = if length > omission.len() {
        length - omission.len()
    } else {
        0
    };

    // For patterns like ", " or " ", split the string and reconstruct
    let parts: Vec<&str> = s.split(separator).collect();
    let mut current_length = 0;
    let mut last_valid_index = 0;

    for (i, part) in parts.iter().enumerate() {
        let part_length = if i == 0 {
            part.len()
        } else {
            part.len() + separator.len() // Add separator length for all parts except first
        };

        if current_length + part_length > available_length {
            break;
        }

        current_length += part_length;
        last_valid_index = i;
    }

    let mut result = String::new();
    for (i, part) in parts.iter().take(last_valid_index + 1).enumerate() {
        if i > 0 {
            result.push_str(separator);
        }
        result.push_str(part);
    }

    result.push_str(omission);
    result
}