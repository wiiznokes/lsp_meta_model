# lsp_meta_model

[![crates.io](https://img.shields.io/crates/v/lsp_meta_model?style=flat-square&logo=rust)](https://crates.io/crates/lsp_meta_model)
[![docs.rs](https://img.shields.io/badge/docs.rs-lsp_meta_model-blue?style=flat-square&logo=docs.rs)](https://docs.rs/lsp_meta_model)
[![license](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](#license)

This is a rust implementation of the [meta model](https://microsoft.github.io/language-server-protocol/specifications/specification-current/#metaModel) of the [language-server-protocol](https://microsoft.github.io/language-server-protocol/overviews/lsp/overview/).

This is useful to parse the json meta file and generate rust code for a lsp implementation.

The goal is to use it in [async-lsp](https://crates.io/crates/async-lsp) to replace the macros.

## License

Mit
