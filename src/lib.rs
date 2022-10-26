#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::missing_docs_in_private_items
)]
///Module to hold [`either::Either`]
pub mod either;
///Module to hold commonly-used and widely applicable macros
pub mod macros;
///Module to hold circular list cache
pub mod memcache;
///Module to hold structs which deal with time
pub mod time_based_structs;

#[cfg(feature = "piston_cacher")]
///Utility cacher for `G2dTexture` objects in a set directory
pub mod cacher;
///Module to hold Error Extension traits. Can appear empty if not many features are enabled
pub mod error_ext;

///Private to crate
mod crate_private {
    ///Trait which cannot be externally implemented
    pub trait Sealed {}
}

/*
Testing Template:

#[cfg(test)]
mod tests {
    #[cfg(feature = "anyhow")]
    mod ah {}
    #[cfg(not(feature = "anyhow"))]
    mod nah {}
}

 */
