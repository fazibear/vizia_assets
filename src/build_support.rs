use std::{env, fs, path::PathBuf};

pub fn generate() {
    let crate_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let (project_dir, manifest, manifest_path) = find_consumer(&out_dir, &crate_dir)
        .expect("consumer Cargo.toml must contain [package.metadata.vizia-assets]");
    println!("cargo:rerun-if-changed={}", manifest_path.display());

    let config = asset_config(&manifest).expect("[package.metadata.vizia-assets]");
    let asset_dir = project_dir.join(
        config["asset-dir"]
            .as_str()
            .expect("asset-dir must be a string"),
    );
    println!("cargo:rerun-if-changed={}", asset_dir.display());

    let mut body = String::new();

    for key in ["images", "fonts", "stylesheets"] {
        for asset in config[key]
            .as_array()
            .expect("asset entries must be arrays")
            .iter()
            .flat_map(|asset| {
                let pattern = asset.as_str().expect("asset name must be a string");
                let pattern_path = asset_dir.join(pattern);
                let pattern = pattern_path.to_string_lossy();
                let mut matches = glob::glob(&pattern)
                    .unwrap_or_else(|error| panic!("invalid asset glob {pattern}: {error}"))
                    .map(|path| path.expect("failed to read asset path"))
                    .filter(|path| path.is_file())
                    .collect::<Vec<_>>();
                matches.sort();
                if matches.is_empty() && !has_wildcards(&pattern) {
                    matches.push(pattern_path);
                } else if matches.is_empty() {
                    panic!("asset glob matched no files: {pattern}");
                }
                matches
            })
        {
            println!("cargo:rerun-if-changed={}", asset.display());
            let extension = asset
                .extension()
                .and_then(|extension| extension.to_str())
                .unwrap_or_default();
            let name = asset
                .file_stem()
                .and_then(|name| name.to_str())
                .expect("asset name must be valid UTF-8");
            let path = asset.to_string_lossy();
            match key {
                "images" if extension.eq_ignore_ascii_case("svg") => {
                    body.push_str(&format!("    svg!(cx, {name}, {path:?});\n"));
                }
                "images" if extension.eq_ignore_ascii_case("png") => {
                    body.push_str(&format!("    png!(cx, {name}, {path:?});\n"));
                }
                "images" => panic!("unsupported image format: {}", asset.display()),
                "fonts" => {
                    body.push_str(&format!("    cx.add_font_mem(include_bytes!({path:?}));\n"));
                }
                "stylesheets" => {
                    body.push_str(&format!("    css!(cx, {name}, {path:?})?;\n"));
                }
                _ => unreachable!(),
            }
        }
    }

    body.insert_str(0, "{\n");
    body.push_str("    Ok(())\n}\n");
    let output = out_dir.join("load_assets_body");
    fs::write(output, body).expect("write generated asset loader body");
}

fn has_wildcards(pattern: &str) -> bool {
    pattern
        .chars()
        .any(|character| matches!(character, '*' | '?' | '[' | '{'))
}

fn asset_config(manifest: &toml::Value) -> Option<&toml::Value> {
    manifest
        .get("package")
        .and_then(|package| package.get("metadata"))
        .and_then(|metadata| metadata.get("vizia-assets"))
        .or_else(|| manifest.get("vizia-assets"))
}

fn find_consumer(
    out_dir: &std::path::Path,
    crate_dir: &std::path::Path,
) -> Option<(PathBuf, toml::Value, PathBuf)> {
    let mut directories = Vec::new();
    for ancestor in out_dir.ancestors() {
        directories.push(ancestor.to_path_buf());
        if let Ok(entries) = fs::read_dir(ancestor) {
            directories.extend(entries.filter_map(Result::ok).map(|entry| entry.path()));
        }
    }
    if let Some(parent) = crate_dir.parent() {
        if let Ok(entries) = fs::read_dir(parent) {
            directories.extend(entries.filter_map(Result::ok).map(|entry| entry.path()));
        }
    }

    directories.into_iter().find_map(|directory| {
        let manifest_path = directory.join("Cargo.toml");
        let text = fs::read_to_string(&manifest_path).ok()?;
        let manifest = toml::from_str::<toml::Value>(&text).ok()?;
        asset_config(&manifest)?;
        Some((directory, manifest, manifest_path))
    })
}
