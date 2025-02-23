#[test]
fn align_baseline_child_multiline() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(50f32),
                    height: sprawl::style::Dimension::Points(60f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node10 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(25f32),
                    height: sprawl::style::Dimension::Points(20f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node11 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(25f32),
                    height: sprawl::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node12 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(25f32),
                    height: sprawl::style::Dimension::Points(20f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node13 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(25f32),
                    height: sprawl::style::Dimension::Points(10f32),
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
                flex_wrap: sprawl::style::FlexWrap::Wrap,
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Points(50f32), ..Default::default() },
                ..Default::default()
            },
            &[node10, node11, node12, node13],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                align_items: sprawl::style::AlignItems::Baseline,
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Points(100f32), ..Default::default() },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 80f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 50f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 60f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.width, 50f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.height, 40f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.x, 50f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.y, 40f32);
    assert_eq!(sprawl.layout(node10).unwrap().size.width, 25f32);
    assert_eq!(sprawl.layout(node10).unwrap().size.height, 20f32);
    assert_eq!(sprawl.layout(node10).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node10).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node11).unwrap().size.width, 25f32);
    assert_eq!(sprawl.layout(node11).unwrap().size.height, 10f32);
    assert_eq!(sprawl.layout(node11).unwrap().location.x, 25f32);
    assert_eq!(sprawl.layout(node11).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node12).unwrap().size.width, 25f32);
    assert_eq!(sprawl.layout(node12).unwrap().size.height, 20f32);
    assert_eq!(sprawl.layout(node12).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node12).unwrap().location.y, 20f32);
    assert_eq!(sprawl.layout(node13).unwrap().size.width, 25f32);
    assert_eq!(sprawl.layout(node13).unwrap().size.height, 10f32);
    assert_eq!(sprawl.layout(node13).unwrap().location.x, 25f32);
    assert_eq!(sprawl.layout(node13).unwrap().location.y, 20f32);
}
