use std::collections::HashMap;

pub mod html;
pub mod md;

#[derive(Debug, Clone)]
pub enum Tag {
    Root,
    Heading(Heading),
    Paragraph,
    ThematicBreak,
    Blockquote,
    List(List),
    ListItem(ListItem),
    Text(Text),
    Emphasis,
    Strong,
    Del,
    Pre,
    Code(Code),
    InlineCode,
    Link(Link),
    Image(Image),
    Def,
    DefRef,
    NotSupport,
}
#[derive(Debug, Clone)]
pub struct Heading {
    pub depth: usize,
}
#[derive(Debug, Clone)]
pub struct List {
    pub is_order: bool,
    pub start: usize,
}
#[derive(Debug, Clone)]
pub struct ListItem {
    pub is_order: bool,
    pub index: usize,
    pub checked: Option<bool>,
}
#[derive(Debug, Clone)]
pub struct Text {
    pub content: String,
}
#[derive(Debug, Clone)]
pub struct Link {
    pub href: String,
}
#[derive(Debug, Clone)]
pub struct Image {
    pub src: String,
    pub alt: String,
}
#[derive(Debug, Clone)]
pub struct Code {
    pub language: String,
}

#[derive(Debug, Clone)]
struct Node {
    pub tag: String,
    pub text: Option<String>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone)]
struct NodeTag {
    pub tag: Tag,
    pub children: Vec<NodeTag>,
}

impl Node {
    pub fn new(tag: String) -> Self {
        Self {
            tag: tag,
            children: vec![],
            text: None,
        }
    }
    pub fn text(text: String) -> Self {
        Self {
            tag: String::from("text"),
            children: vec![],
            text: Some(text),
        }
    }
    fn to_markdown(&self) -> String {
        let mut md = String::new();
        if self.tag == String::from("root") {
            let sub = self.to_sub_md();
            md = format!("{}{}", md, sub);
        }
        md
    }
    fn to_sub_md(&self) -> String {
        let mut sub = String::new();
        for item in self.children.iter().enumerate() {
            let mut str = String::new();
            item.1.to_md(&mut str);
            sub = format!("{}{}", sub, str);
        }
        sub
    }
    fn to_md(&self, md: &mut String) {
        match self.tag.as_str() {
            "text" => {
                let mut sub = String::new();
                if let Some(text) = &self.text {
                    sub = format!("{}", text);
                }
                sub = format!("{}{}", sub, self.to_sub_md());
                *md = format!("{}{}", md, sub);
            }
            "h1" => {
                *md = format!("{}\n# {}", md, self.to_sub_md());
            }
            "h2" => {
                *md = format!("{}\n## {}", md, self.to_sub_md());
            }
            "h3" => {
                *md = format!("{}\n### {}", md, self.to_sub_md());
            }
            "h4" => {
                *md = format!("{}\n#### {}", md, self.to_sub_md());
            }
            "h5" => {
                *md = format!("{}\n##### {}", md, self.to_sub_md());
            }
            "h6" => {
                *md = format!("{}\n###### {}", md, self.to_sub_md());
            }

            "p" => {
                *md = format!("{}\n{}", md, self.to_sub_md());
            }
            "strong" => {
                *md = format!("{}**{}**", md, self.to_sub_md());
            }
            "blockquote" => {
                *md = format!("{}\n> {}", md, self.to_sub_md());
            }
            "em" => {
                *md = format!("{}*{}*", md, self.to_sub_md());
            }
            "del" => {
                *md = format!("{}~~{}~~", md, self.to_sub_md());
            }
            "a" => {
                *md = format!("{}[{}](https://www.baidu.com)", md, self.to_sub_md());
            }
            "ul" => {
                *md = format!("{}{}", md, self.to_sub_md());
            }
            "li" => {
                *md = format!("{}\n* {}", md, self.to_sub_md());
            }
            _ => {}
        }
    }
}

impl NodeTag {
    pub fn new(tag: Tag) -> Self {
        Self {
            tag: tag,
            children: vec![],
        }
    }
    pub fn not_support() -> Self {
        Self {
            tag: Tag::NotSupport,
            children: vec![],
        }
    }
    fn to_markdown(&self) -> String {
        let mut md = String::new();
        match self.tag {
            Tag::Root => {
                let sub = self.to_sub_md();
                md = format!("{}{}", md, sub);
            }
            _ => {}
        }
        md
    }
    fn to_sub_md(&self) -> String {
        let mut sub = String::new();
        for item in self.children.iter().enumerate() {
            let mut str = String::new();
            item.1.to_md(&mut str);
            sub = format!("{}{}", sub, str);
        }
        sub
    }
    fn to_md(&self, md: &mut String) {
        match &self.tag {
            Tag::Text(text) => {
                let mut sub = String::from(text.content.clone());
                sub = format!("{}{}", sub, self.to_sub_md());
                *md = format!("{}{}", md, sub);
            }
            Tag::Heading(heading) => {
                *md = format!("{}\n{} {}", md, "#".repeat(heading.depth), self.to_sub_md());
            }
            Tag::Paragraph => {
                *md = format!("{}\n{}", md, self.to_sub_md());
            }
            Tag::Strong => {
                *md = format!("{}**{}**", md, self.to_sub_md());
            }
            Tag::Blockquote => {
                *md = format!("{}\n> {}", md, self.to_sub_md());
            }
            Tag::Emphasis => {
                *md = format!("{}*{}*", md, self.to_sub_md());
            }
            Tag::Del => {
                *md = format!("{}~~{}~~", md, self.to_sub_md());
            }
            Tag::Link(link) => {
                *md = format!("{}[{}]({})", md, self.to_sub_md(), link.href);
            }
            Tag::Image(image) => {
                *md = format!("{}![{}]({})", md, image.alt, image.src);
            }
            Tag::List(list) => {
                *md = format!("{}{}", md, self.to_sub_md());
            }
            Tag::InlineCode => {
                *md = format!("{}``{}``", md, self.to_sub_md());
            }
            Tag::Pre => {
                *md = format!("{}{}", md, self.to_sub_md());
            }
            Tag::Code(code) => {
                *md = format!("{}\n```{}\n{}\n```", md, code.language, self.to_sub_md());
            }
            Tag::ListItem(item) => {
                let sub = self.to_sub_md();
                if item.is_order {
                    *md = format!("{}\n{}. {}", md, item.index, sub);
                } else {
                    *md = format!("{}\n* {}", md, sub);
                }
            }
            _ => {}
        }
    }
}
