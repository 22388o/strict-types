// Strict encoding schema library, implementing validation and parsing
// strict encoded data against a schema.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2022-2023 by
//     Dr. Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright 2022-2023 UBIDECO Institute
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use encoding::stl::{
    Alpha, AlphaCaps, AlphaCapsNum, AlphaNum, AlphaNumDash, AlphaNumLodash, AlphaSmall,
    AsciiPrintable, Bool, Dec, HexDecCaps, HexDecSmall, U4,
};
use encoding::{LIB_NAME_STD, STRICT_TYPES_LIB};

use crate::typeobj::Transpiler;
use crate::typesys::{TypeSymbol, TypeSysId};
use crate::{TranspileError, TypeLib, TypeObjects, TypeSystem};

pub const LIB_ID_STD: &str = "regard_light_ninja_9fsgNpJdXxPiJztBbJVZQKcRbxF8DtpBcYQduxUgXKps";
pub const LIB_ID_STRICT_TYPES: &str =
    "sardine_mimic_stella_AP9Ttup5cS6XFY8ZsPFcYV4REjrgaT85wQhRmXN2tuZ9";

fn _std_stl() -> Result<TypeObjects, TranspileError> {
    Transpiler::new(libname!(LIB_NAME_STD), none!())
        .transpile::<Bool>()
        .transpile::<U4>()
        .transpile::<AsciiPrintable>()
        .transpile::<Alpha>()
        .transpile::<AlphaCaps>()
        .transpile::<AlphaSmall>()
        .transpile::<Dec>()
        .transpile::<HexDecCaps>()
        .transpile::<HexDecSmall>()
        .transpile::<AlphaNum>()
        .transpile::<AlphaCapsNum>()
        .transpile::<AlphaNumDash>()
        .transpile::<AlphaNumLodash>()
        .compile_symbols()
}

pub fn std_stl() -> TypeObjects { _std_stl().expect("invalid strict type Std library") }

fn _strict_types_stl() -> Result<TypeObjects, TranspileError> {
    Transpiler::new(libname!(STRICT_TYPES_LIB), none!())
        .transpile::<TypeLib>()
        .transpile::<TypeSystem>()
        .transpile::<TypeSysId>()
        .transpile::<TypeSymbol>()
        .compile_symbols()
}

pub fn strict_types_stl() -> TypeObjects {
    _strict_types_stl().expect("invalid strict type StrictTypes library")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn std_lib_id() {
        let lib = std_stl().compile().unwrap();
        assert_eq!(lib.id().to_string(), LIB_ID_STD);
    }

    #[test]
    fn strict_types_lib_id() {
        let lib = strict_types_stl().compile().unwrap();
        assert_eq!(lib.id().to_string(), LIB_ID_STRICT_TYPES);
    }
}
