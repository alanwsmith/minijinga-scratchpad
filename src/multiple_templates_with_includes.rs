use minijinja::Environment;
use serde_json::{Result, Value};

pub fn multiple_templates_with_includes() -> Result<()> {
    let mut env = Environment::new();

    let base_string = r#"
<div>Hello, {% include "name" %}</div>
"#;

    let name_block_string = r#"
<strong>World</strong>
"#;

    env.add_template("base", base_string).unwrap();
    env.add_template("name", name_block_string).unwrap();

    let json_string = r#"{ "name": "World" }"#;
    let json_data: Value = serde_json::from_str(json_string)?;

    let tmpl = env.get_template("base").unwrap();
    println!("{}", tmpl.render(json_data).unwrap());
    Ok(())
}
