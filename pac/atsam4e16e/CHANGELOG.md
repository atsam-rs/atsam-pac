# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.3.0 (2022-12-06)

### Bug Fixes

 - <csr-id-2b9a3080f81274d29310899f9f2e700c6d6a1f49/> Fix #24 (missing critical-section feature)
   - Recent update of svd2rust requires this feature to use
     Peripherals::take()

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 6 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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

 - 19 commits contributed to the release over the course of 603 calendar days.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#10](https://github.com/atsam-rs/atsam-pac/issues/10), [#14](https://github.com/atsam-rs/atsam-pac/issues/14), [#18](https://github.com/atsam-rs/atsam-pac/issues/18)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#10](https://github.com/atsam-rs/atsam-pac/issues/10)**
    - Added support for ATSAME54 and ATSAM4S16 ([`e120060`](https://github.com/atsam-rs/atsam-pac/commit/e120060695b87b48d1c27061c28470a006b2abee))
 * **[#14](https://github.com/atsam-rs/atsam-pac/issues/14)**
    - Adding missing NVIC interrupts for atsam4e and atsam4s ([`2d799ca`](https://github.com/atsam-rs/atsam-pac/commit/2d799ca714c9c8ccfa50b961c6b5fd3276a8ec22))
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
    - Merge pull request #3 from atsam4-rs/john/svd_updates_from_keil ([`20495a6`](https://github.com/atsam-rs/atsam-pac/commit/20495a6a821625762ff2a82bbf9e941f1e1d2c58))
    - Updated SVD's from Keil (which are newer) instead of Atmel. ([`4ad3e20`](https://github.com/atsam-rs/atsam-pac/commit/4ad3e20c44ba3d904c4b525069df198a0581448c))
    - Merge pull request #1 from haata/github_actions ([`081950d`](https://github.com/atsam-rs/atsam-pac/commit/081950d5e8d6dee85d018c6f841e55d3800e042a))
    - Merge branch 'master' into github_actions ([`9db368c`](https://github.com/atsam-rs/atsam-pac/commit/9db368cc54be0b935ad0521d6d9cfd414113f9a3))
    - Adding GitHub Actions ([`1ecf432`](https://github.com/atsam-rs/atsam-pac/commit/1ecf43256f4e6e5af7e32cabd609543a8d133297))
    - Added patch to SAM4E svds to add PASSWD enumeration to relevant KEY fields of the SUPC and PMC peripherals. ([`60d65f0`](https://github.com/atsam-rs/atsam-pac/commit/60d65f0e0f5c5d11f6b22381a15cee1782f34157))
    - Updated sam4e based crate versions to 1.5. ([`849dd8f`](https://github.com/atsam-rs/atsam-pac/commit/849dd8fcf3be0074d98b8fc65e4fb03fdfd4b6b1))
    - Updated SAM4x PACs from latest SVDs. ([`87f4b39`](https://github.com/atsam-rs/atsam-pac/commit/87f4b39163ad7711854e174851ea4814846984c9))
    - Initial checkin of consolodated PACs from seperate repos. ([`04b0589`](https://github.com/atsam-rs/atsam-pac/commit/04b05897a0f7d9fa19fee5536c6b51b9f8f46fc2))
</details>

<csr-unknown>
cortex-m-rt -> 0.7.2svd2rust -> 0.27.2form -> 0.10.0Deprecated atsamd51*, atsame51*, atsame53* and atsame54* as these pacsare already covered by atsamd-halpacs using svd2rust (pactool.sh -f generate) Add initial CHANGELOGs<csr-unknown/>

