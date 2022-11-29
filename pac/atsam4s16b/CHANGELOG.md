# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.2.2 (2022-11-29)

### Bug Fixes

 - <csr-id-54fe3c1f9705e2a9f96176dd8c467fbaed648702/> Remove unused dependency bare-metal
 - <csr-id-63c755d5cb29e4a0d6eec4a1f24498cd2b2801d8/> Update dependencies
   - cortex-m -> 0.7.6
   - cortex-m-rt -> 0.7.2
   - svd2rust -> 0.27.2
   - form -> 0.10.0
   - Deprecated atsamd51*, atsame51*, atsame53* and atsame54* as these pacs
     are already covered by atsamd-hal
   - pacs using svd2rust (pactool.sh -f generate)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 597 calendar days.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#10](https://github.com/atsam-rs/atsam-pac/issues/10), [#18](https://github.com/atsam-rs/atsam-pac/issues/18)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#10](https://github.com/atsam-rs/atsam-pac/issues/10)**
    - Added support for ATSAME54 and ATSAM4S16 ([`e120060`](https://github.com/atsam-rs/atsam-pac/commit/e120060695b87b48d1c27061c28470a006b2abee))
 * **[#18](https://github.com/atsam-rs/atsam-pac/issues/18)**
    - Bumped PAC versions to reflect interrupt changes in SVDs in recent PRs. ([`bc207c5`](https://github.com/atsam-rs/atsam-pac/commit/bc207c585bb70dffa31842ea9c94c887b2afb980))
 * **Uncategorized**
    - Remove unused dependency bare-metal ([`54fe3c1`](https://github.com/atsam-rs/atsam-pac/commit/54fe3c1f9705e2a9f96176dd8c467fbaed648702))
    - Update dependencies ([`63c755d`](https://github.com/atsam-rs/atsam-pac/commit/63c755d5cb29e4a0d6eec4a1f24498cd2b2801d8))
    - Update pac dependencies ([`8ca7aca`](https://github.com/atsam-rs/atsam-pac/commit/8ca7acab12a2e8af4c6f49d25d79d3c379d4fd35))
    - Update pacs to svd2rust 0.19.0 ([`07a2d93`](https://github.com/atsam-rs/atsam-pac/commit/07a2d930b057726763d359204c406a994661aacc))
</details>

