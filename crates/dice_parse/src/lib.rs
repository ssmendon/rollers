#[cfg(feature = "unstable-pratt")]
pub mod winnow_ext;

#[cfg(not(feature = "unstable-pratt"))]
mod winnow_ext;
