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
    if let Some(el) = node.element {
        let mut x = x + node.position.x;
        let mut y = y + node.position.y;
        if let Some(origin) = el.origin {
            x += origin.0 * node.width;
            y += origin.1 * node.height;
        }
        let w = node.width;
        let h = node.height;
        let element = el.element;
        let value = el.value.as_deref().unwrap_or("");
        write!(svg, r#"<{element} x="{x}" y="{y}"  width="{w}" height="{h}" "#)
            .unwrap();
        for (attrib_name, attrib_value) in el.attributes.iter() {
            write!(svg, r#"{attrib_name}="{attrib_value}" "#).unwrap();
        }
        write!(svg, r#">"#).unwrap();
        write!(svg, r#"{value}"#).unwrap();
        writeln!(svg, r#"</{element}>"#).unwrap();
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
