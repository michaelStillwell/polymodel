use convert_case::{Case, Casing};
use std::path::{Path, PathBuf};

use crate::generator::*;

pub struct TypescriptExporter;

impl Exporter for TypescriptExporter {
    fn export(
        output: &PolymodelOutput,
        models: &PolymodelList,
    ) -> anyhow::Result<Vec<(PathBuf, String)>> {
        let mut res = Vec::new();

        for model in models.iter() {
            let (name, fields) = model.iter().next().unwrap();

            let (file_name, val) = Self::model_to_string(name, fields);
            let path = Path::new(&output.dir);
            res.push((path.join(file_name), val));
        }

        Ok(res)
    }

    fn model_to_string(name: &str, fields: &PolymodelFields) -> (PathBuf, String) {
        let mut res = String::new();

        res.push_str(&format!(
            "export interface {} {{\n",
            name.to_case(Case::Pascal)
        ));

        let mut fs: Vec<String> = Vec::new();
        for (k, v) in fields {
            fs.push(Self::field_to_string((
                &k.to_case(Case::Camel),
                &Self::type_to_string(&v),
            )));
        }

        res.push_str(&fs.join(";\n"));
        res.push_str(";\n}");

        let name = format!("{}.ts", name.to_case(Case::Camel));
        let file_name = Path::new(&name);

        (file_name.into(), res)
    }

    fn field_to_string((k, v): (&str, &str)) -> String {
        format!("\t{k}: {v}")
    }

    fn type_to_string(t: &PolymodelType) -> String {
        match t {
            PolymodelType::String => "string",
            PolymodelType::Number => "number",
            PolymodelType::Float => "float",
            PolymodelType::Email => "string",
        }
        .into()
    }
}
