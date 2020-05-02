//! Wrappers around CF/CT types, with nice interfaces.

use std::ops::Deref;

use core_foundation::{
    array::{CFArray, CFArrayRef},
    attributed_string::CFMutableAttributedString,
    base::TCFType,
    dictionary::CFDictionaryRef,
    number::CFNumber,
    string::CFString,
};
use core_foundation_sys::base::CFRange;
use core_graphics::{
    base::CGFloat,
    geometry::{CGPoint, CGSize},
    path::CGPathRef,
};
use core_text::{
    font::CTFont,
    frame::{CTFrame, CTFrameRef},
    framesetter::{CTFramesetter, CTFramesetterRef},
    line::{CTLine, CTLineRef},
    string_attributes,
};

#[derive(Clone)]
pub(crate) struct AttributedString(pub(crate) CFMutableAttributedString);
#[derive(Debug, Clone)]
pub(crate) struct Framesetter(CTFramesetter);
#[derive(Debug, Clone)]
pub(crate) struct Frame(pub(crate) CTFrame);
#[derive(Debug, Clone)]
pub(crate) struct Line<'a>(pub(crate) &'a CTLine);

#[derive(Default, Debug, Copy, Clone)]
pub(crate) struct TypographicBounds {
    pub(crate) width: CGFloat,
    pub(crate) ascent: CGFloat,
    pub(crate) descent: CGFloat,
    pub(crate) leading: CGFloat,
}

impl AttributedString {
    pub(crate) fn new(text: &str, font: &CTFont) -> Self {
        let mut string = CFMutableAttributedString::new();
        let range = CFRange::init(0, 0);
        string.replace_str(&CFString::new(text), range);

        let str_len = string.char_len();
        let char_range = CFRange::init(0, str_len);

        unsafe {
            string.set_attribute(
                char_range,
                string_attributes::kCTFontAttributeName,
                font.clone(),
            );
            string.set_attribute::<CFNumber>(
                char_range,
                string_attributes::kCTForegroundColorFromContextAttributeName,
                1i32.into(),
            );
        }
        AttributedString(string)
    }

    pub(crate) fn range(&self) -> CFRange {
        CFRange::init(0, self.0.char_len())
    }
}

impl Framesetter {
    pub(crate) fn new(attributed_string: &AttributedString) -> Self {
        Framesetter(CTFramesetter::new_with_attributed_string(
            attributed_string.0.as_concrete_TypeRef(),
        ))
    }

    /// returns the suggested size and the range of the string that fits.
    pub(crate) fn suggest_frame_size(
        &self,
        range: CFRange,
        constraints: CGSize,
    ) -> (CGSize, CFRange) {
        unsafe {
            let mut fit_range = CFRange::init(0, 0);
            let size = CTFramesetterSuggestFrameSizeWithConstraints(
                self.0.as_concrete_TypeRef(),
                range,
                std::ptr::null(),
                constraints,
                &mut fit_range,
            );
            (size, fit_range)
        }
    }

    pub(crate) fn create_frame(&self, range: CFRange, path: &CGPathRef) -> Frame {
        Frame(self.0.create_frame(range, path))
    }
}

impl Frame {
    pub(crate) fn get_lines(&self) -> CFArray<CTLine> {
        unsafe { TCFType::wrap_under_get_rule(CTFrameGetLines(self.0.as_concrete_TypeRef())) }
    }

    pub(crate) fn get_line_origins(&self, range: CFRange) -> Vec<CGPoint> {
        let mut origins = vec![CGPoint::new(0.0, 0.0); range.length as usize];
        unsafe {
            CTFrameGetLineOrigins(self.0.as_concrete_TypeRef(), range, origins.as_mut_ptr());
        }
        origins
    }
}

impl<'a> Line<'a> {
    pub(crate) fn new(inner: &'a impl Deref<Target = CTLine>) -> Line<'a> {
        Line(inner.deref())
    }

    pub(crate) fn get_string_range(&self) -> CFRange {
        unsafe { CTLineGetStringRange(self.0.as_concrete_TypeRef()) }
    }

    pub(crate) fn get_typographic_bounds(&self) -> TypographicBounds {
        let mut out = TypographicBounds::default();
        let width = unsafe {
            CTLineGetTypographicBounds(
                self.0.as_concrete_TypeRef(),
                &mut out.ascent,
                &mut out.descent,
                &mut out.leading,
            )
        };
        out.width = width;
        out
    }
}

#[link(name = "CoreText", kind = "framework")]
extern "C" {
    fn CTFramesetterSuggestFrameSizeWithConstraints(
        framesetter: CTFramesetterRef,
        string_range: CFRange,
        frame_attributes: CFDictionaryRef,
        constraints: CGSize,
        fitRange: *mut CFRange,
    ) -> CGSize;

    fn CTFrameGetLines(frame: CTFrameRef) -> CFArrayRef;
    fn CTFrameGetLineOrigins(frame: CTFrameRef, range: CFRange, origins: *mut CGPoint);

    fn CTLineGetStringRange(line: CTLineRef) -> CFRange;
    fn CTLineGetTypographicBounds(
        line: CTLineRef,
        ascent: *mut CGFloat,
        descent: *mut CGFloat,
        leading: *mut CGFloat,
    ) -> CGFloat;
}