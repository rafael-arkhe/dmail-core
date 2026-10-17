use serde::{Serialize, Deserialize, Serializer};

use ciborium::Value;

// We wrap the Digest to ensure it serializes as a byte string in CBOR, not as an array of ints.
pub type Digest = [u8; 32];

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DigestWrapper(#[serde(with = "serde_bytes")] pub Digest);

impl Serialize for DigestWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

// Domain strings
pub const DOMAIN_VERIFY: &str = "AKG:VERIFY:v1\0";
pub const DOMAIN_ASSESS: &str = "AKG:ASSESS:v1\0";
pub const DOMAIN_BRANCH: &str = "AKG:BRANCH:v1\0";
pub const DOMAIN_FINDING: &str = "AKG:FINDING:v1\0";
pub const DOMAIN_ENV: &str = "AKG:ENV:v1\0";
pub const DOMAIN_ENV_COMP: &str = "AKG:ENVCOMP:v1\0";
pub const DOMAIN_POLICY: &str = "AKG:POLICY:v1\0";

// B02-01 PolicyRef
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyRef {
    #[serde(with = "serde_bytes")]
    pub digest: Digest,
}

// B02-02 IdentityRef
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityRef {
    pub scheme: String,
    pub value: String,
}

// B02-03 PropertyRef
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropertyRef {
    pub name: String,
    pub version: u32,
}

// B02-04 ScopeExpr
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ScopeExpr {
    #[serde(rename = "artifact_set")]
    ArtifactSet { values: Vec<String> },
    #[serde(rename = "component")]
    Component { value: String },
    #[serde(rename = "symbol_set")]
    SymbolSet { values: Vec<String> },
}

// B02-05 Finding and FindingDisposition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum SubjectRef {
    #[serde(rename = "digest")]
    Digest { #[serde(with = "serde_bytes")] value: Digest },
    #[serde(rename = "id")]
    Id { value: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceRef {
    pub evidence_class: String,
    #[serde(with = "serde_bytes")]
    pub content_digest: Digest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_ref: Option<IdentityRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    pub code: String,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_ref: Option<SubjectRef>,
    pub evidence_refs: Vec<EvidenceRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "serde_bytes_opt")]
    pub details_ref: Option<Digest>,
}

mod serde_bytes_opt {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(val: &Option<Digest>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(v) = val {
            serializer.serialize_bytes(v)
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Digest>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<serde_bytes::ByteBuf> = Option::deserialize(deserializer)?;
        if let Some(buf) = opt {
            let slice = buf.as_slice();
            if slice.len() == 32 {
                let mut arr = [0u8; 32];
                arr.copy_from_slice(slice);
                Ok(Some(arr))
            } else {
                Err(serde::de::Error::custom("expected 32 bytes"))
            }
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindingDisposition {
    #[serde(with = "serde_bytes")]
    pub finding_ref: Digest,
    pub disposition: String, // INFO | WARNING | BLOCKING
    pub policy_ref: PolicyRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "serde_bytes_opt")]
    pub rationale_ref: Option<Digest>,
}

// B02-06 EnvironmentDescriptor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Measurement {
    pub mkey: String,
    #[serde(with = "serde_bytes")]
    pub mval: Digest,
    pub algorithm: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_by: Option<IdentityRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentDescriptorUnsigned {
    pub environment_version: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<IdentityRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<IdentityRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<IdentityRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolchain: Option<IdentityRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tee_type: Option<String>,
    pub measurements: Vec<Measurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "serde_bytes_opt")]
    pub container_image: Option<Digest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "serde_bytes_opt")]
    pub dependencies_digest: Option<Digest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "serde_bytes_opt")]
    pub config_digest: Option<Digest>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentDescriptor {
    pub environment_version: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<IdentityRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<IdentityRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<IdentityRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolchain: Option<IdentityRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tee_type: Option<String>,
    pub measurements: Vec<Measurement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "serde_bytes_opt")]
    pub container_image: Option<Digest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "serde_bytes_opt")]
    pub dependencies_digest: Option<Digest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "serde_bytes_opt")]
    pub config_digest: Option<Digest>,
    #[serde(with = "serde_bytes")]
    pub environment_digest: Digest,
}

// B02-07 ExplicitOrderingEdge
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderingEdge {
    #[serde(with = "serde_bytes")]
    pub source: Digest,
    #[serde(with = "serde_bytes")]
    pub target: Digest,
    pub kind: String, // "EXPLICIT_ORDERING"
    #[serde(with = "serde_bytes")]
    pub reason_ref: Digest,
}

// B02-08 StrongStateCommutation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrongStateCommutationPayload {
    #[serde(with = "serde_bytes")]
    pub subject_a: Digest,
    #[serde(with = "serde_bytes")]
    pub subject_b: Digest,
    #[serde(with = "serde_bytes")]
    pub state_profile_ref: Digest,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "serde_bytes_opt")]
    pub preconditions_ref: Option<Digest>,
    pub equivalence_kind: String, // "exact_canonical_state_equality"
    #[serde(with = "serde_bytes")]
    pub proof_digest: Digest,
}

// B02-11 SignatureEnvelope
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignatureEnvelope {
    pub algorithm_profile: String, // "AKG-SIG-ED25519-v1"
    pub key_ref: IdentityRef,
    #[serde(with = "serde_bytes")]
    pub signature_bytes: Vec<u8>,
}

// B02-12 BranchIntegrationStateCore and BranchAttestation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyEdge {
    #[serde(with = "serde_bytes")]
    pub source: Digest,
    #[serde(with = "serde_bytes")]
    pub target: Digest,
    pub kind: String, // "STRUCTURAL" | "SEMANTIC" | "VERIFICATION"
}

mod serde_bytes_vec {
    use super::*;
    use serde::{Serializer, ser::SerializeSeq};

    pub fn serialize<S>(val: &Vec<Digest>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(val.len()))?;
        for e in val {
            seq.serialize_element(&serde_bytes::Bytes::new(e))?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D>(_deserializer: D) -> Result<Vec<Digest>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        unimplemented!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchIntegrationStateCore {
    pub branch_id: String,
    pub state_version: u16,
    #[serde(with = "serde_bytes_vec")]
    pub integrated_intents: Vec<Digest>,
    pub dependency_graph: Vec<DependencyEdge>,
    pub explicit_ordering_edges: Vec<OrderingEdge>,
    #[serde(with = "serde_bytes_vec")]
    pub integration_frontier: Vec<Digest>,
    pub policy_ref: PolicyRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchAttestation {
    #[serde(with = "serde_bytes")]
    pub branch_digest: Digest,
    pub signer: IdentityRef,
    pub signature: SignatureEnvelope,
}

pub fn object_digest<T: Serialize>(domain: &str, unsigned_value: &T) -> Digest {
    let canonical_bytes = canonical_cbor(unsigned_value);

    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    hasher.update(&canonical_bytes);

    *hasher.finalize().as_bytes()
}

fn sort_value(value: &mut Value) {
    match value {
        Value::Map(map) => {
            for (_, v) in map.iter_mut() {
                sort_value(v);
            }
            // Sort by CBOR byte length, then lexicographically.
            map.sort_by(|(k1, _), (k2, _)| {
                let mut buf1 = Vec::new();
                let mut buf2 = Vec::new();
                ciborium::into_writer(k1, &mut buf1).unwrap();
                ciborium::into_writer(k2, &mut buf2).unwrap();
                buf1.len().cmp(&buf2.len()).then_with(|| buf1.cmp(&buf2))
            });
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                sort_value(v);
            }
            // In a complete implementation, this is where we'd sort semantic sets.
            // As a simplification for the PoC, we will assume they're already
            // pre-sorted where required, or we just sort everything as if it's a semantic set
            // for canonicalization purposes. For safety, we only sort map keys universally.
        }
        _ => {}
    }
}

pub fn canonical_cbor<T: Serialize>(value: &T) -> Vec<u8> {
    let mut buf = Vec::new();
    ciborium::into_writer(value, &mut buf).unwrap();
    let mut ciborium_val: Value = ciborium::from_reader(buf.as_slice()).unwrap();
    sort_value(&mut ciborium_val);
    let mut canonical_buf = Vec::new();
    ciborium::into_writer(&ciborium_val, &mut canonical_buf).unwrap();
    canonical_buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_ref() {
        let policy_ref = PolicyRef {
            digest: [1u8; 32],
        };
        let cbor = canonical_cbor(&policy_ref);
        assert!(!cbor.is_empty());
        let digest = object_digest(DOMAIN_POLICY, &policy_ref);
        assert_eq!(digest.len(), 32);
    }

    #[test]
    fn test_finding_disposition() {
        let disp = FindingDisposition {
            finding_ref: [2u8; 32],
            disposition: "BLOCKING".to_string(),
            policy_ref: PolicyRef { digest: [3u8; 32] },
            rationale_ref: None,
        };
        let cbor = canonical_cbor(&disp);
        assert!(!cbor.is_empty());
    }

    #[test]
    fn test_identity_ref() {
        let ident = IdentityRef {
            scheme: "did".to_string(),
            value: "did:example:123".to_string(),
        };
        let digest = object_digest(DOMAIN_VERIFY, &ident);
        assert_eq!(digest.len(), 32);
    }
}
