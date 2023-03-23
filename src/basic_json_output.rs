use minijinja::Environment;
use serde_json::{Result, Value};

pub fn basic_json_output() -> Result<()> {
    // Make the HTML and JSON
    let template_string = "<div>Hello, {{ name }}</div>";
    let json_string = r#"{ "name": "World" }"#;
    let json_data: Value = serde_json::from_str(json_string)?;

    // Setup and Render
    let mut env = Environment::new();
    env.add_template("template_name", template_string).unwrap();
    let tmpl = env.get_template("template_name").unwrap();
    println!("{}", tmpl.render(json_data).unwrap());
    Ok(())
}
