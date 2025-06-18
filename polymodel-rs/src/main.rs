use std::{fs::File, io::Write, path::Path};

use crate::generator::PolymodelGenerator;

mod exporter;
mod generator;

const YAML: &'static str = r#"
targets: [csharp,rust,typescript]
outputs:
  rust:
      dir: test_outputs/rust_output
  typescript:
      dir: test_outputs/typescript_output
  csharp:
      dir: test_outputs/csharp_output
models:
  - user:
      email: email
      password: string
      id: number
  - vehicle:
      id: string
      nickname: string
  - site_config:
      site_id: string
      config: string
  - account:
      balance: float
      accountNumber: number

"#;

fn main() -> anyhow::Result<()> {
    let g = PolymodelGenerator::parse(YAML)?;

    let outputs = g.export()?;
    for output in outputs {
        for (path, value) in output {
            let parent = Path::new(&path).parent().unwrap();
            std::fs::create_dir_all(&parent)?;
            let mut file = File::create(path)?;
            file.write_all(value.as_bytes())?;
        }
    }

    Ok(())
}
