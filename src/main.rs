use std::{env::args, fs, io::Result, ops::Range, path::Path, time::Duration};
mod app;
mod key_controller;
mod key_table_parser;
mod text_buffer;
mod tree;

use ratatui::text::ToText;
// use app::App;
use tree_sitter::{Node, Parser, Tree, TreeCursor};

pub const DESIRED_FRAME_TIME: Duration = Duration::from_millis(10);

fn main() -> Result<()> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("Error loading Rust grammar");
    // let mut app = App::new(args().collect::<Vec<String>>().get(1))?;
    // app.run()?;
    let tree = parser
        .parse(fs::read("src/main.rs").unwrap(), None)
        .unwrap();
    traverse((0, tree.root_node(), 0));
    Ok(())
}

fn traverse<'a>((idx, node, prev_line): (usize, Node<'a>, usize)) {
    let word_question_mark = |str: &'static str| {
        if str.len() > 2 {
            "word".to_string()
        } else {
            str.to_string()
        }
    };
    print!("{}", word_question_mark(node.kind()));
    if prev_line != node.start_position().row {
        println!();
    }

    let mut walk = node.walk();
    node.children(&mut walk)
        .enumerate()
        .map(|(i, n)| (i, n, node.start_position().row))
        .for_each(traverse);
}
