## [3.4.2](https://github.com/ocni-dtu/lcax/compare/v3.4.1...v3.4.2) (2026-01-23)

### Bug Fixes

* **ci:** update node ([30d3c4d](https://github.com/ocni-dtu/lcax/commit/30d3c4d58d4db88398d95d013c575a6730953f94))

## [3.4.1](https://github.com/ocni-dtu/lcax/compare/v3.4.0...v3.4.1) (2026-01-23)

### Bug Fixes

* **ci:** fixed npm token permissions ([21af941](https://github.com/ocni-dtu/lcax/commit/21af9411a2330e61b55831b5b795144631890b33))

## [3.4.0](https://github.com/ocni-dtu/lcax/compare/v3.3.0...v3.4.0) (2026-01-23)

### Features

* remove `format_version` field from EPD and Generic Data ([#134](https://github.com/ocni-dtu/lcax/issues/134)) ([3463b03](https://github.com/ocni-dtu/lcax/commit/3463b032547f047d43f87e86ccc18f26c9900b37))

## [3.3.0](https://github.com/ocni-dtu/lcax/compare/v3.2.0...v3.3.0) (2025-12-08)

### Features

* **calculation:** Add dependency lockfile and calculation options features ([#133](https://github.com/ocni-dtu/lcax/issues/133)) ([a5ee065](https://github.com/ocni-dtu/lcax/commit/a5ee0652fadb52e178b11520cd0027815ec107f7))

## [3.2.0](https://github.com/ocni-dtu/lcax/compare/v3.1.0...v3.2.0) (2025-11-26)

### Features

* add iterable enums and made roof_type optional ([8c84923](https://github.com/ocni-dtu/lcax/commit/8c8492358f0f067aba051b1d2d627154c934be83))

## [3.1.0](https://github.com/ocni-dtu/lcax/compare/v3.0.2...v3.1.0) (2025-05-03)

### Features

* **docs:** Added new documentation site ([051e0fe](https://github.com/ocni-dtu/lcax/commit/051e0fe16ae5ccdc201dbe2b124e236c2b2deb2b)), closes [#55](https://github.com/ocni-dtu/lcax/issues/55)
* Improving interfaces and documentation site ([26b1354](https://github.com/ocni-dtu/lcax/commit/26b13541699061088bb01c63656c943c5e7a363a))
* **js:** Improved JS interfaces. ([7e924da](https://github.com/ocni-dtu/lcax/commit/7e924daebb0b01ddbbd6ac958b8c813ca8cff663)), closes [#115](https://github.com/ocni-dtu/lcax/issues/115)
* **python:** Improved Python interfaces. ([b015fc6](https://github.com/ocni-dtu/lcax/commit/b015fc6f5a1b340a92816b1cb36f13c1862ba1ab)), closes [#114](https://github.com/ocni-dtu/lcax/issues/114)

## [3.0.2](https://github.com/ocni-dtu/lcax/compare/v3.0.1...v3.0.2) (2025-03-30)

### Bug Fixes

* fixed cargo features on release ([e92d5b2](https://github.com/ocni-dtu/lcax/commit/e92d5b2ec28e5e9063be5210f2541be1a3fef585))

## [3.0.1](https://github.com/ocni-dtu/lcax/compare/v3.0.0...v3.0.1) (2025-03-30)

### Bug Fixes

* fixed semantic release ([98e23a2](https://github.com/ocni-dtu/lcax/commit/98e23a2b7411d0ce6e45d051bd56d6768dcb2e8c))

## [3.0.0](https://github.com/ocni-dtu/lcax/compare/v2.6.4...v3.0.0) (2025-03-30)

### ⚠ BREAKING CHANGES

* Bumping to v3.0

### Features

* add methods to deal with impacts ([#110](https://github.com/ocni-dtu/lcax/issues/110)) ([825c325](https://github.com/ocni-dtu/lcax/commit/825c325cd15e262cd3cf134ff302686196669b96))
* Added software version ([#90](https://github.com/ocni-dtu/lcax/issues/90)) ([96dd984](https://github.com/ocni-dtu/lcax/commit/96dd984534a88a13c71e3671b2d451fb3c624e1c))
* added toLCAbyg for JS ([2f67e06](https://github.com/ocni-dtu/lcax/commit/2f67e06c6e09a363751c7c08df302fd2c57f0d5b))
* changed inputs to to_lcabyg in Python ([ec7eea3](https://github.com/ocni-dtu/lcax/commit/ec7eea3626c1d19094656e177a3b4a74dd05221e))
* Renamed TechFlow to GenericData ([#91](https://github.com/ocni-dtu/lcax/issues/91)) ([60b8e20](https://github.com/ocni-dtu/lcax/commit/60b8e202ea948e36bbf56ad2f4ee4e5cc57d2208))

### Bug Fixes

* **convert:** Added more LCAbyg test examples ([#106](https://github.com/ocni-dtu/lcax/issues/106)) ([007c08d](https://github.com/ocni-dtu/lcax/commit/007c08d278900dc9ca79e7f113229ab06fca7c6f))
* Fixed issues in LCAByg converter ([309b298](https://github.com/ocni-dtu/lcax/commit/309b298594d17cdb6a6324baca47045ccd524de9))
* fixing cargo build ([229b635](https://github.com/ocni-dtu/lcax/commit/229b6357cbf07e097bbeb4cc6eaad6c4f7a1f03d))
* Included comments from ILCD ([#109](https://github.com/ocni-dtu/lcax/issues/109)) ([515b083](https://github.com/ocni-dtu/lcax/commit/515b083250d1d53ff38bab6e5023fabc7cb35b65)), closes [#107](https://github.com/ocni-dtu/lcax/issues/107)

### Reverts

* Revert "Chrk/77 update lcabyg ([#103](https://github.com/ocni-dtu/lcax/issues/103))" ([#104](https://github.com/ocni-dtu/lcax/issues/104)) ([7ea74e4](https://github.com/ocni-dtu/lcax/commit/7ea74e41a176e71819d7f0742f303c925c93b326))

### Continuous Integration

* semantic-release v3.0-beta ([4e1376a](https://github.com/ocni-dtu/lcax/commit/4e1376a9a71598467476a4ecc291ff70aa5d3ac7))

## [3.0.0-beta.7](https://github.com/ocni-dtu/lcax/compare/v3.0.0-beta.6...v3.0.0-beta.7) (2025-03-30)

### Features

* added toLCAbyg for JS ([2f67e06](https://github.com/ocni-dtu/lcax/commit/2f67e06c6e09a363751c7c08df302fd2c57f0d5b))
* changed inputs to to_lcabyg in Python ([ec7eea3](https://github.com/ocni-dtu/lcax/commit/ec7eea3626c1d19094656e177a3b4a74dd05221e))

## [3.0.0-beta.6](https://github.com/ocni-dtu/lcax/compare/v3.0.0-beta.5...v3.0.0-beta.6) (2025-03-26)

### Features

* add methods to deal with impacts ([#110](https://github.com/ocni-dtu/lcax/issues/110)) ([825c325](https://github.com/ocni-dtu/lcax/commit/825c325cd15e262cd3cf134ff302686196669b96))

## [3.0.0-beta.5](https://github.com/ocni-dtu/lcax/compare/v3.0.0-beta.4...v3.0.0-beta.5) (2025-03-25)

### Bug Fixes

* Included comments from ILCD ([#109](https://github.com/ocni-dtu/lcax/issues/109)) ([515b083](https://github.com/ocni-dtu/lcax/commit/515b083250d1d53ff38bab6e5023fabc7cb35b65)), closes [#107](https://github.com/ocni-dtu/lcax/issues/107)

## [3.0.0-beta.4](https://github.com/ocni-dtu/lcax/compare/v3.0.0-beta.3...v3.0.0-beta.4) (2025-03-25)

### Bug Fixes

* Fixed issues in LCAByg converter ([309b298](https://github.com/ocni-dtu/lcax/commit/309b298594d17cdb6a6324baca47045ccd524de9))

## [3.0.0-beta.3](https://github.com/ocni-dtu/lcax/compare/v3.0.0-beta.2...v3.0.0-beta.3) (2025-03-16)

### Bug Fixes

* **convert:** Added more LCAbyg test examples ([#106](https://github.com/ocni-dtu/lcax/issues/106)) ([007c08d](https://github.com/ocni-dtu/lcax/commit/007c08d278900dc9ca79e7f113229ab06fca7c6f))

## [3.0.0-beta.2](https://github.com/ocni-dtu/lcax/compare/v3.0.0-beta.1...v3.0.0-beta.2) (2025-03-14)

### Bug Fixes

* fixing cargo build ([229b635](https://github.com/ocni-dtu/lcax/commit/229b6357cbf07e097bbeb4cc6eaad6c4f7a1f03d))

## [3.0.0-beta.1](https://github.com/ocni-dtu/lcax/compare/v2.6.4...v3.0.0-beta.1) (2025-03-14)

### ⚠ BREAKING CHANGES

* Bumping to v3.0

### Features

* Added software version ([#90](https://github.com/ocni-dtu/lcax/issues/90)) ([96dd984](https://github.com/ocni-dtu/lcax/commit/96dd984534a88a13c71e3671b2d451fb3c624e1c))
* Renamed TechFlow to GenericData ([#91](https://github.com/ocni-dtu/lcax/issues/91)) ([60b8e20](https://github.com/ocni-dtu/lcax/commit/60b8e202ea948e36bbf56ad2f4ee4e5cc57d2208))

### Continuous Integration

* semantic-release v3.0-beta ([4e1376a](https://github.com/ocni-dtu/lcax/commit/4e1376a9a71598467476a4ecc291ff70aa5d3ac7))

## [2.6.4](https://github.com/ocni-dtu/lcax/compare/v2.6.3...v2.6.4) (2025-01-24)

### Bug Fixes

* graceful error handling when subtype anie is invalid ([#98](https://github.com/ocni-dtu/lcax/issues/98)) ([4c6a120](https://github.com/ocni-dtu/lcax/commit/4c6a1203480f1d7b7a3eeeeff9f1739ee969a3ff)), closes [#97](https://github.com/ocni-dtu/lcax/issues/97)

## [2.6.3](https://github.com/ocni-dtu/lcax/compare/v2.6.2...v2.6.3) (2024-12-16)

### Bug Fixes

* Updated ILCD Parser ([#88](https://github.com/ocni-dtu/lcax/issues/88)) ([7e134ce](https://github.com/ocni-dtu/lcax/commit/7e134ce697d3625fddabaed2a9a4e0dac7682f85)), closes [#87](https://github.com/ocni-dtu/lcax/issues/87)

## [2.6.2](https://github.com/ocni-dtu/lcax/compare/v2.6.1...v2.6.2) (2024-12-15)

### Bug Fixes

* LCIMethodAndAllocation missing "other" field ([#86](https://github.com/ocni-dtu/lcax/issues/86)) ([14bd934](https://github.com/ocni-dtu/lcax/commit/14bd934596a1eda1695d0896bb060fe61bfa791a)), closes [#85](https://github.com/ocni-dtu/lcax/issues/85)

## [2.6.1](https://github.com/ocni-dtu/lcax/compare/v2.6.0...v2.6.1) (2024-11-04)

### Bug Fixes

* made metadata fields except any kind of data type ([1ea567e](https://github.com/ocni-dtu/lcax/commit/1ea567eef362ced89cc951569610fb17f32bac1b))

## [2.6.0](https://github.com/ocni-dtu/lcax/compare/v2.5.2...v2.6.0) (2024-09-27)

### Features

* exposing calculation functionality to js and py ([512df66](https://github.com/ocni-dtu/lcax/commit/512df667b930dc46c4519c5c3a620ca58a648ccc))

### Bug Fixes

* adding deprecation warning for SLiCE removal in 3.0.0 ([adc5332](https://github.com/ocni-dtu/lcax/commit/adc5332afaef92fd0b1d6156d645f6281c118452))

## [2.5.2](https://github.com/ocni-dtu/lcax/compare/v2.5.1...v2.5.2) (2024-09-16)

### Bug Fixes

* added more options to building typology, building type and roof type ([cf344b9](https://github.com/ocni-dtu/lcax/commit/cf344b92d2c918660b6726df02c683b5d66ad4c9)), closes [#72](https://github.com/ocni-dtu/lcax/issues/72) [#73](https://github.com/ocni-dtu/lcax/issues/73)
* updated cargo token ([bbe6717](https://github.com/ocni-dtu/lcax/commit/bbe6717fbd099050b3d9059f9567ce1ebcd20c4f))

## [2.5.1](https://github.com/ocni-dtu/lcax/compare/v2.5.0...v2.5.1) (2024-08-20)

### Bug Fixes

* fix to how enums are handled in Python ([b984cfd](https://github.com/ocni-dtu/lcax/commit/b984cfd2d60aaedb8b89f1a39c2e8d85746aece7))

## [2.5.0](https://github.com/ocni-dtu/lcax/compare/v2.4.0...v2.5.0) (2024-08-19)

### Features

* Added linux arm build for Python ([5f6f071](https://github.com/ocni-dtu/lcax/commit/5f6f0712ceabd7aeac7e96eaaec84b8adac08513))

## [2.4.0](https://github.com/ocni-dtu/lcax/compare/v2.3.2...v2.4.0) (2024-07-26)

### Features

* Adding A0 and B8 life cycle stages ([2ae8f61](https://github.com/ocni-dtu/lcax/commit/2ae8f6112b7ea3d561831aeda1f0e83992dff99d)), closes [#63](https://github.com/ocni-dtu/lcax/issues/63)
* Changing BuildingModelScope to vector of enums. ([5ece309](https://github.com/ocni-dtu/lcax/commit/5ece309d34168ceeff0f0af8bf98aa993ba0ea7c)), closes [#58](https://github.com/ocni-dtu/lcax/issues/58)
* Updating building type ([e7d7211](https://github.com/ocni-dtu/lcax/commit/e7d7211a15e75dc150bde9fb67b3f598d73e019a)), closes [#65](https://github.com/ocni-dtu/lcax/issues/65)
* Updating building typology to be a vector of enums ([993acac](https://github.com/ocni-dtu/lcax/commit/993acac66a5f9736e3ff2919d27bcf8cc4aeafb6)), closes [#66](https://github.com/ocni-dtu/lcax/issues/66)
* Updating project phases ([0c7beec](https://github.com/ocni-dtu/lcax/commit/0c7beec06e3f14bf73ff629d438e67d76effba80)), closes [#64](https://github.com/ocni-dtu/lcax/issues/64)

## [2.3.2](https://github.com/ocni-dtu/lcax/compare/v2.3.1...v2.3.2) (2024-07-23)

### Bug Fixes

* typo ([9d814f2](https://github.com/ocni-dtu/lcax/commit/9d814f2329df4ad22bc19ffec12702548e0c704f))

## [2.3.1](https://github.com/ocni-dtu/lcax/compare/v2.3.0...v2.3.1) (2024-07-23)

### Bug Fixes

* updating release ([8faf90f](https://github.com/ocni-dtu/lcax/commit/8faf90fe29767a9993b51c7258a94bbdbeba8cc9))

## [2.3.0](https://github.com/ocni-dtu/lcax/compare/v2.2.4...v2.3.0) (2024-07-23)

### Features

* adding kgm3 to units. resolves [#59](https://github.com/ocni-dtu/lcax/issues/59) ([41b0738](https://github.com/ocni-dtu/lcax/commit/41b0738ab1360c9e54311b7195169f7e7094004a))
* adding results to SLiCE conversion ([81b23a1](https://github.com/ocni-dtu/lcax/commit/81b23a199ae7b41a88768e2066475be673f866fe))
* initial implementation of the calculation module ([f1ee5e0](https://github.com/ocni-dtu/lcax/commit/f1ee5e0f889dd01fb7354025d81596ed0a888305))

### Bug Fixes

* fixed issue with enums from Rust ([8809bb2](https://github.com/ocni-dtu/lcax/commit/8809bb2c74a38aafa1215f41eb70962637300069))

## [2.2.4](https://github.com/ocni-dtu/lcax/compare/v2.2.3...v2.2.4) (2024-07-06)

### Bug Fixes

* fixing pypi release ([df5a428](https://github.com/ocni-dtu/lcax/commit/df5a428c58a7ed3f8f18fe58a0366b7b2ddb0a98))

## [2.2.3](https://github.com/ocni-dtu/lcax/compare/v2.2.2...v2.2.3) (2024-07-06)

### Bug Fixes

* explicitly added rust target ([87c88b9](https://github.com/ocni-dtu/lcax/commit/87c88b901c2b641dba4d4a48deaa6d148b738f6b))

## [2.2.2](https://github.com/ocni-dtu/lcax/compare/v2.2.1...v2.2.2) (2024-07-06)

### Bug Fixes

* adding more compile targets for Python ([371a56d](https://github.com/ocni-dtu/lcax/commit/371a56d839b668df439bb1a9b390e9dd67c2f32d))

## [2.2.1](https://github.com/ocni-dtu/lcax/compare/v2.2.0...v2.2.1) (2024-06-05)

### Bug Fixes

* naming fix to Transport ([fd17d3d](https://github.com/ocni-dtu/lcax/commit/fd17d3daf852f3e073bb327528a5f473e630f673))

## [2.2.0](https://github.com/ocni-dtu/lcax/compare/v2.1.2...v2.2.0) (2024-06-05)

### Features

* added kWh as unit ([e7f450a](https://github.com/ocni-dtu/lcax/commit/e7f450a5a42f5cdbd16334bd28a684b88cc248ea))
* Made the transport field of Product a list, so more transport objects can be included. ([e149b2b](https://github.com/ocni-dtu/lcax/commit/e149b2b6f9de983299dceb025b1e6b6f33716a1b))
* Unified object reference and made it possible to reference assemblies and products ([57a94af](https://github.com/ocni-dtu/lcax/commit/57a94af78b6a932e8d82e43c8af5e715bc80c9c4))

## [2.1.2](https://github.com/ocni-dtu/lcax/compare/v2.1.1...v2.1.2) (2024-05-09)


### Bug Fixes

* another naming correction ([d7f9beb](https://github.com/ocni-dtu/lcax/commit/d7f9beb578576562ac5bac84bcc50bcfc84bd25c))

## [2.1.1](https://github.com/ocni-dtu/lcax/compare/v2.1.0...v2.1.1) (2024-05-09)


### Bug Fixes

* corrected naming serialization of models ([62c8f35](https://github.com/ocni-dtu/lcax/commit/62c8f3517523f1fa8aaeebcabbf3a5ca60311ac2))

## [2.1.0](https://github.com/ocni-dtu/lcax/compare/v2.0.6...v2.1.0) (2024-05-09)


### Features

* corrected serialization of models ([d77a0a4](https://github.com/ocni-dtu/lcax/commit/d77a0a4a05c4e941776d76a9ae1dca6c16a4a778))

## [2.0.6](https://github.com/ocni-dtu/lcax/compare/v2.0.5...v2.0.6) (2024-05-09)


### Bug Fixes

* made description of assembly and product optional ([f371cff](https://github.com/ocni-dtu/lcax/commit/f371cfff0964dd0026b07cd8cecc67e6707a403b))
* solved test and build issues ([ecde85b](https://github.com/ocni-dtu/lcax/commit/ecde85b345488660bf0faf3e94c642a11380836c))

## [2.0.5](https://github.com/ocni-dtu/lcax/compare/v2.0.4...v2.0.5) (2024-05-08)


### Bug Fixes

* fixed cargo packages ([c094ee3](https://github.com/ocni-dtu/lcax/commit/c094ee35833e27ac48308fc75f37e309faa84329))

## [2.0.4](https://github.com/ocni-dtu/lcax/compare/v2.0.3...v2.0.4) (2024-05-08)


### Bug Fixes

* bumping version ([e3aecae](https://github.com/ocni-dtu/lcax/commit/e3aecae6c31cc1d993dca2f48ed8ee5296623f4b))

## [2.0.3](https://github.com/ocni-dtu/lcax/compare/v2.0.2...v2.0.3) (2024-05-08)


### Bug Fixes

* multiple cargo releases ([0ca74ee](https://github.com/ocni-dtu/lcax/commit/0ca74ee8493b76718debc81419de2dd55f2e6251))

## [2.0.2](https://github.com/ocni-dtu/lcax/compare/v2.0.1...v2.0.2) (2024-05-08)


### Bug Fixes

* changes to semantic-release-cargo ([fab2dcf](https://github.com/ocni-dtu/lcax/commit/fab2dcff9b63b5677129936c2e3ccb117a15473c))
* changes to semantic-release-cargo ([ec7a3ab](https://github.com/ocni-dtu/lcax/commit/ec7a3ab22afaca80b581220921c69fb2082f5f29))

## [2.0.1](https://github.com/ocni-dtu/lcax/compare/v2.0.0...v2.0.1) (2024-05-08)


### Bug Fixes

* formatting to bump version ([afe368a](https://github.com/ocni-dtu/lcax/commit/afe368ac2a5c5417ab3290c13978ab53037f0993))

## [2.0.0](https://github.com/ocni-dtu/lcax/compare/v1.7.0...v2.0.0) (2024-05-08)


### ⚠ BREAKING CHANGES

* Merging EPDx into LCAx and changing EPD fields.

### Features

* Added converter for SLiCE datasets ([5110d0a](https://github.com/ocni-dtu/lcax/commit/5110d0a3cc1a958df9a7d3f95afebc2a8de5a668))
* Refactor and merge EPDx ([0553137](https://github.com/ocni-dtu/lcax/commit/05531373447b0f028af6077cff47ab43f6ec8a30))

## [1.7.0](https://github.com/ocni-dtu/lcax/compare/v1.6.0...v1.7.0) (2024-04-08)


### Features

* [#41](https://github.com/ocni-dtu/lcax/issues/41) Added CarbEnMats MetaData ([1ae39d7](https://github.com/ocni-dtu/lcax/commit/1ae39d7ad1094076e97038fd7fdf1fb7ea401b43))

## [1.6.0](https://github.com/ocni-dtu/lcax/compare/v1.5.0...v1.6.0) (2024-03-26)


### Features

* renamed fields ([e37d35b](https://github.com/ocni-dtu/lcax/commit/e37d35bd6da80b0295d2a8f2be5ac280699aaff8))

## [1.5.0](https://github.com/ocni-dtu/lcax/compare/v1.4.2...v1.5.0) (2024-03-26)


### Features

* added convert functions to JS and Python ([7d22438](https://github.com/ocni-dtu/lcax/commit/7d22438fa4793d67965ede591fa1613dda2ff535))
* added support for LCAbyg result files ([0dd026f](https://github.com/ocni-dtu/lcax/commit/0dd026f08f7a3fede61b43e138e39fde1f7381aa))
* initial implementation of parsing LCAbyg projects ([84dae7e](https://github.com/ocni-dtu/lcax/commit/84dae7e92b2c0b39c0f8fcaf0352de65cd31c16f))
* updated LCAx data structure ([f0c69f2](https://github.com/ocni-dtu/lcax/commit/f0c69f2fc59ac0bfc72385f493d5db0bc6ad5786))


### Bug Fixes

* resolved test issues ([e25287d](https://github.com/ocni-dtu/lcax/commit/e25287dd6a87c9be56af7c9b78d6ad30fea410d4))

## [1.4.2](https://github.com/ocni-dtu/lcax/compare/v1.4.1...v1.4.2) (2024-01-01)


### Bug Fixes

* updated C# types generation ([aeb9fb8](https://github.com/ocni-dtu/lcax/commit/aeb9fb886b2ebe00943b87f2489a8f3b741e44dd))

## [1.4.1](https://github.com/ocni-dtu/lcax/compare/v1.4.0...v1.4.1) (2023-12-31)


### Bug Fixes

* updated working dir cs ([63180d2](https://github.com/ocni-dtu/lcax/commit/63180d28aa60eb72bb2c3b249d0cfdf6f982f504))

## [1.4.0](https://github.com/ocni-dtu/lcax/compare/v1.3.1...v1.4.0) (2023-12-31)


### Features

* added nuget package ([803a212](https://github.com/ocni-dtu/lcax/commit/803a212eb8f86097ab2242a0a63adabf87ce91fc))

## [1.3.1](https://github.com/ocni-dtu/lcax/compare/v1.3.0...v1.3.1) (2023-08-25)


### Bug Fixes

* fixed npm build and publish ([3c1a464](https://github.com/ocni-dtu/lcax/commit/3c1a46420ea8354950b9e1d5bd3167eb0b79d743))

## [1.3.0](https://github.com/ocni-dtu/lcax/compare/v1.2.5...v1.3.0) (2023-08-25)


### Features

* creating NPM package for LCAx ([b429581](https://github.com/ocni-dtu/lcax/commit/b429581b4562708d23bfee5fbdaed1af7a962bb7))

## [1.2.5](https://github.com/ocni-dtu/lcax/compare/v1.2.4...v1.2.5) (2023-08-23)


### Bug Fixes

* added pydantic.py to release/wheel ([1909c79](https://github.com/ocni-dtu/lcax/commit/1909c79afe1e0d3a8bc9f822dfc3ead8ddd6f708))

## [1.2.4](https://github.com/ocni-dtu/lcax/compare/v1.2.3...v1.2.4) (2023-08-23)


### Bug Fixes

* updated python publish command ([4b6412d](https://github.com/ocni-dtu/lcax/commit/4b6412d6a4c8c5ab87d20929b6173e70244968fe))

## [1.2.3](https://github.com/ocni-dtu/lcax/compare/v1.2.2...v1.2.3) (2023-08-23)


### Bug Fixes

* updated python dependencies ([1ba400c](https://github.com/ocni-dtu/lcax/commit/1ba400cd7ce653c66e1e3b4dc669bde8db42fc43))

## [1.2.2](https://github.com/ocni-dtu/lcax/compare/v1.2.1...v1.2.2) (2023-08-22)


### Bug Fixes

* updated Pydantic model generation ([bb61375](https://github.com/ocni-dtu/lcax/commit/bb61375d1cb25bc6881474d4a1487b71067e24cb))

## [1.2.1](https://github.com/ocni-dtu/lcax/compare/v1.2.0...v1.2.1) (2023-08-22)


### Bug Fixes

* corrected pydantic location ([9268be1](https://github.com/ocni-dtu/lcax/commit/9268be12a5b2289cb1d81ee3a82fa275aadd50f5))

## [1.2.0](https://github.com/ocni-dtu/lcax/compare/v1.1.1...v1.2.0) (2023-08-21)


### Features

* made enums lowercase and structs camelCase ([23a81f0](https://github.com/ocni-dtu/lcax/commit/23a81f0242ba14c67fd19c1e3db3ef196a515c11))

## [1.1.1](https://github.com/ocni-dtu/lcax/compare/v1.1.0...v1.1.1) (2023-08-17)


### Bug Fixes

* added test cases and fixed issues with Python packaging ([39680eb](https://github.com/ocni-dtu/lcax/commit/39680eb98d07fa5f218d0bbbea09648d7314645b))

## [1.1.0](https://github.com/ocni-dtu/lcax/compare/v1.0.0...v1.1.0) (2023-08-16)


### Features

* updated result object ([feafef6](https://github.com/ocni-dtu/lcax/commit/feafef6e7870b02befddf9dc8aef9f062020bb8b))
* updated result object ([8e096c0](https://github.com/ocni-dtu/lcax/commit/8e096c07af19dd0bf53520481b86a5773400f7b3))
* updated transport object ([98da16b](https://github.com/ocni-dtu/lcax/commit/98da16b31f76f35cfe7b43579d29cd4e52395511))

## 1.0.0 (2023-08-16)


### Bug Fixes

* updated epdx package ([d474eac](https://github.com/ocni-dtu/lcax/commit/d474eac39d4544c5fde2f1383ddcf86d0845d894))
