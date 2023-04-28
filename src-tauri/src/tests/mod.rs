
use std::{vec, borrow::Borrow, fmt::format};

use markdown::mdast::{Heading, Node};

use crate::parser;



#[test]
fn test_gfm() -> Result<(), String> {
  println!(
      "{}",
      markdown::to_html_with_options(
          r#"# title
          * [x] contact@example.com ~~strikethrough~~
          * [ ] contact@example.com ~~strikethrough~~
          > a
          # Hello, world!
          This is an **example** *page.*

          ## h2 [abc](https://www.baidu.com)
          ```rust
          fn main()->{
            println!("rust");
          }
          ```"#,
          &markdown::Options::gfm()
      )?
  );

  Ok(())
}

#[test]
fn test_ast() -> Result<(), String> {
  println!(
    "{:?}",
    markdown::to_html_with_options("# Hey, *you*!\n> a\n```rust\nfn main(){}\n```",  &markdown::Options::gfm())?
  );
  println!(
    "{:?}",
    markdown::to_mdast("", &markdown::ParseOptions::default())?
  );

  Ok(())
}




#[test]
fn test_html(){
  let md=parser::html::parse(r#"
  <h1>1</h1>
  <ul>
    <li>a</li>
    <li>b</li>
  </ul>
  <p>aaaaa<em>ab</em>bbbbbb</p>
  <blockquote>Block<del>quote</del></blockquote>
  "#).unwrap();
  println!("{}",md);
  let html=parser::md::parse(md.as_str()).unwrap();
  println!("{}",html);
}

#[test]
fn test_html_tag(){
  let md=parser::html::parse_tag(r#"
  <h1>1</h1>
  <ul>
    <li>a</li>
    <li>b</li>
  </ul>
  <ol start="5">
    <li>a</li>
    <li>b</li>
  </ol>
  <p>aaaaa<em>ab</em>bbbb<code>bb</code></p>
  <pre>
    <code class="language-rust">
    fn main(){
      println!("rust")
    }</code>
  </pre>
  <blockquote>Block<del>quote</del></blockquote>
  <p><a href="https://www.baidu.com">百度</a><img src="https://img1.baidu.com/it/u=2812417321,4100104782&fm=253&fmt=auto&app=138&f=JPEG?w=500&h=750" /></p>
  "#).unwrap();
  println!("{}",md);
  let html=parser::md::parse(md.as_str()).unwrap();
  println!("{}",html);
}