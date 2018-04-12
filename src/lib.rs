// This file is part of the shakmaty-syzygy library.
// Copyright (C) 2017-2018 Niklas Fiekas <niklas.fiekas@backscattering.de>
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

//! Probe Syzygy tablebases.

#![doc(html_root_url = "https://docs.rs/shakmaty-syzygy/0.1.0")]

#![warn(missing_debug_implementations)]

#![feature(try_trait)]

extern crate arrayvec;
#[macro_use]
extern crate bitflags;
extern crate bit_vec;
extern crate byteorder;
#[macro_use]
extern crate failure;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate num_integer;
extern crate shakmaty;
extern crate positioned_io;
extern crate fnv;
extern crate double_checked_cell;

mod material;
mod types;
mod table;
mod tablebases;

pub use types::{Syzygy, Wdl, Dtz, SyzygyError};
pub use tablebases::Tablebases;
