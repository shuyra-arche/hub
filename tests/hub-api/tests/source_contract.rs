use std::fs;
use std::path::{Path, PathBuf};

fn api() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../api")
}

fn read(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    fs::read_to_string(path).unwrap_or_else(|error| {
        panic!("read {}: {error}", path.display())
    })
}

fn dependencies(manifest: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut inside = false;
    for line in manifest.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            inside = line == "[dependencies]";
            continue;
        }
        if inside && !line.is_empty() && !line.starts_with('#') {
            if let Some((name, _)) = line.split_once('=') {
                result.push(name.trim().to_owned());
            }
        }
    }
    result
}

/// Source system-shape `lib.rs:209`: the coordination API carries no IOVA owner.
#[test]
fn coordination_api_has_no_iova_ownership() {
    let coord = read(api().join("src/coord.rs"));
    assert!(!coord.contains("IOVA"));
}

/// Source system-shape `lib.rs:255`: the DMA claim has no requester-trust shape.
/// The claim type now lives in `frame/dma/dma_claim.rs`; the check covers the
/// owning file and the whole split `dma` module.
#[test]
fn dma_claim_has_no_requester_trust_shape() {
    let dma = api().join("src/frame/dma");
    let claim = dma.join("dma_claim.rs");
    assert!(claim.is_file(), "DmaClaim owner file is missing");
    let mut source = read(api().join("src/frame/dma.rs"));
    let mut children = fs::read_dir(&dma)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    children.sort();
    for child in children {
        if child.extension().is_some_and(|extension| extension == "rs") {
            source.push_str(&read(&child));
        }
    }
    assert!(!source.contains("pub requester: u32,\n    pub sensor"));
}

/// Source system-shape `lib.rs:575`: the API uses only the four allowed providers.
#[test]
fn hub_api_dependencies_stay_env_wire_mold_system_api() {
    let manifest = read(api().join("Cargo.toml"));
    let dependencies = dependencies(&manifest);
    assert!(
        dependencies
            .iter()
            .all(|name| ["env", "wire", "mold", "system-api"].contains(&name.as_str())),
        "unexpected hub-api dependency: {dependencies:?}"
    );
}

/// Source system-shape `lib.rs:710`: the provider metadata is generated in the API.
#[test]
fn hub_api_generates_interface_metadata() {
    let source = read(api().join("src/lib.rs"));
    assert!(source.contains("mold::interface"));
}
