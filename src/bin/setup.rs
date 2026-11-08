// src/bin/setup.rs is needed for generating src/main.rs file
//
// to generate/update src/main.rs you need to add name of
// the html file (it's preferred to be in the sources directory)
// to the list.txt file
//
// Also you should have root.html file as main.html (because
// "/" is named as a root, not as something else!), if you want
// to put html files in new subdirectory (for example
// sources/sas/sas.html) you should add "sas/sas.html" to the
// list.txt file (if you have 2 or more files in sources/sas,
// then you should put both to the list.txt file!)

const BEGIN: &str = r#"
use axum::{
    routing::get,
    Router,
    response::Html,
};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run -r -- \"ip:port\"");
        std::process::exit(1);
    }

    let listener = tokio::net::TcpListener::bind(&args[1]).await?;
"#;
const END: &str = r#"

    println!("{:?}", listener);
    axum::serve(listener, app).await?;
    Ok(())
}
"#;

fn main() -> Result<(), String> {
    // creates a string that contains a Router profile
    let mut router = format!("let app = Router::new().route(\"/\", get(async || {{ Html(tokio::fs::read_to_string(\"sources/root.html\").await.unwrap()) }}))");
    let list = parse_list()?;

    for i in list.iter() {
        // to create the form we need to add new "routes"
        let output = format!("/{}", i);         // first arg for route() method
        let input = format!("sources/{}", i);   // second arg for route() method
        let form = format!(
            ".route(\"{}\", get(async || {{ Html(tokio::fs::read_to_string(\"{}\").await.unwrap()) }}))",
            output, input
        ); // string that will be added to the router variable

        router = format!("{}\n\t\t{}", router, form);
    }

    router = format!("\t{};", router);
    let code = format!("{}{}{}", BEGIN, router, END);

    use std::{fs::File, io::Write};

    let mut file = match File::create("src/main.rs") {
        Ok(f) => f,
        Err(e) => return Err(format!("{:?}", e)),
    };

    match file.write_all(code.as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(format!("{:?}", e)),
    };

    println!("setup complete!");
    Ok(())
}

// this function is needed for parsing list.txt file (in root of the project)
//
// list.txt is needed for more comfortable adding new files in the project
//
// syntax of list.txt:
//      # comment
//      path/to/file.html
//      sas.html
// wrong syntax:
//      sas.html # this line will be dropped!
fn parse_list() -> Result<Vec<String>, String> {
    let file = match std::fs::read_to_string("list.txt") {
        Ok(f) => f,
        Err(_) => return Err(String::from("Can't read the list.txt")),
    };
    let mut vec: Vec<String> = Vec::new();

    for i in file.lines() {
        if i.contains("#") { continue }
        if i == "".to_string() { continue }
        vec.push(i.to_string());
    }

    Ok(vec)
}
