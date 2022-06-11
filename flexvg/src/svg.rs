use crate::prelude::*;
use std::{fs::File, io::Read, path::Path};

#[inline]
fn push_line(s: &mut String, line: &str) {
    *s += line;
    *s += "\n";
}

pub(crate) fn compute_svg_string(
    node: FlexGeomNode,
    base_path: Option<&Path>,
) -> Result<String, Error> {
    let mut svg = String::new();
    let header = format!(
        r#"<svg version="1.0" width="{}" height="{}" xmlns="http://www.w3.org/2000/svg"
        xmlns:xlink="http://www.w3.org/1999/xlink">
        "#,
        node.width, node.height
    );
    let footer = r#"</svg>"#;
    push_line(&mut svg, &header);
    if let Some(root) = &node.root {
        if let Some(stylesheet) = &root.stylesheet {
            let stylesheet_path = if let Some(base_path) = base_path {
                base_path.join(stylesheet)
            } else {
                Path::new(stylesheet).to_path_buf()
            };
            let contents = if stylesheet_path.exists() {
                let mut contents = String::new();
                File::open(stylesheet_path)?.read_to_string(&mut contents)?;
                contents
            } else {
                stylesheet.clone()
            };
            push_line(&mut svg, "<style>");
            push_line(&mut svg, &contents);
            push_line(&mut svg, "</style>");
        }
    }
    compute_svg_string_recursive(node, &mut svg, 0.0, 0.0)?;
    push_line(&mut svg, footer);
    Ok(svg)
}

fn compute_svg_string_recursive(
    node: FlexGeomNode,
    svg: &mut String,
    x: f32,
    y: f32,
) -> Result<(), Error> {
    let has_elements = node.nodes.iter().any(|n| n.element.is_some());
    if has_elements {
        push_line(svg, "<g>");
    }
    if let Some(element) = node.element {
        let element = match element {
            FlexElement::Text(el) => {
                let x = x + 0.5 * node.width + node.position.x;
                let y = y + 0.5 * node.height + node.position.y;
                let id = el.global.id;
                let class = el.global.class;
                let style = el.global.style;
                let value = el.text;
                format!(
                    r#"<text id="{id}" class="{class}" x="{x}" y="{y}" style="{style}">{value}</text>"#
                )
            }
            FlexElement::Rect(el) => {
                let x = x + node.position.x;
                let y = y + node.position.y;
                let w = node.width;
                let h = node.height;
                let id = el.global.id;
                let class = el.global.class;
                let style = el.global.style;
                format!(
                    r#"<rect id="{id}" class="{class}" x="{x}" y="{y}" width="{w}" height="{h}" style="{style}"></rect>"#
                )
            }
        };
        push_line(svg, &element);
    }
    let mut children = node.nodes;
    // @NOTE(jshrake): Reverse the children so that the
    // first nodes specified in a list draw on top
    children.reverse();
    for child in children {
        compute_svg_string_recursive(
            child,
            svg,
            x + node.position.x,
            y + node.position.y,
        )?;
    }
    if has_elements {
        push_line(svg, "</g>");
    }
    Ok(())
}
