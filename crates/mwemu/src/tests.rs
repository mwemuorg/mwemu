use crate::{cli, cli_api_mode, cli_loop_limit};
use libmwemu::ApiMode;

#[test]
fn loop_limit_only_applies_with_noloops() {
    for (options, expected) in [
        (vec![], None),
        (vec!["--loop-limit", "2"], None),
        (vec!["--noloops"], Some(1_000_000)),
        (vec!["--noloops", "--loop-limit", "2"], Some(2)),
        (vec!["--noloops", "--loop-limit", "0"], Some(0)),
    ] {
        let matches = cli().get_matches_from(["mwemu"].into_iter().chain(options));
        assert_eq!(cli_loop_limit(&matches), expected);
    }
    assert!(
        cli()
            .get_matches_from_safe(["mwemu", "--loop-limit", "invalid"])
            .is_err()
    );
}

#[test]
fn api_policy_defaults() {
    let defaults = cli().get_matches_from(["mwemu"]);
    assert_eq!(cli_api_mode(&defaults), (None, false));

    let banzai = cli().get_matches_from(["mwemu", "--banzai"]);
    assert_eq!(
        cli_api_mode(&banzai),
        (Some(ApiMode::SimulateSuccess), true)
    );
}

#[test]
fn explicit_api_mode_overrides_banzai_without_warning() {
    for (option, mode) in [
        ("strict", ApiMode::Strict),
        ("simulate-fail", ApiMode::SimulateFail),
        ("simulate-success", ApiMode::SimulateSuccess),
    ] {
        for arguments in [
            vec!["mwemu", "--api-mode", option],
            vec!["mwemu", "--banzai", "--api-mode", option],
            vec!["mwemu", "--api-mode", option, "--banzai"],
        ] {
            let matches = cli().get_matches_from(arguments.clone());
            assert_eq!(cli_api_mode(&matches), (Some(mode), false), "{arguments:?}");
        }
    }
}

#[test]
fn strict_api_overrides_mode_and_banzai_without_warning() {
    for option in [
        None,
        Some("strict"),
        Some("simulate-fail"),
        Some("simulate-success"),
    ] {
        for banzai in [false, true] {
            let mut arguments = vec!["mwemu", "--strict-api"];
            if let Some(option) = option {
                arguments.extend(["--api-mode", option]);
            }
            if banzai {
                arguments.push("--banzai");
            }
            let matches = cli().get_matches_from(arguments.clone());
            assert_eq!(
                cli_api_mode(&matches),
                (Some(ApiMode::Strict), false),
                "{arguments:?}"
            );
        }
    }
}

#[test]
fn invalid_api_mode_is_rejected_even_with_banzai() {
    assert!(
        cli()
            .get_matches_from_safe(["mwemu", "--banzai", "--api-mode", "unknown"])
            .is_err()
    );
}
