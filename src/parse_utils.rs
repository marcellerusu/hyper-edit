use biome_css_syntax::{AnyCssSelector, CssDeclarationWithSemicolon, CssSyntaxKind};

pub fn parse_selector(str: &String) -> AnyCssSelector {
    let rule = biome_css_parser::parse_css(
        format!("{} {{}}", str).as_str(),
        biome_css_parser::CssParserOptions::default(),
    )
    .tree()
    .rules()
    .into_iter()
    .next()
    .unwrap();

    rule.as_css_qualified_rule()
        .unwrap()
        .prelude()
        .into_iter()
        .next()
        .unwrap()
        .unwrap()
}

fn parse_one(rule: String) -> biome_css_syntax::CssQualifiedRule {
    let rules = biome_css_parser::parse_css(&rule, biome_css_parser::CssParserOptions::default())
        .tree()
        .rules();
    assert!((&rules).into_iter().len() == 1);
    let rule = rules.into_iter().next().unwrap();

    rule.as_css_qualified_rule().unwrap().to_owned()
}

pub fn name_of_item(item: &CssDeclarationWithSemicolon) -> String {
    let decl = item.declaration();
    let property = decl.unwrap().property().unwrap();
    let property = property.as_css_generic_property().unwrap();
    let name_node = property.name().unwrap();
    let value = name_node
        .as_css_identifier()
        .unwrap()
        .value_token()
        .unwrap();
    value.token_text_trimmed().to_string()
}

pub fn parse_raw_property(property_str: &String) -> CssDeclarationWithSemicolon {
    let dummy_rule = parse_one(format!(".a {{ {} }}", property_str));
    let block = dummy_rule.block().unwrap();
    let block = block.as_css_declaration_or_rule_block().unwrap();
    assert!(block.items().into_iter().len() == 1);
    let item = block.items().into_iter().next().unwrap();
    println!("{:?}", item);
    item.as_css_declaration_with_semicolon().unwrap().to_owned()
}

pub fn parse_property(name: &String, value: &String) -> CssDeclarationWithSemicolon {
    let property_str = format!("{}: {};", name, value);
    let dummy_rule = parse_one(format!(".a {{ {} }}", property_str));
    let block = dummy_rule.block().unwrap();
    let block = block.as_css_declaration_or_rule_block().unwrap();
    assert!(block.items().into_iter().len() == 1);
    let item = block.items().into_iter().next().unwrap();
    item.as_css_declaration_with_semicolon().unwrap().to_owned()
}

pub fn get_combinator_type(token_kind: CssSyntaxKind) -> String {
    match token_kind {
        CssSyntaxKind::CSS_SPACE_LITERAL => "descendent".to_string(),
        CssSyntaxKind::EOF => todo!(),
        CssSyntaxKind::UNICODE_BOM => todo!(),
        CssSyntaxKind::SEMICOLON => todo!(),
        CssSyntaxKind::COMMA => todo!(),
        CssSyntaxKind::L_PAREN => todo!(),
        CssSyntaxKind::R_PAREN => todo!(),
        CssSyntaxKind::L_CURLY => todo!(),
        CssSyntaxKind::R_CURLY => todo!(),
        CssSyntaxKind::L_BRACK => todo!(),
        CssSyntaxKind::R_BRACK => todo!(),
        CssSyntaxKind::L_ANGLE => todo!(),
        CssSyntaxKind::R_ANGLE => todo!(),
        CssSyntaxKind::TILDE => todo!(),
        CssSyntaxKind::HASH => todo!(),
        CssSyntaxKind::AMP => todo!(),
        CssSyntaxKind::PIPE => todo!(),
        CssSyntaxKind::PIPE2 => todo!(),
        CssSyntaxKind::PLUS => todo!(),
        CssSyntaxKind::STAR => todo!(),
        CssSyntaxKind::SLASH => todo!(),
        CssSyntaxKind::CARET => todo!(),
        CssSyntaxKind::PERCENT => todo!(),
        CssSyntaxKind::DOT => todo!(),
        CssSyntaxKind::COLON => todo!(),
        CssSyntaxKind::COLON2 => todo!(),
        CssSyntaxKind::EQ => todo!(),
        CssSyntaxKind::BANG => todo!(),
        CssSyntaxKind::NEQ => todo!(),
        CssSyntaxKind::MINUS => todo!(),
        CssSyntaxKind::LTEQ => todo!(),
        CssSyntaxKind::GTEQ => todo!(),
        CssSyntaxKind::PLUSEQ => todo!(),
        CssSyntaxKind::PIPEEQ => todo!(),
        CssSyntaxKind::AMPEQ => todo!(),
        CssSyntaxKind::CARETEQ => todo!(),
        CssSyntaxKind::SLASHEQ => todo!(),
        CssSyntaxKind::STAREQ => todo!(),
        CssSyntaxKind::PERCENTEQ => todo!(),
        CssSyntaxKind::AT => todo!(),
        CssSyntaxKind::DOLLAR_EQ => todo!(),
        CssSyntaxKind::TILDE_EQ => todo!(),
        CssSyntaxKind::CDC => todo!(),
        CssSyntaxKind::CDO => todo!(),
        CssSyntaxKind::MEDIA_KW => todo!(),
        CssSyntaxKind::KEYFRAMES_KW => todo!(),
        CssSyntaxKind::NOT_KW => todo!(),
        CssSyntaxKind::AND_KW => todo!(),
        CssSyntaxKind::ONLY_KW => todo!(),
        CssSyntaxKind::OR_KW => todo!(),
        CssSyntaxKind::I_KW => todo!(),
        CssSyntaxKind::IMPORTANT_KW => todo!(),
        CssSyntaxKind::HIGHLIGHT_KW => todo!(),
        CssSyntaxKind::PART_KW => todo!(),
        CssSyntaxKind::DIR_KW => todo!(),
        CssSyntaxKind::LOCAL_KW => todo!(),
        CssSyntaxKind::GLOBAL_KW => todo!(),
        CssSyntaxKind::ANY_KW => todo!(),
        CssSyntaxKind::CURRENT_KW => todo!(),
        CssSyntaxKind::PAST_KW => todo!(),
        CssSyntaxKind::FUTURE_KW => todo!(),
        CssSyntaxKind::HOST_KW => todo!(),
        CssSyntaxKind::HOST_CONTEXT_KW => todo!(),
        CssSyntaxKind::MATCHES_KW => todo!(),
        CssSyntaxKind::IS_KW => todo!(),
        CssSyntaxKind::WHERE_KW => todo!(),
        CssSyntaxKind::HAS_KW => todo!(),
        CssSyntaxKind::LANG_KW => todo!(),
        CssSyntaxKind::NTH_CHILD_KW => todo!(),
        CssSyntaxKind::NTH_LAST_CHILD_KW => todo!(),
        CssSyntaxKind::NTH_OF_TYPE_KW => todo!(),
        CssSyntaxKind::NTH_LAST_OF_TYPE_KW => todo!(),
        CssSyntaxKind::NTH_COL_KW => todo!(),
        CssSyntaxKind::NTH_LAST_COL_KW => todo!(),
        CssSyntaxKind::CHARSET_KW => todo!(),
        CssSyntaxKind::COLOR_PROFILE_KW => todo!(),
        CssSyntaxKind::COUNTER_STYLE_KW => todo!(),
        CssSyntaxKind::PROPERTY_KW => todo!(),
        CssSyntaxKind::CONTAINER_KW => todo!(),
        CssSyntaxKind::STYLE_KW => todo!(),
        CssSyntaxKind::LTR_KW => todo!(),
        CssSyntaxKind::RTL_KW => todo!(),
        CssSyntaxKind::N_KW => todo!(),
        CssSyntaxKind::EVEN_KW => todo!(),
        CssSyntaxKind::ODD_KW => todo!(),
        CssSyntaxKind::OF_KW => todo!(),
        CssSyntaxKind::FROM_KW => todo!(),
        CssSyntaxKind::TO_KW => todo!(),
        CssSyntaxKind::VAR_KW => todo!(),
        CssSyntaxKind::URL_KW => todo!(),
        CssSyntaxKind::SRC_KW => todo!(),
        CssSyntaxKind::FONT_PALETTE_VALUES_KW => todo!(),
        CssSyntaxKind::FONT_FEATURE_VALUES_KW => todo!(),
        CssSyntaxKind::STYLISTIC_KW => todo!(),
        CssSyntaxKind::HISTORICAL_FORMS_KW => todo!(),
        CssSyntaxKind::STYLESET_KW => todo!(),
        CssSyntaxKind::CHARACTER_VARIANT_KW => todo!(),
        CssSyntaxKind::SWASH_KW => todo!(),
        CssSyntaxKind::ORNAMENTS_KW => todo!(),
        CssSyntaxKind::ANNOTATION_KW => todo!(),
        CssSyntaxKind::AUTO_KW => todo!(),
        CssSyntaxKind::THIN_KW => todo!(),
        CssSyntaxKind::MEDIUM_KW => todo!(),
        CssSyntaxKind::THICK_KW => todo!(),
        CssSyntaxKind::NONE_KW => todo!(),
        CssSyntaxKind::HIDDEN_KW => todo!(),
        CssSyntaxKind::DOTTED_KW => todo!(),
        CssSyntaxKind::DASHED_KW => todo!(),
        CssSyntaxKind::SOLID_KW => todo!(),
        CssSyntaxKind::DOUBLE_KW => todo!(),
        CssSyntaxKind::GROOVE_KW => todo!(),
        CssSyntaxKind::RIDGE_KW => todo!(),
        CssSyntaxKind::INSET_KW => todo!(),
        CssSyntaxKind::OUTSET_KW => todo!(),
        CssSyntaxKind::INITIAL_KW => todo!(),
        CssSyntaxKind::INHERIT_KW => todo!(),
        CssSyntaxKind::UNSET_KW => todo!(),
        CssSyntaxKind::REVERT_KW => todo!(),
        CssSyntaxKind::REVERT_LAYER_KW => todo!(),
        CssSyntaxKind::DEFAULT_KW => todo!(),
        CssSyntaxKind::EM_KW => todo!(),
        CssSyntaxKind::REM_KW => todo!(),
        CssSyntaxKind::EX_KW => todo!(),
        CssSyntaxKind::REX_KW => todo!(),
        CssSyntaxKind::CAP_KW => todo!(),
        CssSyntaxKind::RCAP_KW => todo!(),
        CssSyntaxKind::CH_KW => todo!(),
        CssSyntaxKind::RCH_KW => todo!(),
        CssSyntaxKind::IC_KW => todo!(),
        CssSyntaxKind::RIC_KW => todo!(),
        CssSyntaxKind::LH_KW => todo!(),
        CssSyntaxKind::RLH_KW => todo!(),
        CssSyntaxKind::VW_KW => todo!(),
        CssSyntaxKind::SVW_KW => todo!(),
        CssSyntaxKind::LVW_KW => todo!(),
        CssSyntaxKind::DVW_KW => todo!(),
        CssSyntaxKind::VH_KW => todo!(),
        CssSyntaxKind::SVH_KW => todo!(),
        CssSyntaxKind::LVH_KW => todo!(),
        CssSyntaxKind::DVH_KW => todo!(),
        CssSyntaxKind::VI_KW => todo!(),
        CssSyntaxKind::SVI_KW => todo!(),
        CssSyntaxKind::LVI_KW => todo!(),
        CssSyntaxKind::DVI_KW => todo!(),
        CssSyntaxKind::VB_KW => todo!(),
        CssSyntaxKind::SVB_KW => todo!(),
        CssSyntaxKind::LVB_KW => todo!(),
        CssSyntaxKind::DVB_KW => todo!(),
        CssSyntaxKind::VMIN_KW => todo!(),
        CssSyntaxKind::SVMIN_KW => todo!(),
        CssSyntaxKind::LVMIN_KW => todo!(),
        CssSyntaxKind::DVMIN_KW => todo!(),
        CssSyntaxKind::VMAX_KW => todo!(),
        CssSyntaxKind::SVMAX_KW => todo!(),
        CssSyntaxKind::LVMAX_KW => todo!(),
        CssSyntaxKind::DVMAX_KW => todo!(),
        CssSyntaxKind::CM_KW => todo!(),
        CssSyntaxKind::MM_KW => todo!(),
        CssSyntaxKind::Q_KW => todo!(),
        CssSyntaxKind::IN_KW => todo!(),
        CssSyntaxKind::PC_KW => todo!(),
        CssSyntaxKind::PT_KW => todo!(),
        CssSyntaxKind::PX_KW => todo!(),
        CssSyntaxKind::MOZMM_KW => todo!(),
        CssSyntaxKind::RPX_KW => todo!(),
        CssSyntaxKind::CQW_KW => todo!(),
        CssSyntaxKind::CQH_KW => todo!(),
        CssSyntaxKind::CQI_KW => todo!(),
        CssSyntaxKind::CQB_KW => todo!(),
        CssSyntaxKind::CQMIN_KW => todo!(),
        CssSyntaxKind::CQMAX_KW => todo!(),
        CssSyntaxKind::DEG_KW => todo!(),
        CssSyntaxKind::GRAD_KW => todo!(),
        CssSyntaxKind::RAD_KW => todo!(),
        CssSyntaxKind::TURN_KW => todo!(),
        CssSyntaxKind::S_KW => todo!(),
        CssSyntaxKind::MS_KW => todo!(),
        CssSyntaxKind::HZ_KW => todo!(),
        CssSyntaxKind::KHZ_KW => todo!(),
        CssSyntaxKind::DPI_KW => todo!(),
        CssSyntaxKind::DPCM_KW => todo!(),
        CssSyntaxKind::DPPX_KW => todo!(),
        CssSyntaxKind::X_KW => todo!(),
        CssSyntaxKind::FR_KW => todo!(),
        CssSyntaxKind::PAGE_KW => todo!(),
        CssSyntaxKind::LEFT_KW => todo!(),
        CssSyntaxKind::RIGHT_KW => todo!(),
        CssSyntaxKind::FIRST_KW => todo!(),
        CssSyntaxKind::BLANK_KW => todo!(),
        CssSyntaxKind::TOP_LEFT_CORNER_KW => todo!(),
        CssSyntaxKind::TOP_LEFT_KW => todo!(),
        CssSyntaxKind::TOP_CENTER_KW => todo!(),
        CssSyntaxKind::TOP_RIGHT_KW => todo!(),
        CssSyntaxKind::TOP_RIGHT_CORNER_KW => todo!(),
        CssSyntaxKind::BOTTOM_LEFT_CORNER_KW => todo!(),
        CssSyntaxKind::BOTTOM_LEFT_KW => todo!(),
        CssSyntaxKind::BOTTOM_CENTER_KW => todo!(),
        CssSyntaxKind::BOTTOM_RIGHT_KW => todo!(),
        CssSyntaxKind::BOTTOM_RIGHT_CORNER_KW => todo!(),
        CssSyntaxKind::LEFT_TOP_KW => todo!(),
        CssSyntaxKind::LEFT_MIDDLE_KW => todo!(),
        CssSyntaxKind::LEFT_BOTTOM_KW => todo!(),
        CssSyntaxKind::RIGHT_TOP_KW => todo!(),
        CssSyntaxKind::RIGHT_MIDDLE_KW => todo!(),
        CssSyntaxKind::RIGHT_BOTTOM_KW => todo!(),
        CssSyntaxKind::LAYER_KW => todo!(),
        CssSyntaxKind::SCOPE_KW => todo!(),
        CssSyntaxKind::SUPPORTS_KW => todo!(),
        CssSyntaxKind::SELECTOR_KW => todo!(),
        CssSyntaxKind::IMPORT_KW => todo!(),
        CssSyntaxKind::NAMESPACE_KW => todo!(),
        CssSyntaxKind::STARTING_STYLE_KW => todo!(),
        CssSyntaxKind::DOCUMENT_KW => todo!(),
        CssSyntaxKind::URL_PREFIX_KW => todo!(),
        CssSyntaxKind::DOMAIN_KW => todo!(),
        CssSyntaxKind::MEDIA_DOCUMENT_KW => todo!(),
        CssSyntaxKind::REGEXP_KW => todo!(),
        CssSyntaxKind::FONT_FACE_KW => todo!(),
        CssSyntaxKind::CSS_STRING_LITERAL => todo!(),
        CssSyntaxKind::CSS_NUMBER_LITERAL => todo!(),
        CssSyntaxKind::CSS_DASHED_IDENTIFIER => todo!(),
        CssSyntaxKind::CSS_CUSTOM_IDENTIFIER => todo!(),
        CssSyntaxKind::CSS_URL_VALUE_RAW_LITERAL => todo!(),
        CssSyntaxKind::CSS_COLOR_LITERAL => todo!(),
        CssSyntaxKind::CSS_DIMENSION_VALUE => todo!(),
        CssSyntaxKind::CSS_PERCENTAGE_VALUE => todo!(),
        CssSyntaxKind::ERROR_TOKEN => todo!(),
        CssSyntaxKind::IDENT => todo!(),
        CssSyntaxKind::NEWLINE => todo!(),
        CssSyntaxKind::WHITESPACE => todo!(),
        CssSyntaxKind::COMMENT => todo!(),
        CssSyntaxKind::MULTILINE_COMMENT => todo!(),
        CssSyntaxKind::CSS_ROOT => todo!(),
        CssSyntaxKind::CSS_RULE_LIST => todo!(),
        CssSyntaxKind::CSS_QUALIFIED_RULE => todo!(),
        CssSyntaxKind::CSS_NESTED_QUALIFIED_RULE => todo!(),
        CssSyntaxKind::CSS_SELECTOR_LIST => todo!(),
        CssSyntaxKind::CSS_ANY_FUNCTION => todo!(),
        CssSyntaxKind::CSS_DECLARATION_LIST_BLOCK => todo!(),
        CssSyntaxKind::CSS_RULE_LIST_BLOCK => todo!(),
        CssSyntaxKind::CSS_DECLARATION_OR_AT_RULE_BLOCK => todo!(),
        CssSyntaxKind::CSS_DECLARATION_OR_RULE_BLOCK => todo!(),
        CssSyntaxKind::CSS_DECLARATION_OR_RULE_LIST => todo!(),
        CssSyntaxKind::CSS_DECLARATION_OR_AT_RULE_LIST => todo!(),
        CssSyntaxKind::CSS_DECLARATION_WITH_SEMICOLON => todo!(),
        CssSyntaxKind::CSS_DECLARATION => todo!(),
        CssSyntaxKind::CSS_IDENTIFIER => todo!(),
        CssSyntaxKind::CSS_NUMBER => todo!(),
        CssSyntaxKind::CSS_PARAMETER => todo!(),
        CssSyntaxKind::CSS_PERCENTAGE => todo!(),
        CssSyntaxKind::CSS_RATIO => todo!(),
        CssSyntaxKind::CSS_FUNCTION => todo!(),
        CssSyntaxKind::CSS_STRING => todo!(),
        CssSyntaxKind::CSS_VAR_FUNCTION => todo!(),
        CssSyntaxKind::CSS_VAR_FUNCTION_VALUE => todo!(),
        CssSyntaxKind::CSS_ATTRIBUTE_LIST => todo!(),
        CssSyntaxKind::CSS_DECLARATION_LIST => todo!(),
        CssSyntaxKind::CSS_COMPONENT_VALUE_LIST => todo!(),
        CssSyntaxKind::CSS_GENERIC_COMPONENT_VALUE_LIST => todo!(),
        CssSyntaxKind::CSS_GENERIC_DELIMITER => todo!(),
        CssSyntaxKind::CSS_GENERIC_PROPERTY => todo!(),
        CssSyntaxKind::CSS_UNKNOWN_PROPERTY_VALUE => todo!(),
        CssSyntaxKind::CSS_PARAMETER_LIST => todo!(),
        CssSyntaxKind::CSS_DECLARATION_IMPORTANT => todo!(),
        CssSyntaxKind::CSS_REGULAR_DIMENSION => todo!(),
        CssSyntaxKind::CSS_UNKNOWN_DIMENSION => todo!(),
        CssSyntaxKind::CSS_NAMESPACE => todo!(),
        CssSyntaxKind::CSS_NAMED_NAMESPACE_PREFIX => todo!(),
        CssSyntaxKind::CSS_UNIVERSAL_NAMESPACE_PREFIX => todo!(),
        CssSyntaxKind::CSS_ANY_SELECTOR_LIST => todo!(),
        CssSyntaxKind::CSS_COMPLEX_SELECTOR => todo!(),
        CssSyntaxKind::CSS_COMPOUND_SELECTOR => todo!(),
        CssSyntaxKind::CSS_SUB_SELECTOR_LIST => todo!(),
        CssSyntaxKind::CSS_ID_SELECTOR => todo!(),
        CssSyntaxKind::CSS_CLASS_SELECTOR => todo!(),
        CssSyntaxKind::CSS_TYPE_SELECTOR => todo!(),
        CssSyntaxKind::CSS_UNIVERSAL_SELECTOR => todo!(),
        CssSyntaxKind::CSS_PSEUDO_CLASS_SELECTOR => todo!(),
        CssSyntaxKind::CSS_PSEUDO_CLASS_SELECTOR_PARAMETERS => todo!(),
        CssSyntaxKind::CSS_PSEUDO_ELEMENT_SELECTOR => todo!(),
        CssSyntaxKind::CSS_PSEUDO_ELEMENT_IDENTIFIER => todo!(),
        CssSyntaxKind::CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR => todo!(),
        CssSyntaxKind::CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER => todo!(),
        CssSyntaxKind::CSS_PSEUDO_CLASS_IDENTIFIER => todo!(),
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER => todo!(),
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_SELECTOR => todo!(),
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST => todo!(),
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST => todo!(),
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR => todo!(),
        CssSyntaxKind::CSS_COMPOUND_SELECTOR_LIST => todo!(),
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST => todo!(),
        CssSyntaxKind::CSS_RELATIVE_SELECTOR_LIST => todo!(),
        CssSyntaxKind::CSS_RELATIVE_SELECTOR => todo!(),
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST => todo!(),
        CssSyntaxKind::CSS_PSEUDO_VALUE_LIST => todo!(),
        CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_NTH => todo!(),
        CssSyntaxKind::CSS_PSEUDO_CLASS_NTH_SELECTOR => todo!(),
        CssSyntaxKind::CSS_PSEUDO_CLASS_NTH => todo!(),
        CssSyntaxKind::CSS_PSEUDO_CLASS_NTH_NUMBER => todo!(),
        CssSyntaxKind::CSS_PSEUDO_CLASS_NTH_IDENTIFIER => todo!(),
        CssSyntaxKind::CSS_NTH_OFFSET => todo!(),
        CssSyntaxKind::CSS_PSEUDO_CLASS_OF_NTH_SELECTOR => todo!(),
        CssSyntaxKind::CSS_ATTRIBUTE_SELECTOR => todo!(),
        CssSyntaxKind::CSS_ATTRIBUTE => todo!(),
        CssSyntaxKind::CSS_ATTRIBUTE_NAME => todo!(),
        CssSyntaxKind::CSS_ATTRIBUTE_MATCHER => todo!(),
        CssSyntaxKind::CSS_ATTRIBUTE_MATCHER_VALUE => todo!(),
        CssSyntaxKind::CSS_PARENTHESIZED_EXPRESSION => todo!(),
        CssSyntaxKind::CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION => todo!(),
        CssSyntaxKind::CSS_BINARY_EXPRESSION => todo!(),
        CssSyntaxKind::CSS_URL_VALUE_RAW => todo!(),
        CssSyntaxKind::CSS_URL_FUNCTION => todo!(),
        CssSyntaxKind::CSS_URL_MODIFIER_LIST => todo!(),
        CssSyntaxKind::CSS_COLOR => todo!(),
        CssSyntaxKind::CSS_BORDER => todo!(),
        CssSyntaxKind::CSS_AT_RULE => todo!(),
        CssSyntaxKind::CSS_CHARSET_AT_RULE => todo!(),
        CssSyntaxKind::CSS_COLOR_PROFILE_AT_RULE => todo!(),
        CssSyntaxKind::CSS_COUNTER_STYLE_AT_RULE => todo!(),
        CssSyntaxKind::CSS_PROPERTY_AT_RULE => todo!(),
        CssSyntaxKind::CSS_CONTAINER_AT_RULE => todo!(),
        CssSyntaxKind::CSS_CONTAINER_NOT_QUERY => todo!(),
        CssSyntaxKind::CSS_CONTAINER_AND_QUERY => todo!(),
        CssSyntaxKind::CSS_CONTAINER_OR_QUERY => todo!(),
        CssSyntaxKind::CSS_CONTAINER_QUERY_IN_PARENS => todo!(),
        CssSyntaxKind::CSS_CONTAINER_STYLE_QUERY_IN_PARENS => todo!(),
        CssSyntaxKind::CSS_CONTAINER_SIZE_FEATURE_IN_PARENS => todo!(),
        CssSyntaxKind::CSS_CONTAINER_STYLE_NOT_QUERY => todo!(),
        CssSyntaxKind::CSS_CONTAINER_STYLE_AND_QUERY => todo!(),
        CssSyntaxKind::CSS_CONTAINER_STYLE_OR_QUERY => todo!(),
        CssSyntaxKind::CSS_CONTAINER_STYLE_IN_PARENS => todo!(),
        CssSyntaxKind::CSS_FONT_FACE_AT_RULE => todo!(),
        CssSyntaxKind::CSS_FONT_FEATURE_VALUES_AT_RULE => todo!(),
        CssSyntaxKind::CSS_FONT_FEATURE_VALUES_BLOCK => todo!(),
        CssSyntaxKind::CSS_FONT_FEATURE_VALUES_ITEM => todo!(),
        CssSyntaxKind::CSS_FONT_FEATURE_VALUES_ITEM_LIST => todo!(),
        CssSyntaxKind::CSS_FONT_FEATURE_VALUES_STYLISTIC => todo!(),
        CssSyntaxKind::CSS_FONT_FEATURE_VALUES_HISTORICAL_FORMS => todo!(),
        CssSyntaxKind::CSS_FONT_FEATURE_VALUES_STYLESET => todo!(),
        CssSyntaxKind::CSS_FONT_FEATURE_VALUES_CHARACTER_VARIANT => todo!(),
        CssSyntaxKind::CSS_FONT_FEATURE_VALUES_SWASH => todo!(),
        CssSyntaxKind::CSS_FONT_FEATURE_VALUES_ORNAMENTS => todo!(),
        CssSyntaxKind::CSS_FONT_FEATURE_VALUES_ANNOTATION => todo!(),
        CssSyntaxKind::CSS_FONT_PALETTE_VALUES_AT_RULE => todo!(),
        CssSyntaxKind::CSS_KEYFRAMES_AT_RULE => todo!(),
        CssSyntaxKind::CSS_KEYFRAMES_BODY => todo!(),
        CssSyntaxKind::CSS_MEDIA_AT_RULE => todo!(),
        CssSyntaxKind::CSS_MEDIA_QUERY_LIST => todo!(),
        CssSyntaxKind::CSS_MEDIA_QUERY => todo!(),
        CssSyntaxKind::CSS_MEDIA_CONDITION_QUERY => todo!(),
        CssSyntaxKind::CSS_MEDIA_TYPE_QUERY => todo!(),
        CssSyntaxKind::CSS_MEDIA_AND_TYPE_QUERY => todo!(),
        CssSyntaxKind::CSS_MEDIA_TYPE => todo!(),
        CssSyntaxKind::CSS_MEDIA_NOT_CONDITION => todo!(),
        CssSyntaxKind::CSS_MEDIA_AND_CONDITION => todo!(),
        CssSyntaxKind::CSS_MEDIA_OR_CONDITION => todo!(),
        CssSyntaxKind::CSS_MEDIA_CONDITION_IN_PARENS => todo!(),
        CssSyntaxKind::CSS_MEDIA_FEATURE_IN_PARENS => todo!(),
        CssSyntaxKind::CSS_QUERY_FEATURE_PLAIN => todo!(),
        CssSyntaxKind::CSS_QUERY_FEATURE_BOOLEAN => todo!(),
        CssSyntaxKind::CSS_QUERY_FEATURE_RANGE => todo!(),
        CssSyntaxKind::CSS_QUERY_FEATURE_REVERSE_RANGE => todo!(),
        CssSyntaxKind::CSS_QUERY_FEATURE_RANGE_INTERVAL => todo!(),
        CssSyntaxKind::CSS_QUERY_FEATURE_RANGE_COMPARISON => todo!(),
        CssSyntaxKind::CSS_KEYFRAMES_BLOCK => todo!(),
        CssSyntaxKind::CSS_KEYFRAMES_ITEM_LIST => todo!(),
        CssSyntaxKind::CSS_KEYFRAMES_ITEM => todo!(),
        CssSyntaxKind::CSS_KEYFRAMES_IDENT_SELECTOR => todo!(),
        CssSyntaxKind::CSS_KEYFRAMES_PERCENTAGE_SELECTOR => todo!(),
        CssSyntaxKind::CSS_KEYFRAMES_SELECTOR_LIST => todo!(),
        CssSyntaxKind::CSS_PAGE_AT_RULE => todo!(),
        CssSyntaxKind::CSS_PAGE_SELECTOR_LIST => todo!(),
        CssSyntaxKind::CSS_PAGE_SELECTOR => todo!(),
        CssSyntaxKind::CSS_PAGE_SELECTOR_PSEUDO_LIST => todo!(),
        CssSyntaxKind::CSS_PAGE_SELECTOR_PSEUDO => todo!(),
        CssSyntaxKind::CSS_PAGE_AT_RULE_BLOCK => todo!(),
        CssSyntaxKind::CSS_PAGE_AT_RULE_ITEM_LIST => todo!(),
        CssSyntaxKind::CSS_MARGIN_AT_RULE => todo!(),
        CssSyntaxKind::CSS_LAYER_AT_RULE => todo!(),
        CssSyntaxKind::CSS_LAYER_REFERENCE => todo!(),
        CssSyntaxKind::CSS_LAYER_REFERENCE_LIST => todo!(),
        CssSyntaxKind::CSS_LAYER_NAME_LIST => todo!(),
        CssSyntaxKind::CSS_LAYER_DECLARATION => todo!(),
        CssSyntaxKind::CSS_SUPPORTS_AT_RULE => todo!(),
        CssSyntaxKind::CSS_SUPPORTS_NOT_CONDITION => todo!(),
        CssSyntaxKind::CSS_SUPPORTS_AND_CONDITION => todo!(),
        CssSyntaxKind::CSS_SUPPORTS_OR_CONDITION => todo!(),
        CssSyntaxKind::CSS_SUPPORTS_CONDITION_IN_PARENS => todo!(),
        CssSyntaxKind::CSS_SUPPORTS_FEATURE_DECLARATION => todo!(),
        CssSyntaxKind::CSS_SUPPORTS_FEATURE_SELECTOR => todo!(),
        CssSyntaxKind::CSS_SCOPE_AT_RULE => todo!(),
        CssSyntaxKind::CSS_SCOPE_RANGE_START => todo!(),
        CssSyntaxKind::CSS_SCOPE_RANGE_END => todo!(),
        CssSyntaxKind::CSS_SCOPE_RANGE_INTERVAL => todo!(),
        CssSyntaxKind::CSS_SCOPE_EDGE => todo!(),
        CssSyntaxKind::CSS_IMPORT_AT_RULE => todo!(),
        CssSyntaxKind::CSS_IMPORT_ANONYMOUS_LAYER => todo!(),
        CssSyntaxKind::CSS_IMPORT_NAMED_LAYER => todo!(),
        CssSyntaxKind::CSS_IMPORT_SUPPORTS => todo!(),
        CssSyntaxKind::CSS_NAMESPACE_AT_RULE => todo!(),
        CssSyntaxKind::CSS_STARTING_STYLE_AT_RULE => todo!(),
        CssSyntaxKind::CSS_DOCUMENT_AT_RULE => todo!(),
        CssSyntaxKind::CSS_DOCUMENT_MATCHER_LIST => todo!(),
        CssSyntaxKind::CSS_DOCUMENT_CUSTOM_MATCHER => todo!(),
        CssSyntaxKind::CSS_BOGUS => todo!(),
        CssSyntaxKind::CSS_BOGUS_BLOCK => todo!(),
        CssSyntaxKind::CSS_BOGUS_KEYFRAMES_ITEM => todo!(),
        CssSyntaxKind::CSS_BOGUS_RULE => todo!(),
        CssSyntaxKind::CSS_BOGUS_SELECTOR => todo!(),
        CssSyntaxKind::CSS_BOGUS_SUB_SELECTOR => todo!(),
        CssSyntaxKind::CSS_BOGUS_PSEUDO_CLASS => todo!(),
        CssSyntaxKind::CSS_BOGUS_PSEUDO_ELEMENT => todo!(),
        CssSyntaxKind::CSS_BOGUS_AT_RULE => todo!(),
        CssSyntaxKind::CSS_BOGUS_LAYER => todo!(),
        CssSyntaxKind::CSS_BOGUS_PAGE_SELECTOR_PSEUDO => todo!(),
        CssSyntaxKind::CSS_BOGUS_DECLARATION_ITEM => todo!(),
        CssSyntaxKind::CSS_BOGUS_COMPONENT_VALUE => todo!(),
        CssSyntaxKind::CSS_BOGUS_PARAMETER => todo!(),
        CssSyntaxKind::CSS_BOGUS_PROPERTY => todo!(),
        CssSyntaxKind::CSS_BOGUS_PROPERTY_VALUE => todo!(),
        CssSyntaxKind::CSS_BOGUS_MEDIA_QUERY => todo!(),
        CssSyntaxKind::CSS_BOGUS_SCOPE_RANGE => todo!(),
        CssSyntaxKind::CSS_BOGUS_URL_MODIFIER => todo!(),
        CssSyntaxKind::CSS_BOGUS_DOCUMENT_MATCHER => todo!(),
        CssSyntaxKind::CSS_BOGUS_FONT_FEATURE_VALUES_ITEM => todo!(),
        _ => todo!(),
    }
}
