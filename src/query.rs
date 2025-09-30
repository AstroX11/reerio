use crate::nodes::HtmlNode;

/// Search criteria for finding HTML nodes
#[derive(Debug, Clone)]
pub enum SearchCriteria {
    Tag(String),
    Attribute(String, String),
    Text(String),
    AttributeKey(String),
}

pub fn find<'a>(html_buffer: &'a HtmlNode, value: &SearchCriteria) -> Vec<&'a HtmlNode> {
    let mut results = Vec::new();
    find_recursive(html_buffer, value, &mut results);
    results
}

fn find_recursive<'a>(
    node: &'a HtmlNode,
    criteria: &SearchCriteria,
    results: &mut Vec<&'a HtmlNode>,
) {
    if matches_criteria(node, criteria) {
        results.push(node);
    }

    for child in &node.children {
        find_recursive(child, criteria, results);
    }
}

fn matches_criteria(node: &HtmlNode, criteria: &SearchCriteria) -> bool {
    match criteria {
        SearchCriteria::Tag(tag_name) => node.tag.eq_ignore_ascii_case(tag_name),
        SearchCriteria::Attribute(key, value) => {
            node.attributes.iter().any(|(k, v)| {
                if k == key {
                    if k == "class" {
                        // Special handling for class attribute: split by spaces
                        v.split_whitespace().any(|cls| cls == value)
                    } else {
                        v == value
                    }
                } else {
                    false
                }
            })
        }
        SearchCriteria::AttributeKey(key) => node.attributes.iter().any(|(k, _)| k == key),
        SearchCriteria::Text(text) => {
            if let Some(node_text) = &node.text {
                node_text.contains(text)
            } else {
                false
            }
        }
    }
}

pub fn find_by_tag<'a>(html_buffer: &'a HtmlNode, tag_name: &str) -> Vec<&'a HtmlNode> {
    find(html_buffer, &SearchCriteria::Tag(tag_name.to_string()))
}

pub fn find_by_attribute<'a>(
    html_buffer: &'a HtmlNode,
    key: &str,
    value: &str,
) -> Vec<&'a HtmlNode> {
    find(
        html_buffer,
        &SearchCriteria::Attribute(key.to_string(), value.to_string()),
    )
}

pub fn find_by_text<'a>(html_buffer: &'a HtmlNode, text: &str) -> Vec<&'a HtmlNode> {
    find(html_buffer, &SearchCriteria::Text(text.to_string()))
}
