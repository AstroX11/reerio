#[derive(Debug, Clone)]
pub struct HtmlNode {
    pub tag: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<HtmlNode>,
    pub text: Option<String>,
}

#[allow(dead_code)]
/// Static HTML tags reference (all HTML5 tags)
pub struct HtmlTags {
    // Document metadata
    pub html: &'static str,
    pub head: &'static str,
    pub title: &'static str,
    pub base: &'static str,
    pub link: &'static str,
    pub meta: &'static str,
    pub style: &'static str,

    // Sectioning root
    pub body: &'static str,

    // Content sectioning
    pub address: &'static str,
    pub article: &'static str,
    pub aside: &'static str,
    pub footer: &'static str,
    pub header: &'static str,
    pub h1: &'static str,
    pub h2: &'static str,
    pub h3: &'static str,
    pub h4: &'static str,
    pub h5: &'static str,
    pub h6: &'static str,
    pub hgroup: &'static str,
    pub main: &'static str,
    pub nav: &'static str,
    pub section: &'static str,
    pub search: &'static str,

    // Text content
    pub blockquote: &'static str,
    pub dd: &'static str,
    pub div: &'static str,
    pub dl: &'static str,
    pub dt: &'static str,
    pub figcaption: &'static str,
    pub figure: &'static str,
    pub hr: &'static str,
    pub li: &'static str,
    pub menu: &'static str,
    pub ol: &'static str,
    pub p: &'static str,
    pub pre: &'static str,
    pub ul: &'static str,

    // Inline text semantics
    pub a: &'static str,
    pub abbr: &'static str,
    pub b: &'static str,
    pub bdi: &'static str,
    pub bdo: &'static str,
    pub br: &'static str,
    pub cite: &'static str,
    pub code: &'static str,
    pub data: &'static str,
    pub dfn: &'static str,
    pub em: &'static str,
    pub i: &'static str,
    pub kbd: &'static str,
    pub mark: &'static str,
    pub q: &'static str,
    pub rp: &'static str,
    pub rt: &'static str,
    pub ruby: &'static str,
    pub s: &'static str,
    pub samp: &'static str,
    pub small: &'static str,
    pub span: &'static str,
    pub strong: &'static str,
    pub sub: &'static str,
    pub sup: &'static str,
    pub time: &'static str,
    pub u: &'static str,
    pub var: &'static str,
    pub wbr: &'static str,

    // Image and multimedia
    pub area: &'static str,
    pub audio: &'static str,
    pub img: &'static str,
    pub map: &'static str,
    pub track: &'static str,
    pub video: &'static str,

    // Embedded content
    pub embed: &'static str,
    pub iframe: &'static str,
    pub object: &'static str,
    pub picture: &'static str,
    pub portal: &'static str,
    pub source: &'static str,

    // SVG and MathML
    pub svg: &'static str,
    pub math: &'static str,

    // Scripting
    pub canvas: &'static str,
    pub noscript: &'static str,
    pub script: &'static str,

    // Demarcating edits
    pub del: &'static str,
    pub ins: &'static str,

    // Table content
    pub caption: &'static str,
    pub col: &'static str,
    pub colgroup: &'static str,
    pub table: &'static str,
    pub tbody: &'static str,
    pub td: &'static str,
    pub tfoot: &'static str,
    pub th: &'static str,
    pub thead: &'static str,
    pub tr: &'static str,

    // Forms
    pub button: &'static str,
    pub datalist: &'static str,
    pub fieldset: &'static str,
    pub form: &'static str,
    pub input: &'static str,
    pub label: &'static str,
    pub legend: &'static str,
    pub meter: &'static str,
    pub optgroup: &'static str,
    pub option: &'static str,
    pub output: &'static str,
    pub progress: &'static str,
    pub select: &'static str,
    pub textarea: &'static str,

    // Interactive elements
    pub details: &'static str,
    pub dialog: &'static str,
    pub summary: &'static str,

    // Web Components
    pub slot: &'static str,
    pub template: &'static str,
}

#[allow(dead_code)]
/// A single constant instance holding all tag names
pub const TAGS: HtmlTags = HtmlTags {
    // Document metadata
    html: "html",
    head: "head",
    title: "title",
    base: "base",
    link: "link",
    meta: "meta",
    style: "style",

    // Sectioning root
    body: "body",

    // Content sectioning
    address: "address",
    article: "article",
    aside: "aside",
    footer: "footer",
    header: "header",
    h1: "h1",
    h2: "h2",
    h3: "h3",
    h4: "h4",
    h5: "h5",
    h6: "h6",
    hgroup: "hgroup",
    main: "main",
    nav: "nav",
    section: "section",
    search: "search",

    // Text content
    blockquote: "blockquote",
    dd: "dd",
    div: "div",
    dl: "dl",
    dt: "dt",
    figcaption: "figcaption",
    figure: "figure",
    hr: "hr",
    li: "li",
    menu: "menu",
    ol: "ol",
    p: "p",
    pre: "pre",
    ul: "ul",

    // Inline text semantics
    a: "a",
    abbr: "abbr",
    b: "b",
    bdi: "bdi",
    bdo: "bdo",
    br: "br",
    cite: "cite",
    code: "code",
    data: "data",
    dfn: "dfn",
    em: "em",
    i: "i",
    kbd: "kbd",
    mark: "mark",
    q: "q",
    rp: "rp",
    rt: "rt",
    ruby: "ruby",
    s: "s",
    samp: "samp",
    small: "small",
    span: "span",
    strong: "strong",
    sub: "sub",
    sup: "sup",
    time: "time",
    u: "u",
    var: "var",
    wbr: "wbr",

    // Image and multimedia
    area: "area",
    audio: "audio",
    img: "img",
    map: "map",
    track: "track",
    video: "video",

    // Embedded content
    embed: "embed",
    iframe: "iframe",
    object: "object",
    picture: "picture",
    portal: "portal",
    source: "source",

    // SVG and MathML
    svg: "svg",
    math: "math",

    // Scripting
    canvas: "canvas",
    noscript: "noscript",
    script: "script",

    // Demarcating edits
    del: "del",
    ins: "ins",

    // Table content
    caption: "caption",
    col: "col",
    colgroup: "colgroup",
    table: "table",
    tbody: "tbody",
    td: "td",
    tfoot: "tfoot",
    th: "th",
    thead: "thead",
    tr: "tr",

    // Forms
    button: "button",
    datalist: "datalist",
    fieldset: "fieldset",
    form: "form",
    input: "input",
    label: "label",
    legend: "legend",
    meter: "meter",
    optgroup: "optgroup",
    option: "option",
    output: "output",
    progress: "progress",
    select: "select",
    textarea: "textarea",

    // Interactive elements
    details: "details",
    dialog: "dialog",
    summary: "summary",

    // Web Components
    slot: "slot",
    template: "template",
};
