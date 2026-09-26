//! NEARA package — NEAR token launchpad tools over a keyless hosted MCP server.
//! Assets: the manifest plus one pinned tool's input schema and prompt doc (no
//! bundled WASM; dispatched via MCP). The asset directory is `neara-mcp/` while
//! the in-package schema/prompt paths use `neara/`.

use std::borrow::Cow;

use ironclaw_host_api::capability::EffectKind;

use super::{PackageBundle, PackageOnboarding, bytes_asset};

pub(super) const ID: &str = "neara";

const MANIFEST: &str = include_str!("../../../packages/neara-mcp/manifest.toml");

pub(super) fn bundle() -> PackageBundle {
    PackageBundle {
        id: ID,
        display_name: "NEARA",
        manifest_toml: Cow::Borrowed(MANIFEST),
        assets: vec![
            bytes_asset("manifest.toml", MANIFEST.as_bytes()),
            bytes_asset(
                "schemas/neara/list_tokens.input.v1.json",
                include_bytes!(
                    "../../../packages/neara-mcp/schemas/neara/list_tokens.input.v1.json"
                ),
            ),
            bytes_asset(
                "prompts/neara/list_tokens.md",
                include_bytes!("../../../packages/neara-mcp/prompts/neara/list_tokens.md"),
            ),
        ],
        onboarding: Some(PackageOnboarding {
            instructions: "NEARA needs no account or key: its tools read public chain data and \
                build unsigned transactions."
                .to_string(),
            credential_instructions: None,
            setup_url: None,
            credential_next_step: "Build tools return a sign_url; the account owner approves each \
                transaction in their own NEAR wallet."
                .to_string(),
        }),
        // Keyless MCP extension: Dispatch + Network.
        trust_effects: Some(vec![EffectKind::DispatchCapability, EffectKind::Network]),
    }
}
