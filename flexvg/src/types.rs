#[cfg(feature = "serde")]
use convert_case::{Case, Casing};
#[cfg(feature = "serde")]
use lazy_static::lazy_static;
#[cfg(feature = "serde")]
use regex::Regex;
#[cfg(feature = "serde")]
use serde::{
    de::{value::StringDeserializer, DeserializeOwned, IntoDeserializer},
    Deserialize, Deserializer,
};
#[cfg(feature = "serde")]
use std::str::FromStr;
use taffy::node::Node as TaffyNode;
use taffy::prelude::*;

/*
8888888888 888                   888b    888               888
888        888                   8888b   888               888
888        888                   88888b  888               888
8888888    888  .d88b.  888  888 888Y88b 888  .d88b.   .d88888  .d88b.
888        888 d8P  Y8b `Y8bd8P' 888 Y88b888 d88""88b d88" 888 d8P  Y8b
888        888 88888888   X88K   888  Y88888 888  888 888  888 88888888
888        888 Y8b.     .d8""8b. 888   Y8888 Y88..88P Y88b 888 Y8b.
888        888  "Y8888  888  888 888    Y888  "Y88P"   "Y88888  "Y8888
 */

///
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(all(feature = "serde"), serde(rename_all = "kebab-case"))]
pub struct FlexNode {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub element: Option<FlexElement>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub layout: FlexLayout,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub root: Option<FlexRootData>,
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Vec::is_empty", default)
    )]
    pub nodes: Vec<FlexNode>,
}

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(all(feature = "serde"), serde(rename_all = "kebab-case"))]
pub struct FlexRootData {
    pub stylesheet: Option<String>,
}

/*
8888888888 888                    .d8888b.                                  888b    888               888
888        888                   d88P  Y88b                                 8888b   888               888
888        888                   888    888                                 88888b  888               888
8888888    888  .d88b.  888  888 888         .d88b.   .d88b.  88888b.d88b.  888Y88b 888  .d88b.   .d88888  .d88b.
888        888 d8P  Y8b `Y8bd8P' 888  88888 d8P  Y8b d88""88b 888 "888 "88b 888 Y88b888 d88""88b d88" 888 d8P  Y8b
888        888 88888888   X88K   888    888 88888888 888  888 888  888  888 888  Y88888 888  888 888  888 88888888
888        888 Y8b.     .d8""8b. Y88b  d88P Y8b.     Y88..88P 888  888  888 888   Y8888 Y88..88P Y88b 888 Y8b.
888        888  "Y8888  888  888  "Y8888P88  "Y8888   "Y88P"  888  888  888 888    Y888  "Y88P"   "Y88888  "Y8888



 */
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(all(feature = "serde"), serde(rename_all = "kebab-case"))]
pub struct FlexGeomNode {
    pub width: f32,
    pub height: f32,
    pub position: FlexPoint,
    pub element: Option<FlexElement>,
    pub root: Option<FlexRootData>,
    pub nodes: Vec<FlexGeomNode>,
}

/*
8888888888 888                   8888888888 888                                          888
888        888                   888        888                                          888
888        888                   888        888                                          888
8888888    888  .d88b.  888  888 8888888    888  .d88b.  88888b.d88b.   .d88b.  88888b.  888888
888        888 d8P  Y8b `Y8bd8P' 888        888 d8P  Y8b 888 "888 "88b d8P  Y8b 888 "88b 888
888        888 88888888   X88K   888        888 88888888 888  888  888 88888888 888  888 888
888        888 Y8b.     .d8""8b. 888        888 Y8b.     888  888  888 Y8b.     888  888 Y88b.
888        888  "Y8888  888  888 8888888888 888  "Y8888  888  888  888  "Y8888  888  888  "Y888

*/
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FlexElement {
    Text(FlexText),
    Rect(FlexRect),
}

/*
8888888888 888                          d8888 888    888            d8b 888
888        888                         d88888 888    888            Y8P 888
888        888                        d88P888 888    888                888
8888888    888  .d88b.  888  888     d88P 888 888888 888888 888d888 888 88888b.  .d8888b
888        888 d8P  Y8b `Y8bd8P'    d88P  888 888    888    888P"   888 888 "88b 88K
888        888 88888888   X88K     d88P   888 888    888    888     888 888  888 "Y8888b.
888        888 Y8b.     .d8""8b.  d8888888888 Y88b.  Y88b.  888     888 888 d88P      X88
888        888  "Y8888  888  888 d88P     888  "Y888  "Y888 888     888 88888P"   88888P'
 */

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct FlexGlobalAttribs {
    pub id: String,
    pub class: String,
    pub style: String,
}

/*
8888888888 888               88888888888                888
888        888                   888                    888
888        888                   888                    888
8888888    888  .d88b.  888  888 888   .d88b.  888  888 888888
888        888 d8P  Y8b `Y8bd8P' 888  d8P  Y8b `Y8bd8P' 888
888        888 88888888   X88K   888  88888888   X88K   888
888        888 Y8b.     .d8""8b. 888  Y8b.     .d8""8b. Y88b.
888        888  "Y8888  888  888 888   "Y8888  888  888  "Y888

 */

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FlexText {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub global: FlexGlobalAttribs,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub attribs: FlexTextAttribs,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde"), serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct FlexTextAttribs {}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde"), serde(rename_all = "kebab-case"))]
pub enum FlexSvgTextAnchor {
    Start,
    Middle,
    End,
}

impl Default for FlexSvgTextAnchor {
    fn default() -> Self {
        Self::Middle
    }
}

/*
8888888888 888                   8888888b.                   888
888        888                   888   Y88b                  888
888        888                   888    888                  888
8888888    888  .d88b.  888  888 888   d88P .d88b.   .d8888b 888888
888        888 d8P  Y8b `Y8bd8P' 8888888P" d8P  Y8b d88P"    888
888        888 88888888   X88K   888 T88b  88888888 888      888
888        888 Y8b.     .d8""8b. 888  T88b Y8b.     Y88b.    Y88b.
888        888  "Y8888  888  888 888   T88b "Y8888   "Y8888P  "Y888



 */
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FlexRect {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub global: FlexGlobalAttribs,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub attribs: FlexRectAttribs,
    pub rect: bool,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct FlexRectAttribs {
    pub rx: FlexDimension,
    pub ry: FlexDimension,
}

/*
8888888888 888                   888                                          888
888        888                   888                                          888
888        888                   888                                          888
8888888    888  .d88b.  888  888 888       8888b.  888  888  .d88b.  888  888 888888
888        888 d8P  Y8b `Y8bd8P' 888          "88b 888  888 d88""88b 888  888 888
888        888 88888888   X88K   888      .d888888 888  888 888  888 888  888 888
888        888 Y8b.     .d8""8b. 888      888  888 Y88b 888 Y88..88P Y88b 888 Y88b.
888        888  "Y8888  888  888 88888888 "Y888888  "Y88888  "Y88P"   "Y88888  "Y888
                                                        888
                                                   Y8b d88P
                                                    "Y88P"
 */

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(all(feature = "serde"), serde(rename_all = "kebab-case"))]
pub struct FlexLayout {
    /*
    // @TODO(jshrake): https://github.com/DioxusLabs/taffy/pull/153
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_simple_enum_as_pascal")
    )]
    */
    pub display: Display,
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_simple_enum_as_pascal")
    )]
    pub position_type: PositionType,
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_simple_enum_as_pascal")
    )]
    pub flex_direction: FlexDirection,
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_simple_enum_as_pascal")
    )]
    pub flex_wrap: FlexWrap,
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_simple_enum_as_pascal")
    )]
    pub align_items: AlignItems,
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_simple_enum_as_pascal")
    )]
    pub align_self: AlignSelf,
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_simple_enum_as_pascal")
    )]
    pub align_content: AlignContent,
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_simple_enum_as_pascal")
    )]
    pub justify_content: JustifyContent,
    pub position: Rect<Dimension>,
    pub margin: Rect<Dimension>,
    pub padding: Rect<Dimension>,
    pub border: Rect<Dimension>,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Dimension,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub size: FlexSize<FlexDimension>,
    pub min_size: Size<Dimension>,
    pub max_size: Size<Dimension>,
    pub aspect_ratio: Number,
}

impl Default for FlexLayout {
    fn default() -> Self {
        Self {
            display: Default::default(),
            position_type: Default::default(),
            flex_direction: Default::default(),
            flex_wrap: Default::default(),
            align_items: Default::default(),
            align_self: Default::default(),
            align_content: Default::default(),
            justify_content: Default::default(),
            position: Default::default(),
            margin: Default::default(),
            padding: Default::default(),
            border: Default::default(),
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Dimension::Auto,
            size: Default::default(),
            min_size: Default::default(),
            max_size: Default::default(),
            aspect_ratio: Default::default(),
        }
    }
}

impl From<FlexLayout> for Style {
    fn from(s: FlexLayout) -> Style {
        Style {
            display: s.display,
            position_type: s.position_type,
            flex_direction: s.flex_direction,
            flex_wrap: s.flex_wrap,
            align_items: s.align_items,
            align_self: s.align_self,
            align_content: s.align_content,
            justify_content: s.justify_content,
            position: s.position,
            margin: s.margin,
            padding: s.padding,
            border: s.border,
            flex_grow: s.flex_grow,
            flex_shrink: s.flex_shrink,
            flex_basis: s.flex_basis,
            size: s.size.into(),
            min_size: s.min_size,
            max_size: s.max_size,
            aspect_ratio: s.aspect_ratio,
        }
    }
}

// @NOTE(jshrake):
// - https://github.com/DioxusLabs/taffy/pull/84
#[cfg(feature = "serde")]
pub fn deserialize_simple_enum_as_pascal<'de, T, D>(
    deserializer: D,
) -> Result<T, D::Error>
where
    T: DeserializeOwned,
    D: Deserializer<'de>,
{
    let val = String::deserialize(deserializer)?;
    let pascal = val.to_case(Case::Pascal);
    let deserializer: StringDeserializer<D::Error> = pascal.into_deserializer();
    T::deserialize(deserializer).map_err(serde::de::Error::custom)
}

/*
8888888888 888                    .d8888b.  d8b
888        888                   d88P  Y88b Y8P
888        888                   Y88b.
8888888    888  .d88b.  888  888  "Y888b.   888 88888888  .d88b.
888        888 d8P  Y8b `Y8bd8P'     "Y88b. 888    d88P  d8P  Y8b
888        888 88888888   X88K         "888 888   d88P   88888888
888        888 Y8b.     .d8""8b. Y88b  d88P 888  d88P    Y8b.
888        888  "Y8888  888  888  "Y8888P"  888 88888888  "Y8888
*/

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct FlexSize<T> {
    pub width: T,
    pub height: T,
}

impl Default for FlexSize<f32> {
    fn default() -> Self {
        FlexSize { width: 0.0, height: 0.0 }
    }
}

impl Default for FlexSize<FlexDimension> {
    fn default() -> Self {
        FlexSize {
            width: FlexDimension::default(),
            height: FlexDimension::default(),
        }
    }
}

impl<T, U: std::convert::From<T>> From<FlexSize<T>> for Size<U> {
    fn from(s: FlexSize<T>) -> Size<U> {
        Size { width: s.width.into(), height: s.height.into() }
    }
}

/*
8888888888 888                   8888888b.  d8b                                          d8b
888        888                   888  "Y88b Y8P                                          Y8P
888        888                   888    888
8888888    888  .d88b.  888  888 888    888 888 88888b.d88b.   .d88b.  88888b.  .d8888b  888  .d88b.  88888b.
888        888 d8P  Y8b `Y8bd8P' 888    888 888 888 "888 "88b d8P  Y8b 888 "88b 88K      888 d88""88b 888 "88b
888        888 88888888   X88K   888    888 888 888  888  888 88888888 888  888 "Y8888b. 888 888  888 888  888
888        888 Y8b.     .d8""8b. 888  .d88P 888 888  888  888 Y8b.     888  888      X88 888 Y88..88P 888  888
888        888  "Y8888  888  888 8888888P"  888 888  888  888  "Y8888  888  888  88888P' 888  "Y88P"  888  888
 */

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FlexDimension {
    Auto,
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_percent")
    )]
    Percent(f32),
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_point")
    )]
    Points(f32),
}

impl Default for FlexDimension {
    fn default() -> Self {
        FlexDimension::Percent(1.0)
        //FlexDimension::Auto
    }
}

impl From<FlexDimension> for Dimension {
    fn from(s: FlexDimension) -> Dimension {
        match s {
            FlexDimension::Auto => Dimension::Auto,
            FlexDimension::Percent(value) => Dimension::Percent(value),
            FlexDimension::Points(value) => Dimension::Points(value),
        }
    }
}

#[cfg(feature = "serde")]
lazy_static! {
    static ref RE_F32: Regex = Regex::new(r"(\d[-+]?[0-9]*\.?[0-9]+)").unwrap();
}

#[cfg(feature = "serde")]
fn deserialize_percent<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    // @TODO(jshrake): String allocation
    // https://users.rust-lang.org/t/serde-deserialize-string-field-in-json-to-a-different-type/12942/13
    let s = String::deserialize(deserializer)?;
    if !s.ends_with('%') {
        return Err(serde::de::Error::custom("expecting %"));
    }
    let cap = RE_F32
        .captures(&s)
        .and_then(|c| c.get(1))
        .ok_or_else(|| serde::de::Error::custom("invalid percent"))?;
    f32::from_str(cap.as_str())
        .map(|value| (value / 100.0))
        .map_err(serde::de::Error::custom)
}

#[cfg(feature = "serde")]
fn deserialize_point<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    // @TODO(jshrake): String allocation
    // https://users.rust-lang.org/t/serde-deserialize-string-field-in-json-to-a-different-type/12942/13
    let s = String::deserialize(deserializer)?;
    if !s.ends_with("pt") {
        return Err(serde::de::Error::custom("expecting pt"));
    }
    let cap = RE_F32
        .captures(&s)
        .and_then(|c| c.get(1))
        .ok_or_else(|| serde::de::Error::custom("invalid point"))?;
    f32::from_str(cap.as_str()).map_err(serde::de::Error::custom)
}

/*
8888888888 888                   8888888b.          d8b          888
888        888                   888   Y88b         Y8P          888
888        888                   888    888                      888
8888888    888  .d88b.  888  888 888   d88P .d88b.  888 88888b.  888888
888        888 d8P  Y8b `Y8bd8P' 8888888P" d88""88b 888 888 "88b 888
888        888 88888888   X88K   888       888  888 888 888  888 888
888        888 Y8b.     .d8""8b. 888       Y88..88P 888 888  888 Y88b.
888        888  "Y8888  888  888 888        "Y88P"  888 888  888  "Y888

 */

#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct FlexPoint {
    pub x: f32,
    pub y: f32,
}

/*
888b     d888 d8b
8888b   d8888 Y8P
88888b.d88888
888Y88888P888 888 .d8888b   .d8888b
888 Y888P 888 888 88K      d88P"
888  Y8P  888 888 "Y8888b. 888
888   "   888 888      X88 Y88b.
888       888 888  88888P'  "Y8888P
 */

#[derive(Debug, Clone)]
pub(crate) struct FlexTaffyNode {
    pub taffy_node: TaffyNode,
    pub element: Option<FlexElement>,
    pub root: Option<FlexRootData>,
    pub nodes: Vec<FlexTaffyNode>,
}
