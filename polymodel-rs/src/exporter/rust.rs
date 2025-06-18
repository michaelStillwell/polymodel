use convert_case::{Case, Casing};
use std::path::{Path, PathBuf};

use crate::generator::*;

pub struct RustExporter;

impl Exporter for RustExporter {
    fn export(
        output: &PolymodelOutput,
        models: &PolymodelList,
    ) -> anyhow::Result<Vec<(PathBuf, String)>> {
        let mut res = Vec::new();

        for model in models.iter() {
            let (name, fields) = model.iter().next().unwrap();

            let (file_name, value) = Self::model_to_string(name, fields);
            let path = Path::new(&output.dir);
            res.push((path.join(file_name), value));
        }

        Ok(res)
    }

    fn model_to_string(name: &str, fields: &PolymodelFields) -> (PathBuf, String) {
        let mut res = String::new();

        res.push_str(&format!("pub struct {} {{\n", name.to_case(Case::Pascal)));

        let mut fs: Vec<String> = Vec::new();
        for (k, v) in fields {
            fs.push(Self::field_to_string((
                &k.to_case(Case::Snake),
                &Self::type_to_string(v),
            )));
        }

        res.push_str(&fs.join(",\n"));
        res.push_str("\n}");

        let name = format!("{}.rs", name.to_case(Case::Snake));
        let file_name = Path::new(&name);

        (file_name.into(), res)
    }

    fn field_to_string((k, v): (&str, &str)) -> String {
        format!("\tpub {k}: {v}")
    }

    fn type_to_string(t: &PolymodelType) -> String {
        match t {
            PolymodelType::String => "String",
            PolymodelType::Number => "i32",
            PolymodelType::Float => "f32",
            PolymodelType::Email => "String",
        }
        .into()
    }
}
