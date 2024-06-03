use std::fs;

use biome_css_syntax::{AnyCssSelector::*, AnyCssSubSelector::*, CssDeclarationWithSemicolon};

use sha1::Digest;

fn hash(string: String) -> String {
    let mut hasher = sha1::Sha1::new();
    hasher.update(string.as_bytes());
    let h = hasher.finalize();
    format!("{:x}", h)
}

fn parse_one(rule: String) -> biome_css_syntax::CssQualifiedRule {
    let rules = biome_css_parser::parse_css(&rule, biome_css_parser::CssParserOptions::default())
        .tree()
        .rules();
    assert!(rules.clone().into_iter().len() == 1);
    let rule = rules.into_iter().next().unwrap();

    rule.as_css_qualified_rule().unwrap().to_owned()
}

fn parse_property(name: String, value: String) -> CssDeclarationWithSemicolon {
    let property_str = format!("{}: {};", name, value);
    let dummy_rule = parse_one(format!(".a {{ {} }}", property_str));
    let block = dummy_rule.block().unwrap();
    let block = block.as_css_declaration_or_rule_block();
    assert!(block.unwrap().items().into_iter().len() == 1);
    let item = block.unwrap().items().into_iter().next().unwrap();
    item.as_css_declaration_with_semicolon().unwrap().to_owned()
}

pub fn store_property(selector: biome_css_syntax::AnyCssSelector, name: String, value: String) {
    let db_path = selector.to_path_parts().join("/");
    let path = format!("db/{}", db_path);
    match fs::read_to_string(path.clone()) {
        Ok(rule) => {
            let _: Option<biome_css_syntax::AnyCssDeclarationOrRule> = None;

            let rule = parse_one(rule);
            let new_property = parse_property(name, value);
            let block = rule.block().unwrap();
            let block = block.as_css_declaration_or_rule_block().unwrap();
            let selector = rule.prelude().into_iter().next().unwrap().unwrap();
            assert!(selector.to_path_parts().join("/n") == db_path);

            let old_properties = block
                .items()
                .into_iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join("\n");

            let new_rule = format!("{} {{ {}\n  {}\n}}", selector, old_properties, new_property);
            fs::write(path, new_rule).unwrap();
        }
        Err(e) => todo!("{:?}", e),
    }
}

pub trait Storage {
    fn to_path_parts(&self) -> Vec<String>;
}

impl Storage for biome_css_syntax::AnyCssSelector {
    fn to_path_parts(&self) -> Vec<String> {
        match self {
            CssBogusSelector(_) => todo!(),
            CssComplexSelector(_) => todo!(),
            CssCompoundSelector(selector) => selector.to_path_parts(),
        }
    }
}

impl Storage for biome_css_syntax::CssCompoundSelector {
    fn to_path_parts(&self) -> Vec<String> {
        self.sub_selectors()
            .into_iter()
            .flat_map(|selector| selector.to_path_parts())
            .collect()
    }
}

impl Storage for biome_css_syntax::AnyCssSubSelector {
    fn to_path_parts(&self) -> Vec<String> {
        match self {
            CssAttributeSelector(_) => todo!(),
            CssBogusSubSelector(_) => todo!(),
            CssClassSelector(class) => {
                let n = class.name().unwrap().to_string();
                // TODO: remove trim
                let name = n.trim();
                assert!(name == "btn".to_string());
                return vec![hash(".".to_string() + &name)];
            }
            CssIdSelector(_) => todo!(),
            CssPseudoClassSelector(_) => todo!(),
            CssPseudoElementSelector(_) => todo!(),
        }
    }
}
