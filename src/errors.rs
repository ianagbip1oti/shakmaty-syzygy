// This file is part of the shakmaty-syzygy library.
// Copyright (C) 2017-2022 Niklas Fiekas <niklas.fiekas@backscattering.de>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::{backtrace::Backtrace, error::Error, fmt, io};

use crate::{material::Material, types::Metric};

pub type SyzygyResult<T> = Result<T, SyzygyError>;

pub type ProbeResult<T> = Result<T, ProbeError>;

/// Error when probing tablebase.
#[derive(Debug)]
pub enum SyzygyError {
    /// Position has castling rights, but Syzygy tables do not contain
    /// positions with castling rights.
    Castling,
    /// Position has too many pieces. Syzygy tables only support up to
    /// 6 or 7 pieces.
    TooManyPieces,
    /// Missing table.
    MissingTable {
        #[allow(missing_docs)]
        metric: Metric,
        #[allow(missing_docs)]
        material: Material,
    },
    /// Probe failed.
    ProbeFailed {
        #[allow(missing_docs)]
        metric: Metric,
        #[allow(missing_docs)]
        material: Material,
        #[allow(missing_docs)]
        error: Box<ProbeError>,
    },
}

impl fmt::Display for SyzygyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyzygyError::Castling => write!(
                f,
                "syzygy tables do not contain position with castling rights"
            ),
            SyzygyError::TooManyPieces => write!(f, "too many pieces"),
            SyzygyError::MissingTable { metric, material } => {
                write!(f, "required {metric} table not found: {material}")
            }
            SyzygyError::ProbeFailed {
                metric,
                material,
                error,
            } => write!(f, "failed to probe {metric} table {material}: {error}"),
        }
    }
}

impl Error for SyzygyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SyzygyError::ProbeFailed { error, .. } => Some(error),
            _ => None,
        }
    }
}

/// Error when probing a table.
#[derive(Debug)]
pub enum ProbeError {
    /// I/O error.
    Read {
        #[allow(missing_docs)]
        error: io::Error,
    },
    /// Table file has unexpected magic header bytes.
    Magic {
        #[allow(missing_docs)]
        magic: [u8; 4],
    },
    /// Corrupted table.
    CorruptedTable {
        #[allow(missing_docs)]
        backtrace: Backtrace,
    },
}

impl fmt::Display for ProbeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProbeError::Read { error } => write!(f, "i/o error reading table file: {error}"),
            ProbeError::Magic { magic } => write!(f, "invalid magic header bytes: {magic:x?}"),
            ProbeError::CorruptedTable { backtrace } => write!(f, "corrupted table: {backtrace}"),
        }
    }
}

impl Error for ProbeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ProbeError::Read { error } => Some(error),
            _ => None,
        }
    }
}

pub trait ProbeResultExt<T> {
    fn ctx(self, metric: Metric, material: Material) -> SyzygyResult<T>;
}

impl<T> ProbeResultExt<T> for ProbeResult<T> {
    fn ctx(self, metric: Metric, material: Material) -> SyzygyResult<T> {
        self.map_err(|error| SyzygyError::ProbeFailed {
            metric,
            material: material.into_normalized(),
            error: Box::new(error),
        })
    }
}

impl From<io::Error> for ProbeError {
    fn from(error: io::Error) -> ProbeError {
        match error.kind() {
            io::ErrorKind::UnexpectedEof => ProbeError::CorruptedTable {
                backtrace: Backtrace::capture(),
            },
            _ => ProbeError::Read { error },
        }
    }
}

/// Return a `CorruptedTable` error.
macro_rules! throw {
    () => {
        return Err(crate::errors::ProbeError::CorruptedTable {
            backtrace: ::std::backtrace::Backtrace::capture(),
        })
    };
}

/// Unwrap an `Option` or return a `CorruptedTable` error.
macro_rules! u {
    ($e:expr) => {
        match $e {
            Some(ok) => ok,
            None => throw!(),
        }
    };
}

/// Ensure that a condition holds. Otherwise return a `CorruptedTable` error.
macro_rules! ensure {
    ($cond:expr) => {
        if !$cond {
            throw!();
        }
    };
}
