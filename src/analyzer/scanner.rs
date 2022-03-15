use super::entry;

pub fn scan<'a>(src: &'a String) -> Vec<entry::EntryHolder> {
    let mut lines = src.lines().peekable();
    let mut entries = Vec::<entry::EntryHolder>::new();
    let mut description = Vec::<String>::new();
    let mut bnf_syntax = Vec::<String>::new();

    while lines.peek().is_some() {
        if let Some(l) = lines.next() {
            if l.starts_with("//") {
                // bnf syntax's description, assume it's written in Markdown.
                description.push(l[2..].trim().to_string());
            } else if !l.trim().is_empty() {
                // bnf syntax
                bnf_syntax.push(l.to_string());
            } else {
                add_entry(&mut entries, &mut description, &mut bnf_syntax);
            }
        }
    }

    // Post handle if file is not ended with an empty line
    if !description.is_empty() || !bnf_syntax.is_empty() {
        add_entry(&mut entries, &mut description, &mut bnf_syntax);
    }

    return entries;
}

fn add_entry(
    entries:  &mut Vec<entry::EntryHolder>,
    description: &mut Vec<String>,
    bnf_syntax: &mut Vec<String>,
) {
    entries.push(entry::EntryHolder::new(description.clone(), bnf_syntax.clone()));
    description.clear();
    bnf_syntax.clear();
}
