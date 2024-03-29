# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.3.1 (2023-04-29)

### Bug Fixes

 - <csr-id-350e7515159199ef5cfd37ffdca2b511ab4486dc/> Update dependencies and deprecate atsame7 pacs
   - cortex-m -> 0.7.7
   - cortex-m-rt -> 0.7.3
   - critical-section -> 1.1.1
   - Renamed LICENSE_MIT -> LICENSE-MIT
   - Deprecated (handled by https://github.com/atsams-rs/atsamx7x-rust)
     * atsame70j19
     * atsame70j19b
     * atsame70j20
     * atsame70j20b
     * atsame70j21
     * atsame70j21b
     * atsame70n19
     * atsame70n19b
     * atsame70n20
     * atsame70n20b
     * atsame70n21
     * atsame70n21b
     * atsame70q19
     * atsame70q19b
     * atsame70q20
     * atsame70q20b
     * atsame70q21
     * atsame70q21b

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 144 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update dependencies and deprecate atsame7 pacs ([`350e751`](https://github.com/atsam-rs/atsam-pac/commit/350e7515159199ef5cfd37ffdca2b511ab4486dc))
</details>

## v0.3.0 (2022-12-06)

### Bug Fixes

 - <csr-id-2b9a3080f81274d29310899f9f2e700c6d6a1f49/> Fix #24 (missing critical-section feature)
   - Recent update of svd2rust requires this feature to use
   Peripherals::take()

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 6 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release atsam4e16c-pac v0.3.0, atsam4e16e-pac v0.3.0, atsam4e8c-pac v0.3.0, atsam4e8e-pac v0.3.0, atsam4lc2a-pac v0.3.0, atsam4lc2b-pac v0.3.0, atsam4lc2c-pac v0.3.0, atsam4lc4a-pac v0.3.0, atsam4lc4b-pac v0.3.0, atsam4lc4c-pac v0.3.0, atsam4lc8a-pac v0.3.0, atsam4lc8b-pac v0.3.0, atsam4lc8c-pac v0.3.0, atsam4ls2a-pac v0.3.0, atsam4ls2b-pac v0.3.0, atsam4ls2c-pac v0.3.0, atsam4ls4a-pac v0.3.0, atsam4ls4b-pac v0.3.0, atsam4ls4c-pac v0.3.0, atsam4ls8a-pac v0.3.0, atsam4ls8b-pac v0.3.0, atsam4ls8c-pac v0.3.0, atsam4n16b-pac v0.3.0, atsam4n16c-pac v0.3.0, atsam4n8a-pac v0.3.0, atsam4n8b-pac v0.3.0, atsam4n8c-pac v0.3.0, atsam4s16b-pac v0.3.0, atsam4s16c-pac v0.3.0, atsam4s2a-pac v0.3.0, atsam4s2b-pac v0.3.0, atsam4s2c-pac v0.3.0, atsam4s4a-pac v0.3.0, atsam4s4b-pac v0.3.0, atsam4s4c-pac v0.3.0, atsam4s8b-pac v0.3.0, atsam4s8c-pac v0.3.0, atsam4sa16b-pac v0.3.0, atsam4sa16c-pac v0.3.0, atsam4sd16b-pac v0.3.0, atsam4sd16c-pac v0.3.0, atsam4sd32b-pac v0.3.0, atsam4sd32c-pac v0.3.0, atsam4sp32a-pac v0.3.0, atsame70j19-pac v0.3.0, atsame70j19b-pac v0.3.0, atsame70j20-pac v0.3.0, atsame70j20b-pac v0.3.0, atsame70j21-pac v0.3.0, atsame70j21b-pac v0.3.0, atsame70n19-pac v0.3.0, atsame70n19b-pac v0.3.0, atsame70n20-pac v0.3.0, atsame70n20b-pac v0.3.0, atsame70n21-pac v0.3.0, atsame70n21b-pac v0.3.0, atsame70q19-pac v0.3.0, atsame70q19b-pac v0.3.0, atsame70q20-pac v0.3.0, atsame70q20b-pac v0.3.0, atsame70q21-pac v0.3.0, atsame70q21b-pac v0.3.0 ([`57604e8`](https://github.com/atsam-rs/atsam-pac/commit/57604e8f8ca65b498fd70a541f3e183a6d9e1d4a))
    - Fix #24 (missing critical-section feature) ([`2b9a308`](https://github.com/atsam-rs/atsam-pac/commit/2b9a3080f81274d29310899f9f2e700c6d6a1f49))
</details>

## v0.2.2 (2022-11-29)

### Bug Fixes

<csr-id-5a8f9b8190f7be9dc59af66d0ba0fb48b8ee8c84/>

 - <csr-id-54fe3c1f9705e2a9f96176dd8c467fbaed648702/> Remove unused dependency bare-metal
 - <csr-id-63c755d5cb29e4a0d6eec4a1f24498cd2b2801d8/> Update dependencies
   - cortex-m -> 0.7.6

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 597 calendar days.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#10](https://github.com/atsam-rs/atsam-pac/issues/10), [#18](https://github.com/atsam-rs/atsam-pac/issues/18)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#10](https://github.com/atsam-rs/atsam-pac/issues/10)**
    - Added support for ATSAME54 and ATSAM4S16 ([`e120060`](https://github.com/atsam-rs/atsam-pac/commit/e120060695b87b48d1c27061c28470a006b2abee))
 * **[#18](https://github.com/atsam-rs/atsam-pac/issues/18)**
    - Bumped PAC versions to reflect interrupt changes in SVDs in recent PRs. ([`bc207c5`](https://github.com/atsam-rs/atsam-pac/commit/bc207c585bb70dffa31842ea9c94c887b2afb980))
 * **Uncategorized**
    - Release atsam4e16c-pac v0.2.2, atsam4e16e-pac v0.2.2, atsam4e8c-pac v0.2.2, atsam4e8e-pac v0.2.2, atsam4lc2a-pac v0.2.2, atsam4lc2b-pac v0.2.2, atsam4lc2c-pac v0.2.2, atsam4lc4a-pac v0.2.2, atsam4lc4b-pac v0.2.2, atsam4lc4c-pac v0.2.2, atsam4lc8a-pac v0.2.2, atsam4lc8b-pac v0.2.2, atsam4lc8c-pac v0.2.2, atsam4ls2a-pac v0.2.2, atsam4ls2b-pac v0.2.2, atsam4ls2c-pac v0.2.2, atsam4ls4a-pac v0.2.2, atsam4ls4b-pac v0.2.2, atsam4ls4c-pac v0.2.2, atsam4ls8a-pac v0.2.2, atsam4ls8b-pac v0.2.2, atsam4ls8c-pac v0.2.2, atsam4n16b-pac v0.2.2, atsam4n16c-pac v0.2.2, atsam4n8a-pac v0.2.2, atsam4n8b-pac v0.2.2, atsam4n8c-pac v0.2.2, atsam4s16b-pac v0.2.2, atsam4s16c-pac v0.2.2, atsam4s2a-pac v0.2.2, atsam4s2b-pac v0.2.2, atsam4s2c-pac v0.2.2, atsam4s4a-pac v0.2.2, atsam4s4b-pac v0.2.2, atsam4s4c-pac v0.2.2, atsam4s8b-pac v0.2.2, atsam4s8c-pac v0.2.2, atsam4sa16b-pac v0.2.2, atsam4sa16c-pac v0.2.2, atsam4sd16b-pac v0.2.2, atsam4sd16c-pac v0.2.2, atsam4sd32b-pac v0.2.2, atsam4sd32c-pac v0.2.2, atsam4sp32a-pac v0.2.2, atsame70j19-pac v0.2.2, atsame70j19b-pac v0.2.2, atsame70j20-pac v0.2.2, atsame70j20b-pac v0.2.2, atsame70j21-pac v0.2.2, atsame70j21b-pac v0.2.2, atsame70n19-pac v0.2.2, atsame70n19b-pac v0.2.2, atsame70n20-pac v0.2.2, atsame70n20b-pac v0.2.2, atsame70n21-pac v0.2.2, atsame70n21b-pac v0.2.2, atsame70q19-pac v0.2.2, atsame70q19b-pac v0.2.2, atsame70q20-pac v0.2.2, atsame70q20b-pac v0.2.2, atsame70q21-pac v0.2.2, atsame70q21b-pac v0.2.2 ([`c3f31b0`](https://github.com/atsam-rs/atsam-pac/commit/c3f31b039d2bc2b40c4d9c0c302c1775cc0bb84b))
    - Add initial CHANGELOGs ([`5a8f9b8`](https://github.com/atsam-rs/atsam-pac/commit/5a8f9b8190f7be9dc59af66d0ba0fb48b8ee8c84))
    - Release atsam4e16c-pac v0.2.2, atsam4e16e-pac v0.2.2, atsam4e8c-pac v0.2.2, atsam4e8e-pac v0.2.2, atsam4lc2a-pac v0.2.2, atsam4lc2b-pac v0.2.2, atsam4lc2c-pac v0.2.2, atsam4lc4a-pac v0.2.2, atsam4lc4b-pac v0.2.2, atsam4lc4c-pac v0.2.2, atsam4lc8a-pac v0.2.2, atsam4lc8b-pac v0.2.2, atsam4lc8c-pac v0.2.2, atsam4ls2a-pac v0.2.2, atsam4ls2b-pac v0.2.2, atsam4ls2c-pac v0.2.2, atsam4ls4a-pac v0.2.2, atsam4ls4b-pac v0.2.2, atsam4ls4c-pac v0.2.2, atsam4ls8a-pac v0.2.2, atsam4ls8b-pac v0.2.2, atsam4ls8c-pac v0.2.2, atsam4n16b-pac v0.2.2, atsam4n16c-pac v0.2.2, atsam4n8a-pac v0.2.2, atsam4n8b-pac v0.2.2, atsam4n8c-pac v0.2.2, atsam4s16b-pac v0.2.2, atsam4s16c-pac v0.2.2, atsam4s2a-pac v0.2.2, atsam4s2b-pac v0.2.2, atsam4s2c-pac v0.2.2, atsam4s4a-pac v0.2.2, atsam4s4b-pac v0.2.2, atsam4s4c-pac v0.2.2, atsam4s8b-pac v0.2.2, atsam4s8c-pac v0.2.2, atsam4sa16b-pac v0.2.2, atsam4sa16c-pac v0.2.2, atsam4sd16b-pac v0.2.2, atsam4sd16c-pac v0.2.2, atsam4sd32b-pac v0.2.2, atsam4sd32c-pac v0.2.2, atsam4sp32a-pac v0.2.2, atsame70j19-pac v0.2.2, atsame70j19b-pac v0.2.2, atsame70j20-pac v0.2.2, atsame70j20b-pac v0.2.2, atsame70j21-pac v0.2.2, atsame70j21b-pac v0.2.2, atsame70n19-pac v0.2.2, atsame70n19b-pac v0.2.2, atsame70n20-pac v0.2.2, atsame70n20b-pac v0.2.2, atsame70n21-pac v0.2.2, atsame70n21b-pac v0.2.2, atsame70q19-pac v0.2.2, atsame70q19b-pac v0.2.2, atsame70q20-pac v0.2.2, atsame70q20b-pac v0.2.2, atsame70q21-pac v0.2.2, atsame70q21b-pac v0.2.2 ([`a628b97`](https://github.com/atsam-rs/atsam-pac/commit/a628b974a612113c93a46bbc2724d403358abb1f))
    - Remove unused dependency bare-metal ([`54fe3c1`](https://github.com/atsam-rs/atsam-pac/commit/54fe3c1f9705e2a9f96176dd8c467fbaed648702))
    - Update dependencies ([`63c755d`](https://github.com/atsam-rs/atsam-pac/commit/63c755d5cb29e4a0d6eec4a1f24498cd2b2801d8))
    - Update pac dependencies ([`8ca7aca`](https://github.com/atsam-rs/atsam-pac/commit/8ca7acab12a2e8af4c6f49d25d79d3c379d4fd35))
    - Update pacs to svd2rust 0.19.0 ([`07a2d93`](https://github.com/atsam-rs/atsam-pac/commit/07a2d930b057726763d359204c406a994661aacc))
</details>

