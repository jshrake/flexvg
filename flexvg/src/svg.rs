use crate::prelude::*;
use std::fmt::Write;
use std::{fs::File, io::Read, path::Path};

pub(crate) fn compute_svg_string(
    node: FlexGeomNode,
    base_path: Option<&Path>,
) -> Result<String, Error> {
    let mut svg = String::with_capacity(8096);
    writeln!(
        svg,
        r#"<svg version="1.0" width="{}" height="{}" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">"#,
        node.width, node.height
    ).unwrap();
    if let Some(root) = &node.root {
        if let Some(stylesheet) = &root.stylesheet {
            let stylesheet_path = if let Some(base_path) = base_path {
                base_path.join(stylesheet)
            } else {
                Path::new(stylesheet).to_path_buf()
            };
            writeln!(svg, "<style>").unwrap();
            if stylesheet_path.exists() {
                let mut contents = String::new();
                File::open(stylesheet_path)?.read_to_string(&mut contents)?;
                writeln!(svg, "{contents}").unwrap();
            } else {
                writeln!(svg, "{stylesheet}").unwrap();
            };
            writeln!(svg, "</style>").unwrap();
        }
    }
    compute_svg_string_recursive(node, &mut svg, 0.0, 0.0)?;
    writeln!(svg, r#"</svg>"#).unwrap();
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
        writeln!(svg, "<g>").unwrap();
    }
    if let Some(element) = node.element {
        match element {
            FlexElement::Text(el) => {
                let x = x + 0.5 * node.width + node.position.x;
                let y = y + 0.5 * node.height + node.position.y;
                let id = el.global.id;
                let class = el.global.class;
                let style = el.global.style;
                let value = el.text;
                writeln!(
                    svg,
                    r#"<text id="{id}" class="{class}" x="{x}" y="{y}" style="{style}">{value}</text>"#).unwrap();
            }
            FlexElement::Rect(el) => {
                let x = x + node.position.x;
                let y = y + node.position.y;
                let w = node.width;
                let h = node.height;
                let id = el.global.id;
                let class = el.global.class;
                let style = el.global.style;
                writeln!(svg,
                    r#"<rect id="{id}" class="{class}" x="{x}" y="{y}" width="{w}" height="{h}" style="{style}"></rect>"#).unwrap();
            }
        }
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
        writeln!(svg, "</g>").unwrap();
    }
    Ok(())
}
