use enum_iterator::{all, Sequence};
use lol_html::html_content::Element;

#[derive(Copy, Clone)]
pub struct Tag {
    pub name: &'static str,
    pub index: usize,
    // Note that while lol_html::Element has `.is_self_closing()`, it's not
    // accurate. For example, `<br>` is self-closing, but not explicitly as such (`<br/>`),
    // so it's not considered self-closing by lol_html. We need to know this when removing
    // tags and text, so we need to track it ourselves.
    pub self_closing: bool,
}

#[derive(Copy, Clone, Sequence)]
pub enum HTMLTag {
    HTML = 0,
    HEAD,
    TITLE,
    BASE,
    LINK,
    META,
    STYLE,
    SCRIPT,
    NOSCRIPT,
    TEMPLATE,
    BODY,
    ARTICLE,
    SECTION,
    NAV,
    ASIDE,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    HGROUP,
    HEADER,
    FOOTER,
    ADDRESS,
    P,
    HR,
    PRE,
    BLOCKQUOTE,
    OL,
    UL,
    LI,
    DL,
    DT,
    DD,
    FIGURE,
    FIGCAPTION,
    MAIN,
    DIV,
    A,
    EM,
    STRONG,
    SMALL,
    S,
    CITE,
    Q,
    DFN,
    ABBR,
    DATA,
    TIME,
    CODE,
    VAR,
    SAMP,
    KBD,
    SUB,
    SUP,
    I,
    B,
    U,
    MARK,
    RUBY,
    RT,
    RP,
    BDI,
    BDO,
    SPAN,
    BR,
    WBR,
    INS,
    DEL,
    IMAGE,
    IMG,
    IFRAME,
    EMBED,
    OBJECT,
    PARAM,
    VIDEO,
    AUDIO,
    SOURCE,
    TRACK,
    CANVAS,
    MAP,
    AREA,
    MATH,
    MI,
    MO,
    MN,
    MS,
    MTEXT,
    MGLYPH,
    MALIGNMARK,
    ANNOTATION, // Gumbo calls this "GUMBO_TAG_ANNOTATION_XML"
    SVG,
    FOREIGNOBJECT,
    DESC,
    TABLE,
    CAPTION,
    COLGROUP,
    COL,
    TBODY,
    THEAD,
    TFOOT,
    TR,
    TD,
    TH,
    FORM,
    FIELDSET,
    LEGEND,
    LABEL,
    INPUT,
    BUTTON,
    SELECT,
    DATALIST,
    OPTGROUP,
    OPTION,
    TEXTAREA,
    KEYGEN,
    OUTPUT,
    PROGRESS,
    METER,
    DETAILS,
    SUMMARY,
    MENU,
    MENUITEM,
    APPLET,
    ACRONYM,
    BGSOUND,
    DIR,
    FRAME,
    FRAMESET,
    NOFRAMES,
    LISTING,
    XMP,
    NEXTID,
    NOEMBED,
    PLAINTEXT,
    RB,
    STRIKE,
    BASEFONT,
    BIG,
    BLINK,
    CENTER,
    FONT,
    MARQUEE,
    MULTICOL,
    NOBR,
    SPACER,
    TT,
    RTC,
    DIALOG,
    UNKNOWN,
}

impl Tag {
    pub const TAG_COUNT: usize = 151;

    /// Identifies whether this is an HTML tag whose contents
    /// are considered "text nodes", and thus, must be removed
    pub fn has_text_content(tag: Tag) -> bool {
        tag.index == HTMLTag::SCRIPT as usize
            || tag.index == HTMLTag::STYLE as usize
            || tag.index == HTMLTag::MATH as usize
            || tag.index == HTMLTag::SVG as usize
    }

    /// Is this tag an `<iframe>`?
    pub fn is_iframe(tag: Tag) -> bool {
        tag.index == HTMLTag::IFRAME as usize
    }

    /// Is this tag a `<meta>`?
    pub fn is_meta(tag: Tag) -> bool {
        tag.index == HTMLTag::META as usize
    }

    /// Is this tag something which needs to be removed?
    pub fn is_tag_escapeworthy(tag: Tag) -> bool {
        tag.index == HTMLTag::TITLE as usize
            || tag.index == HTMLTag::IFRAME as usize
            || tag.index == HTMLTag::MATH as usize
            || tag.index == HTMLTag::NOEMBED as usize
            || tag.index == HTMLTag::NOFRAMES as usize
            || tag.index == HTMLTag::NOSCRIPT as usize
            || tag.index == HTMLTag::PLAINTEXT as usize
            || tag.index == HTMLTag::SCRIPT as usize
            || tag.index == HTMLTag::STYLE as usize
            || tag.index == HTMLTag::SVG as usize
            || tag.index == HTMLTag::TEXTAREA as usize
            || tag.index == HTMLTag::XMP as usize
    }

    pub const ESCAPEWORTHY_TAGS_CSS: &str =
        "title, textarea, style, xmp, iframe, noembed, noframes, script, plaintext";

    pub fn html_tags() -> Vec<HTMLTag> {
        all::<HTMLTag>().collect::<Vec<_>>()
    }

    pub fn tag_from_element(element: &mut Element) -> Tag {
        Self::tag_from_tag_name(element.tag_name().to_lowercase().as_str())
    }

    pub fn tag_from_tag_name(tag_name: &str) -> Tag {
        {
            match tag_name {
                "html" => Tag {
                    name: "html",
                    index: 0,
                    self_closing: false,
                },
                "head" => Tag {
                    name: "head",
                    index: 1,
                    self_closing: false,
                },
                "title" => Tag {
                    name: "title",
                    index: 2,
                    self_closing: false,
                },
                "base" => Tag {
                    name: "base",
                    index: 3,
                    self_closing: true,
                },
                "link" => Tag {
                    name: "link",
                    index: 4,
                    self_closing: true,
                },
                "meta" => Tag {
                    name: "meta",
                    index: 5,
                    self_closing: true,
                },
                "style" => Tag {
                    name: "style",
                    index: 6,
                    self_closing: false,
                },
                "script" => Tag {
                    name: "script",
                    index: 7,
                    self_closing: false,
                },
                "noscript" => Tag {
                    name: "noscript",
                    index: 8,
                    self_closing: false,
                },
                "template" => Tag {
                    name: "template",
                    index: 9,
                    self_closing: false,
                },
                "body" => Tag {
                    name: "body",
                    index: 10,
                    self_closing: false,
                },
                "article" => Tag {
                    name: "article",
                    index: 11,
                    self_closing: false,
                },
                "section" => Tag {
                    name: "section",
                    index: 12,
                    self_closing: false,
                },
                "nav" => Tag {
                    name: "nav",
                    index: 13,
                    self_closing: false,
                },
                "aside" => Tag {
                    name: "aside",
                    index: 14,
                    self_closing: false,
                },
                "h1" => Tag {
                    name: "h1",
                    index: 15,
                    self_closing: false,
                },
                "h2" => Tag {
                    name: "h2",
                    index: 16,
                    self_closing: false,
                },
                "h3" => Tag {
                    name: "h3",
                    index: 17,
                    self_closing: false,
                },
                "h4" => Tag {
                    name: "h4",
                    index: 18,
                    self_closing: false,
                },
                "h5" => Tag {
                    name: "h5",
                    index: 19,
                    self_closing: false,
                },
                "h6" => Tag {
                    name: "h6",
                    index: 20,
                    self_closing: false,
                },
                "hgroup" => Tag {
                    name: "hgroup",
                    index: 21,
                    self_closing: false,
                },
                "header" => Tag {
                    name: "header",
                    index: 22,
                    self_closing: false,
                },
                "footer" => Tag {
                    name: "footer",
                    index: 23,
                    self_closing: false,
                },
                "address" => Tag {
                    name: "address",
                    index: 24,
                    self_closing: false,
                },
                "p" => Tag {
                    name: "p",
                    index: 25,
                    self_closing: false,
                },
                "hr" => Tag {
                    name: "hr",
                    index: 26,
                    self_closing: true,
                },
                "pre" => Tag {
                    name: "pre",
                    index: 27,
                    self_closing: false,
                },
                "blockquote" => Tag {
                    name: "blockquote",
                    index: 28,
                    self_closing: false,
                },
                "ol" => Tag {
                    name: "ol",
                    index: 29,
                    self_closing: false,
                },
                "ul" => Tag {
                    name: "ul",
                    index: 30,
                    self_closing: false,
                },
                "li" => Tag {
                    name: "li",
                    index: 31,
                    self_closing: false,
                },
                "dl" => Tag {
                    name: "dl",
                    index: 32,
                    self_closing: false,
                },
                "dt" => Tag {
                    name: "dt",
                    index: 33,
                    self_closing: false,
                },
                "dd" => Tag {
                    name: "dd",
                    index: 34,
                    self_closing: false,
                },
                "figure" => Tag {
                    name: "figure",
                    index: 35,
                    self_closing: false,
                },
                "figcaption" => Tag {
                    name: "figcaption",
                    index: 36,
                    self_closing: false,
                },
                "main" => Tag {
                    name: "main",
                    index: 37,
                    self_closing: false,
                },
                "div" => Tag {
                    name: "div",
                    index: 38,
                    self_closing: false,
                },
                "a" => Tag {
                    name: "a",
                    index: 39,
                    self_closing: false,
                },
                "em" => Tag {
                    name: "em",
                    index: 40,
                    self_closing: false,
                },
                "strong" => Tag {
                    name: "strong",
                    index: 41,
                    self_closing: false,
                },
                "small" => Tag {
                    name: "small",
                    index: 42,
                    self_closing: false,
                },
                "s" => Tag {
                    name: "s",
                    index: 43,
                    self_closing: false,
                },
                "cite" => Tag {
                    name: "cite",
                    index: 44,
                    self_closing: false,
                },
                "q" => Tag {
                    name: "q",
                    index: 45,
                    self_closing: false,
                },
                "dfn" => Tag {
                    name: "dfn",
                    index: 46,
                    self_closing: false,
                },
                "abbr" => Tag {
                    name: "abbr",
                    index: 47,
                    self_closing: false,
                },
                "data" => Tag {
                    name: "data",
                    index: 48,
                    self_closing: false,
                },
                "time" => Tag {
                    name: "time",
                    index: 49,
                    self_closing: false,
                },
                "code" => Tag {
                    name: "code",
                    index: 50,
                    self_closing: false,
                },
                "var" => Tag {
                    name: "var",
                    index: 51,
                    self_closing: false,
                },
                "samp" => Tag {
                    name: "samp",
                    index: 52,
                    self_closing: false,
                },
                "kbd" => Tag {
                    name: "kbd",
                    index: 53,
                    self_closing: false,
                },
                "sub" => Tag {
                    name: "sub",
                    index: 54,
                    self_closing: false,
                },
                "sup" => Tag {
                    name: "sup",
                    index: 55,
                    self_closing: false,
                },
                "i" => Tag {
                    name: "i",
                    index: 56,
                    self_closing: false,
                },
                "b" => Tag {
                    name: "b",
                    index: 57,
                    self_closing: false,
                },
                "u" => Tag {
                    name: "u",
                    index: 58,
                    self_closing: false,
                },
                "mark" => Tag {
                    name: "mark",
                    index: 59,
                    self_closing: false,
                },
                "ruby" => Tag {
                    name: "ruby",
                    index: 60,
                    self_closing: false,
                },
                "rt" => Tag {
                    name: "rt",
                    index: 61,
                    self_closing: false,
                },
                "rp" => Tag {
                    name: "rp",
                    index: 62,
                    self_closing: false,
                },
                "bdi" => Tag {
                    name: "bdi",
                    index: 63,
                    self_closing: false,
                },
                "bdo" => Tag {
                    name: "bdo",
                    index: 64,
                    self_closing: false,
                },
                "span" => Tag {
                    name: "span",
                    index: 65,
                    self_closing: false,
                },
                "br" => Tag {
                    name: "br",
                    index: 66,
                    self_closing: true,
                },
                "wbr" => Tag {
                    name: "wbr",
                    index: 67,
                    self_closing: true,
                },
                "ins" => Tag {
                    name: "ins",
                    index: 68,
                    self_closing: false,
                },
                "del" => Tag {
                    name: "del",
                    index: 69,
                    self_closing: false,
                },
                "image" => Tag {
                    name: "image",
                    index: 70,
                    self_closing: false,
                },
                "img" => Tag {
                    name: "img",
                    index: 71,
                    self_closing: true,
                },
                "iframe" => Tag {
                    name: "iframe",
                    index: 72,
                    self_closing: false,
                },
                "embed" => Tag {
                    name: "embed",
                    index: 73,
                    self_closing: true,
                },
                "object" => Tag {
                    name: "object",
                    index: 74,
                    self_closing: false,
                },
                "param" => Tag {
                    name: "param",
                    index: 75,
                    self_closing: true,
                },
                "video" => Tag {
                    name: "video",
                    index: 76,
                    self_closing: false,
                },
                "audio" => Tag {
                    name: "audio",
                    index: 77,
                    self_closing: false,
                },
                "source" => Tag {
                    name: "source",
                    index: 78,
                    self_closing: true,
                },
                "track" => Tag {
                    name: "track",
                    index: 79,
                    self_closing: true,
                },
                "canvas" => Tag {
                    name: "canvas",
                    index: 80,
                    self_closing: false,
                },
                "map" => Tag {
                    name: "map",
                    index: 81,
                    self_closing: false,
                },
                "area" => Tag {
                    name: "area",
                    index: 82,
                    self_closing: true,
                },
                "math" => Tag {
                    name: "math",
                    index: 83,
                    self_closing: false,
                },
                "mi" => Tag {
                    name: "mi",
                    index: 84,
                    self_closing: false,
                },
                "mo" => Tag {
                    name: "mo",
                    index: 85,
                    self_closing: false,
                },
                "mn" => Tag {
                    name: "mn",
                    index: 86,
                    self_closing: false,
                },
                "ms" => Tag {
                    name: "ms",
                    index: 87,
                    self_closing: false,
                },
                "mtext" => Tag {
                    name: "mtext",
                    index: 88,
                    self_closing: false,
                },
                "mglyph" => Tag {
                    name: "mglyph",
                    index: 89,
                    self_closing: false,
                },
                "malignmark" => Tag {
                    name: "malignmark",
                    index: 90,
                    self_closing: false,
                },
                "annotation" => Tag {
                    name: "annotation",
                    index: 91,
                    self_closing: false,
                },
                "svg" => Tag {
                    name: "svg",
                    index: 92,
                    self_closing: false,
                },
                "foreignobject" => Tag {
                    name: "foreignobject",
                    index: 93,
                    self_closing: false,
                },
                "desc" => Tag {
                    name: "desc",
                    index: 94,
                    self_closing: false,
                },
                "table" => Tag {
                    name: "table",
                    index: 95,
                    self_closing: false,
                },
                "caption" => Tag {
                    name: "caption",
                    index: 96,
                    self_closing: false,
                },
                "colgroup" => Tag {
                    name: "colgroup",
                    index: 97,
                    self_closing: false,
                },
                "col" => Tag {
                    name: "col",
                    index: 98,
                    self_closing: true,
                },
                "tbody" => Tag {
                    name: "tbody",
                    index: 99,
                    self_closing: false,
                },
                "thead" => Tag {
                    name: "thead",
                    index: 100,
                    self_closing: false,
                },
                "tfoot" => Tag {
                    name: "tfoot",
                    index: 101,
                    self_closing: false,
                },
                "tr" => Tag {
                    name: "tr",
                    index: 102,
                    self_closing: false,
                },
                "td" => Tag {
                    name: "td",
                    index: 103,
                    self_closing: false,
                },
                "th" => Tag {
                    name: "th",
                    index: 104,
                    self_closing: false,
                },
                "form" => Tag {
                    name: "form",
                    index: 105,
                    self_closing: false,
                },
                "fieldset" => Tag {
                    name: "fieldset",
                    index: 106,
                    self_closing: false,
                },
                "legend" => Tag {
                    name: "legend",
                    index: 107,
                    self_closing: false,
                },
                "label" => Tag {
                    name: "label",
                    index: 108,
                    self_closing: false,
                },
                "input" => Tag {
                    name: "input",
                    index: 109,
                    self_closing: true,
                },
                "button" => Tag {
                    name: "button",
                    index: 110,
                    self_closing: false,
                },
                "select" => Tag {
                    name: "select",
                    index: 111,
                    self_closing: false,
                },
                "datalist" => Tag {
                    name: "datalist",
                    index: 112,
                    self_closing: false,
                },
                "optgroup" => Tag {
                    name: "optgroup",
                    index: 113,
                    self_closing: false,
                },
                "option" => Tag {
                    name: "option",
                    index: 114,
                    self_closing: false,
                },
                "textarea" => Tag {
                    name: "textarea",
                    index: 115,
                    self_closing: false,
                },
                "keygen" => Tag {
                    name: "keygen",
                    index: 116,
                    self_closing: true,
                },
                "output" => Tag {
                    name: "output",
                    index: 117,
                    self_closing: false,
                },
                "progress" => Tag {
                    name: "progress",
                    index: 118,
                    self_closing: false,
                },
                "meter" => Tag {
                    name: "meter",
                    index: 119,
                    self_closing: false,
                },
                "details" => Tag {
                    name: "details",
                    index: 120,
                    self_closing: false,
                },
                "summary" => Tag {
                    name: "summary",
                    index: 121,
                    self_closing: false,
                },
                "menu" => Tag {
                    name: "menu",
                    index: 122,
                    self_closing: false,
                },
                "menuitem" => Tag {
                    name: "menuitem",
                    index: 123,
                    self_closing: true,
                },
                "applet" => Tag {
                    name: "applet",
                    index: 124,
                    self_closing: false,
                },
                "acronym" => Tag {
                    name: "acronym",
                    index: 125,
                    self_closing: false,
                },
                "bgsound" => Tag {
                    name: "bgsound",
                    index: 126,
                    self_closing: true,
                },
                "dir" => Tag {
                    name: "dir",
                    index: 127,
                    self_closing: false,
                },
                "frame" => Tag {
                    name: "frame",
                    index: 128,
                    self_closing: true,
                },
                "frameset" => Tag {
                    name: "frameset",
                    index: 129,
                    self_closing: false,
                },
                "noframes" => Tag {
                    name: "noframes",
                    index: 130,
                    self_closing: false,
                },
                "listing" => Tag {
                    name: "listing",
                    index: 131,
                    self_closing: false,
                },
                "xmp" => Tag {
                    name: "xmp",
                    index: 132,
                    self_closing: false,
                },
                "nextid" => Tag {
                    name: "nextid",
                    index: 133,
                    self_closing: false,
                },
                "noembed" => Tag {
                    name: "noembed",
                    index: 134,
                    self_closing: false,
                },
                "plaintext" => Tag {
                    name: "plaintext",
                    index: 135,
                    self_closing: false,
                },
                "rb" => Tag {
                    name: "rb",
                    index: 136,
                    self_closing: false,
                },
                "strike" => Tag {
                    name: "strike",
                    index: 137,
                    self_closing: false,
                },
                "basefont" => Tag {
                    name: "basefont",
                    index: 138,
                    self_closing: true,
                },
                "big" => Tag {
                    name: "big",
                    index: 139,
                    self_closing: false,
                },
                "blink" => Tag {
                    name: "blink",
                    index: 140,
                    self_closing: false,
                },
                "center" => Tag {
                    name: "center",
                    index: 141,
                    self_closing: false,
                },
                "font" => Tag {
                    name: "font",
                    index: 142,
                    self_closing: false,
                },
                "marquee" => Tag {
                    name: "marquee",
                    index: 143,
                    self_closing: false,
                },
                "multicol" => Tag {
                    name: "multicol",
                    index: 144,
                    self_closing: false,
                },
                "nobr" => Tag {
                    name: "nobr",
                    index: 145,
                    self_closing: false,
                },
                "spacer" => Tag {
                    name: "spacer",
                    index: 146,
                    self_closing: false,
                },
                "tt" => Tag {
                    name: "tt",
                    index: 147,
                    self_closing: false,
                },
                "rtc" => Tag {
                    name: "rtc",
                    index: 148,
                    self_closing: false,
                },
                "dialog" => Tag {
                    name: "dialog",
                    index: 149,
                    self_closing: false,
                },
                &_ => Tag {
                    name: "unknown",
                    index: 150,
                    self_closing: false,
                },
            }
        }
    }

    pub fn element_name_from_enum(html_tag: &HTMLTag) -> &'static str {
        match html_tag {
            HTMLTag::HTML => "html",
            HTMLTag::HEAD => "head",
            HTMLTag::TITLE => "title",
            HTMLTag::BASE => "base",
            HTMLTag::LINK => "link",
            HTMLTag::META => "meta",
            HTMLTag::STYLE => "style",
            HTMLTag::SCRIPT => "script",
            HTMLTag::NOSCRIPT => "noscript",
            HTMLTag::TEMPLATE => "template",
            HTMLTag::BODY => "body",
            HTMLTag::ARTICLE => "article",
            HTMLTag::SECTION => "section",
            HTMLTag::NAV => "nav",
            HTMLTag::ASIDE => "aside",
            HTMLTag::H1 => "h1",
            HTMLTag::H2 => "h2",
            HTMLTag::H3 => "h3",
            HTMLTag::H4 => "h4",
            HTMLTag::H5 => "h5",
            HTMLTag::H6 => "h6",
            HTMLTag::HGROUP => "hgroup",
            HTMLTag::HEADER => "header",
            HTMLTag::FOOTER => "footer",
            HTMLTag::ADDRESS => "address",
            HTMLTag::P => "p",
            HTMLTag::HR => "hr",
            HTMLTag::PRE => "pre",
            HTMLTag::BLOCKQUOTE => "blockquote",
            HTMLTag::OL => "ol",
            HTMLTag::UL => "ul",
            HTMLTag::LI => "li",
            HTMLTag::DL => "dl",
            HTMLTag::DT => "dt",
            HTMLTag::DD => "dd",
            HTMLTag::FIGURE => "figure",
            HTMLTag::FIGCAPTION => "figcaption",
            HTMLTag::MAIN => "main",
            HTMLTag::DIV => "div",
            HTMLTag::A => "a",
            HTMLTag::EM => "em",
            HTMLTag::STRONG => "strong",
            HTMLTag::SMALL => "small",
            HTMLTag::S => "s",
            HTMLTag::CITE => "cite",
            HTMLTag::Q => "q",
            HTMLTag::DFN => "dfn",
            HTMLTag::ABBR => "abbr",
            HTMLTag::DATA => "data",
            HTMLTag::TIME => "time",
            HTMLTag::CODE => "code",
            HTMLTag::VAR => "var",
            HTMLTag::SAMP => "samp",
            HTMLTag::KBD => "kbd",
            HTMLTag::SUB => "sub",
            HTMLTag::SUP => "sup",
            HTMLTag::I => "i",
            HTMLTag::B => "b",
            HTMLTag::U => "u",
            HTMLTag::MARK => "mark",
            HTMLTag::RUBY => "ruby",
            HTMLTag::RT => "rt",
            HTMLTag::RP => "rp",
            HTMLTag::BDI => "bdi",
            HTMLTag::BDO => "bdo",
            HTMLTag::SPAN => "span",
            HTMLTag::BR => "br",
            HTMLTag::WBR => "wbr",
            HTMLTag::INS => "ins",
            HTMLTag::DEL => "del",
            HTMLTag::IMAGE => "image",
            HTMLTag::IMG => "img",
            HTMLTag::IFRAME => "iframe",
            HTMLTag::EMBED => "embed",
            HTMLTag::OBJECT => "object",
            HTMLTag::PARAM => "param",
            HTMLTag::VIDEO => "video",
            HTMLTag::AUDIO => "audio",
            HTMLTag::SOURCE => "source",
            HTMLTag::TRACK => "track",
            HTMLTag::CANVAS => "canvas",
            HTMLTag::MAP => "map",
            HTMLTag::AREA => "area",
            HTMLTag::MATH => "math",
            HTMLTag::MI => "mi",
            HTMLTag::MO => "mo",
            HTMLTag::MN => "mn",
            HTMLTag::MS => "ms",
            HTMLTag::MTEXT => "mtext",
            HTMLTag::MGLYPH => "mglyph",
            HTMLTag::MALIGNMARK => "malignmark",
            HTMLTag::ANNOTATION => "annotation",
            HTMLTag::SVG => "svg",
            HTMLTag::FOREIGNOBJECT => "foreignobject",
            HTMLTag::DESC => "desc",
            HTMLTag::TABLE => "table",
            HTMLTag::CAPTION => "caption",
            HTMLTag::COLGROUP => "colgroup",
            HTMLTag::COL => "col",
            HTMLTag::TBODY => "tbody",
            HTMLTag::THEAD => "thead",
            HTMLTag::TFOOT => "tfoot",
            HTMLTag::TR => "tr",
            HTMLTag::TD => "td",
            HTMLTag::TH => "th",
            HTMLTag::FORM => "form",
            HTMLTag::FIELDSET => "fieldset",
            HTMLTag::LEGEND => "legend",
            HTMLTag::LABEL => "label",
            HTMLTag::INPUT => "input",
            HTMLTag::BUTTON => "button",
            HTMLTag::SELECT => "select",
            HTMLTag::DATALIST => "datalist",
            HTMLTag::OPTGROUP => "optgroup",
            HTMLTag::OPTION => "option",
            HTMLTag::TEXTAREA => "textarea",
            HTMLTag::KEYGEN => "keygen",
            HTMLTag::OUTPUT => "output",
            HTMLTag::PROGRESS => "progress",
            HTMLTag::METER => "meter",
            HTMLTag::DETAILS => "details",
            HTMLTag::SUMMARY => "summary",
            HTMLTag::MENU => "menu",
            HTMLTag::MENUITEM => "menuitem",
            HTMLTag::APPLET => "applet",
            HTMLTag::ACRONYM => "acronym",
            HTMLTag::BGSOUND => "bgsound",
            HTMLTag::DIR => "dir",
            HTMLTag::FRAME => "frame",
            HTMLTag::FRAMESET => "frameset",
            HTMLTag::NOFRAMES => "noframes",
            HTMLTag::LISTING => "listing",
            HTMLTag::XMP => "xmp",
            HTMLTag::NEXTID => "nextid",
            HTMLTag::NOEMBED => "noembed",
            HTMLTag::PLAINTEXT => "plaintext",
            HTMLTag::RB => "rb",
            HTMLTag::STRIKE => "strike",
            HTMLTag::BASEFONT => "basefont",
            HTMLTag::BIG => "big",
            HTMLTag::BLINK => "blink",
            HTMLTag::CENTER => "center",
            HTMLTag::FONT => "font",
            HTMLTag::MARQUEE => "marquee",
            HTMLTag::MULTICOL => "multicol",
            HTMLTag::NOBR => "nobr",
            HTMLTag::SPACER => "spacer",
            HTMLTag::TT => "tt",
            HTMLTag::RTC => "rtc",
            HTMLTag::DIALOG => "dialog",
            _ => "unknown",
        }
    }
}
