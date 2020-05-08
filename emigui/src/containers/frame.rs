//! Frame container

use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Frame {
    pub margin: Option<Vec2>,
}

impl Frame {
    pub fn margin(mut self, margin: Vec2) -> Self {
        self.margin = Some(margin);
        self
    }
}

impl Frame {
    pub fn show(self, region: &mut Region, add_contents: impl FnOnce(&mut Region)) {
        let style = region.style();
        let margin = self.margin.unwrap_or_default();

        let outer_pos = region.cursor();
        let inner_rect =
            Rect::from_min_size(outer_pos + margin, region.available_space() - 2.0 * margin);
        let where_to_put_background = region.paint_list_len();

        let mut child_region = region.child_region(inner_rect);
        add_contents(&mut child_region);

        let inner_size = child_region.bounding_size();
        let inner_size = inner_size.ceil(); // TODO: round to pixel

        let outer_rect = Rect::from_min_size(outer_pos, margin + inner_size + margin);

        let corner_radius = style.window.corner_radius;
        let fill_color = style.background_fill_color();
        region.insert_paint_cmd(
            where_to_put_background,
            PaintCmd::Rect {
                corner_radius,
                fill_color: Some(fill_color),
                outline: Some(Outline::new(1.0, color::WHITE)),
                rect: outer_rect,
            },
        );

        region.expand_to_include_child(child_region.child_bounds().expand2(margin));
        // TODO: move up cursor?
    }
}
