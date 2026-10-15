use crate::{Corpus, Document, record::*, sha256};
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Debug, Serialize)]
pub struct IndexPlan {
    pub schema_version: u32,
    pub audience: &'static str,
    pub corpus_sha256: String,
    pub embedding_status: &'static str,
    pub chunks: Vec<Chunk>,
    pub excluded: Vec<Excluded>,
}

#[derive(Debug, Serialize)]
pub struct Chunk {
    pub chunk_id: String,
    pub record_id: String,
    pub revision: u32,
    pub content_sha256: String,
    pub statement: String,
    pub language: String,
    pub claim_type: ClaimType,
    pub evidence_status: EvidenceStatus,
    pub formal_status: FormalStatus,
    pub verification: &'static str,
    pub sources: Vec<SourceSpan>,
    pub dependencies: Vec<Dependency>,
    pub positions: Vec<PositionContext>,
    pub markdown: String,
}

#[derive(Debug, Serialize)]
pub struct PositionContext {
    pub record_id: String,
    pub revision: u32,
    pub content_sha256: String,
    pub fen: String,
    pub ruleset: String,
    pub history: History,
}

#[derive(Debug, Serialize)]
pub struct Excluded {
    pub record_id: String,
    pub reason: &'static str,
}

fn local_gate(doc: &Document, public: bool) -> bool {
    if doc.record.review.state != ReviewState::Approved {
        return false;
    }
    match &doc.record.payload {
        Payload::Source { rights, .. } => !public || *rights != Rights::Private,
        Payload::Claim {
            evidence_status, ..
        } => *evidence_status == EvidenceStatus::Supported,
        Payload::Position { .. } => true,
    }
}

pub fn build_index_plan(corpus: &Corpus, public: bool) -> IndexPlan {
    let mut ready = BTreeSet::new();
    loop {
        let next: Vec<_> = corpus
            .documents
            .iter()
            .filter(|(id, d)| {
                !ready.contains(*id)
                    && local_gate(d, public)
                    && d.record
                        .dependencies
                        .iter()
                        .all(|dep| ready.contains(&dep.id))
            })
            .map(|(id, _)| id.clone())
            .collect();
        if next.is_empty() {
            break;
        }
        ready.extend(next);
    }
    let manifest = corpus
        .documents
        .iter()
        .map(|(id, doc)| format!("{id}:{}:{}\n", doc.record.revision, doc.content_sha256))
        .collect::<String>();
    let mut chunks = Vec::new();
    let mut excluded = Vec::new();
    for (id, doc) in &corpus.documents {
        let Payload::Claim {
            statement,
            claim_type,
            evidence_status,
            formal_status,
            sources,
            position_ids,
            ..
        } = &doc.record.payload
        else {
            continue;
        };
        if !ready.contains(id) {
            excluded.push(Excluded {
                record_id: id.clone(),
                reason: if !local_gate(doc, public) {
                    "record_not_approved_and_supported"
                } else {
                    "dependency_not_eligible_for_audience"
                },
            });
            continue;
        }
        chunks.push(Chunk {
            chunk_id: sha256(
                format!("xqt-v1:{id}:{}:{}", doc.record.revision, doc.content_sha256).as_bytes(),
            ),
            record_id: id.clone(),
            revision: doc.record.revision,
            content_sha256: doc.content_sha256.clone(),
            statement: statement.clone(),
            language: doc.record.language.clone(),
            claim_type: *claim_type,
            evidence_status: *evidence_status,
            formal_status: *formal_status,
            verification: "editorial_metadata_only",
            sources: sources.clone(),
            dependencies: doc.record.dependencies.clone(),
            positions: position_ids
                .iter()
                .filter_map(|p| {
                    let position = corpus.documents.get(p)?;
                    match &position.record.payload {
                        Payload::Position {
                            fen,
                            ruleset,
                            history,
                            ..
                        } => Some(PositionContext {
                            record_id: p.clone(),
                            revision: position.record.revision,
                            content_sha256: position.content_sha256.clone(),
                            fen: fen.clone(),
                            ruleset: ruleset.clone(),
                            history: history.clone(),
                        }),
                        _ => None,
                    }
                })
                .collect(),
            markdown: doc.body.clone(),
        });
    }
    IndexPlan {
        schema_version: 1,
        audience: if public { "public" } else { "private" },
        corpus_sha256: sha256(manifest.as_bytes()),
        embedding_status: "not_embedded",
        chunks,
        excluded,
    }
}
