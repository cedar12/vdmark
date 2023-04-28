use std::{borrow::Borrow, collections::HashMap};

use html5ever::{local_name, parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

use crate::parser::Node;

use super::{Heading, Image, Link, List, ListItem, NodeTag, Tag, Text, Code};

pub fn parse(html: &str) -> anyhow::Result<String> {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())?;
    let mut node = Node::new("root".into());
    for top_child in dom.document.children.borrow().iter() {
        for child in top_child.children.borrow().iter() {
            match child.data {
                NodeData::Element { ref name, .. } => {
                    if name.local == local_name!("body") {
                        for child_item in child.children.borrow().iter() {
                            print_node(child_item, 0, &mut node);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(node.to_markdown())
}

fn print_node(node: &Handle, level: usize, ast: &mut Node) {
    let node_data = node.data.borrow();
    match node_data {
        NodeData::Text { ref contents } => {
            let content = contents.borrow();
            let str = content.chars().as_str();
            let text = Node::text(str.into());
            ast.children.push(text);
        }
        NodeData::Element {
            ref name, attrs, ..
        } => {
            // println!("{}<{} {:?}>", " ".repeat(level), name.local, attrs);
            let mut schma = Node::new(name.local.to_string());
            for child in node.children.borrow().iter() {
                print_node(child, level + 2, &mut schma);
            }
            ast.children.push(schma);
        }
        _ => {}
    }
}

pub fn parse_tag(html: &str) -> anyhow::Result<String> {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())?;
    let mut node = NodeTag::new(Tag::Root);
    for top_child in dom.document.children.borrow().iter() {
        for child in top_child.children.borrow().iter() {
            match child.data {
                NodeData::Element { ref name, .. } => {
                    if name.local == local_name!("body") {
                        for child_item in child.children.borrow().iter() {
                            print_node_tag(child_item, 0, &mut node);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    println!("{:?}",node);
    Ok(node.to_markdown())
}

fn print_node_tag(node: &Handle, level: usize, ast: &mut NodeTag) {
    let node_data = node.data.borrow();
    match node_data {
        NodeData::Text { ref contents } => {
            let content = contents.borrow();
            let str = content.chars().as_str();
            println!("->{}",str.trim());
            if str.trim()!=""{
              let text = NodeTag::new(Tag::Text(Text {
                  content: str.to_string(),
              }));
              ast.children.push(text);
            }
            
        }
        NodeData::Element {
            ref name, attrs, ..
        } => {
            let attrs = attrs.borrow();
            let mut attr_map = HashMap::new();
            for attr in attrs.iter() {
                attr_map.insert(
                    attr.name.local.chars().as_str(),
                    attr.value.chars().as_str(),
                );
            }
            // println!("{}<{} {:?}>", " ".repeat(level), name.local, attrs);
            let mut schma = match name.local {
                local_name!("h1") => NodeTag::new(Tag::Heading(Heading { depth: 1 })),
                local_name!("h2") => NodeTag::new(Tag::Heading(Heading { depth: 2 })),
                local_name!("h3") => NodeTag::new(Tag::Heading(Heading { depth: 3 })),
                local_name!("h4") => NodeTag::new(Tag::Heading(Heading { depth: 4 })),
                local_name!("h5") => NodeTag::new(Tag::Heading(Heading { depth: 5 })),
                local_name!("h6") => NodeTag::new(Tag::Heading(Heading { depth: 6 })),
                local_name!("p") => NodeTag::new(Tag::Paragraph),
                local_name!("del") => NodeTag::new(Tag::Del),
                local_name!("blockquote") => NodeTag::new(Tag::Blockquote),
                local_name!("em") => NodeTag::new(Tag::Emphasis),
                local_name!("strong") => NodeTag::new(Tag::Strong),
                local_name!("ul") => NodeTag::new(Tag::List(List {
                    is_order: false,
                    start: 1,
                })),
                local_name!("ol") => {
                    let start = match attr_map.get("start") {
                        Some(s) => match s.parse::<usize>() {
                            Ok(s) => s,
                            Err(_) => 1,
                        },
                        None => 1,
                    };
                    NodeTag::new(Tag::List(List {
                        is_order: true,
                        start: start,
                    }))
                }
                local_name!("li") => match &ast.tag {
                    Tag::List(list) => NodeTag::new(Tag::ListItem(ListItem {
                        is_order: list.is_order,
                        index: list.start+ast.children.len()-1,
                        checked: None,
                    })),
                    _ => NodeTag::not_support(),
                },
                local_name!("a") => {
                    let herf = match attr_map.get("href") {
                        Some(h) => h.to_string(),
                        None => String::new(),
                    };
                    NodeTag::new(Tag::Link(Link { href: herf }))
                }
                local_name!("img") => {
                    let src = match attr_map.get("src") {
                        Some(h) => h.to_string(),
                        None => String::new(),
                    };
                    let alt = match attr_map.get("alt") {
                        Some(h) => h.to_string(),
                        None => String::new(),
                    };
                    NodeTag::new(Tag::Image(Image { src: src, alt: alt }))
                }
                local_name!("code")=>{
                  let language = match attr_map.get("class") {
                    Some(c) => {
                      let class_list:Vec<&str>=c.split(" ").collect();
                      let language:Vec<&str>=class_list.iter().filter(|&c|c.trim().starts_with("language-")).cloned().collect();
                      if language.len()>0{
                        language[0].replace("language-", "").to_string()
                      }else{
                        String::new()
                      }
                    },
                    None => String::new(),
                  };
                  println!("{:?}",ast.tag);
                  match &ast.tag {
                    Tag::Pre => NodeTag::new(Tag::Code(Code{ language: language })),
                    _ => NodeTag::new(Tag::InlineCode),
                  }
                },
                local_name!("pre")=>NodeTag::new(Tag::Pre),
                _ => NodeTag::not_support(),
            };
            for child in node.children.borrow().iter() {
                print_node_tag(child, level + 2, &mut schma);
            }
            ast.children.push(schma);
        }
        _ => {}
    }
}
