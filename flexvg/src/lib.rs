#[cfg_attr(feature = "serde", macro_use)]
#[cfg(feature = "serde")]
extern crate serde;

pub mod error;
pub mod prelude;
pub mod svg;
pub mod types;

use std::path::Path;

use log::debug;
use prelude::*;
use taffy::prelude::Number as TaffyNumber;
use taffy::prelude::Size as TaffySize;
use taffy::prelude::Style as TaffyStyle;
use taffy::Taffy;

pub fn compute_svg_string(
    root: FlexNode,
    base_path: Option<&Path>,
) -> Result<String, Error> {
    let layout_root = compute_layout_root(root)?;
    crate::svg::compute_svg_string(layout_root, base_path)
}

fn compute_layout_root(node: FlexNode) -> Result<FlexGeomNode, Error> {
    let size = TaffySize {
        height: TaffyNumber::Defined(100.0),
        width: TaffyNumber::Defined(100.0),
    };
    let mut taffy = Taffy::new();
    let flex_taffy_root = compute_flex_taffy_recursive(node, &mut taffy)?;
    taffy.compute_layout(flex_taffy_root.taffy_node, size)?;
    let root = compute_layout_root_recursive(flex_taffy_root, &mut taffy)?;
    Ok(root)
}

fn compute_layout_root_recursive(
    parent: FlexTaffyNode,
    taffy: &mut Taffy,
) -> Result<FlexGeomNode, Error> {
    let mut children_nodes = vec![];
    for child in parent.nodes {
        let layout_node = compute_layout_root_recursive(child, taffy)?;
        children_nodes.push(layout_node);
    }
    let taffy_layout = taffy.layout(parent.taffy_node)?;

    Ok(FlexGeomNode {
        element: parent.element,
        position: FlexPoint {
            x: taffy_layout.location.x,
            y: taffy_layout.location.y,
        },
        width: taffy_layout.size.width,
        height: taffy_layout.size.height,
        root: parent.root,
        nodes: children_nodes,
    })
}

fn compute_flex_taffy_recursive(
    parent: FlexNode,
    taffy: &mut Taffy,
) -> Result<FlexTaffyNode, Error> {
    let child_len = parent.nodes.len();
    let mut flex_taffy_children = Vec::with_capacity(child_len);
    let mut taffy_children = Vec::with_capacity(child_len);

    for child in parent.nodes {
        let child = compute_flex_taffy_recursive(child, taffy)?;
        taffy_children.push(child.taffy_node);
        flex_taffy_children.push(child);
    }
    let taffy_node =
        taffy.new_node(TaffyStyle::from(parent.layout), &taffy_children)?;
    debug!("[taffy-new-node] {:?}, {:?}", &taffy_node, taffy_children);
    Ok(FlexTaffyNode {
        taffy_node,
        element: parent.element,
        root: parent.root,
        nodes: flex_taffy_children,
    })
}
