# Changelog

## [0.2.0-alpha8](https://github.com/Waelwindows/objset/compare/v0.1.0-alpha8...v0.2.0-alpha8) (2022-01-04)


### ⚠ BREAKING CHANGES

* **dep:** Removed `(To|From)Primitive` for `PrimitiveType` and `IndexType`
* **dep:** removed `Bone::local_bind_pose` and corresponding functions in the python FFI

### Features

* Add print support to python types + refactor ([1b077dd](https://github.com/Waelwindows/objset/commit/1b077dd996d412d8e5721e29e670eb41ab98acec))
* Parse materials ([ba0e77e](https://github.com/Waelwindows/objset/commit/ba0e77ed71f5984b707a94b2f38680a2c5bc7b34))
* Read `Texture` and `ShaderType` ([25130c1](https://github.com/Waelwindows/objset/commit/25130c1b132c5d95362a233e7ad3741aab39fa25))
* Support Bones and weights, and refactor ([9a2df04](https://github.com/Waelwindows/objset/commit/9a2df0419d8b069f21b8436209983a4fb0f1a772))


### Bug Fixes

* Add `pyo3` support for macOS ([406b432](https://github.com/Waelwindows/objset/commit/406b432429a125b1ae0eb27f3b8c20ec2a3f327c))
* **ci:** Typo in `release-please` ([8cee923](https://github.com/Waelwindows/objset/commit/8cee9230609ab6559bea689a2ec6b841296b1f06))
* Fix bone ids, and `BoneWeight`'s pyffi ([09c8ddf](https://github.com/Waelwindows/objset/commit/09c8ddf53530c2a44bfced3ffddcd6b2c5435a07))
* Parse the rest of `SubMesh` ([cab26af](https://github.com/Waelwindows/objset/commit/cab26afb18c9288bed9a8f747a285542ac499ff0))
* Parse the rest of `SubMesh` and fix others ([b37584c](https://github.com/Waelwindows/objset/commit/b37584ce2a9bca6f685de883ded99174f1f4a550))
* Parse UV indices, updated tristrips algo ([fecf1fb](https://github.com/Waelwindows/objset/commit/fecf1fb47f99bb0a58e9606395e0e8932f706548))
* Remove unneccessary nightly feature ([7314927](https://github.com/Waelwindows/objset/commit/73149277116a8b0da7582692f1681155fcdff499))


### Build System

* **dep:** Remove `cgmath` from dependencies ([3ec1c8a](https://github.com/Waelwindows/objset/commit/3ec1c8aade52f710268cb9c2a0e271d6b8f6048e))
* **dep:** Remove `num-derive` and `num-traits` ([cf004ce](https://github.com/Waelwindows/objset/commit/cf004ce39a48045be54a6b989a07601b698d697c))
