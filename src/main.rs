// TODO(elsuizo:2020-08-16): todos
// [ ] maybe add the dependencies in the config.yaml
// [X] add the package.xml for all the nodes
// [X] add the basic dependencies in all nodes

use askama::Template;
use serde::{Serialize, Deserialize};

use std::process::Command;
use std::env;
use std::error::Error;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::{read_to_string, create_dir_all, File};

const CONFIG_STRING: &'static str = "[build]\ntarget-dir = \"build/cargo\"";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    project_name: String,
    version: String,
    nodes: Vec<String>
}

#[derive(Template)]
#[template(path = "Cargo.txt")]
struct WorkspaceTemplate {
    nodes: Vec<String>
}

#[derive(Template)]
#[template(path = "CMakeLists.txt")]
struct CMakeTemplate {
    node: String
}

#[derive(Template)]
#[template(path = "package.txt")]
struct PackageTemplate {
    node: String,
    version: String,
}

fn write_file(out: &mut dyn Write, content: &str) -> std::io::Result<()> {
    out.write(content.as_bytes())?;
    out.flush()
}

fn main() -> Result<(), Box<dyn Error>> {

    let current_dir = env::current_dir()?;
    let string_config = read_to_string("config.yaml").expect("you must have a configuration file config.yaml");
    let user_config: Config = serde_yaml::from_str(&string_config)?;

    // create the workspace Cargo.toml file filling the template
    let ws = WorkspaceTemplate {nodes: user_config.nodes.clone()};
    let mut workspace_file = File::create("Cargo.toml")?;
    write_file(&mut workspace_file, ws.render()?.as_str())?;

    // create the .config dir and the config file
    create_dir_all(current_dir.join(".cargo"))?;
    let mut config_cargo = File::create(".cargo/config")?;
    write_file(&mut config_cargo, CONFIG_STRING)?;

    // create all the nodes
    for node in &user_config.nodes {
        let cmake_template = CMakeTemplate {node: node.to_string()};
        env::set_current_dir(current_dir.join("src"))?;

        Command::new("cargo")
            .args(&["new", node.as_str()])
            .output()?;

        // create the CMakeLists.txt file in the node
        let node_path = Path::new(node);
        let mut cmake_file = File::create(node_path.join("CMakeLists.txt"))?;
        write_file(&mut cmake_file, cmake_template.render()?.as_str())?;
        // add the basics dependencies to the node
        let mut cargo_file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(node_path.join("Cargo.toml")).expect("missing Cargo.toml file");

        writeln!(cargo_file, "rosrust = \"0.9.6\"")?;
        writeln!(cargo_file, "rosrust_msg = \"0.1.2\"")?;
        // // create the package.xml file in the node
        let package_template = PackageTemplate {node: node.to_string(), version: user_config.version.clone()};
        let mut package_file = File::create(node_path.join("package.xml"))?;
        write_file(&mut package_file, package_template.render()?.as_str())?;

    }

    println!("{:} project generated!!!", user_config.project_name);

    Ok(())
}
