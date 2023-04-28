use markdown::{to_html_with_options, Options};


pub fn parse(md:&str)->anyhow::Result<String,String>{
  let html=to_html_with_options(md,&Options::gfm())?;
  Ok(html)
}