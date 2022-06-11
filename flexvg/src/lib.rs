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
use taffy::prelude::Number as SprawlNumber;
use taffy::prelude::Size as SprawlSize;
use taffy::prelude::Style as SprawlStyle;
use taffy::Taffy;

pub fn compute_svg_string(
    root: FlexNode,
    base_path: Option<&Path>,
) -> Result<String, Error> {
    debug!("root: {:#?}", root);
    let layout_root = compute_layout_root(root)?;
    crate::svg::compute_svg_string(layout_root, base_path)
}

fn compute_layout_root(node: FlexNode) -> Result<FlexGeomNode, Error> {
    let size = SprawlSize {
        height: SprawlNumber::Defined(100.0),
        width: SprawlNumber::Defined(100.0),
    };
    let mut taffy = Taffy::new();
    let stacked_sprawl_root =
        compute_stacked_sprawl_recursive(node, &mut taffy)?;
    taffy.compute_layout(stacked_sprawl_root.taffy_node, size)?;
    let root = compute_layout_root_recursive(stacked_sprawl_root, &mut taffy)?;
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
    let sprawl_layout = taffy.layout(parent.taffy_node)?;

    Ok(FlexGeomNode {
        element: parent.element,
        position: FlexPoint {
            x: sprawl_layout.location.x,
            y: sprawl_layout.location.y,
        },
        width: sprawl_layout.size.width,
        height: sprawl_layout.size.height,
        root: parent.root,
        nodes: children_nodes,
    })
}

fn compute_stacked_sprawl_recursive(
    parent: FlexNode,
    taffy: &mut Taffy,
) -> Result<FlexTaffyNode, Error> {
    let child_len = parent.nodes.len();
    let mut stacked_sprawl_children = Vec::with_capacity(child_len);
    let mut sprawl_children = Vec::with_capacity(child_len);

    for child in parent.nodes {
        let child = compute_stacked_sprawl_recursive(child, taffy)?;
        sprawl_children.push(child.taffy_node);
        stacked_sprawl_children.push(child);
    }
    let sprawl_node =
        taffy.new_node(SprawlStyle::from(parent.layout), &sprawl_children)?;
    debug!("{:?}, {:?}", &sprawl_node, sprawl_children);
    Ok(FlexTaffyNode {
        taffy_node: sprawl_node,
        element: parent.element,
        root: parent.root,
        nodes: stacked_sprawl_children,
    })
}

#[cfg(test)]
mod tests {

    use std::sync::Once;

    //use crate::{compute_layout_root, compute_svg_string, StackedConfigNode};

    static INIT: Once = Once::new();

    pub fn initialize() {
        INIT.call_once(|| {
            env_logger::init();
        });
    }

    #[test]
    fn simple_diagram_0() {
        initialize();
        /*
               let b = include_bytes!("../examples/hello-world.yaml");
               let deserialized_point: StackedConfigNode =
                   serde_yaml::from_slice(b).expect("deserialize from yaml");
               let node = compute_layout_root(deserialized_point).expect("computed");
               let svg = svg_string(&node).expect("svg");
               std::fs::write("tmp.svg", svg).expect("Unable to write file");
        */
    }
}
