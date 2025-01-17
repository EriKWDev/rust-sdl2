/* automatically generated by rust-bindgen */

use crate::*;

pub const TTF_MAJOR_VERSION: u32 = 2;
pub const TTF_MINOR_VERSION: u32 = 0;
pub const TTF_PATCHLEVEL: u32 = 15;
pub const TTF_STYLE_NORMAL: u32 = 0;
pub const TTF_STYLE_BOLD: u32 = 1;
pub const TTF_STYLE_ITALIC: u32 = 2;
pub const TTF_STYLE_UNDERLINE: u32 = 4;
pub const TTF_STYLE_STRIKETHROUGH: u32 = 8;
pub const TTF_HINTING_NORMAL: u32 = 0;
pub const TTF_HINTING_LIGHT: u32 = 1;
pub const TTF_HINTING_MONO: u32 = 2;
pub const TTF_HINTING_NONE: u32 = 3;
pub type __uint8_t = core::ffi::c_uchar;
pub type __uint16_t = core::ffi::c_ushort;
pub type __uint32_t = core::ffi::c_uint;
pub type __int64_t = core::ffi::c_long;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
pub type Uint8 = u8;
pub type Uint16 = u16;
pub type Uint32 = u32;
pub type Sint64 = i64;
extern "C" {
    pub fn TTF_Linked_Version() -> *const SDL_version;
}
extern "C" {
    pub fn TTF_ByteSwappedUNICODE(swapped: core::ffi::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _TTF_Font {
    _unused: [u8; 0],
}
pub type TTF_Font = _TTF_Font;
extern "C" {
    pub fn TTF_Init() -> core::ffi::c_int;
}
extern "C" {
    pub fn TTF_OpenFont(file: *const core::ffi::c_char, ptsize: core::ffi::c_int) -> *mut TTF_Font;
}
extern "C" {
    pub fn TTF_OpenFontIndex(
        file: *const core::ffi::c_char,
        ptsize: core::ffi::c_int,
        index: core::ffi::c_long,
    ) -> *mut TTF_Font;
}
extern "C" {
    pub fn TTF_OpenFontRW(
        src: *mut SDL_RWops,
        freesrc: core::ffi::c_int,
        ptsize: core::ffi::c_int,
    ) -> *mut TTF_Font;
}
extern "C" {
    pub fn TTF_OpenFontIndexRW(
        src: *mut SDL_RWops,
        freesrc: core::ffi::c_int,
        ptsize: core::ffi::c_int,
        index: core::ffi::c_long,
    ) -> *mut TTF_Font;
}
extern "C" {
    pub fn TTF_GetFontStyle(font: *const TTF_Font) -> core::ffi::c_int;
}
extern "C" {
    pub fn TTF_SetFontStyle(font: *mut TTF_Font, style: core::ffi::c_int);
}
extern "C" {
    pub fn TTF_GetFontOutline(font: *const TTF_Font) -> core::ffi::c_int;
}
extern "C" {
    pub fn TTF_SetFontOutline(font: *mut TTF_Font, outline: core::ffi::c_int);
}
extern "C" {
    pub fn TTF_GetFontHinting(font: *const TTF_Font) -> core::ffi::c_int;
}
extern "C" {
    pub fn TTF_SetFontHinting(font: *mut TTF_Font, hinting: core::ffi::c_int);
}
extern "C" {
    pub fn TTF_FontHeight(font: *const TTF_Font) -> core::ffi::c_int;
}
extern "C" {
    pub fn TTF_FontAscent(font: *const TTF_Font) -> core::ffi::c_int;
}
extern "C" {
    pub fn TTF_FontDescent(font: *const TTF_Font) -> core::ffi::c_int;
}
extern "C" {
    pub fn TTF_FontLineSkip(font: *const TTF_Font) -> core::ffi::c_int;
}
extern "C" {
    pub fn TTF_GetFontKerning(font: *const TTF_Font) -> core::ffi::c_int;
}
extern "C" {
    pub fn TTF_SetFontKerning(font: *mut TTF_Font, allowed: core::ffi::c_int);
}
extern "C" {
    pub fn TTF_FontFaces(font: *const TTF_Font) -> core::ffi::c_long;
}
extern "C" {
    pub fn TTF_FontFaceIsFixedWidth(font: *const TTF_Font) -> core::ffi::c_int;
}
extern "C" {
    pub fn TTF_FontFaceFamilyName(font: *const TTF_Font) -> *mut core::ffi::c_char;
}
extern "C" {
    pub fn TTF_FontFaceStyleName(font: *const TTF_Font) -> *mut core::ffi::c_char;
}
extern "C" {
    pub fn TTF_GlyphIsProvided(font: *const TTF_Font, ch: Uint16) -> core::ffi::c_int;
}
extern "C" {
    pub fn TTF_GlyphMetrics(
        font: *mut TTF_Font,
        ch: Uint16,
        minx: *mut core::ffi::c_int,
        maxx: *mut core::ffi::c_int,
        miny: *mut core::ffi::c_int,
        maxy: *mut core::ffi::c_int,
        advance: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
}
extern "C" {
    pub fn TTF_SizeText(
        font: *mut TTF_Font,
        text: *const core::ffi::c_char,
        w: *mut core::ffi::c_int,
        h: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
}
extern "C" {
    pub fn TTF_SizeUTF8(
        font: *mut TTF_Font,
        text: *const core::ffi::c_char,
        w: *mut core::ffi::c_int,
        h: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
}
extern "C" {
    pub fn TTF_SizeUNICODE(
        font: *mut TTF_Font,
        text: *const Uint16,
        w: *mut core::ffi::c_int,
        h: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
}
extern "C" {
    pub fn TTF_RenderText_Solid(
        font: *mut TTF_Font,
        text: *const core::ffi::c_char,
        fg: SDL_Color,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn TTF_RenderUTF8_Solid(
        font: *mut TTF_Font,
        text: *const core::ffi::c_char,
        fg: SDL_Color,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn TTF_RenderUNICODE_Solid(
        font: *mut TTF_Font,
        text: *const Uint16,
        fg: SDL_Color,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn TTF_RenderGlyph_Solid(
        font: *mut TTF_Font,
        ch: Uint16,
        fg: SDL_Color,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn TTF_RenderText_Shaded(
        font: *mut TTF_Font,
        text: *const core::ffi::c_char,
        fg: SDL_Color,
        bg: SDL_Color,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn TTF_RenderUTF8_Shaded(
        font: *mut TTF_Font,
        text: *const core::ffi::c_char,
        fg: SDL_Color,
        bg: SDL_Color,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn TTF_RenderUNICODE_Shaded(
        font: *mut TTF_Font,
        text: *const Uint16,
        fg: SDL_Color,
        bg: SDL_Color,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn TTF_RenderGlyph_Shaded(
        font: *mut TTF_Font,
        ch: Uint16,
        fg: SDL_Color,
        bg: SDL_Color,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn TTF_RenderText_Blended(
        font: *mut TTF_Font,
        text: *const core::ffi::c_char,
        fg: SDL_Color,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn TTF_RenderUTF8_Blended(
        font: *mut TTF_Font,
        text: *const core::ffi::c_char,
        fg: SDL_Color,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn TTF_RenderUNICODE_Blended(
        font: *mut TTF_Font,
        text: *const Uint16,
        fg: SDL_Color,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn TTF_RenderText_Blended_Wrapped(
        font: *mut TTF_Font,
        text: *const core::ffi::c_char,
        fg: SDL_Color,
        wrapLength: Uint32,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn TTF_RenderUTF8_Blended_Wrapped(
        font: *mut TTF_Font,
        text: *const core::ffi::c_char,
        fg: SDL_Color,
        wrapLength: Uint32,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn TTF_RenderUNICODE_Blended_Wrapped(
        font: *mut TTF_Font,
        text: *const Uint16,
        fg: SDL_Color,
        wrapLength: Uint32,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn TTF_RenderGlyph_Blended(
        font: *mut TTF_Font,
        ch: Uint16,
        fg: SDL_Color,
    ) -> *mut SDL_Surface;
}
extern "C" {
    pub fn TTF_CloseFont(font: *mut TTF_Font);
}
extern "C" {
    pub fn TTF_Quit();
}
extern "C" {
    pub fn TTF_WasInit() -> core::ffi::c_int;
}
extern "C" {
    pub fn TTF_GetFontKerningSize(
        font: *mut TTF_Font,
        prev_index: core::ffi::c_int,
        index: core::ffi::c_int,
    ) -> core::ffi::c_int;
}
extern "C" {
    pub fn TTF_GetFontKerningSizeGlyphs(
        font: *mut TTF_Font,
        previous_ch: Uint16,
        ch: Uint16,
    ) -> core::ffi::c_int;
}
