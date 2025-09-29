use crate::nodes::HtmlNode;
use ego_tree::NodeRef;
use scraper::{ElementRef, Html, node::Node};

/// Check if string looks like HTML
pub fn is_html(content: &str) -> bool {
    content.contains('<') && content.contains('>')
}

/// Parse HTML string into our HtmlNode tree
pub fn parse_html_to_nodes(html: &str) -> Result<HtmlNode, Box<dyn std::error::Error>> {
    let document = Html::parse_document(html);

    // Create a root wrapper node
    let root = HtmlNode {
        tag: "document".to_string(),
        attributes: vec![],
        children: document
            .tree
            .root()
            .children()
            .filter_map(|child| convert_node(child))
            .collect(),
        text: None,
    };

    Ok(root)
}

/// Convert a NodeRef<'a, Node> into our HtmlNode
fn convert_node(node_ref: NodeRef<'_, Node>) -> Option<HtmlNode> {
    let node = node_ref.value();

    match node {
        Node::Element(element) => {
            // Wrap it as an ElementRef to get attribute info
            let el_ref = ElementRef::wrap(node_ref).expect("should be element");

            let attrs: Vec<(String, String)> = el_ref
                .value()
                .attrs()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();

            let children = node_ref
                .children()
                .filter_map(|child_ref| convert_node(child_ref))
                .collect();

            Some(HtmlNode {
                tag: element.name().to_string(),
                attributes: attrs,
                children,
                text: None,
            })
        }
        Node::Text(text) => {
            let txt = text.text.trim();
            if !txt.is_empty() {
                Some(HtmlNode {
                    tag: "#text".to_string(),
                    attributes: vec![],
                    children: vec![],
                    text: Some(txt.to_string()),
                })
            } else {
                None
            }
        }
        _ => None,
    }
}
