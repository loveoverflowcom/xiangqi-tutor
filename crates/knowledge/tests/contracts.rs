use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};
use xqt_knowledge::{
    DiagnosticCode, fen::validate_fen, index::build_index_plan, load_corpus, parse_document, sha256,
};

static SEQ: AtomicUsize = AtomicUsize::new(0);

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "xqt-test-{}-{}",
            std::process::id(),
            SEQ.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&root).unwrap();
        for entry in
            fs::read_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../knowledge/examples"))
                .unwrap()
        {
            let entry = entry.unwrap();
            fs::copy(entry.path(), root.join(entry.file_name())).unwrap();
        }
        Self(root)
    }
    fn edit(&self, filename: &str, f: impl FnOnce(&mut Value)) {
        let path = self.0.join(filename);
        let text = fs::read_to_string(&path).unwrap();
        let start = text.find("```xqt\n").unwrap() + 7;
        let end = start + text[start..].find("\n```").unwrap();
        let mut value: Value = serde_json::from_str(&text[start..end]).unwrap();
        f(&mut value);
        fs::write(
            path,
            format!(
                "{}{}{}",
                &text[..start],
                serde_json::to_string_pretty(&value).unwrap(),
                &text[end..]
            ),
        )
        .unwrap();
    }
    fn repin(&self, filename: &str, id: &str, target: &str) {
        let digest = sha256(&fs::read(self.0.join(target)).unwrap());
        self.edit(filename, |v| {
            for dep in v["dependencies"].as_array_mut().unwrap() {
                if dep["id"] == id {
                    dep["content_sha256"] = digest.clone().into();
                }
            }
        });
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn example_corpus_exports_only_the_supported_approved_claim() {
    let f = Fixture::new();
    let corpus = load_corpus(&f.0).unwrap();
    assert_eq!(corpus.documents.len(), 4);
    let plan = build_index_plan(&corpus, true);
    assert_eq!(plan.chunks.len(), 1);
    assert_eq!(plan.chunks[0].record_id, "claim-explicit-turn");
    assert_eq!(plan.chunks[0].verification, "editorial_metadata_only");
    assert_eq!(plan.chunks[0].positions[0].ruleset, "unresolved");
    assert!(matches!(
        plan.chunks[0].positions[0].history,
        xqt_knowledge::record::History::Unknown
    ));
    assert_eq!(plan.embedding_status, "not_embedded");
    assert_eq!(plan.excluded.len(), 1);
    let first = serde_json::to_string(&plan).unwrap();
    let second = serde_json::to_string(&build_index_plan(&corpus, true)).unwrap();
    assert_eq!(first, second);
}

#[test]
fn edits_to_prose_invalidate_downstream_snapshots_even_without_revision_bump() {
    let f = Fixture::new();
    let path = f.0.join("source-demo.md");
    let text = fs::read_to_string(&path).unwrap();
    fs::write(path, format!("{text}\nA changed source note.\n")).unwrap();
    let errors = load_corpus(&f.0).unwrap_err();
    assert!(
        errors
            .iter()
            .any(|e| e.code == DiagnosticCode::StaleDependency)
    );
}

#[test]
fn missing_dependencies_fail_closed() {
    let f = Fixture::new();
    fs::remove_file(f.0.join("source-demo.md")).unwrap();
    assert!(
        load_corpus(&f.0)
            .unwrap_err()
            .iter()
            .any(|e| e.code == DiagnosticCode::MissingDependency)
    );
}

#[test]
fn duplicate_ids_fail() {
    let f = Fixture::new();
    fs::copy(f.0.join("source-demo.md"), f.0.join("copy.md")).unwrap();
    assert!(
        load_corpus(&f.0)
            .unwrap_err()
            .iter()
            .any(|e| e.code == DiagnosticCode::DuplicateId)
    );
}

#[test]
fn self_declared_kernel_proof_is_rejected() {
    let f = Fixture::new();
    f.edit("claim-explicit-turn.md", |v| {
        v["payload"]["formal_status"] = "kernel_checked".into()
    });
    assert!(
        load_corpus(&f.0)
            .unwrap_err()
            .iter()
            .any(|e| e.code == DiagnosticCode::Metadata)
    );
}

#[test]
fn arbitrary_metadata_fields_are_rejected() {
    let f = Fixture::new();
    f.edit("claim-explicit-turn.md", |v| {
        v["payload"]["trusted"] = true.into()
    });
    assert!(
        load_corpus(&f.0)
            .unwrap_err()
            .iter()
            .any(|e| e.code == DiagnosticCode::Metadata)
    );
}

#[test]
fn unknown_turn_and_wrong_board_dimensions_are_not_accepted() {
    let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
    assert!(validate_fen(fen).is_ok());
    assert!(validate_fen(&fen.replace(" w ", " ? ")).is_err());
    assert!(validate_fen(&fen.replacen("/9/", "/8/", 1)).is_err());
}

#[test]
fn incomplete_or_doubled_metadata_is_rejected() {
    assert!(parse_document(Path::new("bad.md"), "# No contract").is_err());
    let f = Fixture::new();
    let text = fs::read_to_string(f.0.join("source-demo.md")).unwrap();
    assert!(parse_document(Path::new("bad.md"), &format!("{text}\n{text}")).is_err());
}

#[test]
fn conflicting_claims_are_excluded_without_being_deleted() {
    let f = Fixture::new();
    f.edit("claim-explicit-turn.md", |v| {
        v["payload"]["evidence_status"] = "conflicted".into()
    });
    let corpus = load_corpus(&f.0).unwrap();
    assert_eq!(corpus.documents.len(), 4);
    assert!(build_index_plan(&corpus, false).chunks.is_empty());
}

#[test]
fn private_source_exclusion_propagates_through_position_to_claim() {
    let f = Fixture::new();
    f.edit("source-demo.md", |v| {
        v["payload"]["rights"] = "private".into()
    });
    f.repin("position-start.md", "source-demo", "source-demo.md");
    for name in ["claim-explicit-turn.md", "claim-open-file.md"] {
        f.repin(name, "source-demo", "source-demo.md");
        f.repin(name, "position-start", "position-start.md");
    }
    let corpus = load_corpus(&f.0).unwrap();
    assert_eq!(build_index_plan(&corpus, false).chunks.len(), 1);
    assert!(build_index_plan(&corpus, true).chunks.is_empty());
}

#[test]
fn rejected_dependency_is_not_index_eligible() {
    let f = Fixture::new();
    f.edit("position-start.md", |v| {
        v["review"]["state"] = "rejected".into()
    });
    for name in ["claim-explicit-turn.md", "claim-open-file.md"] {
        f.repin(name, "position-start", "position-start.md");
    }
    assert!(
        build_index_plan(&load_corpus(&f.0).unwrap(), false)
            .chunks
            .is_empty()
    );
}

#[test]
fn source_reference_cannot_point_to_a_position() {
    let f = Fixture::new();
    f.edit("claim-explicit-turn.md", |v| {
        v["payload"]["sources"][0]["source_id"] = "position-start".into()
    });
    assert!(
        load_corpus(&f.0)
            .unwrap_err()
            .iter()
            .any(|e| e.code == DiagnosticCode::ReferenceType)
    );
}

#[test]
fn argumentative_links_can_be_cyclic_without_becoming_derivation_cycles() {
    let f = Fixture::new();
    f.edit(
        "claim-explicit-turn.md",
        |v| {
            v["payload"]["relations"] =
                serde_json::json!([{"kind":"challenges","target_id":"claim-open-file"}])
        },
    );
    f.edit(
        "claim-open-file.md",
        |v| {
            v["payload"]["relations"] =
                serde_json::json!([{"kind":"challenges","target_id":"claim-explicit-turn"}])
        },
    );
    assert!(load_corpus(&f.0).is_ok());
}

#[test]
fn invalid_regions_and_missing_reasoning_sections_fail() {
    let f = Fixture::new();
    f.edit("claim-explicit-turn.md", |v| {
        v["payload"]["sources"][0]["region"] = serde_json::json!([0.9, 0.0, 0.5, 1.0])
    });
    assert!(load_corpus(&f.0).is_err());
    let text = fs::read_to_string(f.0.join("claim-open-file.md")).unwrap();
    assert!(
        parse_document(
            Path::new("bad.md"),
            &text.replace("## [reasoning]", "## Missing")
        )
        .is_err()
    );
}

#[test]
fn empty_corpus_and_unknown_schema_fail() {
    let f = Fixture::new();
    f.edit("claim-explicit-turn.md", |v| v["schema_version"] = 2.into());
    assert!(load_corpus(&f.0).is_err());
    let empty = f.0.join("empty");
    fs::create_dir(&empty).unwrap();
    assert!(load_corpus(&empty).is_err());
}
