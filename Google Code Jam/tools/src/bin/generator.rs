use tera::Tera;
use std::env;
use std::fs;
use std::path::Path;
use std::io::Write;
use std::process::Command;
use toml::Value;

fn create_root_folder(project_name: &str) {
    let root = Path::new(&project_name);
    if !root.exists() {
        fs::create_dir(&project_name).unwrap();
    }
}

fn geneate_project(project_name: &str) {
    let tera = match Tera::new("tools/templates/**") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    use tera::Context;
    // Using the tera Context struct
    let mut context = Context::new();
    context.insert("project_name", &project_name);

    let root = Path::new(&project_name);
    for (file, _) in &tera.templates {
        let path = root.join(file);
        fs::create_dir_all(&path.parent().unwrap()).unwrap();

        let mut out = fs::File::create(&path).unwrap();
        let content = tera.render(file, &context).unwrap();
        out.write_all(content.as_bytes()).unwrap();
    }
}

fn download_test_cases(project_name: &str) {
    let url = format!("https://open.kattis.com/problems/{}/file/statement/samples.zip", project_name);
    let root = Path::new(&project_name);
    let resp = reqwest::blocking::get(&url).unwrap().bytes().unwrap();

    let out_file = root.join("samples.zip");
    let mut out = fs::File::create(&out_file).unwrap();
    out.write_all(&resp).unwrap();
    Command::new("unzip").args(&[out_file.to_str().unwrap(), "-d", root.to_str().unwrap()]).output().expect("Failed to unzip");
    fs::remove_file(&out_file).unwrap();
}

fn inject_to_cargo(project_name: &str) {
    let content = fs::read_to_string("Cargo.toml").unwrap();
    let mut value = content.parse::<Value>().unwrap();
    let members = value.get_mut("workspace").unwrap().get_mut("members").unwrap();
    if let Value::Array(members) = members {
        let project_name_value = Value::String(project_name.to_string());
        if !members.contains(&project_name_value) {
            members.push(project_name_value);
        }
    }

    let mut out = fs::File::create("Cargo.toml").unwrap();
    out.write_all(value.to_string().as_bytes()).unwrap();
}

fn main() {
    let project_name = env::var("project_name").unwrap();
    create_root_folder(&project_name);
    geneate_project(&project_name);
    download_test_cases(&project_name);
    inject_to_cargo(&project_name);
}
