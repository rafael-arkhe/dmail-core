use arkhe_poc::b02::types::*;
use hex;

#[test]
fn generate_golden_vector_policy_ref() {
    let policy_ref = PolicyRef { digest: [0x42; 32] };

    let canonical = canonical_cbor(&policy_ref);
    let hex_cbor = hex::encode(&canonical);

    let expected_digest = object_digest(DOMAIN_POLICY, &policy_ref);
    let hex_digest = hex::encode(expected_digest);

    println!("PolicyRef Canonical CBOR: {}", hex_cbor);
    println!("PolicyRef Object Digest: {}", hex_digest);

    assert!(!hex_cbor.is_empty());
    assert_eq!(hex_digest.len(), 64);
}

#[test]
fn generate_golden_vector_identity_ref() {
    let ident = IdentityRef {
        scheme: "did".to_string(),
        value: "did:example:123".to_string(),
    };

    let canonical = canonical_cbor(&ident);
    let hex_cbor = hex::encode(&canonical);

    let expected_digest = object_digest(DOMAIN_VERIFY, &ident);
    let hex_digest = hex::encode(expected_digest);

    println!("IdentityRef Canonical CBOR: {}", hex_cbor);
    println!("IdentityRef Object Digest: {}", hex_digest);
}

#[test]
fn generate_golden_vector_finding_disposition() {
    let disp = FindingDisposition {
        finding_ref: [0x77; 32],
        disposition: "BLOCKING".to_string(),
        policy_ref: PolicyRef { digest: [0x42; 32] },
        rationale_ref: None,
    };

    let canonical = canonical_cbor(&disp);
    let hex_cbor = hex::encode(&canonical);

    let expected_digest = object_digest(DOMAIN_ASSESS, &disp);
    let hex_digest = hex::encode(expected_digest);

    println!("FindingDisposition Canonical CBOR: {}", hex_cbor);
    println!("FindingDisposition Object Digest: {}", hex_digest);
}

#[test]
fn generate_golden_vector_env_descriptor() {
    let env = EnvironmentDescriptorUnsigned {
        environment_version: 1,
        os: Some(IdentityRef {
            scheme: "os".to_string(),
            value: "linux".to_string(),
        }),
        architecture: Some(IdentityRef {
            scheme: "arch".to_string(),
            value: "x86_64".to_string(),
        }),
        runtime: None,
        toolchain: None,
        tee_type: Some("INTEL_TDX".to_string()),
        measurements: vec![Measurement {
            mkey: "host_fw".to_string(),
            mval: [0x11; 32],
            algorithm: "BLAKE3".to_string(),
            authorized_by: None,
        }],
        container_image: None,
        dependencies_digest: None,
        config_digest: None,
    };

    let canonical = canonical_cbor(&env);
    let expected_digest = object_digest(DOMAIN_ENV, &env);

    println!(
        "EnvironmentDescriptorUnsigned CBOR: {}",
        hex::encode(&canonical)
    );
    println!(
        "EnvironmentDescriptor Digest: {}",
        hex::encode(expected_digest)
    );
}
