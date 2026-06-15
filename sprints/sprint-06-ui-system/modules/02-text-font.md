# 文本与字体系统需求

## 模块概述

文本与字体系统模块负责字体加载、字形渲染、文本布局和富文本支持。该模块基于 `engine-ui` crate，提供完整的文本渲染能力，包括 TTF/OTF 字体加载、字形图集构建、多行文本布局以及简化版富文本功能。

---

## 需求清单

### 字体加载

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 31 | `Font`：字体加载 TTF/OTF | P0 |
| 55 | `Font::from_file(path) -> Result<Font>` | P0 |
| 161 | `Font::load(path)` 从 TTF 加载 | P0 |
| 162 | `Font::load_from_bytes(bytes)` 从内存加载 | P1 |
| 163 | `Font::name(&self) -> &str` | P2 |
| 164 | `Font::has_glyph(ch)` | P1 |
| 165 | `Font::line_height(size)` | P1 |
| 56 | `Font::get_glyph(ch, size)` 获取字形 | P0 |
| 166 | `Font::get_glyph(ch, size) -> Glyph` | P0 |
| 57 | `Font::get_kerning(a, b)` | P1 |
| 88 | `Font::measure(text, size) -> Vec2` | P0 |

### 字形与字形图集

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 35 | `Glyph`：字形索引 + 位置 + 大小 | P0 |
| 58 | `FontAtlas`：字体纹理图集 | P0 |
| 59 | `TextLayout`：字/行/段落布局 | P0 |
| 60 | `Text2d`：组件封装（简化版文本绘制） | P1 |
| 89 | `FontAtlasBuilder::new()` + `add(font, size, chars)` + `build() -> FontAtlas` | P0 |
| 114 | `FontAtlasBuilder::new()` + `add(font, size, chars)` + `build() -> FontAtlas` | P0 |
| 90 | `FontAtlas::texture() -> TextureHandle` | P0 |
| 115 | `FontAtlas::texture() -> TextureHandle` | P0 |
| 91 | `FontAtlas::get_uv(ch)` 返回字形 UV | P0 |
| 116 | `FontAtlas::get_uv(ch) -> Option<Rect>` | P0 |
| 92 | `FontAtlas::get_kerning(a, b)` 返回字距 | P1 |
| 117 | `FontAtlas::get_kerning(a, b)` 返回字距 | P1 |
| 164 | `FontAtlasBuilder::add_font(font, size, charset)` | P0 |
| 204 | `FontAtlasBuilder::new()` | P0 |
| 205 | `FontAtlasBuilder::add_font(font, size, charset)` | P0 |
| 206 | `FontAtlasBuilder::build(&self, ctx) -> Result<FontAtlas>` | P0 |
| 207 | `FontAtlas::texture(&self) -> TextureHandle` | P0 |
| 208 | `FontAtlas::get_uv(ch) -> Option<Rect>` | P0 |
| 209 | `FontAtlas::get_glyph(ch) -> Option<Glyph>` | P0 |
| 210 | `FontAtlas::font_size(&self) -> f32` | P1 |

### 文本布局

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 93 | `TextLayout::new(font, size, text, width, align)` | P0 |
| 118 | `TextLayout::new(font, size, text, width, align)` | P0 |
| 94 | `TextLayout::glyphs() -> &[Glyph]` | P0 |
| 119 | `TextLayout::glyphs() -> &[Glyph]` | P0 |
| 95 | `TextLayout::lines() -> &[Line]` | P0 |
| 120 | `TextLayout::lines() -> &[Line]` | P0 |
| 96 | `TextLayout::size() -> Vec2` | P0 |
| 121 | `TextLayout::size() -> Vec2` | P0 |
| 175 | `TextLayout::new(font, size, text, max_width, align)` | P0 |
| 176 | `TextLayout::glyphs(&self) -> &[Glyph]` | P0 |
| 177 | `TextLayout::lines(&self) -> &[Line]` | P0 |
| 178 | `TextLayout::size(&self) -> Vec2` | P0 |
| 179 | `TextLayout::char_index_at(pos) -> usize` | P1 |
| 180 | `TextLayout::line_wrap: Wrap / Clip / Ellipsis` | P1 |

### 文本枚举与结构

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 38 | `FontWeight`：Thin/ExtraLight/Light/Normal/Medium/SemiBold/Bold/ExtraBold/Black | P1 |
| 39 | `FontStyle`：Normal / Italic / Oblique | P1 |
| 40 | `TextAlign`：Left / Center / Right / Justify | P0 |
| 41 | `TextOverflow`：Wrap / Clip / Ellipsis | P1 |
| 61 | `RichText`：样式段（颜色、字号、字重） | P1 |
| 62 | `TextSection`：文本段 | P1 |

### 富文本

| 需求ID | 描述 | 优先级 |
|--------|------|--------|
| 97 | 富文本：`RichText` + `TextSection` | P1 |
| 98 | 富文本：颜色段、大小段、字体段 | P1 |
| 99 | 富文本：支持 \n 换行 | P1 |
| 122 | 富文本：`RichText` + `TextSection` | P1 |
| 123 | 富文本：颜色段、大小段、字体段 | P1 |
| 124 | 富文本：支持 \n 换行 | P1 |
| 181 | `RichText::new()` | P1 |
| 182 | `RichText::push(section)` | P1 |
| 183 | `TextSection::new(text, style)` | P1 |
| 184 | `TextSection::with_color(color)` | P1 |
| 185 | `TextSection::with_size(size)` | P1 |
| 186 | `TextSection::with_bold()` | P1 |
| 187 | `TextSection::with_italic()` | P1 |
| 188 | `TextSection::with_font(font_handle)` | P1 |
| 189 | `RichTextLayout`：计算富文本 | P1 |
| 190 | `Glyph::uv_rect / position / size` 公开字段 | P0 |

---

## API 签名

### Font

```rust
pub struct Font {
    // 内部数据
}

impl Font {
    pub fn from_file(path: &str) -> Result<Font>;
    pub fn load(path: &str) -> Result<Font>;
    pub fn load_from_bytes(bytes: &[u8]) -> Result<Font>;
    pub fn name(&self) -> &str;
    pub fn has_glyph(&self, ch: char) -> bool;
    pub fn line_height(&self, size: f32) -> f32;
    pub fn get_glyph(&self, ch: char, size: f32) -> Glyph;
    pub fn get_kerning(&self, a: char, b: char) -> f32;
    pub fn measure(&self, text: &str, size: f32) -> Vec2;
}
```

### Glyph

```rust
pub struct Glyph {
    pub index: u32,
    pub position: Vec2,
    pub size: Vec2,
    pub uv_rect: Rect,
}
```

### FontAtlasBuilder

```rust
pub struct FontAtlasBuilder {
    // 内部数据
}

impl FontAtlasBuilder {
    pub fn new() -> Self;
    pub fn add_font(&mut self, font: &Font, size: f32, charset: &str);
    pub fn build(&self, ctx: &mut Context) -> Result<FontAtlas>;
}
```

### FontAtlas

```rust
pub struct FontAtlas {
    // 内部数据
}

impl FontAtlas {
    pub fn texture(&self) -> TextureHandle;
    pub fn get_uv(&self, ch: char) -> Option<Rect>;
    pub fn get_glyph(&self, ch: char) -> Option<Glyph>;
    pub fn font_size(&self) -> f32;
    pub fn get_kerning(&self, a: char, b: char) -> f32;
}
```

### TextLayout

```rust
pub struct TextLayout {
    // 内部数据
}

impl TextLayout {
    pub fn new(
        font: &Font,
        size: f32,
        text: &str,
        max_width: f32,
        align: TextAlign,
    ) -> Self;
    pub fn glyphs(&self) -> &[Glyph];
    pub fn lines(&self) -> &[Line];
    pub fn size(&self) -> Vec2;
    pub fn char_index_at(&self, pos: Vec2) -> usize;
}
```

### RichText

```rust
pub struct RichText {
    sections: Vec<TextSection>,
}

impl RichText {
    pub fn new() -> Self;
    pub fn push(&mut self, section: TextSection);
}
```

### TextSection

```rust
pub struct TextSection {
    pub text: String,
    pub color: Option<Color>,
    pub size: Option<f32>,
    pub bold: bool,
    pub italic: bool,
    pub font: Option<Handle<Font>>,
}

impl TextSection {
    pub fn new(text: &str, style: &TextStyle) -> Self;
    pub fn with_color(mut self, color: Color) -> Self;
    pub fn with_size(mut self, size: f32) -> Self;
    pub fn with_bold(mut self) -> Self;
    pub fn with_italic(mut self) -> Self;
    pub fn with_font(mut self, font: Handle<Font>) -> Self;
}
```

### 枚举类型

```rust
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

pub enum TextOverflow {
    Wrap,
    Clip,
    Ellipsis,
}
```

---

## 输入/输出

| 模块 | 输入 | 输出 |
|------|------|------|
| Font::from_file | 字体文件路径 | Result<Font> |
| Font::measure | 文本字符串、字号 | Vec2 (宽高) |
| FontAtlasBuilder::build | 字体、字号、字符集 | Result<FontAtlas> |
| FontAtlas::get_uv | 字符 | Option<Rect> |
| TextLayout::new | 字体、字号、文本、最大宽度、对齐方式 | TextLayout 实例 |
| TextLayout::glyphs | - | &[Glyph] |
| RichText::push | TextSection | - |

---

## 验收标准

- [ ] `Font::from_file` 可正确加载 TTF 字体文件
- [ ] `Font::measure` 返回的尺寸与实际渲染尺寸一致
- [ ] `FontAtlasBuilder::build` 成功生成字形图集纹理
- [ ] `FontAtlas::get_uv` 对所有预渲染字符返回正确 UV
- [ ] `TextLayout::glyphs` 返回正确的字形位置序列
- [ ] `TextLayout::lines` 正确处理换行
- [ ] `RichText` 支持多样式文本段
- [ ] 富文本正确处理 \n 换行

---

## 依赖关系

- 依赖 `engine-render` crate（纹理、材质）
- 依赖 `engine-assets` crate（字体资源加载）
- 被 `UiText` 组件依赖

---

## 优先级说明

- **P0**：核心字体渲染缺失会导致文本无法显示
- **P1**：重要增强功能，影响用户体验
- **P2**：辅助功能，可后续补充
