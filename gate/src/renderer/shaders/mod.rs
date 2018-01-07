// Copyright 2017-2018 Matthew D. Michelotti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::os::raw::c_char;

pub const VS_SPRITE_SRC: *const c_char = include_c_str!("sprite.vert");
pub const FS_SPRITE_SRC: *const c_char = include_c_str!("sprite.frag");

pub const VS_TILED_SRC: *const c_char = include_c_str!("tiled.vert");
pub const FS_TILED_SRC: *const c_char = include_c_str!("tiled.frag");
