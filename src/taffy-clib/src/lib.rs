use std::f32;
use std::os::raw::*;
use taffy::geometry::*;
use taffy::node::*;
use taffy::style::*;
use taffy::layout::*;
use taffy::prelude::LayoutTree;

#[repr(C)]
pub struct TaffySize {
    width: f32,
    height: f32,
}

#[repr(C)]
pub struct TaffyStyleDimension {
    dimen_type: i32,
    dimen_value: f32,
}

impl Into<Dimension> for TaffyStyleDimension {
    fn into(self) -> Dimension {
        match self.dimen_type {
            0 => Dimension::Points(self.dimen_value),
            1 => Dimension::Percent(self.dimen_value),
            2 => Dimension::Auto,
            _ => panic!(),
        }
    }
}

impl Into<LengthPercentageAuto> for TaffyStyleDimension {
    fn into(self) -> LengthPercentageAuto {
        match self.dimen_type {
            0 => LengthPercentageAuto::Points(self.dimen_value),
            1 => LengthPercentageAuto::Percent(self.dimen_value),
            2 => LengthPercentageAuto::Auto,
            _ => panic!(),
        }
    }
}

impl Into<LengthPercentage> for TaffyStyleDimension {
    fn into(self) -> LengthPercentage {
        match self.dimen_type {
            0 => LengthPercentage::Points(self.dimen_value),
            1 => LengthPercentage::Percent(self.dimen_value),
            _ => panic!(),
        }
    }
}

#[repr(C)]
pub struct TaffyStyleRect {
    left: TaffyStyleDimension,
    right: TaffyStyleDimension,
    top: TaffyStyleDimension,
    bottom: TaffyStyleDimension,
}

#[repr(C)]
pub struct TaffyStyleSize {
    width: TaffyStyleDimension,
    height: TaffyStyleDimension,
}

#[no_mangle]
pub unsafe extern "C" fn taffy_style_create(
    display: i32,
    position_type: i32,
    flex_direction: i32,
    flex_wrap: i32,
    align_items: i32,
    align_self: i32,
    align_content: i32,
    justify_content: i32,

    position: TaffyStyleRect,
    margin: TaffyStyleRect,
    padding: TaffyStyleRect,
    border: TaffyStyleRect,
    gap: TaffyStyleSize,

    flex_grow: f32,
    flex_shrink: f32,

    flex_basis: TaffyStyleDimension,

    size: TaffyStyleSize,
    min_size: TaffyStyleSize,
    max_size: TaffyStyleSize,

    aspect_ratio: f32,
) -> *mut c_void {
    Box::into_raw(Box::new(Style {
        display: match display {
            0 => Display::Flex,
            1 => Display::None,
            _ => panic!(),
        },

        position_type: match position_type {
            0 => PositionType::Relative,
            1 => PositionType::Absolute,
            _ => panic!(),
        },

        flex_direction: match flex_direction {
            0 => FlexDirection::Row,
            1 => FlexDirection::Column,
            2 => FlexDirection::RowReverse,
            3 => FlexDirection::ColumnReverse,
            _ => panic!(),
        },

        flex_wrap: match flex_wrap {
            0 => FlexWrap::NoWrap,
            1 => FlexWrap::Wrap,
            2 => FlexWrap::WrapReverse,
            _ => panic!(),
        },

        align_items: match align_items {
            0 => AlignItems::FlexStart,
            1 => AlignItems::FlexEnd,
            2 => AlignItems::Center,
            3 => AlignItems::Baseline,
            4 => AlignItems::Stretch,
            _ => panic!(),
        },

        align_self: match align_self {
            0 => AlignSelf::Auto,
            1 => AlignSelf::FlexStart,
            2 => AlignSelf::FlexEnd,
            3 => AlignSelf::Center,
            4 => AlignSelf::Baseline,
            5 => AlignSelf::Stretch,
            _ => panic!(),
        },

        align_content: match align_content {
            0 => AlignContent::FlexStart,
            1 => AlignContent::FlexEnd,
            2 => AlignContent::Center,
            3 => AlignContent::Stretch,
            4 => AlignContent::SpaceBetween,
            5 => AlignContent::SpaceAround,
            _ => panic!(),
        },

        justify_content: match justify_content {
            0 => JustifyContent::FlexStart,
            1 => JustifyContent::FlexEnd,
            2 => JustifyContent::Center,
            3 => JustifyContent::SpaceBetween,
            4 => JustifyContent::SpaceAround,
            5 => JustifyContent::SpaceEvenly,
            _ => panic!(),
        },

        position: Rect {
            left: position.left.into(),
            right: position.right.into(),
            top: position.top.into(),
            bottom: position.bottom.into(),
        },

        margin: Rect {
            left: margin.left.into(),
            right: margin.right.into(),
            top: margin.top.into(),
            bottom: margin.bottom.into(),
        },

        padding: Rect {
            left: padding.left.into(),
            right: padding.right.into(),
            top: padding.top.into(),
            bottom: padding.bottom.into(),
        },

        border: Rect {
            left: border.left.into(),
            right: border.right.into(),
            top: border.top.into(),
            bottom: border.bottom.into(),
        },

        gap: Size {
            width: gap.width.into(),
            height: gap.height.into(),
        },


        flex_grow,
        flex_shrink,

        flex_basis: flex_basis.into(),

        size: Size { width: size.width.into(), height: size.height.into() },
        min_size: Size { width: min_size.width.into(), height: min_size.height.into() },
        max_size: Size { width: max_size.width.into(), height: max_size.height.into() },

        aspect_ratio: if f32::is_nan(aspect_ratio) { None } else { Some(aspect_ratio) },
    })) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn taffy_style_free(style: *mut c_void) {
    let _style = Box::from_raw(style as *mut Style);
}

#[no_mangle]
pub unsafe extern "C" fn taffy_init() -> *mut c_void {
    let stretch = taffy::node::Taffy::new();
    Box::into_raw(Box::new(stretch)) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn taffy_free(stretch: *mut c_void) {
    let _stretch = Box::from_raw(stretch as *mut Taffy);
}

#[no_mangle]
pub unsafe extern "C" fn taffy_node_create(stretch: *mut c_void, style: *mut c_void) -> *mut c_void {
    let mut stretch = Box::from_raw(stretch as *mut Taffy);
    let style = Box::from_raw(style as *mut Style);
    let node = stretch.new_with_children(*style, &[]).unwrap();

    Box::leak(style);
    Box::leak(stretch);

    Box::into_raw(Box::new(node)) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn taffy_node_free(stretch: *mut c_void, node: *mut c_void) {
    let mut stretch = Box::from_raw(stretch as *mut Taffy);
    let node = Box::from_raw(node as *mut Node);

    match stretch.remove(*node) {
        Ok(_) => (),
        Err(error) => {
            panic!("{:?}", error)
        },
    };

    Box::leak(stretch);
}

#[no_mangle]
pub unsafe extern "C" fn taffy_node_set_measure(
    stretch: *mut c_void,
    node: *mut c_void,
    swift_ptr: *mut c_void,
    measure: unsafe extern "C" fn(*const c_void, f32, f32) -> TaffySize,
) {
    let mut stretch = Box::from_raw(stretch as *mut Taffy);
    let node = Box::from_raw(node as *mut Node);
    stretch
        .set_measure(
            *node,
            Some(taffy::node::MeasureFunc::BoxedNotSendSync(Box::new(move |constraint, _| {
                let size = measure(swift_ptr, constraint.width.unwrap_or(f32::NAN), constraint.height.unwrap_or(f32::NAN));
                taffy::geometry::Size { height: size.height, width: size.width }
            }))),
        )
        .unwrap();

    Box::leak(stretch);
    Box::leak(node);
}

#[no_mangle]
pub unsafe extern "C" fn taffy_node_set_style(stretch: *mut c_void, node: *mut c_void, style: *mut c_void) {
    let mut stretch = Box::from_raw(stretch as *mut Taffy);
    let node = Box::from_raw(node as *mut Node);
    let style = Box::from_raw(style as *mut Style);

    stretch.set_style(*node, *style).unwrap();

    Box::leak(stretch);
    Box::leak(node);
    Box::leak(style);
}

#[no_mangle]
pub unsafe extern "C" fn taffy_node_dirty(stretch: *mut c_void, node: *mut c_void) -> bool {
    let stretch = Box::from_raw(stretch as *mut Taffy);
    let node = Box::from_raw(node as *mut Node);
    let dirty = stretch.dirty(*node).unwrap();

    Box::leak(stretch);
    Box::leak(node);

    dirty
}

#[no_mangle]
pub unsafe extern "C" fn taffy_node_mark_dirty(stretch: *mut c_void, node: *mut c_void) {
    let mut stretch = Box::from_raw(stretch as *mut Taffy);
    let node = Box::from_raw(node as *mut Node);

    stretch.mark_dirty(*node).unwrap();

    Box::leak(stretch);
    Box::leak(node);
}

#[no_mangle]
pub unsafe extern "C" fn taffy_node_add_child(stretch: *mut c_void, node: *mut c_void, child: *mut c_void) {
    let mut stretch = Box::from_raw(stretch as *mut Taffy);
    let node = Box::from_raw(node as *mut Node);
    let child = Box::from_raw(child as *mut Node);

    stretch.add_child(*node, *child).unwrap();

    Box::leak(stretch);
    Box::leak(node);
    Box::leak(child);
}

#[no_mangle]
pub unsafe extern "C" fn taffy_node_replace_child_at_index(
    stretch: *mut c_void,
    node: *mut c_void,
    index: usize,
    child: *mut c_void,
) {
    let mut stretch = Box::from_raw(stretch as *mut Taffy);
    let node = Box::from_raw(node as *mut Node);
    let child = Box::from_raw(child as *mut Node);

    stretch.replace_child_at_index(*node, index, *child).unwrap();

    Box::leak(stretch);
    Box::leak(node);
    Box::leak(child);
}

#[no_mangle]
pub unsafe extern "C" fn taffy_node_remove_child(stretch: *mut c_void, node: *mut c_void, child: *mut c_void) {
    let mut stretch = Box::from_raw(stretch as *mut Taffy);
    let node = Box::from_raw(node as *mut Node);
    let child = Box::from_raw(child as *mut Node);

    stretch.remove_child(*node, *child).unwrap();

    Box::leak(stretch);
    Box::leak(node);
    Box::leak(child);
}

#[no_mangle]
pub unsafe extern "C" fn taffy_node_remove_child_at_index(stretch: *mut c_void, node: *mut c_void, index: usize) {
    let mut stretch = Box::from_raw(stretch as *mut Taffy);
    let node = Box::from_raw(node as *mut Node);

    stretch.remove_child_at_index(*node, index).unwrap();

    Box::leak(stretch);
    Box::leak(node);
}

#[no_mangle]
pub unsafe extern "C" fn taffy_node_compute_layout(
    stretch: *mut c_void,
    node: *mut c_void,
    width: f32,
    height: f32,
    create_layout: unsafe extern "C" fn(*const f32) -> *mut c_void,
) -> *mut c_void {
    let mut stretch = Box::from_raw(stretch as *mut Taffy);
    let node = Box::from_raw(node as *mut Node);

    stretch
        .compute_layout(
            *node,
            Size {
                width: if f32::is_nan(width) { AvailableSpace::MaxContent } else { AvailableSpace::Definite(width) },
                height: if f32::is_nan(height) { AvailableSpace::MaxContent } else { AvailableSpace::Definite(height) },
            },
        )
        .unwrap();

    let mut output = vec![];
    copy_output(&stretch, *node, &mut output);

    Box::leak(stretch);
    Box::leak(node);

    create_layout(output.as_ptr())
}

fn copy_output(stretch: &Taffy, node: Node, output: &mut Vec<f32>) {
    let layout = stretch.layout(node).unwrap();
    let children = stretch.children(node).unwrap();

    output.push(layout.location.x);
    output.push(layout.location.y);
    output.push(layout.size.width);
    output.push(layout.size.height);
    output.push(children.len() as f32);

    for child in &children {
        copy_output(stretch, *child, output);
    }
}
