use minijinja::Environment;
use serde_json::{Result, Value};

// NOTE: You call the template that has
// the `extends`` tag in it. Not the other
// way around. It's easier for me to get
// my head around the include method

pub fn multiple_templates_via_extends() -> Result<()> {
    let mut env = Environment::new();

    let base_string = r#"
<div>Hello, {% block name %}{% endblock %}</div>
"#;

    let name_block_string = r#"{% extends "base" %}
{% block name %}<strong>World</strong>{% endblock %}
"#;

    env.add_template("name", name_block_string).unwrap();
    env.add_template("base", base_string).unwrap();

    let json_string = r#"{ "name": "World" }"#;
    let json_data: Value = serde_json::from_str(json_string)?;

    let tmpl = env.get_template("name").unwrap();
    println!("{}", tmpl.render(json_data).unwrap());
    Ok(())
}
