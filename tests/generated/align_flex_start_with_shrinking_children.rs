#[test]
fn align_flex_start_with_shrinking_children() {
    let mut sprawl = sprawl::Sprawl::new();
    let node000 = sprawl
        .new_node(sprawl::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, &[])
        .unwrap();
    let node00 = sprawl
        .new_node(sprawl::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, &[node000])
        .unwrap();
    let node0 = sprawl
        .new_node(
            sprawl::style::Style { align_items: sprawl::style::AlignItems::FlexStart, ..Default::default() },
            &[node00],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(500f32),
                    height: sprawl::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 500f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 500f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 500f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.width, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.height, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node000).unwrap().size.width, 0f32);
    assert_eq!(sprawl.layout(node000).unwrap().size.height, 0f32);
    assert_eq!(sprawl.layout(node000).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node000).unwrap().location.y, 0f32);
}
