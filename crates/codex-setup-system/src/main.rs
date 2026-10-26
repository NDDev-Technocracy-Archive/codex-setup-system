//! The Codex CLI setup system.
//!
//! This file is the harness's *facts*. Every command over them lives in
//! [`harness_runtime`], shared with every other setup system, so a change to
//! behaviour lands once and a change to Codex CLI's surface lands here.
//!
//! This provider owns the configuration *and* the program: `src/software.rs`
//! carries the artifacts its vendor publishes -- codex's are the ones whose member
//! path carries the target triple, so they genuinely differ per platform -- and
//! the software operations take a `--prefix` distinct from the `--target`.
//!
//! This line used to say *"Codex installs itself, so this provider owns the
//! configuration only"*, which was the owner's original assignment rather than
//! what the build does, and false from `7d156c2` onward.

use std::process::ExitCode;

mod software;

#[cfg(test)]
mod project_tests;

use harness_runtime::{Harness, LaunchBinding, PreservationSurface, Scoped, Shadow};
use provider_v3::{ComponentKind, ProjectionKind, TargetScope};

/// Everything specific to Codex CLI, verified against `codex-baseline.json`.
pub const CODEX: Harness = Harness {
    harness_id: "codex",
    provider_id: "codex-setup-system",
    version: env!("CARGO_PKG_VERSION"),
    product: "Codex CLI",
    vendor: "OpenAI",
    documented_config_home: "~/.codex",
    config_home_env: "CODEX_HOME",
    // Measured 2026-08-28 by asking: launched through this provider, the
    // product named the target as its own `codex_home` in its output.
    launch_binding: LaunchBinding::Complete {
        how: "measured by asking the product which home it resolved",
    },
    // **Asked, and there is none.** Measured 2026-08-31 against the pinned
    // 0.151.0 artifact, its digest checked against the artifact table:
    // 26 `CODEX_*` names appear in it, and not one of them
    // carries `UPDATE` or `UPGRADE`. An invented name was searched in the
    // same run and found zero times, so the search discriminates.
    //
    // Empty here used to mean nobody had looked, which reads the same as
    // this and is a different statement. Three of the seven do carry one --
    // claude's `DISABLE_UPDATES`, opencode's `OPENCODE_DISABLE_AUTOUPDATE`,
    // grok's `GROK_DISABLE_AUTOUPDATER` -- so the absence is a property of
    // this product rather than of the question.
    updates_off_env: "",
    // One home, one variable: nothing here is conditional.
    config_home_note: "",
    control_directory: ".codex-setup-system",
    state_file: "NDDEV-CODEX-PROVIDER.json",
    predecessor_state_file: "NDDEV-CODEX-SETUP.json",
    // Renamed from `codex/native-and-plugins/1` on 2026-08-30. That name
    // promised a projection this build does not declare: `projection_kinds` here
    // is `[NativeFiles]` alone, and the plugin surfaces it seemed to announce are
    // in `declined` with their reasons. A profile id is read by people deciding
    // what a provider does before they read anything else, so a name that
    // overpromises is the same defect class as a stale note -- a sentence that
    // outlived what it described.
    //
    // **Surveyed before renaming, and the survey changed the answer.** A review
    // reported three harnesses carrying this name as though all three were
    // wrong. They are not: grok declares `Marketplace` and `Plugin` beside
    // `NativeFiles`, and cursor declares `Plugin`, so for those two the name is
    // accurate and only this one overpromises. Four others *understate* --
    // claude's name omits `Plugin`, and pi, opencode and antigravity all read
    // `native-files/1` while declaring a second kind. Understating is a
    // convention question; this was a false statement, and only it is changed
    // here.
    //
    // Safe to move now rather than never: the consumer measured that nothing on
    // its side compares, stores or persists the id string -- it is read out of
    // `provider-info` and never matched. It *is* an input to the profile digest,
    // and this release moves that digest anyway by declaring the agent kind. The
    // one consequence, named by the consumer and correct: an operation prepared
    // before this release and resumed after it is refused with
    // `projection_profile_mismatch`, which is this provider refusing to proceed
    // on a plan bound to a profile that no longer exists.
    profile_id: "codex/native-files/1",
    // Everything outside this list is a sibling overlay preserved verbatim.
    // `skills` and `.agents/skills` were here and are gone. Codex searches
    // `$HOME/.agents/skills` -- a *sibling* of `~/.codex`, not a child --
    // so declared against this target the second resolved to
    // `~/.codex/.agents/skills`, which the product never reads. The same
    // shape as the pi `managed_paths` defect, one level up.
    // `agents` was added 2026-08-28. The product has always had it -- the vendor
    // documents custom agents as TOML files under `~/.codex/agents/`, and the
    // path literal is in the binary -- and this provider neither owned it nor
    // recorded it, so a consumer could not route an `agent` component to codex
    // at all. Widening is safe in the direction a consumer reads: it matches a
    // route by membership in this list, so a larger set makes more routes valid
    // and none that were valid invalid.
    native_namespaces: &[
        "AGENTS.md",
        "config.toml",
        "hooks.json",
        "prompts",
        "agents",
    ],
    // The product's own: credentials, session history and runtime caches. Never
    // read, never written, and never copied into a backup slot.
    // One name the product reads and this provider does not own, measured
    // for this baseline: `AGENTS.override.md` is read before `AGENTS.md`
    // at global scope and the first non-empty file wins, so a home
    // holding a non-empty one ignores the instruction file this provider
    // installs. The same name is a deliberate escape at project scope.
    // Still not owned -- an override exists so a person can override, and
    // owning it would let `remove` take that away.
    shadowing_names: &[Shadow {
        name: "AGENTS.override.md",
        over: "AGENTS.md",
        effect: "read before AGENTS.md at the same scope and only the \
                 first non-empty file is used, so a present non-empty \
                 override replaces the instruction file this provider \
                 installed; an empty one is skipped and silences nothing",
    }],
    // Every owned namespace here routes a kind or is filled by a setup,
    // so exact state has something to say about each one.
    custody_namespaces: &[],
    preservation_surfaces: &[
        PreservationSurface {
            scope: None,
            roots: &["AGENTS.override.md", "skills", "plugins"],
            excluded: &[
                "auth.json",
                "sessions",
                "history.jsonl",
                "cache",
                "shell_snapshots",
            ],
        },
        PreservationSurface {
            scope: Some(TargetScope::Project),
            roots: &["AGENTS.override.md"],
            excluded: &[],
        },
    ],
    never_touch: &[
        "auth.json",
        "sessions",
        "history.jsonl",
        "cache",
        "shell_snapshots",
    ],
    // No near neighbour measured for this product. A marker listed here is a
    // refusal waiting to happen, so nothing is listed without evidence.
    foreign_homes: &[],
    permission_profiles: &["default"],
    // `prompts` are Codex's slash commands, so `Command` is declared. MCP
    // servers live under `[mcp_servers.<name>]` inside `config.toml`: a key
    // of a file this provider owns is not a surface it can install, observe
    // and restore on its own, so `Mcp` is not declared. `Skill` and `Plugin`
    // went with the namespaces that turned out not to exist.
    component_kinds: &[
        ComponentKind::Instruction,
        ComponentKind::Setting,
        ComponentKind::Hook,
        ComponentKind::Command,
        // `ComponentKind::Agent` was withdrawn on 2026-08-28 and is back on
        // 2026-08-30, and the way the withdrawal was wrong is worth more than
        // the fact that it was.
        //
        // The reason given was arity: *a role is two files and a component of
        // one kind is one thing in one namespace*, said to survive a change of
        // behaviour because a role would still be a stanza plus the layer it
        // points at. It does not survive, because the premise was false when it
        // was written. `agent-roles/src/discovery.rs` walks this directory and
        // admits every `*.toml` not already named by a stanza, so a standalone
        // `agents/<name>.toml` is a role on its own -- one file, one namespace,
        // which is precisely what a component of one kind installs.
        //
        // **The measurement could not have found this.** It planted an
        // `agents/<name>.md` beside a working stanza and observed that nothing
        // loaded it. The scan filters on `extension == "toml"`. A `.md` there is
        // invisible whatever the truth is, so that control could not have
        // failed, and a negative was recorded from an experiment incapable of
        // producing a positive.
        //
        // Re-measured against the 0.151.0 artifact with a temporary CODEX_HOME,
        // read back through `codex doctor`: a complete file is accepted in
        // silence, one a directory deeper is too, an invented sibling directory
        // is not scanned, and each of `name`, `description` and
        // `developer_instructions` is refused by name when absent. The consumer
        // reproduced it against the same binary before either side moved.
        //
        // The stanza form still works and excludes its own file from the scan;
        // this setup's builder role ships as the standalone
        // `agents/nddev-builder.toml` -- one file, declared like any other
        // payload member.
        ComponentKind::Agent,
    ],
    projection_kinds: &[
        ProjectionKind::NativeFiles,
        // `Marketplace` and `Plugin` were both declared here and neither could
        // land. This harness's plugins are drawn from a hosted directory shared
        // with ChatGPT, and `plugins/cache` is what an install copies into --
        // product state, not a surface a provider writes.
        //
        // A personal marketplace surface does exist, and finding it is why this
        // took two passes: the pinned binary says *"`~/.agents/plugins/
        // marketplace.json` is discovered implicitly, but other marketplace
        // paths are not"*, with no `marketplace add` step.
        //
        // Re-asked of the 0.151.0 linux/x86_64 bytes on 2026-08-30, because the
        // sentence above names no version and nothing re-establishes it: the
        // anchored literal appears eighteen times, that exact sentence is still
        // there, and two invented paths searched in the same run return zero, so
        // the search discriminates. This is the third decline on this harness
        // re-measured that day, and the reason is the `agents` row above -- a
        // decline taken against a release nobody pins is how that one stayed
        // wrong for two. But it sits in `~/.agents` -- the `user_root` scope, whose
        // namespaces here are `["skills"]` alone -- so the declaration was on
        // the *global* profile, where nothing can hold one. **A declaration is
        // per profile, so backing it is per profile.**
        //
        // Owning that file is a decision rather than a narrowing and is left
        // for one: `~/.agents` is read by several products, and a marketplace
        // there is a *source* every one of them would resolve plugins from, so
        // who else reads it is a question about behaviour rather than routing.
        // The `user_root` skill question has the same shape.
    ],
    // Two scopes. Codex's project surfaces live under the workspace root --
    // `AGENTS.md` at it, `.codex/hooks.json` keeping its product-owned parent --
    // and the user-level `.agents` root is a third home entirely.
    //
    // The one root in this estate that belongs to a convention rather than to a
    // product. `learn.chatgpt.com/docs/build-skills` names `$HOME/.agents/skills`
    // as the user-level skills directory -- a *sibling* of `~/.codex`, not a
    // child, so nothing declared against this provider's own target can reach
    // it. That is what `user_root` exists for.
    //
    // **Owning a shared root, weighed rather than assumed.** `.agents` is named
    // for being shared, and an owned namespace is removed whole: a `remove`
    // here takes `~/.agents/skills` entirely, not only what a setup put there.
    // Measured 2026-08-28 across all seven baselines, the user-level `.agents`
    // root looked uncontested: only Codex *documents* reading from it, and
    // Antigravity's `.agents` surfaces are all workspace-level with its global
    // configuration at `~/.gemini/config/`. That sentence closed with *if a
    // second product adopts the user-level root, this declaration is the first
    // thing to re-read*.
    //
    // **Four of the seven do, and the sweep that found them read products
    // rather than pages.** Measured 2026-08-28 from pinned artifacts, digests
    // verified before reading:
    //
    // | product | evidence |
    // | --- | --- |
    // | codex | documented: `learn.chatgpt.com/docs/build-skills` |
    // | grok | its own embedded reference: scans `.agents/skills/` *at each tier*, and the tier table names User |
    // | opencode | vendor lists *Global agent-compatible: `~/.agents/skills/<name>/SKILL.md`*; the binary carries the literal |
    // | pi | source, and no page says it: `package-manager.js:2017` |
    //
    // The same sweep found a second shared surface, in the other direction:
    // `~/.claude/skills` is read by grok (*User tier, Lowest, configurable*) and
    // opencode (*Global Claude-compatible*) as well as by Claude Code. This
    // provider's neighbour owns it, and a namespace is removed whole -- so a
    // remove of the Claude setup changes what two other products see.
    //
    // Pi's is the one worth keeping the method for. Its pinned `0.84.3` bundle,
    // `package/dist/core/package-manager.js`: line 1976 builds
    // `userAgentsSkillsDir = join(getHomeDir(), ".agents", "skills")`, line 2012
    // names the root itself as its `dirname`, and line 2017 loads from it --
    // `addResources("skills", collectAutoSkillEntries(userAgentsSkillsDir,
    // "agents"), ...)`. No Pi page says so. (A neighbouring use in
    // `trust-manager.js:160` *excludes* that directory while walking up for a
    // project-scoped one, so the variable name alone would have misread it; the
    // line that matters is 2017.)
    //
    // **Both sentences that used to close this block have since expired**, and
    // they are kept here as the shape rather than deleted, because a reason
    // nobody re-reads is how a correct record goes quietly wrong.
    //
    // It said *nothing routes to `user_root` yet*. The consumer routes this
    // provider's `skill` there today: `composition.py`,
    // `Rule("skill", "skills", "directory", "codex", target_scope="user_root")`.
    //
    // It said the other four were *deliberately not given the same scoped
    // profile*, because *a namespace is removed whole, so either one's `remove`
    // under this scope takes the other's skills*. True when written, and false
    // from the day `written_paths` shipped: under a scope every verb acts on
    // the files this provider recorded writing, so grok, opencode, pi and
    // cursor declare the same profile now and coexist under one `~/.agents`,
    // each with its own state file. Verified with the shipped binaries.
    //
    // What has not changed is the declaration itself, and the reason to read
    // this block is the method rather than its conclusions: the root was
    // weighed against every other product before it was owned, and the answer
    // moved twice while the measurement stayed right.
    scoped_projections: &[
        Scoped {
            target_scope: TargetScope::UserRoot,
            // Distinct from the global identity, because the digest binds a
            // declaration together with the scope it owns.
            profile_id: "codex/native-files/user-root/1",
            component_kinds: &[ComponentKind::Skill],
            projection_kinds: &[ProjectionKind::NativeFiles],
            // Relative to `~/.agents`, which is the target this scope names.
            native_namespaces: &["skills"],
        },
        Scoped {
            target_scope: TargetScope::Project,
            profile_id: "codex/native-files/project/1",
            component_kinds: &[
                ComponentKind::Instruction,
                ComponentKind::Hook,
                ComponentKind::Skill,
            ],
            projection_kinds: &[ProjectionKind::NativeFiles],
            // Instructions live at the root; hooks and skills retain their
            // documented parents. Only recorded skill files belong to us.
            native_namespaces: &["AGENTS.md", ".codex/hooks.json", ".agents/skills"],
        },
    ],
    max_files: 8192,
    max_bytes: 64 * 1024 * 1024,
    kit_identity: include_str!("../../../provider-kit/v3/KIT-IDENTITY.json"),
    // Generated by `build.rs` from this harness's `setups/` directory, so the
    // binary carries the catalog it is named after instead of hoping to find
    // one on a disk it was never shipped to.
    embedded_setups: include!(concat!(env!("OUT_DIR"), "/embedded_setups.rs")),
    software: Some(software::SOFTWARE),
    instruction_region: Some("AGENTS.md"),
};

fn main() -> ExitCode {
    harness_runtime::run(&CODEX, std::env::args().skip(1).collect())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::*;

    /// The directory name this harness's setups live under in the workspace.
    const TOOL: &str = "codex";
    /// The declaration under test, named once so the shared test below reads
    /// the same in all seven crates.
    const HARNESS: Harness = CODEX;

    /// `build.rs` put the whole catalog in, under the paths it will be read by.
    ///
    /// This does **not** test for staleness, and an earlier version of this
    /// comment claimed it did. It cannot: `build.rs` declares
    /// `rerun-if-changed` on the catalog directory, so editing a setup rebuilds
    /// the table before this runs, and the test would be comparing the tree
    /// with itself. Observed — a deliberately edited setup left it green.
    ///
    /// What it does test is the build script, against a walk written
    /// independently of it: every file present, none invented, bytes exact, and
    /// paths relative and slash-separated. That last one is the one that would
    /// really break — `join("/")` is the only reason these keys are usable on
    /// Windows, and a path built with the platform separator would still look
    /// perfectly correct in the generated source.
    /// The bytes this harness ships, pinned so they cannot change unseen.
    ///
    /// A setup's `definition_digest` is what makes two setups the same setup,
    /// and it appears in `list`, in a plan and in provider state -- and until
    /// this, nothing compared it to anything. A stray character in a setup file
    /// changed what the estate installs and every test stayed green.
    ///
    /// One aggregate rather than one per setup, because the claim is about the
    /// catalogue: sorted definition digests, joined by a newline, hashed. A
    /// deliberate change to a setup updates the line in the baseline, which is
    /// the point -- the peer calls this a golden and it earns itself the first
    /// time a row moves without anyone meaning it to.
    ///
    /// **And it is the three-OS check nothing else makes.** The setups are
    /// embedded with `include_bytes!`, so whatever the checkout holds is what
    /// ships; `.gitattributes` pins `eol=lf` to keep a Windows checkout from
    /// rewriting them, and this is the assertion that would notice if it ever
    /// stopped working. The matrix runs it on all three systems, so a digest
    /// that differed by platform could not stay hidden.
    #[test]
    fn the_catalogue_this_harness_ships_is_the_one_the_baseline_records() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let mut digests: Vec<String> = catalog
            .list()
            .unwrap()
            .iter()
            // **Both digests, because one of them holds nothing a person
            // reads.** `definition_digest` is the payload tree; the manifest --
            // `id`, `sources`, `description` -- was covered by no digest in this
            // estate, and those three are what a consumer renders on the surface
            // that precedes an install. A description was rewritten and the
            // whole gate stayed clean, which is how this was found.
            .map(|setup| format!("{}\n{}", setup.definition_digest, setup.manifest_digest))
            .collect();
        digests.sort();
        let joined = digests.join("\n");
        let aggregate = harness_runtime::digest_of_bytes(&joined);
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references")
            .join(format!("{TOOL}-baseline.json"));
        let baseline: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let recorded = baseline["setup_catalogue_digest"].as_str().unwrap_or("");
        assert_eq!(
            aggregate, recorded,
            "the setups this binary ships are not the ones {TOOL}-baseline.json \
             records; if the change was meant, put this digest there"
        );
    }

    #[test]
    fn the_catalog_this_binary_carries_is_the_one_in_the_tree() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        // The workspace holds one directory per harness; a rendered public tree
        // ships one harness and holds it flat. Same two candidates `build.rs`
        // chooses between, asked the same way.
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };

        // Only the setup directories, which is what the reader lists and what
        // `build.rs` embeds. A rendered public tree also carries a
        // `setups/README.md` at the catalog root, which belongs to no setup.
        let mut on_disk = Vec::new();
        let mut stack: Vec<std::path::PathBuf> = std::fs::read_dir(&root)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.join("setup.json").is_file())
            .collect();
        while let Some(directory) = stack.pop() {
            for entry in std::fs::read_dir(&directory).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    stack.push(path);
                } else {
                    on_disk.push(path);
                }
            }
        }

        assert_eq!(
            HARNESS.embedded_setups.len(),
            on_disk.len(),
            "the binary carries {} files and the tree holds {}",
            HARNESS.embedded_setups.len(),
            on_disk.len()
        );

        for (relative, bytes) in HARNESS.embedded_setups {
            assert!(
                !relative.contains('\\') && !relative.starts_with('/'),
                "{relative:?} is not a relative slash path; a key built with the \
                 platform separator reads correctly on Unix and finds nothing on Windows"
            );
            let path = root.join(relative);
            let found = std::fs::read(&path)
                .unwrap_or_else(|e| panic!("{relative} is compiled in but not in the tree: {e}"));
            assert_eq!(
                &found, bytes,
                "{relative} differs between the binary and the tree"
            );
        }
    }

    #[test]
    fn the_declaration_is_valid_and_names_this_host() {
        let info = CODEX.provider_info().unwrap();
        assert_eq!(info.provider_id, env!("CARGO_PKG_NAME"));
        assert_eq!(info.harness_id, "codex");
        assert_eq!(info.protocol_version, 3);
        assert!(info.supports_this_host());
    }

    /// The kind this build declares can actually be written where it goes.
    ///
    /// `ComponentKind::Agent` is a promise of an install and a rollback, and the
    /// machinery that keeps it is deliberately kind-agnostic: one membership
    /// test decides whether a kind is implemented, and after that every
    /// component takes the same path. So the half that is *not* generic, and the
    /// half this declaration turns on, is whether a role's own path is one this
    /// harness owns.
    ///
    /// Asked at three depths and with a control, because the reading that broke
    /// `.agents/skills` once was matching on the first path component alone --
    /// a namespace check that says yes to a directory it owns and no to the file
    /// inside it would refuse every write to the route the compiler uses.
    #[test]
    fn a_role_lands_where_this_build_says_it_owns() {
        assert!(HARNESS.owns("agents"), "the namespace itself");
        assert!(
            HARNESS.owns("agents/nddev-builder.toml"),
            "a standalone role, which is what one agent component installs"
        );
        assert!(
            HARNESS.owns("agents/nested/deeper.toml"),
            "the product's scan is recursive, so a nested role is still ours"
        );
        // The control. Without it this test passes on a build that owns
        // everything, which is the shape a namespace check fails into.
        assert!(
            !HARNESS.owns("agents-not-ours/role.toml"),
            "a sibling directory nobody declared is not owned by prefix"
        );
        assert!(
            HARNESS
                .component_kinds
                .contains(&provider_v3::ComponentKind::Agent),
            "the path is owned and the kind is not declared, so nothing could install one"
        );
    }

    #[test]
    fn no_namespace_is_both_owned_and_disclaimed() {
        for name in CODEX.never_touch {
            assert!(
                !CODEX.native_namespaces.contains(name),
                "{name} is claimed and disclaimed"
            );
        }
    }

    /// Everything this harness claims to own, against the vendor page that
    /// decided it.
    ///
    /// What this replaced only checked that the baseline parsed. The block it
    /// reads now is hand-authored beside the rest of the baseline, and this is
    /// what keeps that block from being decoration: a namespace no vendor
    /// document names, or a declared kind no owned surface routes, is red here.
    ///
    /// Both directions, because the defect it was written for ran both ways --
    /// `~/.cursor/rules` was owned and does not exist, `~/.pi/agent/prompts`
    /// exists and was not owned. Conformance caught neither: its
    /// `declared_native_route_is_compilable` case asks for **one** route, not
    /// every one.
    #[test]
    fn every_surface_this_harness_owns_is_one_the_vendor_documents() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references")
            .join(format!("{TOOL}-baseline.json"));
        let baseline: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let problems = harness_runtime::surfaces::disagreements(&HARNESS, &baseline);
        assert!(
            problems.is_empty(),
            "the declaration and {TOOL}-baseline.json disagree:
  {}",
            problems.join(
                "
  "
            )
        );
    }

    #[test]
    fn the_control_directory_and_state_file_are_provider_owned_not_product_owned() {
        assert!(CODEX.control_directory.contains("setup-system"));
        assert!(CODEX.state_file.starts_with("NDDEV-"));
        assert!(!CODEX.native_namespaces.contains(&CODEX.state_file));
    }
    /// A setup that writes a configuration file says where its format came from.
    ///
    /// The release before this one made the *surfaces* sourced: a path this
    /// provider owns cites the page that documents it. This is the same rule
    /// one level down, and it was written because two of the seven failed it.
    ///
    /// opencode's baseline set `"permission": "ask"` where the product
    /// documents an object of tool names, and antigravity's set
    /// `toolPermissions` where the product reads `toolPermission` with four
    /// values, none of them the one written. Both were valid JSON in the right
    /// file at the right path. Both installed, verified and restored cleanly.
    /// Neither changed anything about the product — a target that looks
    /// configured and is not, which is the failure this estate refuses one
    /// level up and had been shipping one level down.
    /// Two files in one setup that a case-insensitive filesystem would merge.
    ///
    /// macOS and Windows fold case, so such a pair is one file there and two on
    /// Linux -- the setup would install different content depending on the
    /// machine, and its catalogue digest would differ per platform. The bundle
    /// reader has refused this for an arriving bundle since 0.0.11; this is the
    /// same rule applied to what this repository authors.
    /// Every component entry point describes itself.
    ///
    /// A `SKILL.md` or an agent whose frontmatter lost its `description` still
    /// installs, verifies and restores cleanly -- and the product names it after
    /// its directory and gives the model nothing to choose on. Documents under
    /// `references/` and files under `commands/` are exempt, because the
    /// products measured do not read frontmatter from either.
    /// Supporting documents are reachable from an entry point.
    ///
    /// A `references/` folder whose skill has no `SKILL.md` is prose nothing
    /// routes to. A generator in this repository produced exactly that, and
    /// every other guard passed it: the files are documents, so `unsourced`
    /// exempts them, and there is no `SKILL.md`, so `undescribed` has nothing
    /// to check.
    /// Nothing shipped sends a reader to a file this setup does not carry.
    ///
    /// A routing table naming `references/surfaces.md` in a setup that ships no
    /// such file sends the reader nowhere -- and the reader is a model, which
    /// will not say so. The generator here did exactly that: it pointed every
    /// harness's agent at that path, and codex ships no skill at all.
    #[test]
    fn nothing_shipped_names_a_document_it_does_not_carry() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::dangling_references(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    #[test]
    fn every_reference_folder_has_an_entry_point() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::unreachable_references(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    /// Nothing inside a skill is a file no reader is sent to.
    ///
    /// Two findings in one hour were of exactly this shape and every guard in
    /// this estate was silent on both: an executable validator shipped into
    /// people's homes that nothing named, and eleven authoring pages written
    /// into four harnesses and routed to from none. The estate asked whether a
    /// *named* file exists and never whether an *existing* file is named.
    #[test]
    fn nothing_inside_a_skill_is_stranded() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let found = harness_runtime::catalog::stranded(
            &harness_runtime::Catalog::at(&root).list().unwrap(),
        );
        assert!(found.problems.is_empty(), "{}", found.problems.join("\n  "));
        // codex ships no skill, so this walk reaches nothing. **Zero is the right number and it is why the count is asserted at all** -- an emptiness check here is green over an empty walk, which is the defect this guard's own sibling was found in.
        assert_eq!(
            found.entry_points, 0,
            "the stranded-file guard walked {} files inside skills, not 0",
            found.entry_points
        );
    }

    #[test]
    fn every_component_entry_point_describes_itself() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let examined = harness_runtime::catalog::undescribed(&catalog.list().unwrap());
        assert!(
            examined.problems.is_empty(),
            "{}",
            examined.problems.join("\n  ")
        );
        // Codex ships no SKILL.md and no markdown agent: its skills are `user_root` only and its agent is the standalone `agents/nddev-builder.toml`. **One is the right number and it is the reason this count exists** -- the assertion below it was green on every harness while the guard examined nothing at all here, and the count is what makes the subject visible.
        assert_eq!(
            examined.entry_points, 1,
            "the description guard examined {} entry points, not 1",
            examined.entry_points
        );
    }

    #[test]
    fn no_two_files_in_a_setup_differ_only_in_case() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::colliding(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    #[test]
    fn a_setup_that_writes_configuration_says_where_its_format_came_from() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::unsourced(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
    /// Three postures, on every one of the seven.
    ///
    /// `baseline` is a working floor, `minimal` is the shared autonomous posture and nothing else,
    /// and `full-auto` asks nothing and sandboxes nothing. A caller who learns
    /// them on one product knows them on all seven, which is the whole reason
    /// the names are the estate's rather than each harness's.
    ///
    /// The second half of the check is the one worth having: two setups with
    /// the same bytes mean one of them is a posture in name only, and it would
    /// still read as offered in `list`.
    #[test]
    fn the_three_postures_are_offered_and_are_actually_different() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::asymmetric(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
    /// Nothing this setup ships tells a reader to run something that is not here.
    ///
    /// A setup carries documents an agent reads and acts on -- a skill, a rule,
    /// a command file -- and nothing was checking them. One shipped
    /// `software-status --target <dir> --json` and `list --json` for six
    /// releases; the binary refuses both, and says so in those words.
    ///
    /// Two refusals: a name belonging to the frozen estate, and any line naming
    /// this provider followed by a verb `into_command` does not accept. English
    /// is not judged -- `install` in a sentence is a word, and only
    /// `<provider> install` is an instruction.
    #[test]
    fn nothing_this_harness_ships_names_a_command_it_refuses() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems =
            harness_runtime::catalog::misdirecting(HARNESS.provider_id, &catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    /// The instruction-file shadow the baseline measured is declared, so
    /// `status` can report a target running a file this provider never wrote
    /// instead of answering `managed` beside nothing.
    #[test]
    fn the_measured_instruction_shadows_are_declared() {
        let names: Vec<&str> = CODEX
            .shadowing_names
            .iter()
            .map(|shadow| shadow.name)
            .collect();
        assert!(
            names.contains(&"AGENTS.override.md"),
            "AGENTS.override.md is measured in the baseline and not declared"
        );
    }
}
