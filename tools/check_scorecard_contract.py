"""Enforce the retained public default-branch Scorecard publication contract."""

from __future__ import annotations

import os
import re


def validate(environment: dict[str, str]) -> None:
    if environment.get("SCORECARD_PUBLISH_RESULTS") != "true":
        raise ValueError("publication entrypoint requires publish_results=true")
    if environment.get("SCORECARD_SERVER_URL") != "https://github.com":
        raise ValueError("Scorecard publication supports github.com only")
    if environment.get("SCORECARD_REPOSITORY_PRIVATE") != "false":
        raise ValueError("Scorecard publication requires a public repository")
    if environment.get("SCORECARD_REPOSITORY_FORK") != "false":
        raise ValueError("Scorecard publication does not support fork repositories")
    if environment.get("SCORECARD_EVENT_NAME") not in {"push", "schedule"}:
        raise ValueError("Scorecard publication supports push and schedule only")
    branch = environment.get("SCORECARD_DEFAULT_BRANCH")
    if not branch or environment.get("SCORECARD_REF") != f"refs/heads/{branch}":
        raise ValueError("Scorecard publication requires the actual default-branch ref")
    if not re.fullmatch(
        r"[a-z0-9][a-z0-9._-]{0,63}", environment.get("SCORECARD_SARIF_CATEGORY", "")
    ):
        raise ValueError("Scorecard category must be a lowercase URI-safe value")


if __name__ == "__main__":
    try:
        validate(dict(os.environ))
    except ValueError as error:
        raise SystemExit(f"Scorecard contract rejected: {error}") from None
    print("validated public default-branch Scorecard publication")
