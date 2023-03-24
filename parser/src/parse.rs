use std::{path::Path, io};

use pulldown_cmark::Options;

use crate::link_concat::{LinkMap, link_concat_events};


/// Parse the input file and write the output to the output file.
pub fn parse(links: LinkMap, input_path: &Path, output_path: &Path) -> io::Result<()> {
    let input = std::fs::read_to_string(input_path)?;
    let events = link_concat_events(&input, Options::all(), links);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, events);
    
    std::fs::write(output_path, html_output)?;
    Ok(())
}

/// Parse the input directory and write the output to the output directory recursively
/// 
/// When encountering a file called `header.md`, it will parse it as a link definition file
/// and it will not be included in the output.
/// 
/// `header.md` files will add on to each other, and apply to all the files in their directory
/// and below it. For example consider the following directory structure:
/// 
/// ```txt
/// root
/// ├── header.md
/// ├── file1.md ⟵ these files will have the root/header.md links
/// ├── file2.md ⟵
/// ├── gaming
/// │   ├── header.md
/// │   ├── file3.md ⟵ these files will have both header.md links
/// │   └── file4.md ⟵ root/gaming/header.md will take priority
/// ...
/// ```
pub fn parse_dir_recursive(input_dir: &Path, output_path: &Path) {
    
}

/// This function will watch the input directory and write the output to the output directory when 
/// detecting file changes using the `notify` crate.
/// 
/// See [`parse_dir_recursive`] for more description on how this function behaves
pub fn parse_dir_watch(input_dir: &Path, output_path: &Path) {
    
}