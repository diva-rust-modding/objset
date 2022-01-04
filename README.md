# objset

`objset` is a rust library for manipulating SEGA's proprietary model object format (`_obj.bin`) used in various games.
The games that use this format are (but are not limited to):
+ Virtua Fighter 5 (all versions)
+ Hatsune Miku: Project DIVA (versions based on Virtua Fighter 5; Arcade, Dreamy Theater, F...)
! Note that as of writing, the `objset` library only supports the legacy objset format. In Hatsune Miku: Project DIVA F2nd and X/XHD, the format was rebuilt to be more modern. This crate cannot read the new OSD format. 
+ After Burner Climax (older version of the objset format)

## Usage

`objset` can be embedded into any standard rust crate, and thus can be used to create any utilities.

`objset` also exposes a Python FFI for use with any software of choice, an example is the [blender_io_scene_bin](https://github.com/Waelwindows/blender_io_scene_bin) plugin which is the de facto implementation. The main entry point of this FFI is [src/py_ffi.rs](https://github.com/Waelwindows/objset/blob/master/src/py_ffi.rs).
 
## Building

The recommended way to compile this project is via `cargo` for native rust crates, and `maturin` for the python bindings.

### Python Bindings

This project utilizes the `pyo3` crate to generate its bindings. And thus, it follows its building instructions. Here's an abridged version of the instructions.

#### Manual 

This method produces a dynamically linked library, which is useful for development

``` sh
cargo build --release --all-features
mv ./target/release/
ln -s libobjset.dylib objset.so
```

For *nix OSes, the extension of the `objset` file should be `.so`, meanwhile for Windows it should be `.pyd`

You can verify that your built objects are working by importing the module in a python repl.

``` python
import objset
```

If everything worked, you should not get any exceptions

#### Maturin (Recommended)

This method produces wheels that could be installed using pip, they are generated by the following command to compile the project

``` sh
maturin build --release --no-sdist --cargo-extra-args="--all-features"
```

If prompted to specify the interpreter, use the default system interpreter `-i python`

# License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

# Special Thanks to
- [Skyth](https://github.com/blueskythlikesclouds)
- [korenkonder](https://github.com/korenkonder)
- [chrrox](https://www.deviantart.com/chrrox)
- [minmode](https://www.deviantart.com/minmode)
- [feat_jinsoul](https://github.com/featjinsoul)
