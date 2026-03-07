use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};

pub fn generate_graphviz(graph: &HashMap<&str, Vec<&str>>) -> io::Result<()> {
    let mut file = File::create("day-11/viz/tree.dot")?;

    writeln!(file, "digraph {{")?;
    for (node, children) in graph {
        let color = match *node {
            "you" => "red",
            "svr" => "orange",
            "dac" => "green",
            "fft" => "blue",
            _ => "white",
        };
        writeln!(file, "\"{}\" [fillcolor={}, style=filled];", node, color)?;

        for child in children {
            writeln!(file, "\"{}\" -> \"{}\"", node, child)?;
        }
    }

    writeln!(file, "\"out\" [fillcolor=purple, style=filled];")?;
    writeln!(file, "}}")?;

    Ok(())
}
