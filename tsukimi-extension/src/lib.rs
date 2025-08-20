wit_bindgen::generate!({
    world: "extension",
    // path: "wit",
    pub_export_macro: true,
    default_bindings_module: "tsukimi_extension",
    skip: [
            "wasi:filesystem",
            "wasi:io",
            "wasi:clocks",
            "wasi:random",
        ]
});

// Ré-export des types générés pour que les plugins puissent les utiliser
// pub use exports::*;

// // Macro pour enregistrer une extension
// #[macro_export]
// macro_rules! register_extension {
//     ($component:ident) => {
//         // wit_bindgen::export!($component);
//         tsukimi_extension::export!($component);
//     };
// }
