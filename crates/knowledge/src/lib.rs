pub mod fen;
pub mod index;
pub mod record;

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag, TagEnd};
use record::*;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct Document {
    pub path: PathBuf,
    pub record: Record,
    pub body: String,
    pub content_sha256: String,
}

#[derive(Debug)]
pub struct Corpus {
    pub documents: BTreeMap<String, Document>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticCode {
    Io,
    Metadata,
    InvalidRecord,
    Fen,
    DuplicateId,
    MissingDependency,
    StaleDependency,
    MissingReference,
    ReferenceType,
    Cycle,
}

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub path: String,
    pub code: DiagnosticCode,
    pub message: String,
}

fn diagnostic(path: &Path, code: DiagnosticCode, message: impl Into<String>) -> Diagnostic {
    Diagnostic {
        path: path.display().to_string(),
        code,
        message: message.into(),
    }
}

pub fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 100
        && id
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
}

fn valid_digest(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// One fenced `xqt` JSON block is the machine contract. All other Markdown is prose.
pub fn parse_document(path: &Path, text: &str) -> Result<Document, Diagnostic> {
    let mut blocks = Vec::new();
    let mut active: Option<(usize, String)> = None;
    for (event, range) in Parser::new(text).into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) if info.as_ref() == "xqt" => {
                active = Some((range.start, String::new()));
            }
            Event::Text(value) => {
                if let Some((_, content)) = &mut active {
                    content.push_str(&value);
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some((start, content)) = active.take() {
                    blocks.push((start..range.end, content));
                }
            }
            _ => {}
        }
    }
    if blocks.len() != 1 {
        return Err(diagnostic(
            path,
            DiagnosticCode::Metadata,
            "expected exactly one fenced xqt JSON block",
        ));
    }
    let (range, json) = &blocks[0];
    let record: Record = serde_json::from_str(json)
        .map_err(|e| diagnostic(path, DiagnosticCode::Metadata, e.to_string()))?;
    let body = format!("{}{}", &text[..range.start], &text[range.end..]);
    let doc = Document {
        path: path.to_owned(),
        record,
        body,
        content_sha256: sha256(text.as_bytes()),
    };
    validate_document(&doc)?;
    Ok(doc)
}

fn headings(body: &str) -> BTreeSet<String> {
    let mut in_heading = false;
    let mut title = String::new();
    let mut result = BTreeSet::new();
    for event in Parser::new(body) {
        match event {
            Event::Start(Tag::Heading {
                level: HeadingLevel::H2,
                ..
            }) => {
                in_heading = true;
                title.clear();
            }
            Event::Text(t) | Event::Code(t) if in_heading => title.push_str(&t),
            Event::End(TagEnd::Heading(HeadingLevel::H2)) => {
                result.insert(title.clone());
                in_heading = false;
            }
            _ => {}
        }
    }
    result
}

fn validate_document(doc: &Document) -> Result<(), Diagnostic> {
    let r = &doc.record;
    let invalid = |msg: &str| diagnostic(&doc.path, DiagnosticCode::InvalidRecord, msg);
    if r.schema_version != 1 || r.revision == 0 || !valid_id(&r.id) || r.title.trim().is_empty() {
        return Err(invalid(
            "schema_version=1, positive revision, valid id and title are required",
        ));
    }
    if !matches!(r.language.as_str(), "vi" | "zh-Hans" | "zh-Hant" | "en") {
        return Err(invalid("unsupported language"));
    }
    if r.review.state == ReviewState::Approved
        && (r.review.by.as_deref().is_none_or(|v| v.trim().is_empty())
            || r.review.note.trim().is_empty())
    {
        return Err(invalid(
            "approved records require reviewer and note; approval is editorial, not a proof",
        ));
    }
    let mut ids = BTreeSet::new();
    for d in &r.dependencies {
        if !valid_id(&d.id)
            || d.revision == 0
            || !valid_digest(&d.content_sha256)
            || !ids.insert(&d.id)
        {
            return Err(invalid("invalid or duplicate dependency snapshot"));
        }
    }
    let check_span = |s: &SourceSpan| -> Result<(), Diagnostic> {
        if s.pdf_page == 0 || !ids.contains(&s.source_id) {
            return Err(invalid(
                "source span needs a declared dependency and 1-based PDF page",
            ));
        }
        if let Some([x, y, w, h]) = s.region
            && (![x, y, w, h].iter().all(|v| v.is_finite())
                || x < 0.0
                || y < 0.0
                || w <= 0.0
                || h <= 0.0
                || x + w > 1.0
                || y + h > 1.0)
        {
            return Err(invalid(
                "region must be normalized [x,y,width,height] within page",
            ));
        }
        Ok(())
    };
    match &r.payload {
        Payload::Source {
            locator,
            asset_sha256,
            ..
        } => {
            if locator.trim().is_empty() || !valid_digest(asset_sha256) {
                return Err(invalid(
                    "source requires locator and SHA-256 of original bytes",
                ));
            }
        }
        Payload::Position {
            fen,
            ruleset,
            history,
            origin,
        } => {
            fen::validate_fen(fen).map_err(|e| diagnostic(&doc.path, DiagnosticCode::Fen, e))?;
            if ruleset.trim().is_empty() {
                return Err(invalid(
                    "ruleset identifier is required; use unresolved when unknown",
                ));
            }
            if let History::Known { initial_fen, moves } = history {
                fen::validate_fen(initial_fen)
                    .map_err(|e| diagnostic(&doc.path, DiagnosticCode::Fen, e))?;
                if moves.iter().any(|m| !fen::valid_coordinate_move(m)) {
                    return Err(invalid(
                        "history moves require coordinate notation, e.g. a0a1; replay is not implemented",
                    ));
                }
            }
            check_span(origin)?;
        }
        Payload::Claim {
            statement,
            scope,
            position_ids,
            sources,
            conditions,
            exceptions,
            conclusion,
            evidence_status,
            evidence,
            relations,
            ..
        } => {
            if statement.trim().is_empty() || sources.is_empty() {
                return Err(invalid(
                    "claim needs a statement and at least one source span",
                ));
            }
            if matches!(scope, Scope::Position) && position_ids.is_empty() {
                return Err(invalid("position-scoped claim requires position_ids"));
            }
            for id in position_ids {
                if !ids.contains(id) {
                    return Err(invalid("claim references must have dependency snapshots"));
                }
            }
            if relations.iter().any(|r| !valid_id(&r.target_id)) {
                return Err(invalid("invalid relation target id"));
            }
            for source in sources {
                check_span(source)?;
            }
            for atom in conditions
                .iter()
                .chain(exceptions)
                .chain(std::iter::once(conclusion))
            {
                if !valid_id(&atom.predicate) || atom.args.iter().any(|v| v.trim().is_empty()) {
                    return Err(invalid(
                        "invalid predicate/argument; predicates are descriptive, not executed",
                    ));
                }
            }
            if *evidence_status != EvidenceStatus::Unassessed && evidence.is_empty() {
                return Err(invalid("assessed claims require evidence references"));
            }
            for e in evidence {
                if e.reference.trim().is_empty()
                    || e.note.trim().is_empty()
                    || !valid_digest(&e.artifact_sha256)
                {
                    return Err(invalid("evidence needs reference, SHA-256 and note"));
                }
            }
            let titles = headings(&doc.body);
            for required in ["[reasoning]", "[counterexamples]", "[questions]"] {
                if !titles.contains(required) {
                    return Err(invalid(&format!("missing H2 section {required}")));
                }
            }
        }
    }
    Ok(())
}

fn collect_paths(root: &Path, paths: &mut Vec<PathBuf>, errors: &mut Vec<Diagnostic>) {
    let meta = match fs::symlink_metadata(root) {
        Ok(m) => m,
        Err(e) => {
            errors.push(diagnostic(root, DiagnosticCode::Io, e.to_string()));
            return;
        }
    };
    if meta.file_type().is_symlink() {
        errors.push(diagnostic(
            root,
            DiagnosticCode::Io,
            "symlinks are not corpus inputs",
        ));
    } else if meta.is_dir() {
        match fs::read_dir(root) {
            Ok(entries) => {
                let mut children = Vec::new();
                for entry in entries {
                    match entry {
                        Ok(e) => children.push(e.path()),
                        Err(e) => errors.push(diagnostic(root, DiagnosticCode::Io, e.to_string())),
                    }
                }
                children.sort();
                for child in children {
                    collect_paths(&child, paths, errors);
                }
            }
            Err(e) => errors.push(diagnostic(root, DiagnosticCode::Io, e.to_string())),
        }
    } else if root.extension().is_some_and(|ext| ext == "md") {
        paths.push(root.to_owned());
    }
}

pub fn load_corpus(root: &Path) -> Result<Corpus, Vec<Diagnostic>> {
    let mut errors = Vec::new();
    let mut paths = Vec::new();
    collect_paths(root, &mut paths, &mut errors);
    let mut docs = BTreeMap::new();
    for path in paths {
        let result = fs::read_to_string(&path)
            .map_err(|e| diagnostic(&path, DiagnosticCode::Io, e.to_string()))
            .and_then(|s| parse_document(&path, &s));
        match result {
            Ok(doc) => {
                if docs.contains_key(&doc.record.id) {
                    errors.push(diagnostic(
                        &path,
                        DiagnosticCode::DuplicateId,
                        &doc.record.id,
                    ));
                } else {
                    docs.insert(doc.record.id.clone(), doc);
                }
            }
            Err(error) => errors.push(error),
        }
    }
    if docs.is_empty() {
        errors.push(diagnostic(
            root,
            DiagnosticCode::Metadata,
            "no valid Markdown records",
        ));
    }
    for doc in docs.values() {
        for dep in &doc.record.dependencies {
            match docs.get(&dep.id) {
                None => errors.push(diagnostic(
                    &doc.path,
                    DiagnosticCode::MissingDependency,
                    &dep.id,
                )),
                Some(other)
                    if other.record.revision != dep.revision
                        || other.content_sha256 != dep.content_sha256 =>
                {
                    errors.push(diagnostic(
                        &doc.path,
                        DiagnosticCode::StaleDependency,
                        &dep.id,
                    ));
                }
                _ => {}
            }
        }
        let mut refs = Vec::new();
        match &doc.record.payload {
            Payload::Position { origin, .. } => refs.push((&origin.source_id, "source")),
            Payload::Claim {
                position_ids,
                sources,
                relations,
                ..
            } => {
                refs.extend(position_ids.iter().map(|id| (id, "position")));
                refs.extend(sources.iter().map(|s| (&s.source_id, "source")));
                refs.extend(relations.iter().map(|r| (&r.target_id, "claim")));
            }
            _ => {}
        }
        for (id, expected) in refs {
            match docs.get(id) {
                Some(d) if d.record.payload.kind() != expected => {
                    errors.push(diagnostic(
                        &doc.path,
                        DiagnosticCode::ReferenceType,
                        format!("{id} must be {expected}"),
                    ));
                }
                None => errors.push(diagnostic(&doc.path, DiagnosticCode::MissingReference, id)),
                _ => {}
            }
        }
    }
    // Kahn traversal detects cycles without recursive dependency walking.
    let mut settled = BTreeSet::new();
    loop {
        let next: Vec<_> = docs
            .iter()
            .filter(|(id, d)| {
                !settled.contains(*id)
                    && d.record
                        .dependencies
                        .iter()
                        .all(|dep| settled.contains(&dep.id))
            })
            .map(|(id, _)| id.clone())
            .collect();
        if next.is_empty() {
            break;
        }
        settled.extend(next);
    }
    if errors.is_empty() && settled.len() != docs.len() {
        errors.push(diagnostic(
            root,
            DiagnosticCode::Cycle,
            "dependency graph contains a cycle",
        ));
    }
    if errors.is_empty() {
        Ok(Corpus { documents: docs })
    } else {
        Err(errors)
    }
}
