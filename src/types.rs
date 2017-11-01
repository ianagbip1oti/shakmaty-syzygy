// This file is part of the shakmaty-syzygy library.
// Copyright (C) 2017 Niklas Fiekas <niklas.fiekas@backscattering.de>
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

use std::ops::Neg;

use shakmaty::{Color, Outcome, Chess};

/// A chess variant with Syzygy support.
pub trait Syzygy {
    const WDL_SUFFIX: &'static str;
    const DTZ_SUFFIX: &'static str;
    const WDL_MAGIC: [u8; 4];
    const DTZ_MAGIC: [u8; 4];
    const PAWNLESS_WDL_MAGIC: [u8; 4];
    const PAWNLESS_DTZ_MAGIC: [u8; 4];
    const ONE_KING: bool;
    const CONNECTED_KINGS: bool;
    const CAPTURES_COMPULSORY: bool;
}

impl Syzygy for Chess {
    const WDL_SUFFIX: &'static str = "rtbw";
    const DTZ_SUFFIX: &'static str = "rtbz";
    const WDL_MAGIC: [u8; 4] = [0x71, 0xe8, 0x23, 0x5d];
    const DTZ_MAGIC: [u8; 4] = [0xd7, 0x66, 0x0c, 0xa5];
    const PAWNLESS_WDL_MAGIC: [u8; 4] = [0x71, 0xe8, 0x23, 0x5d];
    const PAWNLESS_DTZ_MAGIC: [u8; 4] = [0xd7, 0x66, 0x0c, 0xa5];
    const ONE_KING: bool = true;
    const CONNECTED_KINGS: bool = false;
    const CAPTURES_COMPULSORY: bool = false;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(i8)]
pub enum Wdl {
    Loss = -2,
    BlessedLoss = -1,
    Draw = 0,
    CursedWin = 1,
    Win = 2,
}

impl Wdl {
    pub fn from_outcome(outcome: Outcome, pov: Color) -> Wdl {
        match outcome {
            Outcome::Draw => Wdl::Draw,
            Outcome::Decisive { winner } if winner == pov => Wdl::Win,
            _ => Wdl::Loss,
        }
    }
}

impl Neg for Wdl {
    type Output = Wdl;

    fn neg(self) -> Wdl {
        match self {
            Wdl::Loss => Wdl::Win,
            Wdl::BlessedLoss => Wdl::CursedWin,
            Wdl::Draw => Wdl::Draw,
            Wdl::CursedWin => Wdl::BlessedLoss,
            Wdl::Win => Wdl::Loss,
        }
    }
}

impl From<Wdl> for i8 {
    #[inline]
    fn from(wdl: Wdl) -> i8 {
        wdl as i8
    }
}
