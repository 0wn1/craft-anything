use craft_anything_formats::axml::{Manifest, manifest, parse};
use craft_anything_testkit::android::{AxNode, AxVal, a, axml};

/// Build the binary manifest for a provider with an optional SDK and exported attribute.
fn provider(min: Option<i32>, target: Option<i32>, exported: Option<bool>) -> Manifest {
    let sdk = min
        .map(|n| a("minSdkVersion", 0x0101_020c, AxVal::Int(n)))
        .into_iter()
        .chain(target.map(|n| a("targetSdkVersion", 0x0101_0270, AxVal::Int(n))))
        .collect();
    let mut attrs = vec![a("name", 0x0101_0003, AxVal::Str("Provider"))];
    if let Some(value) = exported {
        attrs.push(a("exported", 0x0101_0010, AxVal::Bool(value)));
    }
    let bytes = axml(&[
        AxNode::Start("manifest", vec![]),
        AxNode::Start("uses-sdk", sdk),
        AxNode::End("uses-sdk"),
        AxNode::Start("application", vec![]),
        AxNode::Start("provider", attrs),
        AxNode::End("provider"),
        AxNode::End("application"),
        AxNode::End("manifest"),
    ]);
    manifest(&parse(&bytes).unwrap())
}

/// A missing target defaults to the minimum SDK for the provider export rule.
#[test]
fn uses_minimum_sdk_for_implicit_provider_exports() {
    let result = provider(Some(23), None, None);
    assert_eq!(result.min_sdk.as_deref(), Some("23"));
    assert!(result.target_sdk.is_none());
    assert!(!result.components[0].exported);
}

/// Older minimum SDKs, explicit targets and explicit exported attributes retain precedence.
#[test]
fn keeps_legacy_and_explicit_provider_exports() {
    assert!(provider(Some(16), None, None).components[0].exported);
    assert!(provider(None, None, None).components[0].exported);
    assert!(!provider(Some(16), Some(23), None).components[0].exported);
    assert!(provider(Some(23), None, Some(true)).components[0].exported);
}
