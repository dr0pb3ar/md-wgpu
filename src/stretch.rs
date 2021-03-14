use std::collections::HashMap;
use stretch::geometry::Size;
use stretch::number::Number;
use stretch::style::*;

pub fn stretch(
    width: f32,
    height: f32,
) -> Result<
    (
        stretch::node::Stretch,
        HashMap<&'static str, stretch::node::Node>,
    ),
    stretch::Error,
> {
    let mut stretch = stretch::node::Stretch::new();

    let font_size = 24.0;

    let info = stretch.new_leaf(
        Style {
            size: Size {
                width: Dimension::Auto,
                height: Dimension::Points(font_size),
            },

            ..Default::default()
        },
        Box::new(|constraint| {
            println!("info constraint {:?}", constraint);
            Ok(Size {
                width: 43.0,
                height: 20.0,
            })
        }),
    )?;

    let target_pinpoint = stretch.new_leaf(
        Style {
            align_self: AlignSelf::Center,
            size: Size {
                //width: Dimension::Percent(1.0),
                width: Dimension::Auto,
                height: Dimension::Auto,
            },
            flex_grow: 1.0,
            aspect_ratio: Number::Defined(1.0),
            ..Default::default()
        },
        Box::new(|constraint| {
            println!("target_pinpoint constraint {:?}", constraint);
            /*if let Number::Defined(width) = constraint.width {
                Ok(Size {
                    width,
                    height: width,
                })
            } else if let Number::Defined(height) = constraint.height {
                Ok(Size {
                    width: height,
                    height,
                })
            } else {*/
            Ok(Size {
                    width: 0.0,
                    height: 0.0
                    //width: 40.0,
                    //height: 40.0,
                })
            //}
        }),
    )?;

    let block_map = stretch.new_leaf(
        Style {
            //position_type: PositionType::Absolute,
            aspect_ratio: Number::Defined(3.0),
            ..Default::default()
        },
        Box::new(|constraint| {
            println!("block_map constraint {:?}", constraint);
            Ok(Size {
                width: 30.0,
                height: 70.0,
            })
        }),
    )?;

    let compass = stretch.new_leaf(
        Style {
            //position_type: PositionType::Absolute,
            size: Size {
                width: Dimension::Auto,
                height: Dimension::Percent(0.05),
            },
            ..Default::default()
        },
        Box::new(|constraint| {
            println!("compass constraint {:?}", constraint);
            Ok(Size {
                width: 20.0,
                height: 10.0,
            })
        }),
    )?;

    /*let scale = stretch.new_leaf(
        Style {
            size: Size {
                width: Dimension::Points(font_size * 5.0),
                height: Dimension::Auto,
            },
            //position_type: PositionType::Absolute,
            ..Default::default()
        },
        Box::new(|constraint| {
            println!("scale constraint {:?}", constraint);
            Ok(Size {
                width: 0.0,
                height: 0.0,
            })
        }),
    )?;*/

    let col1 = stretch.new_node(
        Style {
            //position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            //align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            size: Size {
                width: Dimension::Percent(0.5),
                height: Dimension::Percent(1.0),
            },
            ..Default::default()
        },
        vec![info, target_pinpoint],
    )?;

    let col2 = stretch.new_node(
        Style {
            //position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            size: Size {
                width: Dimension::Auto,
                height: Dimension::Auto,
            },
            aspect_ratio: Number::Defined(0.3),
            ..Default::default()
        },
        vec![block_map, compass],
    )?;

    /*let row = stretch.new_node(
        Style {
            //position_type: PositionType::Absolute,
            //align_items: AlignItems::Center,
            ..Default::default()
        },
        vec![scale, col2],
    )?;*/

    let root = stretch.new_node(
        Style {
            justify_content: JustifyContent::SpaceBetween,
            size: Size {
                width: Dimension::Points(width),
                height: Dimension::Points(height),
            },
            ..Default::default()
        },
        vec![col1, col2],
    )?;

    stretch.compute_layout(root, Size::undefined())?;
    println!("root {:?}", stretch.layout(root));
    println!("col1 {:?}", stretch.layout(col1));
    println!("info {:?}", stretch.layout(info));
    println!("target_pinpoint {:?}", stretch.layout(target_pinpoint));
    //println!("row {:?}", stretch.layout(row));
    //println!("scale {:?}", stretch.layout(scale));
    println!("col2 {:?}", stretch.layout(col2));
    println!("block_map {:?}", stretch.layout(block_map));
    println!("compass {:?}", stretch.layout(compass));

    let mut map = HashMap::new();
    map.insert("root", root);
    map.insert("col1", col1);
    map.insert("info", info);
    map.insert("target_pinpoint", target_pinpoint);
    map.insert("col2", col2);
    map.insert("block_map", block_map);
    map.insert("compass", compass);
    Ok((stretch, map))

    /*let leaf1 = stretch.new_leaf(
        Style::default(),
        Box::new(|constraint| {
            println!("constraint {:?}", constraint);
            Ok(Size {
                width: 29.0,
                height: 0.0,
            })
        }),
    )?;

    let child = stretch.new_node(
        Style {
            size: Size {
                width: Dimension::Percent(0.5),
                height: Dimension::Auto,
            },
            ..Default::default()
        },
        vec![leaf1],
    )?;

    let node = stretch.new_node(
        Style {
            size: Size {
                width: Dimension::Points(100.0),
                height: Dimension::Points(100.0),
            },
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        vec![child],
    )?;

    stretch.compute_layout(node, Size::undefined())?;
    dbg!(stretch.layout(node)?);
    println!("leaf {:?}", stretch.layout(leaf1));*/

    //Ok(())
}

/*
 *The align-items property will align the items on the cross axis.

The initial value for this property is stretch and this is why flex items stretch to the height of the flex container by default. This might be dictated by the height of the tallest item in the container, or by a size set on the flex container itself.

You could instead set align-items to flex-start in order to make the items line up at the start of the flex container, flex-end to align them to the end, or center to align them in the centre. Try this in the live example — I have given the flex container a height in order that you can see how the items can be moved around inside the container. See what happens if you set the value of align-items to:

stretch
flex-start
flex-end
center

The justify-content property is used to align the items on the main axis, the direction in which flex-direction has set the flow. The initial value is flex-start which will line the items up at the start edge of the container, but you could also set the value to flex-end to line them up at the end, or center to line them up in the centre.

You can also use the value space-between to take all the spare space after the items have been laid out, and share it out evenly between the items so there will be an equal amount of space between each item. To cause an equal amount of space on the right and left of each item use the value space-around. With space-around, items have a half-size space on either end. Or, to cause items to have equal space around them use the value space-evenly. With space-evenly, items have a full-size space on either end.

*/
