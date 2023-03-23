use minijinga_scratchpad::basic_json_output::basic_json_output;
use minijinga_scratchpad::multiple_templates_via_extends::multiple_templates_via_extends;
use minijinga_scratchpad::multiple_templates_with_includes::multiple_templates_with_includes;

fn main() {
    basic_json_output().unwrap();
    multiple_templates_with_includes().unwrap();
    multiple_templates_via_extends().unwrap();
}
