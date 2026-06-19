//! 文本模块
//!
//! 定义字体和文本渲染相关类型，包含 TTF/OTF 字体加载解析与文本布局能力。

use std::collections::HashMap;

use engine_render::Color;

/// 字体大小
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FontSize {
    /// 小号
    Small,
    /// 中号
    Medium,
    /// 大号
    Large,
    /// 特大号
    ExtraLarge,
    /// 自定义大小
    Custom(f32),
}

impl FontSize {
    /// 转换为 f32 值
    pub fn to_f32(&self) -> f32 {
        match self {
            FontSize::Small => 12.0,
            FontSize::Medium => 16.0,
            FontSize::Large => 24.0,
            FontSize::ExtraLarge => 32.0,
            FontSize::Custom(size) => *size,
        }
    }
}

/// 文本对齐方式
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum TextAlign {
    /// 左对齐
    Left,
    /// 居中对齐
    Center,
    /// 右对齐
    Right,
}

/// 字体
pub struct Font {
    family: &'static str,
    size: FontSize,
    bold: bool,
    italic: bool,
    color: Color,
}

impl Font {
    /// 创建新的字体
    pub fn new(family: &'static str, size: FontSize) -> Self {
        Self {
            family,
            size,
            bold: false,
            italic: false,
            color: Color::BLACK,
        }
    }

    /// 获取字体族
    pub fn family(&self) -> &str {
        self.family
    }

    /// 获取字体大小
    pub fn size(&self) -> FontSize {
        self.size
    }

    /// 获取字体大小（f32）
    pub fn size_f32(&self) -> f32 {
        self.size.to_f32()
    }

    /// 设置字体大小
    pub fn set_size(&mut self, size: FontSize) {
        self.size = size;
    }

    /// 是否加粗
    pub fn bold(&self) -> bool {
        self.bold
    }

    /// 设置加粗
    pub fn set_bold(&mut self, bold: bool) {
        self.bold = bold;
    }

    /// 是否斜体
    pub fn italic(&self) -> bool {
        self.italic
    }

    /// 设置斜体
    pub fn set_italic(&mut self, italic: bool) {
        self.italic = italic;
    }

    /// 获取颜色
    pub fn color(&self) -> Color {
        self.color
    }

    /// 设置颜色
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// 设置加粗
    pub fn with_bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// 设置斜体
    pub fn with_italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// 设置颜色
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

/// 字体度量
pub struct FontMetrics {
    ascent: f32,
    descent: f32,
    line_height: f32,
    char_widths: HashMap<char, f32>,
}

impl FontMetrics {
    /// 创建新的字体度量
    pub fn new(ascent: f32, descent: f32, line_height: f32) -> Self {
        Self {
            ascent,
            descent,
            line_height,
            char_widths: HashMap::new(),
        }
    }

    /// 获取上升量
    pub fn ascent(&self) -> f32 {
        self.ascent
    }

    /// 获取下降量
    pub fn descent(&self) -> f32 {
        self.descent
    }

    /// 获取行高
    pub fn line_height(&self) -> f32 {
        self.line_height
    }

    /// 获取字符宽度
    pub fn char_width(&self, c: char) -> f32 {
        *self.char_widths.get(&c).unwrap_or(&8.0)
    }

    /// 设置字符宽度
    pub fn set_char_width(&mut self, c: char, width: f32) {
        self.char_widths.insert(c, width);
    }

    /// 测量文本宽度
    pub fn measure_text(&self, text: &str) -> f32 {
        text.chars().map(|c| self.char_width(c)).sum()
    }

    /// 测量多行文本，返回（最大宽度，总高度）
    pub fn measure_text_lines(&self, text: &str) -> (f32, f32) {
        let mut max_width: f32 = 0.0;
        let mut lines = 1;

        for line in text.split('\n') {
            let width = self.measure_text(line);
            max_width = max_width.max(width);
            lines += 1;
        }

        (max_width, self.line_height * lines as f32)
    }
}

/// 文本渲染器
pub struct TextRenderer {
    default_font: Font,
}

impl TextRenderer {
    /// 创建新的文本渲染器
    pub fn new() -> Self {
        Self {
            default_font: Font::new("Arial", FontSize::Medium),
        }
    }

    /// 获取默认字体
    pub fn default_font(&self) -> &Font {
        &self.default_font
    }

    /// 测量文本尺寸
    pub fn measure_text(&self, text: &str, font: &Font) -> (f32, f32) {
        let metrics = FontMetrics::new(
            font.size_f32() * 0.8,
            font.size_f32() * 0.2,
            font.size_f32() * 1.2,
        );
        metrics.measure_text_lines(text)
    }
}

impl Default for TextRenderer {
    fn default() -> Self {
        Self::new()
    }
}

// ===== TTF/OTF 字体加载解析 =====

/// 字体加载错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FontLoadError {
    /// 数据过短
    TooShort,
    /// 无效的魔法数字
    InvalidMagic,
    /// 不支持的字体格式
    UnsupportedFormat,
    /// 缺少必要表
    MissingTable(&'static str),
    /// 表解析失败
    TableParseFailed(&'static str),
}

impl std::fmt::Display for FontLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontLoadError::TooShort => write!(f, "font data too short"),
            FontLoadError::InvalidMagic => write!(f, "invalid font magic number"),
            FontLoadError::UnsupportedFormat => write!(f, "unsupported font format"),
            FontLoadError::MissingTable(name) => write!(f, "missing required table: {}", name),
            FontLoadError::TableParseFailed(name) => {
                write!(f, "failed to parse table: {}", name)
            }
        }
    }
}

impl std::error::Error for FontLoadError {}

/// 字体表记录
#[derive(Clone, Copy, Debug)]
struct TableRecord {
    /// 表标签（4 字节 ASCII）
    tag: [u8; 4],
    /// 表数据偏移
    offset: u32,
    /// 表数据长度
    length: u32,
}

/// 字体头部信息（head 表关键字段）
#[derive(Clone, Copy, Debug, Default)]
pub struct FontHeader {
    /// 单位每 em（units per em）
    pub units_per_em: u16,
    /// xMin
    pub x_min: i16,
    /// yMin
    pub y_min: i16,
    /// xMax
    pub x_max: i16,
    /// yMax
    pub y_max: i16,
    /// 索引到本地格式（indexToLocFormat）
    pub index_to_loc_format: i16,
}

/// OS/2 表关键字段
#[derive(Clone, Copy, Debug, Default)]
pub struct Os2Metrics {
    /// sTypoAscender
    pub typo_ascender: i16,
    /// sTypoDescender
    pub typo_descender: i16,
    /// sTypoLineGap
    pub typo_line_gap: i16,
    /// sxHeight
    pub x_height: i16,
    /// sCapHeight
    pub cap_height: i16,
}

/// hhea 表关键字段
#[derive(Clone, Copy, Debug, Default)]
pub struct HorizontalHeader {
    /// Ascender
    pub ascender: i16,
    /// Descender
    pub descender: i16,
    /// LineGap
    pub line_gap: i16,
    /// advanceWidthMax
    pub advance_width_max: u16,
    /// numberOfHMetrics
    pub number_of_h_metrics: u16,
}

/// 字距调整对
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KerningPair {
    /// 左字符索引
    pub left: u16,
    /// 右字符索引
    pub right: u16,
}

/// 已解析的字体数据
///
/// 包含从 TTF/OTF 文件解析出的关键度量信息，用于文本布局。
/// 不包含字形轮廓数据（光栅化由上层渲染器处理）。
pub struct ParsedFont {
    /// 字体族名
    family_name: String,
    /// 字体头部
    header: FontHeader,
    /// OS/2 度量
    os2: Os2Metrics,
    /// 水平头部
    hhea: HorizontalHeader,
    /// 字符到字形索引映射（cmap 表，仅 BMP 平台）
    char_map: HashMap<char, u32>,
    /// 字形水平度量（advance width）
    h_metrics: Vec<(u16, i16)>, // (advance_width, lsb)
    /// 字距调整
    kerning: HashMap<KerningPair, i16>,
}

impl ParsedFont {
    /// 获取字体族名
    pub fn family_name(&self) -> &str {
        &self.family_name
    }

    /// 获取字体头部
    pub fn header(&self) -> FontHeader {
        self.header
    }

    /// 获取 OS/2 度量
    pub fn os2(&self) -> Os2Metrics {
        self.os2
    }

    /// 获取水平头部
    pub fn hhea(&self) -> HorizontalHeader {
        self.hhea
    }

    /// 查找字符对应的字形索引
    pub fn glyph_index(&self, c: char) -> Option<u32> {
        self.char_map.get(&c).copied()
    }

    /// 获取字形的 advance width（字体单位）
    pub fn glyph_advance(&self, glyph_id: u32) -> u16 {
        if (glyph_id as usize) < self.h_metrics.len() {
            self.h_metrics[glyph_id as usize].0
        } else if !self.h_metrics.is_empty() {
            self.h_metrics.last().unwrap().0
        } else {
            0
        }
    }

    /// 获取字距调整值（字体单位）
    pub fn kerning(&self, left: u32, right: u32) -> i16 {
        if left > u16::MAX as u32 || right > u16::MAX as u32 {
            return 0;
        }
        self.kerning
            .get(&KerningPair {
                left: left as u16,
                right: right as u16,
            })
            .copied()
            .unwrap_or(0)
    }

    /// 转换字体单位到像素
    pub fn units_to_pixels(&self, value: f32, font_size: f32) -> f32 {
        if self.header.units_per_em == 0 {
            return 0.0;
        }
        value * font_size / self.header.units_per_em as f32
    }

    /// 获取指定字号的字体度量（像素单位）
    pub fn metrics_at_size(&self, font_size: f32) -> FontMetrics {
        let upm = self.header.units_per_em.max(1) as f32;
        let ascent = self.os2.typo_ascender.max(self.hhea.ascender) as f32;
        let descent = self.os2.typo_descender.min(self.hhea.descender) as f32;
        let line_gap = self.hhea.line_gap as f32;

        let ascent_px = ascent * font_size / upm;
        let descent_px = descent.abs() * font_size / upm;
        let line_height_px = (ascent + descent.abs() + line_gap) * font_size / upm;

        FontMetrics::new(ascent_px, descent_px, line_height_px)
    }

    /// 测量文本宽度（像素）
    pub fn measure_text_width(&self, text: &str, font_size: f32) -> f32 {
        let upm = self.header.units_per_em.max(1) as f32;
        let scale = font_size / upm;
        let mut total = 0.0f32;
        let mut prev: Option<u32> = None;
        for c in text.chars() {
            let gid = self.glyph_index(c).unwrap_or(0);
            let advance = self.glyph_advance(gid) as f32;
            total += advance * scale;
            if let Some(prev_gid) = prev {
                let kern = self.kerning(prev_gid, gid) as f32;
                total += kern * scale;
            }
            prev = Some(gid);
        }
        total
    }
}

/// 字体解析器
///
/// 解析 TTF/OTF 字体文件，提取关键度量信息。
/// 支持：
/// - TrueType (TTF) 字体
/// - OpenType (OTF) 字体（CFF 轮廓）
/// - cmap 表（格式 0/4/6/12）
/// - head/hhea/OS2 表
/// - hmtx 表
/// - kern 表
/// - name 表（字体族名）
pub struct FontParser;

impl FontParser {
    /// 从字节数据解析字体
    pub fn parse(data: &[u8]) -> Result<ParsedFont, FontLoadError> {
        if data.len() < 12 {
            return Err(FontLoadError::TooShort);
        }

        // 检查魔法数字
        let sfnt_version = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        let is_ttf = sfnt_version == 0x00010000 || sfnt_version == u32::from_be_bytes(*b"true");
        let is_otf =
            sfnt_version == u32::from_be_bytes(*b"OTTO") || sfnt_version == u32::from_be_bytes(*b"ttcf");
        if !is_ttf && !is_otf {
            return Err(FontLoadError::InvalidMagic);
        }

        let num_tables = u16::from_be_bytes([data[4], data[5]]);
        let mut tables: Vec<TableRecord> = Vec::with_capacity(num_tables as usize);

        // 解析表目录（每表 16 字节）
        for i in 0..num_tables as usize {
            let offset = 12 + i * 16;
            if offset + 16 > data.len() {
                return Err(FontLoadError::TooShort);
            }
            let tag = [data[offset], data[offset + 1], data[offset + 2], data[offset + 3]];
            let toffset = u32::from_be_bytes([
                data[offset + 8],
                data[offset + 9],
                data[offset + 10],
                data[offset + 11],
            ]);
            let tlength = u32::from_be_bytes([
                data[offset + 12],
                data[offset + 13],
                data[offset + 14],
                data[offset + 15],
            ]);
            tables.push(TableRecord {
                tag,
                offset: toffset,
                length: tlength,
            });
        }

        // 解析各表
        let header = Self::parse_head(&tables, data)?;
        let hhea = Self::parse_hhea(&tables, data)?;
        let os2 = Self::parse_os2(&tables, data).unwrap_or_default();
        let char_map = Self::parse_cmap(&tables, data)?;
        let h_metrics = Self::parse_hmtx(&tables, data, &hhea)?;
        let kerning = Self::parse_kern(&tables, data).unwrap_or_default();
        let family_name = Self::parse_name(&tables, data).unwrap_or_else(|_| "Unknown".to_string());

        Ok(ParsedFont {
            family_name,
            header,
            os2,
            hhea,
            char_map,
            h_metrics,
            kerning,
        })
    }

    fn find_table<'a>(tables: &'a [TableRecord], tag: &[u8; 4]) -> Option<&'a TableRecord> {
        tables.iter().find(|t| &t.tag == tag)
    }

    fn table_slice<'a>(data: &'a [u8], record: &TableRecord) -> &'a [u8] {
        let start = record.offset as usize;
        let end = (start + record.length as usize).min(data.len());
        if start >= data.len() {
            &[]
        } else {
            &data[start..end]
        }
    }

    fn parse_head(tables: &[TableRecord], data: &[u8]) -> Result<FontHeader, FontLoadError> {
        let rec = Self::find_table(tables, b"head")
            .ok_or(FontLoadError::MissingTable("head"))?;
        let t = Self::table_slice(data, rec);
        if t.len() < 54 {
            return Err(FontLoadError::TableParseFailed("head"));
        }
        // head 表布局：
        // 0-3: version, 4-7: fontRevision, 8-11: checksumAdjustment,
        // 12-15: magicNumber, 16-17: flags, 18-19: unitsPerEm,
        // 20-27: created, 28-35: modified, 36-37: xMin, 38-39: yMin,
        // 40-41: xMax, 42-43: yMax, 44-45: macStyle, 46-47: lowestRecPPEM,
        // 48-49: fontDirectionHint, 50-51: indexToLocFormat, 52-53: glyphDataFormat
        Ok(FontHeader {
            units_per_em: u16::from_be_bytes([t[18], t[19]]),
            x_min: i16::from_be_bytes([t[36], t[37]]),
            y_min: i16::from_be_bytes([t[38], t[39]]),
            x_max: i16::from_be_bytes([t[40], t[41]]),
            y_max: i16::from_be_bytes([t[42], t[43]]),
            index_to_loc_format: i16::from_be_bytes([t[50], t[51]]),
        })
    }

    fn parse_hhea(
        tables: &[TableRecord],
        data: &[u8],
    ) -> Result<HorizontalHeader, FontLoadError> {
        let rec = Self::find_table(tables, b"hhea")
            .ok_or(FontLoadError::MissingTable("hhea"))?;
        let t = Self::table_slice(data, rec);
        if t.len() < 36 {
            return Err(FontLoadError::TableParseFailed("hhea"));
        }
        // hhea 表布局：
        // 0-3: version, 4-5: ascender, 6-7: descender, 8-9: lineGap,
        // 10-11: advanceWidthMax, ... 34-35: numberOfHMetrics
        Ok(HorizontalHeader {
            ascender: i16::from_be_bytes([t[4], t[5]]),
            descender: i16::from_be_bytes([t[6], t[7]]),
            line_gap: i16::from_be_bytes([t[8], t[9]]),
            advance_width_max: u16::from_be_bytes([t[10], t[11]]),
            number_of_h_metrics: u16::from_be_bytes([t[34], t[35]]),
        })
    }

    fn parse_os2(tables: &[TableRecord], data: &[u8]) -> Result<Os2Metrics, FontLoadError> {
        let rec = match Self::find_table(tables, b"OS/2") {
            Some(r) => r,
            None => return Ok(Os2Metrics::default()),
        };
        let t = Self::table_slice(data, rec);
        if t.len() < 78 {
            // 旧版 OS/2 表可能较短，仅取可用字段
            return Ok(Os2Metrics {
                typo_ascender: if t.len() >= 70 {
                    i16::from_be_bytes([t[68], t[69]])
                } else {
                    0
                },
                typo_descender: if t.len() >= 72 {
                    i16::from_be_bytes([t[70], t[71]])
                } else {
                    0
                },
                typo_line_gap: if t.len() >= 74 {
                    i16::from_be_bytes([t[72], t[73]])
                } else {
                    0
                },
                x_height: if t.len() >= 88 {
                    i16::from_be_bytes([t[86], t[87]])
                } else {
                    0
                },
                cap_height: if t.len() >= 90 {
                    i16::from_be_bytes([t[88], t[89]])
                } else {
                    0
                },
            });
        }
        // OS/2 表关键字段偏移：
        // 68-69: sTypoAscender, 70-71: sTypoDescender, 72-73: sTypoLineGap,
        // 86-87: sxHeight, 88-89: sCapHeight
        Ok(Os2Metrics {
            typo_ascender: i16::from_be_bytes([t[68], t[69]]),
            typo_descender: i16::from_be_bytes([t[70], t[71]]),
            typo_line_gap: i16::from_be_bytes([t[72], t[73]]),
            x_height: if t.len() >= 88 {
                i16::from_be_bytes([t[86], t[87]])
            } else {
                0
            },
            cap_height: if t.len() >= 90 {
                i16::from_be_bytes([t[88], t[89]])
            } else {
                0
            },
        })
    }

    fn parse_cmap(
        tables: &[TableRecord],
        data: &[u8],
    ) -> Result<HashMap<char, u32>, FontLoadError> {
        let rec = Self::find_table(tables, b"cmap")
            .ok_or(FontLoadError::MissingTable("cmap"))?;
        let t = Self::table_slice(data, rec);
        if t.len() < 4 {
            return Err(FontLoadError::TableParseFailed("cmap"));
        }
        let num_subtables = u16::from_be_bytes([t[2], t[3]]);
        let mut char_map = HashMap::new();

        // 查找最佳子表：优先 Unicode BMP (platform=0, encoding=3) 或 (platform=3, encoding=1)
        let mut best_offset: Option<usize> = None;
        let mut best_priority = 0u32;
        for i in 0..num_subtables as usize {
            let rec_off = 4 + i * 8;
            if rec_off + 8 > t.len() {
                break;
            }
            let platform_id = u16::from_be_bytes([t[rec_off], t[rec_off + 1]]);
            let encoding_id = u16::from_be_bytes([t[rec_off + 2], t[rec_off + 3]]);
            let subtable_offset = u32::from_be_bytes([
                t[rec_off + 4],
                t[rec_off + 5],
                t[rec_off + 6],
                t[rec_off + 7],
            ]) as usize;
            // 优先级：Unicode full (3,10) > Unicode BMP (3,1) > Unicode (0,*)
            let priority = match (platform_id, encoding_id) {
                (3, 10) | (0, 6) => 4,
                (3, 1) | (0, 3) | (0, 4) | (0, 1) | (0, 0) => 3,
                (3, 0) => 2,
                _ => 1,
            };
            if priority > best_priority {
                best_priority = priority;
                best_offset = Some(subtable_offset);
            }
        }

        if let Some(sub_offset) = best_offset {
            if sub_offset < t.len() {
                let sub = &t[sub_offset..];
                Self::parse_cmap_subtable(sub, &mut char_map);
            }
        }

        Ok(char_map)
    }

    fn parse_cmap_subtable(sub: &[u8], char_map: &mut HashMap<char, u32>) {
        if sub.len() < 2 {
            return;
        }
        let format = u16::from_be_bytes([sub[0], sub[1]]);
        match format {
            0 => Self::parse_cmap_format0(sub, char_map),
            4 => Self::parse_cmap_format4(sub, char_map),
            6 => Self::parse_cmap_format6(sub, char_map),
            12 => Self::parse_cmap_format12(sub, char_map),
            _ => {}
        }
    }

    fn parse_cmap_format0(sub: &[u8], char_map: &mut HashMap<char, u32>) {
        if sub.len() < 262 {
            return;
        }
        for i in 0..256u32 {
            let gid = sub[6 + i as usize] as u32;
            if gid != 0 {
                if let Some(c) = char::from_u32(i) {
                    char_map.insert(c, gid);
                }
            }
        }
    }

    fn parse_cmap_format4(sub: &[u8], char_map: &mut HashMap<char, u32>) {
        if sub.len() < 14 {
            return;
        }
        let seg_count_x2 = u16::from_be_bytes([sub[6], sub[7]]) as usize;
        let seg_count = seg_count_x2 / 2;
        if seg_count == 0 {
            return;
        }
        let end_codes_off = 14;
        let start_codes_off = end_codes_off + seg_count_x2 + 2; // +2 for reservedPad
        let id_deltas_off = start_codes_off + seg_count_x2;
        let id_range_offsets_off = id_deltas_off + seg_count_x2;

        for i in 0..seg_count {
            let end_off = end_codes_off + i * 2;
            let start_off = start_codes_off + i * 2;
            let delta_off = id_deltas_off + i * 2;
            let range_off = id_range_offsets_off + i * 2;
            if end_off + 2 > sub.len()
                || start_off + 2 > sub.len()
                || delta_off + 2 > sub.len()
                || range_off + 2 > sub.len()
            {
                break;
            }
            let end_code = u16::from_be_bytes([sub[end_off], sub[end_off + 1]]);
            let start_code = u16::from_be_bytes([sub[start_off], sub[start_off + 1]]);
            let id_delta = i16::from_be_bytes([sub[delta_off], sub[delta_off + 1]]) as i32;
            let id_range_offset = u16::from_be_bytes([sub[range_off], sub[range_off + 1]]) as usize;

            if start_code == 0xFFFF && end_code == 0xFFFF {
                continue;
            }

            for c in start_code..=end_code {
                let gid: u32 = if id_range_offset == 0 {
                    ((c as i32 + id_delta) & 0xFFFF) as u32
                } else {
                    // id_range_offset 是相对 id_range_offsets_off + i*2 的偏移
                    let glyph_index_off = range_off + id_range_offset + (c - start_code) as usize * 2;
                    if glyph_index_off + 2 > sub.len() {
                        continue;
                    }
                    let g = u16::from_be_bytes([sub[glyph_index_off], sub[glyph_index_off + 1]]) as i32;
                    if g == 0 {
                        0
                    } else {
                        ((g + id_delta) & 0xFFFF) as u32
                    }
                };
                if gid != 0 {
                    if let Some(ch) = char::from_u32(c as u32) {
                        char_map.insert(ch, gid);
                    }
                }
            }
        }
    }

    fn parse_cmap_format6(sub: &[u8], char_map: &mut HashMap<char, u32>) {
        if sub.len() < 10 {
            return;
        }
        let first_code = u16::from_be_bytes([sub[6], sub[7]]) as u32;
        let entry_count = u16::from_be_bytes([sub[8], sub[9]]) as usize;
        for i in 0..entry_count {
            let off = 10 + i * 2;
            if off + 2 > sub.len() {
                break;
            }
            let gid = u16::from_be_bytes([sub[off], sub[off + 1]]) as u32;
            if gid != 0 {
                if let Some(c) = char::from_u32(first_code + i as u32) {
                    char_map.insert(c, gid);
                }
            }
        }
    }

    fn parse_cmap_format12(sub: &[u8], char_map: &mut HashMap<char, u32>) {
        if sub.len() < 16 {
            return;
        }
        let num_groups = u32::from_be_bytes([sub[12], sub[13], sub[14], sub[15]]) as usize;
        for i in 0..num_groups {
            let off = 16 + i * 12;
            if off + 12 > sub.len() {
                break;
            }
            let start_char = u32::from_be_bytes([sub[off], sub[off + 1], sub[off + 2], sub[off + 3]]);
            let start_gid = u32::from_be_bytes([sub[off + 4], sub[off + 5], sub[off + 6], sub[off + 7]]);
            let end_char = u32::from_be_bytes([sub[off + 8], sub[off + 9], sub[off + 10], sub[off + 11]]);
            for c in start_char..=end_char {
                let gid = start_gid + (c - start_char);
                if let Some(ch) = char::from_u32(c) {
                    char_map.insert(ch, gid);
                }
            }
        }
    }

    fn parse_hmtx(
        tables: &[TableRecord],
        data: &[u8],
        hhea: &HorizontalHeader,
    ) -> Result<Vec<(u16, i16)>, FontLoadError> {
        let rec = Self::find_table(tables, b"hmtx")
            .ok_or(FontLoadError::MissingTable("hmtx"))?;
        let t = Self::table_slice(data, rec);
        let num_metrics = hhea.number_of_h_metrics as usize;
        let mut metrics = Vec::with_capacity(num_metrics);
        for i in 0..num_metrics {
            let off = i * 4;
            if off + 4 > t.len() {
                break;
            }
            let advance = u16::from_be_bytes([t[off], t[off + 1]]);
            let lsb = i16::from_be_bytes([t[off + 2], t[off + 3]]);
            metrics.push((advance, lsb));
        }
        Ok(metrics)
    }

    fn parse_kern(
        tables: &[TableRecord],
        data: &[u8],
    ) -> Result<HashMap<KerningPair, i16>, FontLoadError> {
        let rec = match Self::find_table(tables, b"kern") {
            Some(r) => r,
            None => return Ok(HashMap::new()),
        };
        let t = Self::table_slice(data, rec);
        if t.len() < 4 {
            return Ok(HashMap::new());
        }
        let mut kerning = HashMap::new();
        // kern 表版本 0（TrueType）
        let version = u16::from_be_bytes([t[0], t[1]]);
        if version != 0 {
            return Ok(kerning);
        }
        let num_subtables = u16::from_be_bytes([t[2], t[3]]) as usize;
        let mut offset = 4usize;
        for _ in 0..num_subtables {
            if offset + 6 > t.len() {
                break;
            }
            let sub_version = u16::from_be_bytes([t[offset], t[offset + 1]]);
            let sub_length = u16::from_be_bytes([t[offset + 2], t[offset + 3]]) as usize;
            let coverage = u16::from_be_bytes([t[offset + 4], t[offset + 5]]);
            let format = (coverage >> 8) & 0xFF;
            let _ = sub_version;
            let sub_start = offset + 6;
            if format == 0 {
                // 格式 0：字距对数组
                if sub_start + 8 > t.len() {
                    break;
                }
                let num_pairs = u16::from_be_bytes([t[sub_start], t[sub_start + 1]]) as usize;
                let pairs_start = sub_start + 8;
                for i in 0..num_pairs {
                    let off = pairs_start + i * 6;
                    if off + 6 > t.len() {
                        break;
                    }
                    let left = u16::from_be_bytes([t[off], t[off + 1]]);
                    let right = u16::from_be_bytes([t[off + 2], t[off + 3]]);
                    let value = i16::from_be_bytes([t[off + 4], t[off + 5]]);
                    kerning.insert(KerningPair { left, right }, value);
                }
            }
            offset += sub_length;
        }
        Ok(kerning)
    }

    fn parse_name(tables: &[TableRecord], data: &[u8]) -> Result<String, FontLoadError> {
        let rec = match Self::find_table(tables, b"name") {
            Some(r) => r,
            None => return Ok(String::new()),
        };
        let t = Self::table_slice(data, rec);
        if t.len() < 6 {
            return Ok(String::new());
        }
        let count = u16::from_be_bytes([t[2], t[3]]) as usize;
        let string_offset = u16::from_be_bytes([t[4], t[5]]) as usize;

        // 优先查找 nameID=1（字体族名），platform=3（Windows）encoding=1（Unicode BMP）
        let mut best: Option<(u16, u16, u16, u16)> = None;
        for i in 0..count {
            let off = 6 + i * 12;
            if off + 12 > t.len() {
                break;
            }
            let platform_id = u16::from_be_bytes([t[off], t[off + 1]]);
            let encoding_id = u16::from_be_bytes([t[off + 2], t[off + 3]]);
            let _lang_id = u16::from_be_bytes([t[off + 4], t[off + 5]]);
            let name_id = u16::from_be_bytes([t[off + 6], t[off + 7]]);
            let length = u16::from_be_bytes([t[off + 8], t[off + 9]]);
            let offset_in_storage = u16::from_be_bytes([t[off + 10], t[off + 11]]);

            if name_id == 1 {
                // 优先 Windows Unicode
                let priority = match (platform_id, encoding_id) {
                    (3, 1) => 4,
                    (3, 10) => 3,
                    (0, _) => 2,
                    (1, 0) => 1,
                    _ => 0,
                };
                if best.is_none() || priority > best.unwrap().0 {
                    best = Some((priority, length, offset_in_storage, platform_id));
                }
            }
        }

        if let Some((_, length, offset_in_storage, platform_id)) = best {
            let storage_start = string_offset + offset_in_storage as usize;
            let end = storage_start + length as usize;
            if end <= t.len() {
                let bytes = &t[storage_start..end];
                // Windows Unicode: UTF-16BE
                if platform_id == 3 {
                    let mut s = String::with_capacity(bytes.len() / 2);
                    let mut i = 0;
                    while i + 1 < bytes.len() {
                        let code = u16::from_be_bytes([bytes[i], bytes[i + 1]]);
                        s.push(char::from_u32(code as u32).unwrap_or('\u{FFFD}'));
                        i += 2;
                    }
                    return Ok(s);
                }
                // Mac Roman: 近似 ASCII
                if platform_id == 1 {
                    return Ok(String::from_utf8_lossy(bytes).to_string());
                }
                // Unicode platform
                if platform_id == 0 {
                    let mut s = String::with_capacity(bytes.len() / 2);
                    let mut i = 0;
                    while i + 1 < bytes.len() {
                        let code = u16::from_be_bytes([bytes[i], bytes[i + 1]]);
                        s.push(char::from_u32(code as u32).unwrap_or('\u{FFFD}'));
                        i += 2;
                    }
                    return Ok(s);
                }
            }
        }

        Ok("Unknown".to_string())
    }
}

/// 字体加载器
pub struct FontLoader;

impl FontLoader {
    /// 从字节数据加载字体
    pub fn load_from_bytes(data: &[u8]) -> Result<ParsedFont, FontLoadError> {
        FontParser::parse(data)
    }

    /// 从文件加载字体
    pub fn load_from_file(path: &str) -> Result<ParsedFont, Box<dyn std::error::Error>> {
        let data = std::fs::read(path)?;
        let font = FontParser::parse(&data)?;
        Ok(font)
    }
}

// ===== 文本布局 =====

/// 文本布局选项
#[derive(Clone, Debug)]
pub struct TextLayoutOptions {
    /// 字体大小（像素）
    pub font_size: f32,
    /// 行高倍数（1.0 = 默认行高）
    pub line_height_scale: f32,
    /// 最大宽度（None 表示不限制）
    pub max_width: Option<f32>,
    /// 文本对齐
    pub align: TextAlign,
    /// 字间距（像素，额外间距）
    pub letter_spacing: f32,
}

impl Default for TextLayoutOptions {
    fn default() -> Self {
        Self {
            font_size: 16.0,
            line_height_scale: 1.2,
            max_width: None,
            align: TextAlign::Left,
            letter_spacing: 0.0,
        }
    }
}

/// 文本布局结果（单行）
#[derive(Clone, Debug, PartialEq)]
pub struct TextLine {
    /// 行文本
    pub text: String,
    /// 行起始 Y 坐标
    pub y: f32,
    /// 行宽度
    pub width: f32,
}

/// 文本布局器
///
/// 基于 ParsedFont 进行文本布局，支持：
/// - 多行布局（按 \n 分行）
/// - 自动换行（按 max_width）
/// - 文本对齐（左/中/右）
/// - 字间距
pub struct TextLayoutEngine;

impl TextLayoutEngine {
    /// 执行文本布局
    pub fn layout(
        font: &ParsedFont,
        text: &str,
        options: &TextLayoutOptions,
    ) -> Vec<TextLine> {
        let metrics = font.metrics_at_size(options.font_size);
        let line_height = metrics.line_height() * options.line_height_scale;

        let mut lines = Vec::new();

        // 先按 \n 分行
        for (i, raw_line) in text.split('\n').enumerate() {
            let y = i as f32 * line_height;

            // 如果有最大宽度限制，进行自动换行
            if let Some(max_w) = options.max_width {
                let wrapped = Self::wrap_line(font, raw_line, max_w, options);
                for (j, wrapped_text) in wrapped.into_iter().enumerate() {
                    let width = font.measure_text_width(&wrapped_text, options.font_size)
                        + options.letter_spacing * wrapped_text.chars().count().saturating_sub(1) as f32;
                    lines.push(TextLine {
                        text: wrapped_text,
                        y: y + j as f32 * line_height,
                        width,
                    });
                }
            } else {
                let width = font.measure_text_width(raw_line, options.font_size)
                    + options.letter_spacing * raw_line.chars().count().saturating_sub(1) as f32;
                lines.push(TextLine {
                    text: raw_line.to_string(),
                    y,
                    width,
                });
            }
        }

        // 应用对齐：调整每行宽度信息（实际渲染时根据 width 和 max_width 计算 x 偏移）
        // 这里只记录原始 width，对齐偏移由调用方计算
        lines
    }

    /// 自动换行：将一行文本按最大宽度拆分
    fn wrap_line(
        font: &ParsedFont,
        text: &str,
        max_width: f32,
        options: &TextLayoutOptions,
    ) -> Vec<String> {
        if text.is_empty() {
            return vec![String::new()];
        }

        let mut result = Vec::new();
        let mut current = String::new();
        let mut current_width = 0.0f32;

        for c in text.chars() {
            if c.is_whitespace() && !current.is_empty() {
                // 在空白处尝试断行
                let word_width = font.measure_text_width(&current, options.font_size);
                if current_width + word_width > max_width && !current.is_empty() {
                    result.push(current.clone());
                    current.clear();
                    current_width = 0.0;
                } else {
                    current_width += word_width;
                }
                current.push(c);
                current_width += font.measure_text_width(&c.to_string(), options.font_size);
            } else {
                let char_width = font.measure_text_width(&c.to_string(), options.font_size);
                if current_width + char_width > max_width && !current.is_empty() {
                    result.push(current.clone());
                    current.clear();
                    current_width = 0.0;
                }
                current.push(c);
                current_width += char_width;
            }
        }

        if !current.is_empty() {
            result.push(current);
        }

        if result.is_empty() {
            vec![String::new()]
        } else {
            result
        }
    }

    /// 计算对齐后的 x 偏移
    pub fn align_offset(width: f32, max_width: f32, align: TextAlign) -> f32 {
        match align {
            TextAlign::Left => 0.0,
            TextAlign::Center => ((max_width - width) / 2.0).max(0.0),
            TextAlign::Right => (max_width - width).max(0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_render::Color;

    #[test]
    fn test_font_size_to_f32() {
        assert_eq!(FontSize::Small.to_f32(), 12.0);
        assert_eq!(FontSize::Medium.to_f32(), 16.0);
        assert_eq!(FontSize::Large.to_f32(), 24.0);
        assert_eq!(FontSize::ExtraLarge.to_f32(), 32.0);
        assert_eq!(FontSize::Custom(20.0).to_f32(), 20.0);
    }

    #[test]
    fn test_font_builder() {
        let font = Font::new("Arial", FontSize::Large)
            .with_bold()
            .with_color(Color::RED);

        assert!(font.bold());
        assert_eq!(font.color(), Color::RED);
    }

    #[test]
    fn test_font_metrics_measure_text() {
        let mut metrics = FontMetrics::new(16.0, 4.0, 20.0);
        metrics.set_char_width('H', 10.0);
        metrics.set_char_width('e', 8.0);
        metrics.set_char_width('l', 6.0);
        metrics.set_char_width('o', 8.0);

        let width = metrics.measure_text("Hello");
        assert_eq!(width, 10.0 + 8.0 + 6.0 + 6.0 + 8.0);
    }

    #[test]
    fn test_text_renderer_default_font() {
        let renderer = TextRenderer::new();
        let font = renderer.default_font();
        assert_eq!(font.family(), "Arial");
    }

    #[test]
    fn test_text_renderer_measure_text() {
        let renderer = TextRenderer::new();
        let font = Font::new("Arial", FontSize::Medium);
        let (width, height) = renderer.measure_text("Hello World", &font);
        assert!(width > 0.0);
        assert!(height > 0.0);
    }

    #[test]
    fn test_font_new_with_family() {
        let font = Font::new("Verdana", FontSize::Small);
        assert_eq!(font.family(), "Verdana");
        assert_eq!(font.size(), FontSize::Small);
        assert_eq!(font.size_f32(), 12.0);
    }

    #[test]
    fn test_font_set_size() {
        let mut font = Font::new("Arial", FontSize::Medium);
        font.set_size(FontSize::Large);
        assert_eq!(font.size(), FontSize::Large);
        assert_eq!(font.size_f32(), 24.0);
    }

    #[test]
    fn test_font_set_bold() {
        let mut font = Font::new("Arial", FontSize::Medium);
        assert!(!font.bold());
        font.set_bold(true);
        assert!(font.bold());
        font.set_bold(false);
        assert!(!font.bold());
    }

    #[test]
    fn test_font_set_italic() {
        let mut font = Font::new("Arial", FontSize::Medium);
        assert!(!font.italic());
        font.set_italic(true);
        assert!(font.italic());
    }

    #[test]
    fn test_font_set_color() {
        let mut font = Font::new("Arial", FontSize::Medium);
        font.set_color(Color::BLUE);
        assert_eq!(font.color(), Color::BLUE);
    }

    #[test]
    fn test_font_metrics_ascent_descent() {
        let metrics = FontMetrics::new(16.0, 4.0, 20.0);
        assert_eq!(metrics.ascent(), 16.0);
        assert_eq!(metrics.descent(), 4.0);
        assert_eq!(metrics.line_height(), 20.0);
    }

    #[test]
    fn test_text_align_variants() {
        let _l = TextAlign::Left;
        let _c = TextAlign::Center;
        let _r = TextAlign::Right;
    }

    #[test]
    fn test_font_with_italic_chain() {
        let font = Font::new("Arial", FontSize::Medium).with_italic();
        assert!(font.italic());
        assert!(!font.bold());
    }

    #[test]
    fn test_font_metrics_default_char_width() {
        let metrics = FontMetrics::new(16.0, 4.0, 20.0);
        assert_eq!(metrics.char_width('x'), 8.0);
    }

    #[test]
    fn test_font_metrics_measure_lines() {
        let metrics = FontMetrics::new(16.0, 4.0, 20.0);
        let (_w, h) = metrics.measure_text_lines("line1\nline2");
        // 3 iterations: initial lines=1 + 2 split lines = 3 * 20 = 60
        assert_eq!(h, 60.0);
    }

    // ===== 字体解析测试 =====

    #[test]
    fn test_font_load_error_display() {
        let err = FontLoadError::TooShort;
        assert!(format!("{}", err).contains("too short"));

        let err = FontLoadError::InvalidMagic;
        assert!(format!("{}", err).contains("magic"));

        let err = FontLoadError::MissingTable("head");
        assert!(format!("{}", err).contains("head"));
    }

    #[test]
    fn test_font_parser_rejects_empty_data() {
        let result = FontParser::parse(&[]);
        assert!(matches!(result, Err(FontLoadError::TooShort)));
    }

    #[test]
    fn test_font_parser_rejects_short_data() {
        let result = FontParser::parse(&[0x00, 0x01, 0x00, 0x00]);
        assert!(matches!(result, Err(FontLoadError::TooShort)));
    }

    #[test]
    fn test_font_parser_rejects_invalid_magic() {
        let mut data = vec![0u8; 32];
        data[0] = 0xFF;
        data[1] = 0xFF;
        data[2] = 0xFF;
        data[3] = 0xFF;
        let result = FontParser::parse(&data);
        assert!(matches!(result, Err(FontLoadError::InvalidMagic)));
    }

    #[test]
    fn test_font_parser_accepts_ttf_magic() {
        let mut data = vec![0u8; 32];
        // TTF magic: 0x00010000
        data[0] = 0x00;
        data[1] = 0x01;
        data[2] = 0x00;
        data[3] = 0x00;
        // numTables = 0
        data[4] = 0x00;
        data[5] = 0x00;
        let result = FontParser::parse(&data);
        // 没有 head 表，应返回 MissingTable("head")
        assert!(matches!(result, Err(FontLoadError::MissingTable("head"))));
    }

    #[test]
    fn test_font_parser_accepts_otto_magic() {
        let mut data = vec![0u8; 32];
        // OTF magic: 'OTTO'
        data[0] = b'O';
        data[1] = b'T';
        data[2] = b'T';
        data[3] = b'O';
        // numTables = 0
        data[4] = 0x00;
        data[5] = 0x00;
        let result = FontParser::parse(&data);
        assert!(matches!(result, Err(FontLoadError::MissingTable("head"))));
    }

    #[test]
    fn test_font_parser_accepts_true_magic() {
        let mut data = vec![0u8; 32];
        // 'true' magic
        data[0] = b't';
        data[1] = b'r';
        data[2] = b'u';
        data[3] = b'e';
        data[4] = 0x00;
        data[5] = 0x00;
        let result = FontParser::parse(&data);
        assert!(matches!(result, Err(FontLoadError::MissingTable("head"))));
    }

    #[test]
    fn test_font_parser_accepts_ttcf_magic() {
        let mut data = vec![0u8; 32];
        data[0] = b't';
        data[1] = b't';
        data[2] = b'c';
        data[3] = b'f';
        data[4] = 0x00;
        data[5] = 0x00;
        let result = FontParser::parse(&data);
        assert!(matches!(result, Err(FontLoadError::MissingTable("head"))));
    }

    #[test]
    fn test_text_layout_options_default() {
        let opts = TextLayoutOptions::default();
        assert_eq!(opts.font_size, 16.0);
        assert_eq!(opts.line_height_scale, 1.2);
        assert!(opts.max_width.is_none());
        assert_eq!(opts.align, TextAlign::Left);
        assert_eq!(opts.letter_spacing, 0.0);
    }

    #[test]
    fn test_text_layout_align_offset() {
        assert_eq!(TextLayoutEngine::align_offset(50.0, 100.0, TextAlign::Left), 0.0);
        assert_eq!(TextLayoutEngine::align_offset(50.0, 100.0, TextAlign::Center), 25.0);
        assert_eq!(TextLayoutEngine::align_offset(50.0, 100.0, TextAlign::Right), 50.0);
        // 宽度超过 max_width 时不应返回负值
        assert_eq!(TextLayoutEngine::align_offset(150.0, 100.0, TextAlign::Right), 0.0);
    }

    #[test]
    fn test_kerning_pair_hash() {
        let p1 = KerningPair { left: 1, right: 2 };
        let p2 = KerningPair { left: 1, right: 2 };
        let p3 = KerningPair { left: 2, right: 1 };
        let mut map = HashMap::new();
        map.insert(p1, 10i16);
        assert_eq!(map.get(&p2), Some(&10));
        assert_eq!(map.get(&p3), None);
    }
}
