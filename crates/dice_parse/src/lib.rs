#[cfg(feature = "unstable")]
pub mod winnow_ext;

#[cfg(not(feature = "unstable"))]
mod winnow_ext;
