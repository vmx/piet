use std::borrow::Cow;

use libremarkable::framebuffer::common::{
    color as RemarkableColor, display_temp, dither_mode, mxcfb_rect, waveform_mode, DISPLAYHEIGHT,
    DISPLAYWIDTH,
};
use libremarkable::framebuffer::refresh::PartialRefreshMode;
use libremarkable::framebuffer::{self, FramebufferBase, FramebufferDraw, FramebufferRefresh};
use piet::kurbo::{Affine, PathSeg, Point, Rect, Shape};
use piet::{
    self, Color, Error, FixedGradient, ImageFormat, InterpolationMode, IntoBrush, StrokeStyle,
};

mod text;

pub use crate::text::{Text, TextLayout};

type Result<T> = std::result::Result<T, Error>;

fn to_remarkable_color(color: Color) -> RemarkableColor {
    let (red, green, blue, _alpha) = color.as_rgba8();
    RemarkableColor::RGB(red, green, blue)
}

#[derive(Clone, Debug)]
pub enum Brush {
    Solid(u32),
}

impl<'a> IntoBrush<RenderContext<'_>> for Brush {
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

pub struct RenderContext<'a> {
    framebuffer: framebuffer::core::Framebuffer<'a>,
    text: Text,
}

impl<'a> RenderContext<'_> {
    /// Construct an empty `RenderContext`
    pub fn new() -> Self {
        let mut framebuffer = framebuffer::core::Framebuffer::new("/dev/fb0");
        println!("vmx: frambuffer created");

        println!("vmx: clear frambuffer");
        framebuffer.clear();

        println!("vmx: refresh frambuffer");
        framebuffer.full_refresh(
            waveform_mode::WAVEFORM_MODE_INIT,
            display_temp::TEMP_USE_AMBIENT,
            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
            0,
            true,
        );

        println!("vmx: pre text");
        let text = Text::new();
        println!("vmx: post text: {:?}", text);

        Self {
            framebuffer,
            //text: Text::new(),
            text,
        }
    }
}

impl piet::RenderContext for RenderContext<'_> {
    type Brush = Brush;

    type Text = Text;
    type TextLayout = TextLayout;

    type Image = Image;

    fn status(&mut self) -> Result<()> {
        Ok(())
    }

    fn solid_brush(&mut self, color: Color) -> Brush {
        dbg!();
        Brush::Solid(color.as_rgba_u32())
    }

    fn gradient(&mut self, gradient: impl Into<FixedGradient>) -> Result<Brush> {
        dbg!();
        unimplemented!()
    }

    fn clear(&mut self, color: Color) {
        let position = cgmath::Point2::new(0, 0);
        let size = cgmath::Vector2::new(DISPLAYWIDTH.into(), DISPLAYHEIGHT.into());
        self.framebuffer
            .fill_rect(position, size, to_remarkable_color(color));

        let draw_area = mxcfb_rect::from(position.cast().unwrap(), size);
        let marker = self.framebuffer.partial_refresh(
            &draw_area,
            PartialRefreshMode::Async,
            waveform_mode::WAVEFORM_MODE_GC16_FAST,
            display_temp::TEMP_USE_REMARKABLE_DRAW,
            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
            0,
            false,
        );
        self.framebuffer.wait_refresh_complete(marker);
    }

    fn stroke(&mut self, shape: impl Shape, brush: &impl IntoBrush<Self>, width: f64) {
        dbg!();
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
        dbg!();
        unimplemented!()
    }

    fn fill(&mut self, shape: impl Shape, brush: &impl IntoBrush<Self>) {
        if let Some(rect) = shape.as_line() {
            println!("vmx: shape: line");
        } else if let Some(rect) = shape.as_rect() {
            println!("vmx: shape: rect");
        //   let mut x = svg::node::element::Rectangle::new()
        //       .set("x", rect.origin().x)
        //       .set("y", rect.origin().y)
        //       .set("width", rect.width())
        //       .set("height", rect.height());
        //   attrs.apply_to(&mut x);
        //   node.append(x);
        } else if let Some(rect) = shape.as_rounded_rect() {
            println!("vmx: shape: rounded rect");
        //   let mut x = svg::node::element::Rectangle::new()
        //       .set("x", rect.origin().x)
        //       .set("y", rect.origin().y)
        //       .set("width", rect.width())
        //       .set("height", rect.height())
        //       .set("rx", rect.radius())
        //       .set("ry", rect.radius());
        //   attrs.apply_to(&mut x);
        //   node.append(x);
        } else if let Some(circle) = shape.as_circle() {
            println!("vmx: shape: cirlce");
        //   let mut x = svg::node::element::Circle::new()
        //       .set("cx", circle.center.x)
        //       .set("cy", circle.center.y)
        //       .set("r", circle.radius);
        //   attrs.apply_to(&mut x);
        //   node.append(x);
        } else {
            //println!("vmx: shape: else: {:?}", shape.into_bez_path(1e-3));
            let bezPath = shape.into_bez_path(1e-3);
            println!("vmx: shape: bez: path: {:?}", bezPath);
            //let segments = bezPath.segments().collect::<Vec<_>>();
            for segment in bezPath.segments() {
                let curves = match segment {
                    PathSeg::Line(_) => {
                        panic!("should not happen, should be covered by `as_line()`")
                    }
                    PathSeg::Quad(quad) => vec![quad],
                    PathSeg::Cubic(cubic) => cubic.to_quads(1.0).map(|quad| quad.2).collect(),
                };
                for curve in curves {
                    let startpt = cgmath::Point2 {
                        x: curve.p0.x as f32,
                        y: curve.p0.y as f32,
                    };
                    let ctrlpt = cgmath::Point2 {
                        x: curve.p1.x as f32,
                        y: curve.p2.y as f32,
                    };
                    let endpt = cgmath::Point2 {
                        x: curve.p2.x as f32,
                        y: curve.p1.y as f32,
                    };
                    // TODO vmx 2020-08-28: Use proper brush width
                    let width = 4.5;
                    let samples = 10;
                    // TODO vmx 2020-08-28: Use proper brush color
                    let color = RemarkableColor::BLACK;
                    let draw_area = self
                        .framebuffer
                        .draw_bezier(startpt, ctrlpt, endpt, width, samples, color);

                    let marker = self.framebuffer.partial_refresh(
                        &draw_area,
                        PartialRefreshMode::Async,
                        waveform_mode::WAVEFORM_MODE_GC16_FAST,
                        display_temp::TEMP_USE_REMARKABLE_DRAW,
                        dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
                        0,
                        false,
                    );
                }
            }
        }
    }

    fn fill_even_odd(&mut self, shape: impl Shape, brush: &impl IntoBrush<Self>) {
        dbg!();
        unimplemented!()
    }

    fn clip(&mut self, shape: impl Shape) {
        dbg!();
        unimplemented!()
    }

    fn text(&mut self) -> &mut Self::Text {
        dbg!();
        &mut self.text
    }

    fn draw_text(&mut self, _layout: &Self::TextLayout, _pos: impl Into<Point>) {
        println!("vmx: draw text");
        unimplemented!()
    }

    fn save(&mut self) -> Result<()> {
        dbg!();
        unimplemented!()
    }

    fn restore(&mut self) -> Result<()> {
        dbg!();
        unimplemented!()
    }

    fn finish(&mut self) -> Result<()> {
        dbg!();
        Ok(())
    }

    fn transform(&mut self, transform: Affine) {
        dbg!();
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
        dbg!();
        unimplemented!()
    }

    fn draw_image_area(
        &mut self,
        image: &Self::Image,
        src_rect: impl Into<Rect>,
        dst_rect: impl Into<Rect>,
        interp: InterpolationMode,
    ) {
        dbg!();
        unimplemented!()
    }

    fn blurred_rect(&mut self, _rect: Rect, _blur_radius: f64, _brush: &impl IntoBrush<Self>) {
        dbg!();
        unimplemented!()
    }

    fn current_transform(&self) -> Affine {
        dbg!();
        unimplemented!()
    }
}
