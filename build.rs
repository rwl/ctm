use std::{env, fs, path::Path};

use typify::{TypeSpace, TypeSpaceSettings};

fn main() {
    let content = fs::read_to_string("ctm_data_schema.json").unwrap();
    let content = content
        .replace(r#""title": "CTM Data""#, r#""title": "CTM""#)
        .replace(
            r#"https://www.eia.gov/survey/form/eia_923/instructions.pdf"#,
            r#"<https://www.eia.gov/survey/form/eia_923/instructions.pdf>"#,
        );
    // for unit in [
    //     "h",
    //     "s",
    //     "hours",
    //     "seconds",
    //     "MVA",
    //     "MW or pu",
    //     "MVAr or pu",
    //     "kV",
    //     "kV or pu",
    //     "$",
    //     "MW/h or pu/h",
    //     "-",
    //     "MWh or pu*h",
    //     "MW/h or pu/h",
    //     "kA or pu",
    //     "Ohm or pu",
    //     "S or pu",
    //     "MVA or pu",
    //     "deg",
    //     "events/year",
    // ] {
    //     content = content.replace(format!("[{unit}]").as_str(), format!("({unit})").as_str())
    // }
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap();

    let mut type_space = TypeSpace::new(
        TypeSpaceSettings::default()
            .with_struct_builder(true)
            .with_type_mod("CTM"),
    );
    type_space.add_root_schema(schema).unwrap();

    let contents =
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());

    let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("codegen.rs");
    fs::write(out_file, contents).unwrap();

    println!("cargo::rerun-if-changed=ctm_data_schema.json");
}
