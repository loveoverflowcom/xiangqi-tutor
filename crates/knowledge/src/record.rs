use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub schema_version: u32,
    pub id: String,
    pub revision: u32,
    pub title: String,
    pub language: String,
    pub review: Review,
    pub dependencies: Vec<Dependency>,
    pub payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Review {
    pub state: ReviewState,
    pub by: Option<String>,
    pub note: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewState {
    Draft,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dependency {
    pub id: String,
    pub revision: u32,
    pub content_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Payload {
    Source {
        locator: String,
        asset_sha256: String,
        rights: Rights,
    },
    Position {
        fen: String,
        ruleset: String,
        history: History,
        origin: SourceSpan,
    },
    Claim {
        claim_type: ClaimType,
        statement: String,
        scope: Scope,
        position_ids: Vec<String>,
        sources: Vec<SourceSpan>,
        conditions: Vec<Atom>,
        exceptions: Vec<Atom>,
        conclusion: Atom,
        evidence_status: EvidenceStatus,
        formal_status: FormalStatus,
        evidence: Vec<Evidence>,
        relations: Vec<Relation>,
    },
}

impl Payload {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Source { .. } => "source",
            Self::Position { .. } => "position",
            Self::Claim { .. } => "claim",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Rights {
    Private,
    Redistributable,
    Original,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum History {
    Unknown,
    Known {
        initial_fen: String,
        moves: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceSpan {
    pub source_id: String,
    pub pdf_page: u32,
    pub printed_page: Option<String>,
    pub region: Option<[f64; 4]>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimType {
    Observation,
    Rule,
    Heuristic,
    EngineAssessment,
    Hypothesis,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    Position,
    Pattern,
    Universal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceStatus {
    Unassessed,
    Supported,
    Challenged,
    Conflicted,
}

// Kernel-checked is deliberately NOT a writable metadata status in v1.
// A future checker must produce an attestation; author text cannot certify itself.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormalStatus {
    NotFormalized,
    StatementOnly,
    ProofPending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Atom {
    pub predicate: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub kind: EvidenceKind,
    pub reference: String,
    pub artifact_sha256: String,
    pub note: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    Source,
    Human,
    Engine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Relation {
    pub kind: RelationKind,
    pub target_id: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationKind {
    Supports,
    Challenges,
    Refines,
    Supersedes,
}
