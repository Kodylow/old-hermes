async fn handle_readme() -> String {
    let readme = fs::read_to_string("README.md").expect("Could not read README.md");
    info!("Serving README.md");
    readme
}
