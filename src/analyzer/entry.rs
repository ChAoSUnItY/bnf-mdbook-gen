use ebnf::{Node, SymbolKind};

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

#[derive(Debug, Clone)]
pub struct Entry {
    pub grammar: ebnf::Grammar,
    pub description: Vec<String>,
}

impl Entry {
    pub fn new(holder: &EntryHolder) -> Self {
        let grammar = match ebnf::get_grammar(holder.bnf_syntax.join(" ").as_str()) {
            Ok(grammar) => grammar,
            Err(err) => panic!("{}", err),
        };

        let description = holder.description.clone();

        return Entry {
            grammar,
            description,
        };
    }

    pub fn get_content(&mut self) -> Vec<String> {
        let expression = &self.grammar.expressions[0];
        let lhs = expression.lhs.to_owned();
        let rhs = expression.rhs.to_owned();
        let mut builder = vec!["> **<sup>Syntax:</sup>**\\".to_string()];
        let node = build_node(&rhs);

        builder.push(format!("> _{}_ :\\", lhs));
        builder.push(format_node_string(&node));
        builder.push(String::new());
        builder.append(&mut self.description);

        return builder;
    }    
}

fn format_node_string(node_string: &String) -> String {
    let splitted = node_string.split("\n");
    let mut builder = Vec::<String>::new();

    for (i, sp) in splitted.enumerate() {
        let mut current_builder = String::from("> &nbsp;&nbsp; ");
        
        if i == 0 {
            current_builder.push_str("&nbsp;&nbsp; "); 
        }

        current_builder.push_str(sp.trim_end());
        current_builder.push('\\');
        builder.push(current_builder);
    }

    return builder.join("\n");
}

fn build_node<'a>(node: &'a Node) -> String {
        match node {
            Node::Terminal(s) => s.to_owned(),
            Node::String(s) => format!("`{}`", s.to_owned()),
            Node::RegexString(_) => todo!(),
            Node::Multiple(_) => todo!(),
            Node::RegexExt(_, _) => todo!(),
            Node::Symbol(left, kind, right) => format!("{} {} {}", build_node(left), match kind {
                SymbolKind::Alternation => "\n|",
                SymbolKind::Concatenation => "\n,"
            }, build_node(right)),
            Node::Group(_) => todo!(),
            Node::Optional(_) => todo!(),
            Node::Repeat(_) => todo!(),
            Node::Unknown => todo!(),
        }
    }