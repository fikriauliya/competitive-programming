use tera::Tera;
use std::env;
use std::fs;
use std::path::Path;
use std::io::Write;

fn main() {
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
    let project_name = env::var("project_name").unwrap();
    context.insert("project_name", &project_name);

    let root = Path::new(&project_name);
    if !root.exists() {
        fs::create_dir(&project_name).unwrap();
    }
    for (file, _) in &tera.templates {
        let path = root.join(file);
        fs::create_dir_all(&path.parent().unwrap()).unwrap();

        let mut out = fs::File::create(&path).unwrap();
        let content = tera.render(file, &context).unwrap();
        out.write_all(content.as_bytes()).unwrap();
    }
}
