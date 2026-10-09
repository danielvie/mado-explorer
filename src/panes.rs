use crate::{directory::DirectoryLoader, sorting::Sorter, tabs::Tab};
use eframe::egui::{self, Rect, Vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PaneId(pub u64);

pub struct Pane {
    pub id: PaneId,
    pub tabs: Vec<Tab>,
    pub active: usize,
    pub reveal_active_tab: bool,
}

impl Pane {
    pub fn cancel_tab(&self, index: usize, loader: &DirectoryLoader, sorter: &Sorter) {
        let target = self.tabs[index].id.0;
        loader.cancel(target);
        sorter.cancel(target);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    LeftRight,
    TopBottom,
}

pub const MIN_PANE: Vec2 = Vec2::new(320.0, 300.0);
pub const DIVIDER: f32 = 6.0;

pub enum Layout {
    Pane(PaneId),
    Split {
        id: PaneId,
        axis: Axis,
        ratio: f32,
        first: Box<Layout>,
        second: Box<Layout>,
    },
}

pub struct Divider {
    pub id: PaneId,
    pub axis: Axis,
    pub rect: Rect,
    pub bounds: Rect,
    pub first_min: f32,
    pub second_min: f32,
}

impl Axis {
    fn extent(self, size: Vec2) -> f32 {
        match self {
            Self::LeftRight => size.x,
            Self::TopBottom => size.y,
        }
    }
}

impl Layout {
    pub fn minimum(&self) -> Vec2 {
        match self {
            Self::Pane(_) => MIN_PANE,
            Self::Split {
                axis,
                first,
                second,
                ..
            } => {
                let a = first.minimum();
                let b = second.minimum();
                match axis {
                    Axis::LeftRight => Vec2::new(a.x + DIVIDER + b.x, a.y.max(b.y)),
                    Axis::TopBottom => Vec2::new(a.x.max(b.x), a.y + DIVIDER + b.y),
                }
            }
        }
    }

    pub fn split(&mut self, source: PaneId, new: PaneId, axis: Axis, available: Vec2) -> bool {
        match self {
            Self::Pane(id) if *id == source => {
                if axis.extent(available) < axis.extent(MIN_PANE) * 2.0 + DIVIDER
                    || available.x < MIN_PANE.x
                    || available.y < MIN_PANE.y
                {
                    return false;
                }
                *self = Self::Split {
                    id: new,
                    axis,
                    ratio: 0.5,
                    first: Box::new(Self::Pane(source)),
                    second: Box::new(Self::Pane(new)),
                };
                true
            }
            Self::Split { first, second, .. } => {
                first.split(source, new, axis, available)
                    || second.split(source, new, axis, available)
            }
            _ => false,
        }
    }

    // Only called for a non-final leaf, so its sibling always replaces the branch.
    pub fn remove(&mut self, target: PaneId) -> bool {
        let Self::Split { first, second, .. } = self else {
            return false;
        };
        if matches!(&**first, Self::Pane(id) if *id == target) {
            *self = std::mem::replace(&mut **second, Self::Pane(target));
            true
        } else if matches!(&**second, Self::Pane(id) if *id == target) {
            *self = std::mem::replace(&mut **first, Self::Pane(target));
            true
        } else {
            first.remove(target) || second.remove(target)
        }
    }

    pub fn rectangles(
        &self,
        bounds: Rect,
        panes: &mut Vec<(PaneId, Rect)>,
        dividers: &mut Vec<Divider>,
    ) {
        match self {
            Self::Pane(id) => panes.push((*id, bounds)),
            Self::Split {
                id,
                axis,
                ratio,
                first,
                second,
            } => {
                let extent = axis.extent(bounds.size()) - DIVIDER;
                let a_min = axis.extent(first.minimum());
                let b_min = axis.extent(second.minimum());
                let a = (extent * ratio).clamp(a_min, (extent - b_min).max(a_min));
                let mut left = bounds;
                let mut right = bounds;
                let mut divider = bounds;
                match axis {
                    Axis::LeftRight => {
                        left.max.x = bounds.left() + a;
                        divider.min.x = left.right();
                        divider.max.x = divider.min.x + DIVIDER;
                        right.min.x = divider.right();
                    }
                    Axis::TopBottom => {
                        left.max.y = bounds.top() + a;
                        divider.min.y = left.bottom();
                        divider.max.y = divider.min.y + DIVIDER;
                        right.min.y = divider.bottom();
                    }
                }
                dividers.push(Divider {
                    id: *id,
                    axis: *axis,
                    rect: divider,
                    bounds,
                    first_min: a_min,
                    second_min: b_min,
                });
                first.rectangles(left, panes, dividers);
                second.rectangles(right, panes, dividers);
            }
        }
    }

    pub fn resize(&mut self, divider: &Divider, pointer: egui::Pos2) {
        match self {
            Self::Split {
                id,
                ratio,
                axis,
                first,
                second,
            } => {
                if *id == divider.id {
                    let extent = axis.extent(divider.bounds.size()) - DIVIDER;
                    let offset = match axis {
                        Axis::LeftRight => pointer.x - divider.bounds.left(),
                        Axis::TopBottom => pointer.y - divider.bounds.top(),
                    };
                    *ratio = (offset - DIVIDER / 2.0).clamp(
                        divider.first_min,
                        (extent - divider.second_min).max(divider.first_min),
                    ) / extent;
                } else {
                    first.resize(divider, pointer);
                    second.resize(divider, pointer);
                }
            }
            Self::Pane(_) => {}
        }
    }
}

#[cfg(test)]
impl Pane {
    pub fn active_tab_mut(&mut self) -> &mut Tab {
        &mut self.tabs[self.active]
    }
}
