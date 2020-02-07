//**************************************************************************************************
// mapper.rs                                                                                       *
// Copyright (c) 2020 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[derive(Debug)]
pub struct Mapper<'a> {
    table : &'a Pml4Table
}

impl<'a> Mapper<'a> {
    pub fn new(table: &'a Pml4Table) -> Self {
        Self {
            table
        }
    }

    pub fn map(
        virtual_address: VirtualAddress48,
        map_type: MapType,
    ) {

    }
    pub fn translate(virtual_address: VirtualAddress48) -> MapType {

    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MapType {
    None,
    Page4Kb(PhysicalAdddress52),
    Page2Mb(PhysicalAdddress52),
    Page1Gb(PhysicalAdddress52),
}

impl MapType {

}