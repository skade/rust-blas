// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

#[stable]
#[repr(C)]
#[derive(Copy, Clone)]
pub enum Order {
    RowMajor=101,
    ColMajor=102,
}

#[stable]
#[repr(C)]
#[derive(Copy, Clone)]
pub enum Transpose {
    NoTrans=111,
    Trans=112,
    ConjTrans=113,
}

#[stable]
#[repr(C)]
#[derive(Copy, Clone)]
pub enum Symmetry {
    Upper=121,
    Lower=122,
}

#[stable]
#[repr(C)]
#[derive(Copy, Clone)]
pub enum Diagonal {
    NonUnit=131,
    Unit=132,
}

#[stable]
#[repr(C)]
#[derive(Copy, Clone)]
pub enum Side {
    Left=141,
    Right=142,
}