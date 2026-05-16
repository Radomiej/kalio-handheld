use taffy::prelude::*;

/// Thin wrapper around Taffy — flexbox / grid layout engine.
pub struct LayoutEngine {
    tree: TaffyTree,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self { tree: TaffyTree::new() }
    }

    /// Create a flex-column container that fills available space.
    pub fn column(&mut self, gap: f32) -> NodeId {
        self.tree.new_leaf(Style {
            display:        Display::Flex,
            flex_direction: FlexDirection::Column,
            gap:   Size { width: length(gap), height: length(gap) },
            size:  Size { width: percent(1.0), height: percent(1.0) },
            ..Default::default()
        }).expect("layout: create column")
    }

    /// Create a flex-row container.
    pub fn row(&mut self, gap: f32) -> NodeId {
        self.tree.new_leaf(Style {
            display:        Display::Flex,
            flex_direction: FlexDirection::Row,
            gap:   Size { width: length(gap), height: length(gap) },
            size:  Size { width: percent(1.0), height: percent(1.0) },
            ..Default::default()
        }).expect("layout: create row")
    }

    /// Create a fixed-size leaf node.
    pub fn item(&mut self, w: f32, h: f32) -> NodeId {
        self.tree.new_leaf(Style {
            size: Size { width: length(w), height: length(h) },
            ..Default::default()
        }).expect("layout: create item")
    }

    pub fn set_children(&mut self, parent: NodeId, children: &[NodeId]) {
        self.tree.set_children(parent, children).expect("layout: set children");
    }

    /// Compute layout for the whole tree.
    pub fn compute(&mut self, root: NodeId, available_w: f32, available_h: f32) {
        self.tree
            .compute_layout(
                root,
                Size {
                    width:  AvailableSpace::Definite(available_w),
                    height: AvailableSpace::Definite(available_h),
                },
            )
            .expect("layout: compute");
    }

    /// Read back the computed position + size of a node.
    pub fn get(&self, node: NodeId) -> Layout {
        *self.tree.layout(node).expect("layout: get")
    }
}

impl Default for LayoutEngine {
    fn default() -> Self { Self::new() }
}
