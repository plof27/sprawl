#[cfg(test)]
mod measure {
    use sprawl::node::MeasureFunc;
    use sprawl::number::OrElse;

    #[test]
    fn measure_root() {
        let mut sprawl = sprawl::node::Sprawl::new();
        let node = sprawl
            .new_leaf(
                sprawl::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| sprawl::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();

        assert_eq!(sprawl.layout(node).unwrap().size.width, 100.0);
        assert_eq!(sprawl.layout(node).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child() {
        let mut sprawl = sprawl::node::Sprawl::new();

        let child = sprawl
            .new_leaf(
                sprawl::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| sprawl::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = sprawl.new_node(sprawl::style::Style { ..Default::default() }, &[child]).unwrap();
        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();

        assert_eq!(sprawl.layout(node).unwrap().size.width, 100.0);
        assert_eq!(sprawl.layout(node).unwrap().size.height, 100.0);

        assert_eq!(sprawl.layout(child).unwrap().size.width, 100.0);
        assert_eq!(sprawl.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint() {
        let mut sprawl = sprawl::node::Sprawl::new();
        let child = sprawl
            .new_leaf(
                sprawl::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| sprawl::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = sprawl
            .new_node(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Points(50.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();

        assert_eq!(sprawl.layout(node).unwrap().size.width, 50.0);
        assert_eq!(sprawl.layout(node).unwrap().size.height, 100.0);

        assert_eq!(sprawl.layout(child).unwrap().size.width, 50.0);
        assert_eq!(sprawl.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint_padding_parent() {
        let mut sprawl = sprawl::node::Sprawl::new();
        let child = sprawl
            .new_leaf(
                sprawl::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| sprawl::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = sprawl
            .new_node(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Points(50.0),
                        ..Default::default()
                    },
                    padding: sprawl::geometry::Rect {
                        start: sprawl::style::Dimension::Points(10.0),
                        end: sprawl::style::Dimension::Points(10.0),
                        top: sprawl::style::Dimension::Points(10.0),
                        bottom: sprawl::style::Dimension::Points(10.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();
        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();

        assert_eq!(sprawl.layout(node).unwrap().size.width, 50.0);
        assert_eq!(sprawl.layout(node).unwrap().size.height, 120.0);

        assert_eq!(sprawl.layout(child).unwrap().size.width, 30.0);
        assert_eq!(sprawl.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_with_flex_grow() {
        let mut sprawl = sprawl::node::Sprawl::new();
        let child0 = sprawl
            .new_node(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Points(50.0),
                        height: sprawl::style::Dimension::Points(50.0),
                    },
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        let child1 = sprawl
            .new_leaf(
                sprawl::style::Style { flex_grow: 1.0, ..Default::default() },
                MeasureFunc::Raw(|constraint| sprawl::geometry::Size {
                    width: constraint.width.or_else(10.0),
                    height: constraint.height.or_else(50.0),
                }),
            )
            .unwrap();

        let node = sprawl
            .new_node(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Points(100.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();

        assert_eq!(sprawl.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(sprawl.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn measure_child_with_flex_shrink() {
        let mut sprawl = sprawl::node::Sprawl::new();
        let child0 = sprawl
            .new_node(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Points(50.0),
                        height: sprawl::style::Dimension::Points(50.0),
                    },
                    flex_shrink: 0.0,
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        let child1 = sprawl
            .new_leaf(
                sprawl::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| sprawl::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(50.0),
                }),
            )
            .unwrap();

        let node = sprawl
            .new_node(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Points(100.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();

        assert_eq!(sprawl.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(sprawl.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn remeasure_child_after_growing() {
        let mut sprawl = sprawl::node::Sprawl::new();
        let child0 = sprawl
            .new_node(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Points(50.0),
                        height: sprawl::style::Dimension::Points(50.0),
                    },
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        let child1 = sprawl
            .new_leaf(
                sprawl::style::Style { flex_grow: 1.0, ..Default::default() },
                MeasureFunc::Raw(|constraint| {
                    let width = constraint.width.or_else(10.0);
                    let height = constraint.height.or_else(width * 2.0);
                    sprawl::geometry::Size { width, height }
                }),
            )
            .unwrap();

        let node = sprawl
            .new_node(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Points(100.0),
                        ..Default::default()
                    },
                    align_items: sprawl::style::AlignItems::FlexStart,
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();

        assert_eq!(sprawl.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(sprawl.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn remeasure_child_after_shrinking() {
        let mut sprawl = sprawl::node::Sprawl::new();

        let child0 = sprawl
            .new_node(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Points(50.0),
                        height: sprawl::style::Dimension::Points(50.0),
                    },
                    flex_shrink: 0.0,
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        let child1 = sprawl
            .new_leaf(
                sprawl::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| {
                    let width = constraint.width.or_else(100.0);
                    let height = constraint.height.or_else(width * 2.0);
                    sprawl::geometry::Size { width, height }
                }),
            )
            .unwrap();

        let node = sprawl
            .new_node(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Points(100.0),
                        ..Default::default()
                    },
                    align_items: sprawl::style::AlignItems::FlexStart,
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();

        assert_eq!(sprawl.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(sprawl.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn remeasure_child_after_stretching() {
        let mut sprawl = sprawl::node::Sprawl::new();

        let child = sprawl
            .new_leaf(
                sprawl::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| {
                    let height = constraint.height.or_else(50.0);
                    let width = constraint.width.or_else(height);
                    sprawl::geometry::Size { width, height }
                }),
            )
            .unwrap();

        let node = sprawl
            .new_node(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Points(100.0),
                        height: sprawl::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();

        assert_eq!(sprawl.layout(child).unwrap().size.width, 100.0);
        assert_eq!(sprawl.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn width_overrides_measure() {
        let mut sprawl = sprawl::node::Sprawl::new();
        let child = sprawl
            .new_leaf(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Points(50.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                MeasureFunc::Raw(|constraint| sprawl::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = sprawl.new_node(sprawl::style::Style { ..Default::default() }, &[child]).unwrap();
        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();

        assert_eq!(sprawl.layout(child).unwrap().size.width, 50.0);
        assert_eq!(sprawl.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn height_overrides_measure() {
        let mut sprawl = sprawl::node::Sprawl::new();
        let child = sprawl
            .new_leaf(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        height: sprawl::style::Dimension::Points(50.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                MeasureFunc::Raw(|constraint| sprawl::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = sprawl.new_node(sprawl::style::Style { ..Default::default() }, &[child]).unwrap();
        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();

        assert_eq!(sprawl.layout(child).unwrap().size.width, 100.0);
        assert_eq!(sprawl.layout(child).unwrap().size.height, 50.0);
    }

    #[test]
    fn flex_basis_overrides_measure() {
        let mut sprawl = sprawl::node::Sprawl::new();
        let child0 = sprawl
            .new_node(
                sprawl::style::Style {
                    flex_basis: sprawl::style::Dimension::Points(50.0),
                    flex_grow: 1.0,
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        let child1 = sprawl
            .new_leaf(
                sprawl::style::Style {
                    flex_basis: sprawl::style::Dimension::Points(50.0),
                    flex_grow: 1.0,
                    ..Default::default()
                },
                MeasureFunc::Raw(|constraint| sprawl::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = sprawl
            .new_node(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Points(200.0),
                        height: sprawl::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();

        assert_eq!(sprawl.layout(child0).unwrap().size.width, 100.0);
        assert_eq!(sprawl.layout(child0).unwrap().size.height, 100.0);
        assert_eq!(sprawl.layout(child1).unwrap().size.width, 100.0);
        assert_eq!(sprawl.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn stretch_overrides_measure() {
        let mut sprawl = sprawl::node::Sprawl::new();
        let child = sprawl
            .new_leaf(
                sprawl::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| sprawl::geometry::Size {
                    width: constraint.width.or_else(50.0),
                    height: constraint.height.or_else(50.0),
                }),
            )
            .unwrap();

        let node = sprawl
            .new_node(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Points(100.0),
                        height: sprawl::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();

        assert_eq!(sprawl.layout(child).unwrap().size.width, 50.0);
        assert_eq!(sprawl.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_absolute_child() {
        let mut sprawl = sprawl::node::Sprawl::new();
        let child = sprawl
            .new_leaf(
                sprawl::style::Style { position_type: sprawl::style::PositionType::Absolute, ..Default::default() },
                MeasureFunc::Raw(|constraint| sprawl::geometry::Size {
                    width: constraint.width.or_else(50.0),
                    height: constraint.height.or_else(50.0),
                }),
            )
            .unwrap();

        let node = sprawl
            .new_node(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Points(100.0),
                        height: sprawl::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();

        assert_eq!(sprawl.layout(child).unwrap().size.width, 50.0);
        assert_eq!(sprawl.layout(child).unwrap().size.height, 50.0);
    }

    #[test]
    fn ignore_invalid_measure() {
        let mut sprawl = sprawl::node::Sprawl::new();
        let child = sprawl
            .new_leaf(
                sprawl::style::Style { flex_grow: 1.0, ..Default::default() },
                MeasureFunc::Raw(|_| sprawl::geometry::Size { width: 200.0, height: 200.0 }),
            )
            .unwrap();

        let node = sprawl
            .new_node(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Points(100.0),
                        height: sprawl::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();

        assert_eq!(sprawl.layout(child).unwrap().size.width, 100.0);
        assert_eq!(sprawl.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn only_measure_once() {
        use std::sync::atomic;

        let mut sprawl = sprawl::node::Sprawl::new();
        static NUM_MEASURES: atomic::AtomicU32 = atomic::AtomicU32::new(0);

        let grandchild = sprawl
            .new_leaf(
                sprawl::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| {
                    NUM_MEASURES.fetch_add(1, atomic::Ordering::Relaxed);
                    sprawl::geometry::Size {
                        width: constraint.width.or_else(50.0),
                        height: constraint.height.or_else(50.0),
                    }
                }),
            )
            .unwrap();

        let child = sprawl.new_node(sprawl::style::Style { ..Default::default() }, &[grandchild]).unwrap();

        let node = sprawl.new_node(sprawl::style::Style { ..Default::default() }, &[child]).unwrap();
        sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();

        assert_eq!(NUM_MEASURES.load(atomic::Ordering::Relaxed), 2);
    }
}
