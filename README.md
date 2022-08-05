# Tiff-NDArray

This crate serves literally one purpose, making `NDArray` arrays out of stacked `.TIFF` images. The only method of real use is `get_tiff_array` which returns type `MagicTiffArray`.
`MagicTiffArray` is an enum of `GrayU16(Array<u16, Dim<[usize; 3]>>)` and `RgbU8(Array<u8, Dim<[usize; 4]>>)`, which wraps arrays of different types.

`GrayU16(Array<u16, Dim<[usize; 3]>>)` has shape `(n_imgs, dim_X, dim_Y)` and `RgbU8(Array<u8, Dim<[usize; 4]>>)` has shape `(n_imgs, dim_X, dim_Y, 3)`. See [https://github.com/rust-ndarray/ndarray](https://github.com/rust-ndarray/ndarray) and [https://docs.rs/ndarray/latest/ndarray](https://docs.rs/ndarray/latest/ndarray) for more information.

If there are any issues, feel free to raise a GitHub issue or send a PR. Because of workload, I may not immediately respond to issues, however.
