use std::collections::{HashMap, HashSet};
use chrono::NaiveDate;
use language_tags::LanguageTag;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Book {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<LanguageTag>,
    pub short_title: String,
    pub date: Date,
    pub authors: Vec<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<InlineImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Annotation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub epigraphs: Vec<Epigraph>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Footnotes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Footnotes>,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Date {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_date: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub id: Uuid,
    pub full_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Footnotes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    pub content: HashMap<String, Footnote>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Footnote {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    pub content: Vec<Content>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Chapter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Annotation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<Image>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub epigraphs: Vec<Epigraph>,
    pub content: Vec<Content>,
    pub sub_chapters: Vec<Chapter>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Content {
    Paragraph(Paragraph),
    Poem(Poem),
    Subtitle(Paragraph),
    Cite(Cite),
    Table(Table),
    Image(Image),
    EmptyLine,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Annotation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    pub content: Vec<AnnotationElement>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AnnotationElement {
    Paragraph(Paragraph),
    Poem(Poem),
    Cite(Cite),
    Subtitle(Paragraph),
    Table(Table),
    EmptyLine,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Epigraph {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<Paragraph>,
    pub content: Vec<EpigraphElement>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EpigraphElement {
    Paragraph(Paragraph),
    Poem(Poem),
    Cite(Cite),
    EmptyLine,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Poem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub epigraphs: Vec<Epigraph>,
    // TODO: exclude images, something else?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<Paragraph>,
    pub content: Vec<PoemElement>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PoemElement {
    Subtitle(Paragraph),
    Stanza(Stanza),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stanza {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    // TODO: exclude images, something else?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<Paragraph>,
    // TODO: exclude images, something else?
    pub content: Vec<Paragraph>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    // TODO: exclude images, something else?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<Paragraph>,
    pub content: Vec<CiteElement>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CiteElement {
    Paragraph(Paragraph),
    Poem(Poem),
    Subtitle(Paragraph),
    Table(Table),
    EmptyLine,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Title {
    pub content: Vec<TitleElement>,
}

// TODO: exclude images and other non-title items
#[derive(Debug, Deserialize, Serialize)]
pub enum TitleElement {
    Paragraph(Paragraph),
    EmptyLine,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Paragraph {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    pub content: Vec<Span>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Table {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    pub header_column: bool,
    pub header_row: bool,
    pub rows: Vec<TableRow>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TableCell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    pub content: Vec<Span>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Span {
    Footnote(FootnoteLink),
    Link(Link),
    Image(InlineImage),
    Text(Text),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FootnoteLink {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: FootnoteKind,
    pub content: Vec<Text>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum FootnoteKind {
    Note,
    Comment,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    pub href: Href,
    pub content: Vec<Text>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Href {
    Remote(Url),
    Local(String),
}

impl AsRef<str> for Href {
    fn as_ref(&self) -> &str {
        match self {
            Href::Remote(url) => url.as_str(),
            Href::Local(id) => id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineImage {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Text {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<u16>,
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub font_style: HashSet<FontStyle>,
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub decorations: HashSet<TextDecoration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_shift: Option<BaselineShift>,
    pub value: String,
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Text {
            font_weight: None,
            font_style: HashSet::new(),
            decorations: HashSet::new(),
            baseline_shift: None,
            value,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum FontStyle {
    Italic,
    Code,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum TextDecoration {
    LineThrough,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum BaselineShift {
    Subscript,
    Superscript,
}
