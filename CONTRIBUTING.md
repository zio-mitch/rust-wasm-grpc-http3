# Contribution Rules

## Issues
Issues MUST be descriptive and include steps to reproduce the problem.
All issues MUST be opened on [GitHub](https://github.com/http3/grpc-rust/issues) with appropriate labels (e.g., `bug`, `feature request`).

### Feature Requests
Must include a detailed description and a rationale. 
Each request MUST have a unique tracking code: `FR-YYYY-XXXXX` (e.g., `FR-2026-00001`).
> [!TIP]
> See the [Feature Request Template](./.github/ISSUE_TEMPLATE/feature_request.md).

### Bug Reports
Must include reproduction steps and environment details.
Each report MUST have a unique tracking code: `BR-YYYY-XXXXX` (e.g., `BR-2026-00001`).
> [!TIP]
> See the [Bug Report Template](./.github/ISSUE_TEMPLATE/bug_report.md).

---

## Branching Model
We follow the [Git Flow](https://nvie.com/posts/a-successful-git-branching-model/) model.

> [!CAUTION]
> The `main` branch is currently protected. All development and Pull Requests MUST target the `develop` branch.

---

## Pull Requests
* **Target:** ALWAYS `develop` branch.
* **Reference:** Must reference the tracked issue (e.g., `BR-2026-00042`).
* **Description:** Must clearly explain the changes.
> [!TIP]
> See the [Pull Request Template](./.github/PULL_REQUEST_TEMPLATE.md).

---

## Commits
Commit messages MUST follow this structure:
`Issue Code | Description`

| Action | Example |
| :--- | :--- |
| **Fix** | `BR-2026-00042 \| Fix memory leak in transport` |
| **Implementation** | `FR-2026-00001 \| Implement TLS support` |
| **Refactor implementation** | `FR-2026-00010 \| Refactor buffer allocation logic` |

### Refactoring & Linting
1.  **Refactor:** Requires a Feature Request explaining the rationale.
2.  **Linting:** Avoid "noise" in functional commits. Perform linting on unrelated files in a **separate** refactor commit. (e.g., `FR-2026-00011 \| Refactor code formatting`).

### Conventions

1. [Make file standars](./.docs/context/makefile-standards.md)
 