#[test]
fn flex_basis_slightly_smaller_then_content_with_flex_grow_large_size() {
    let mut sprawl = sprawl::Sprawl::new();
    let node00 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(70f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = sprawl
        .new_node(
            sprawl::style::Style {
                flex_direction: sprawl::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: sprawl::style::Dimension::Points(60f32),
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node10 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(20f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = sprawl
        .new_node(
            sprawl::style::Style {
                flex_direction: sprawl::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: sprawl::style::Dimension::Points(0f32),
                ..Default::default()
            },
            &[node10],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Points(100f32), ..Default::default() },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 80f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.width, 70f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.width, 20f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.x, 80f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node10).unwrap().size.width, 20f32);
    assert_eq!(sprawl.layout(node10).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node10).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node10).unwrap().location.y, 0f32);
}
