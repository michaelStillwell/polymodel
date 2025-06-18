pub use {csharp::CSharpExporter, rust::RustExporter, typescript::TypescriptExporter};

mod csharp;
mod rust;
mod typescript;

// pub fn export(g: PolymodelGenerator) -> anyhow::Result<()> {
//     let models = g.models();
//     let outputs = g.outputs();

//     for target in g.targets() {
//         for model in models.iter() {
//             let (file_name, res) = model_to_string(target, model);
//             let path = &outputs[target];
//             std::fs::create_dir_all(&path.dir)?;
//             let mut fs = File::create(format!("{}/{}", path.dir, file_name))?;
//             fs.write_all(res.as_bytes())?;
//         }
//     }

//     Ok(())
// }

// fn model_to_string(target: &PolymodelTarget, m: &Polymodel) -> (String, String) {
//     let (name, fields) = m.iter().next().unwrap();

//     match target {
//         PolymodelTarget::Rust => rust::model_to_string(name, fields),
//         PolymodelTarget::CSharp => csharp::model_to_string(name, fields),
//         PolymodelTarget::Typescript => typescript::model_to_string(name, fields),
//     }
// }
