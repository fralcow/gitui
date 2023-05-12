#![allow(renamed_and_removed_lints, clippy::unknown_clippy_lints)]

use std::{
	num::TryFromIntError, path::StripPrefixError,
	string::FromUtf8Error,
};
use thiserror::Error;

///
#[derive(Error, Debug)]
pub enum Error {
	///
	#[error("`{0}`")]
	Generic(String),

	///
	#[error("git: no head found")]
	NoHead,

	///
	#[error("git: conflict during rebase")]
	RebaseConflict,

	///
	#[error("git: remote url not found")]
	UnknownRemote,

	///
	#[error("git: inconclusive remotes")]
	NoDefaultRemoteFound,

	///
	#[error("git: work dir error")]
	NoWorkDir,

	///
	#[error("git: uncommitted changes")]
	UncommittedChanges,

	///
	#[error("git: can\u{2019}t run blame on a binary file")]
	NoBlameOnBinaryFile,

	///
	#[error("binary file")]
	BinaryFile,

	///
	#[error("io error:{0}")]
	Io(#[from] std::io::Error),

	///
	#[error("git error:{0}")]
	Git(#[from] git2::Error),

	///
	#[error("strip prefix error: {0}")]
	StripPrefix(#[from] StripPrefixError),

	///
	#[error("utf8 error:{0}")]
	Utf8Conversion(#[from] FromUtf8Error),

	///
	#[error("TryFromInt error:{0}")]
	IntConversion(#[from] TryFromIntError),

	///
	#[error("EasyCast error:{0}")]
	EasyCast(#[from] easy_cast::Error),

	///
	#[error("shellexpand error:{0}")]
	Shell(#[from] shellexpand::LookupError<std::env::VarError>),

	///
	#[error("path string error")]
	PathString,

	///
	#[error("no parent of commit found")]
	NoParent,

	///
	#[error("not on a branch")]
	NoBranch,
}

///
pub type Result<T> = std::result::Result<T, Error>;

impl<T> From<std::sync::PoisonError<T>> for Error {
	fn from(error: std::sync::PoisonError<T>) -> Self {
		Self::Generic(format!("poison error: {error}"))
	}
}

impl<T> From<crossbeam_channel::SendError<T>> for Error {
	fn from(error: crossbeam_channel::SendError<T>) -> Self {
		Self::Generic(format!("send error: {error}"))
	}
}
