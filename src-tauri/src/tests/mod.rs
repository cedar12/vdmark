

#[test]
fn test_gfm() -> Result<(), String> {
  println!(
      "{}",
      markdown::to_html_with_options(
          "# title\n* [x] contact@example.com ~~strikethrough~~\n* [ ] contact@example.com ~~strikethrough~~\n> a",
          &markdown::Options::gfm()
      )?
  );

  Ok(())
}

#[test]
fn test_ast() -> Result<(), String> {
  println!(
    "{:?}",
    markdown::to_mdast("# Hey, *you*!\n> a", &markdown::ParseOptions::default())?
  );
  println!(
    "{:?}",
    markdown::to_mdast("", &markdown::ParseOptions::default())?
  );

  Ok(())
}


#[test]
fn test_html_ast() {
  let input = r#"
  <h1>abc</h1>\n<p>123</p>
    "#;
    let md=html2md::parse_html(input);
    println!("{}",md);
}