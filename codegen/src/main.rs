fn main() {
    let src = "./ambient-weather.json";
    let file = std::fs::File::open(src).expect(&format!("Cannot find src file {src:?}"));
    let spec = serde_json::from_reader(file).expect("Error parsing src file");
    let mut generator = progenitor::Generator::default();

    let tokens = generator.generate_tokens(&spec).expect("Error running progenitor code generator");
    let ast = syn::parse2(tokens).expect("Error parsing tokens using syn");
    let content = prettyplease::unparse(&ast);
    std::fs::write("./src/lib.rs", content).expect("Error writing to output file ./src/lib.rs");
    println!("Codegen complete!");
}

