#![allow(unused)]

use crate::exporter::*;
use std::{collections::BTreeMap, path::PathBuf};

pub trait Exporter {
    fn export(
        output: &PolymodelOutput,
        models: &PolymodelList,
    ) -> anyhow::Result<Vec<(PathBuf, String)>>;

    fn model_to_string(name: &str, fields: &PolymodelFields) -> (PathBuf, String);

    fn field_to_string(field: (&str, &str)) -> String;

    fn type_to_string(t: &PolymodelType) -> String;
}

/* TODO: implement this so you can have a better body
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum TypeDef {
    Primitive(String), // "string", "email", etc.
    Union {
        union: Vec<HashMap<String, TypeDef>>,
    },
    Object(HashMap<String, TypeDef>),
    Ref(String), // reference to another typedef like "site"
    Compose {
        extends: String,
        #[serde(default)]
        omit: Vec<String>,
        #[serde(default)]
        optional: Vec<String>,
        #[serde(default)]
        add: HashMap<String, TypeDef>,
    }
}
*/

#[derive(Debug, Default, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum PolymodelTarget {
    #[default]
    Rust,
    CSharp,
    Typescript,
}

#[derive(Debug, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum PolymodelType {
    String,
    Number,
    Float,
    Email,
}

#[derive(Debug, serde::Deserialize)]
pub struct PolymodelOutput {
    pub dir: String,
}

pub type PolymodelOutputs = BTreeMap<PolymodelTarget, PolymodelOutput>;

pub type PolymodelFields = BTreeMap<String, PolymodelType>;

pub type Polymodel = BTreeMap<String, PolymodelFields>;

pub type PolymodelList = Vec<Polymodel>;

#[derive(Debug, serde::Deserialize)]
pub struct PolymodelGenerator {
    targets: Vec<PolymodelTarget>,
    outputs: PolymodelOutputs,
    models: PolymodelList,
}

impl Default for PolymodelGenerator {
    fn default() -> Self {
        Self {
            targets: Vec::default(),
            outputs: BTreeMap::default(),
            models: Vec::default(),
        }
    }
}

impl PolymodelGenerator {
    pub fn parse(yaml: &str) -> anyhow::Result<Self> {
        let gener = serde_yaml::from_str::<PolymodelGenerator>(yaml)?;

        Ok(gener)
    }

    pub fn targets(&self) -> &Vec<PolymodelTarget> {
        &self.targets
    }

    pub fn outputs(&self) -> &PolymodelOutputs {
        &self.outputs
    }

    pub fn models(&self) -> &PolymodelList {
        &self.models
    }

    pub fn export(&self) -> anyhow::Result<Vec<Vec<(PathBuf, String)>>> {
        let mut res = Vec::new();
        let outputs = self.outputs();
        let models = self.models();

        for target in self.targets() {
            let exporter = match target {
                PolymodelTarget::Rust => RustExporter::export(&outputs[target], models)?,
                PolymodelTarget::CSharp => CSharpExporter::export(&outputs[target], models)?,
                PolymodelTarget::Typescript => {
                    TypescriptExporter::export(&outputs[target], models)?
                }
            };

            res.push(exporter);
        }

        Ok(res)
    }
}
