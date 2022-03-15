#[derive(Debug)]
pub struct EntryHolder {
    description: Vec<String>,
    bnf_syntax: Vec<String>,
}

impl EntryHolder {
    pub fn new(description: Vec<String>, bnf_syntax: Vec<String>) -> Self {
        return EntryHolder {
            description,
            bnf_syntax,
        };
    }
}

#[derive(Debug)]
pub struct Entry {
	symbol_name: String,
	syntax_expr: String,
	description: String,
}

impl Entry {
	pub fn new(holder: &EntryHolder) -> Self {
		let syntax = holder.bnf_syntax
			.iter()
			.map(|s| s.trim())
			.collect::<String>()
			.split("::=")
			.map(|s| s.to_string())
			.collect::<Vec<String>>();
		let symbol_name = syntax.get(0)
			.expect(format!("Invalid BNF Syntax {}", syntax.join(" ")).as_str())
			.trim()
			.to_string();
		let syntax_expr = syntax.get(1)
			.expect(format!("Inavlid BNF Syntax {}", syntax.join(" ")).as_str())
			.trim()
			.to_string();
		let description = holder.description.join(" ");
		return Entry {
			symbol_name,
			syntax_expr,
			description
		}
	}
}
