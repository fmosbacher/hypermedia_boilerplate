use std::process::Command;

const INPUT_CSS_PATH: &str = "./tailwind.css";
const OUTPUT_CSS_PATH: &str = "./public/tailwind.css";

fn main() {
    Command::new("npx")
        .arg("tailwindcss")
        .args(["-i", INPUT_CSS_PATH])
        .args(["-o", OUTPUT_CSS_PATH])
        .output()
        .expect("tailwind css build");
}
