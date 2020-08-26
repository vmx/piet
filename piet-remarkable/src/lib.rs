use std::borrow::Cow;

use piet::kurbo::{Affine, Point, Rect, Shape};
use piet::{
    self, Color, Error, FixedGradient, ImageFormat, InterpolationMode, IntoBrush, StrokeStyle,
};

mod text;

pub use crate::text::{Text, TextLayout};

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Brush {
    Solid(u32),
}

impl IntoBrush<RenderContext> for Brush {
    fn make_brush<'b>(
        &'b self,
        _piet: &mut RenderContext,
        _bbox: impl FnOnce() -> Rect,
    ) -> Cow<'b, Brush> {
        Cow::Borrowed(self)
        //Cow::Owned(self.clone())
    }
}

/// reMarkable image (unimplemented)
pub struct Image(());

pub struct RenderContext {}

impl RenderContext {}

impl piet::RenderContext for RenderContext {
    type Brush = Brush;

    type Text = Text;
    type TextLayout = TextLayout;

    type Image = Image;

    fn status(&mut self) -> Result<()> {
        Ok(())
    }

    fn solid_brush(&mut self, color: Color) -> Brush {
        Brush::Solid(color.as_rgba_u32())
    }

    fn gradient(&mut self, gradient: impl Into<FixedGradient>) -> Result<Brush> {
        unimplemented!()
    }

    fn clear(&mut self, color: Color) {
        // Convert color to grayscale and paint the full canvas with it
        unimplemented!()
    }

    fn stroke(&mut self, shape: impl Shape, brush: &impl IntoBrush<Self>, width: f64) {
        //if let Some(circle) = shape.as_circle() {
        //    let mut x = svg::node::element::Circle::new()
        //        .set("cx", circle.center.x)
        //        .set("cy", circle.center.y)
        //        .set("r", circle.radius);
        //    attrs.apply_to(&mut x);
        //    node.append(x);
        //} else if let Some(rect) = shape.as_rounded_rect() {
        //    let mut x = svg::node::element::Rectangle::new()
        //        .set("x", rect.origin().x)
        //        .set("y", rect.origin().y)
        //        .set("width", rect.width())
        //        .set("height", rect.height())
        //        .set("rx", rect.radius())
        //        .set("ry", rect.radius());
        //    attrs.apply_to(&mut x);
        //    node.append(x);
        //} else if let Some(rect) = shape.as_rect() {
        //    let mut x = svg::node::element::Rectangle::new()
        //        .set("x", rect.origin().x)
        //        .set("y", rect.origin().y)
        //        .set("width", rect.width())
        //        .set("height", rect.height());
        //    attrs.apply_to(&mut x);
        //    node.append(x);
        //} else {
        //    let mut path = svg::node::element::Path::new().set("d", shape.into_bez_path(1e-3).to_svg());
        //    attrs.apply_to(&mut path);
        //    node.append(path)
        //}
    }

    fn stroke_styled(
        &mut self,
        shape: impl Shape,
        brush: &impl IntoBrush<Self>,
        width: f64,
        style: &StrokeStyle,
    ) {
        unimplemented!()
    }

    fn fill(&mut self, shape: impl Shape, brush: &impl IntoBrush<Self>) {
        unimplemented!()
    }

    fn fill_even_odd(&mut self, shape: impl Shape, brush: &impl IntoBrush<Self>) {
        unimplemented!()
    }

    fn clip(&mut self, shape: impl Shape) {
        unimplemented!()
    }

    fn text(&mut self) -> &mut Self::Text {
        unimplemented!()
    }

    fn draw_text(&mut self, _layout: &Self::TextLayout, _pos: impl Into<Point>) {
        unimplemented!()
    }

    fn save(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn restore(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn finish(&mut self) -> Result<()> {
        Ok(())
    }

    fn transform(&mut self, transform: Affine) {
        unimplemented!()
    }

    fn make_image(
        &mut self,
        _width: usize,
        _height: usize,
        _buf: &[u8],
        _format: ImageFormat,
    ) -> Result<Self::Image> {
        Err(Error::NotSupported)
    }

    fn draw_image(
        &mut self,
        image: &Self::Image,
        dst_rect: impl Into<Rect>,
        interp: InterpolationMode,
    ) {
        unimplemented!()
    }

    fn draw_image_area(
        &mut self,
        image: &Self::Image,
        src_rect: impl Into<Rect>,
        dst_rect: impl Into<Rect>,
        interp: InterpolationMode,
    ) {
        unimplemented!()
    }

    fn blurred_rect(&mut self, _rect: Rect, _blur_radius: f64, _brush: &impl IntoBrush<Self>) {
        unimplemented!()
    }

    fn current_transform(&self) -> Affine {
        unimplemented!()
    }
}
