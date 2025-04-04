# Changelog

## [0.10.0](https://github.com/gteufelberger/czml-rs/compare/v0.9.1...v0.10.0) (2025-04-04)


### Features

* Add `LINEAR` as interpolation algorithm option ([7465eb2](https://github.com/gteufelberger/czml-rs/commit/7465eb26c948b65dc150512275884d78e5d0049d))
* Add dependabot support ([1b37418](https://github.com/gteufelberger/czml-rs/commit/1b37418901c6ec35bdaa8655e4e1ff5a55ac65ac))
* Add support for `cartesian` in polyline position ([c30d43c](https://github.com/gteufelberger/czml-rs/commit/c30d43cfc5809588c49ce7536ad755390884d44c))
* Add support for `clamp_to_ground` field in polyline object ([0be02c9](https://github.com/gteufelberger/czml-rs/commit/0be02c9d6135934acf76beb4e63ff5c40403a3a8))
* Add support for `forwardExtrapolationType` field in position object ([83c04fa](https://github.com/gteufelberger/czml-rs/commit/83c04fa2183fd5be94787a95ed96f23a9b6c6472))
* Add support for `maximumScale` field in `model` object ([74fc208](https://github.com/gteufelberger/czml-rs/commit/74fc2089ce7132d0fdff96e769aa8805c09c569e))
* Add support for `outline_width` field in label object ([f50974c](https://github.com/gteufelberger/czml-rs/commit/f50974c38bbb7ca543256c1d3309ee60a62915b6))
* Add support for `polylineOutline` object ([4a83bcc](https://github.com/gteufelberger/czml-rs/commit/4a83bcc80d9edd0c66b069af2555f5c3d7a362e4))
* Add support for `resolution` field in path object ([a28f4cb](https://github.com/gteufelberger/czml-rs/commit/a28f4cbbd2a5fde2f6c3cef3946d805313024ce4))
* Add support for `velocityReference` field in `orientation` object ([2448ace](https://github.com/gteufelberger/czml-rs/commit/2448ace9878258fc9183944b991a841a89500e0c))
* Add support for `viewFrom` object ([07eac8d](https://github.com/gteufelberger/czml-rs/commit/07eac8d96c66eb9cd12c7dbbe85c8b96e03cbdc3))


### Bug Fixes

* `width` can also be a float ([7573234](https://github.com/gteufelberger/czml-rs/commit/75732342870dde56a12e913e781d0eba6b55ff64))
* Add missing serde rename ([b0d51fc](https://github.com/gteufelberger/czml-rs/commit/b0d51fcb9d02926c494ce84e73d5a9aa7c961e1f))
* **docs:** Update incorrect comment ([88e56fa](https://github.com/gteufelberger/czml-rs/commit/88e56fa355c8a5b06d36cd2701005ffe3c456489))

## [0.9.1](https://github.com/gteufelberger/czml-rs/compare/v0.9.0...v0.9.1) (2025-04-03)


### Bug Fixes

* Make `material` field of `path` object public ([ebb1db9](https://github.com/gteufelberger/czml-rs/commit/ebb1db9cc9d491b7b1f9ab8065c2654d88c33e39))
* Make fields of `polyline` objects public ([fe39644](https://github.com/gteufelberger/czml-rs/commit/fe39644b6efe88a57b4ba572ba7e674784245072))

## [0.9.0](https://github.com/gteufelberger/czml-rs/compare/v0.8.0...v0.9.0) (2025-04-03)


### Features

* Add enum state `bottom` to `Origin` enum ([338c265](https://github.com/gteufelberger/czml-rs/commit/338c26511dfdc06c4f76874ae50036a13b8d47a1))
* Add enum state `fill` to `Style` enum ([d1496f1](https://github.com/gteufelberger/czml-rs/commit/d1496f1128c333f34bf2968ecf54fa33003a20b1))
* Add enum state `right` to `Origin` enum ([87908ff](https://github.com/gteufelberger/czml-rs/commit/87908ff69cb51821a5fa1d8356d225bb8c575083))
* Add support for `font` field in `label` object ([b51118c](https://github.com/gteufelberger/czml-rs/commit/b51118c7f5fa0f4fc6cc933087ab2a32a3c693b4))
* Add support for `material` field in `path` object ([f862666](https://github.com/gteufelberger/czml-rs/commit/f862666e4ee3c3ce35411fbac4cc3de6fbc2f7a2))
* Add support for `outline_color` field in `label` object ([32a093b](https://github.com/gteufelberger/czml-rs/commit/32a093ba6bd3570279934a2682f200df79d9b019))
* Add support for `scale` field in `label` object ([52f42ba](https://github.com/gteufelberger/czml-rs/commit/52f42ba1d60fc3af3f4961c3787030ffa0000e72))
* Add support for `show` field in `path` object ([0bd75b2](https://github.com/gteufelberger/czml-rs/commit/0bd75b2c8c3126599ff0d43de015d5dc6f05baa8))
* Add support for color intervals ([3ef7c88](https://github.com/gteufelberger/czml-rs/commit/3ef7c884e088d1643228826e1934e93d6fe1a256))
* Add support for number intervals ([a5858da](https://github.com/gteufelberger/czml-rs/commit/a5858da43bcc1a844847b3b45eaddd6f9f694f51))
* Handle case when `show` is set based on an interval ([949e112](https://github.com/gteufelberger/czml-rs/commit/949e112d7b97db2a129c879be08af71b221a7b2c))

## [0.8.0](https://github.com/gteufelberger/czml-rs/compare/v0.7.0...v0.8.0) (2025-04-03)


### Features

* Add support for `polyline` field ([727d0a3](https://github.com/gteufelberger/czml-rs/commit/727d0a349c6c6b062c00949948d9e44ab41cff63))
* Handle availabilities as both single entry and arrays ([e98c7d8](https://github.com/gteufelberger/czml-rs/commit/e98c7d82cc4121b641bd335476388bae979c420a))


### Bug Fixes

* Fill out missing field ([cc04dc5](https://github.com/gteufelberger/czml-rs/commit/cc04dc5f972599048e745157e7721d7acf139179))

## [0.7.0](https://github.com/gteufelberger/czml-rs/compare/v0.6.1...v0.7.0) (2025-04-02)


### Features

* Add support for `parent` field ([dada822](https://github.com/gteufelberger/czml-rs/commit/dada82213071476039a45b99ca5af6ac2b97b69b))

## [0.6.1](https://github.com/gteufelberger/czml-rs/compare/v0.6.0...v0.6.1) (2025-04-02)


### Bug Fixes

* Remove accidentally introduced `position` field ([ee07545](https://github.com/gteufelberger/czml-rs/commit/ee07545b8f6c269085b6c876b2923ca232ddf0c9))

## [0.6.0](https://github.com/gteufelberger/czml-rs/compare/v0.5.0...v0.6.0) (2025-04-02)


### Features

* Add support for `properties` object ([3c50420](https://github.com/gteufelberger/czml-rs/commit/3c504204b23490d1770141143d246dd896133a1b))


### Bug Fixes

* Make `cartesian` field in `Position` struct public ([d98c475](https://github.com/gteufelberger/czml-rs/commit/d98c475e5d61bbadcad5e86c15f80215efa65d5a))

## [0.5.0](https://github.com/gteufelberger/czml-rs/compare/v0.4.1...v0.5.0) (2025-04-02)


### Features

* Add support for `position` object ([6660c4a](https://github.com/gteufelberger/czml-rs/commit/6660c4aa7e340aab31e220fff61e156fea8f8f80))

## [0.4.1](https://github.com/gteufelberger/czml-rs/compare/v0.4.0...v0.4.1) (2025-04-02)


### Bug Fixes

* Make submodules public ([5a03912](https://github.com/gteufelberger/czml-rs/commit/5a039121d730fc634709a472ddeff94fe4337bdd))

## [0.4.0](https://github.com/gteufelberger/czml-rs/compare/v0.3.0...v0.4.0) (2025-04-02)


### Features

* Add support for `label` object ([b6d4c77](https://github.com/gteufelberger/czml-rs/commit/b6d4c775249efd2b45bdcc184363d78acfa026c8))
* Add support for `path` object ([fdaf34e](https://github.com/gteufelberger/czml-rs/commit/fdaf34ec546cf47b854ccd676e2c8476da487a4e))

## [0.3.0](https://github.com/gteufelberger/czml-rs/compare/v0.2.1...v0.3.0) (2025-04-02)


### Features

* Add support for `minimumPixelSize` field ([8e62fad](https://github.com/gteufelberger/czml-rs/commit/8e62fad0c06ae085f4113d1d96061fc9dc0fb858))
* Add support for scale field in model ([86928b9](https://github.com/gteufelberger/czml-rs/commit/86928b915ab6ef6bf3e3c139541c924da26d010d))

## [0.2.1](https://github.com/gteufelberger/czml-rs/compare/v0.2.0...v0.2.1) (2025-04-01)


### Bug Fixes

* Make packet module public ([c7dd26c](https://github.com/gteufelberger/czml-rs/commit/c7dd26c1f71fba44e81b5229db2a79618db5461c))

## [0.2.0](https://github.com/gteufelberger/czml-rs/compare/v0.1.2...v0.2.0) (2025-03-28)


### Features

* Add support for model field ([a18f539](https://github.com/gteufelberger/czml-rs/commit/a18f539f3a34c59982fbdd83fe9a77f6bb927f4b))
* Add support for orientation field ([a71caa8](https://github.com/gteufelberger/czml-rs/commit/a71caa88de20235665a0296ea48e68f2479eb633))


### Bug Fixes

* **docs:** Update incorrect doc comment ([10996ca](https://github.com/gteufelberger/czml-rs/commit/10996ca899870b623a1c8af2942d660eb51e3d11))
* Remove leftover init code ([96a9312](https://github.com/gteufelberger/czml-rs/commit/96a9312886dce609232860c212e9570ee1f7adac))

## [0.1.2](https://github.com/gteufelberger/czml-rs/compare/v0.1.1...v0.1.2) (2025-03-28)


### Bug Fixes

* Set description in Cargo.toml ([fe0d188](https://github.com/gteufelberger/czml-rs/commit/fe0d18810c7ae6a8584d6785dbefc5a1124b6967))

## [0.1.1](https://github.com/gteufelberger/czml-rs/compare/v0.1.0...v0.1.1) (2025-03-28)


### Bug Fixes

* Set license in Cargo.toml ([f45655f](https://github.com/gteufelberger/czml-rs/commit/f45655fd7246f5814607e906e6b604a7ac5d3990))

## 0.1.0 (2025-03-28)


### Features

* Add initial partial packet struct ([ac6d8ae](https://github.com/gteufelberger/czml-rs/commit/ac6d8ae888e6d8d190fad3e760c5eb16b1178070))
* Add support for billboard object ([6584198](https://github.com/gteufelberger/czml-rs/commit/6584198d1761482cb99f7dcb8f9da8365f83839d))
* Add support for clock field ([d27c8a6](https://github.com/gteufelberger/czml-rs/commit/d27c8a677f7436787e8c459b0f7e64bc46670cf9))
* Derive PartialEq for Packet struct ([f2fa984](https://github.com/gteufelberger/czml-rs/commit/f2fa984e2e7a23d87f6f2b9139338f75ee62ae03))


### Bug Fixes

* Use camelCase for object fields ([86c45ad](https://github.com/gteufelberger/czml-rs/commit/86c45ad81a494f00fd162910be7c9c7e733e8888))
