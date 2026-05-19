#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    command_buf_port: CommandBufPort,
    response_buf_port: ResponseBufPort,
    rx_data_port: RxDataPort,
    tx_data_port: TxDataPort,
    ibi_status_buf: IbiStatusBuf,
    _reserved5: [u8; 0x24],
    ibi_data_buf: IbiDataBuf,
    _reserved6: [u8; 0x7c],
    dev_addr_table1_loc: DevAddrTable1Loc,
    dev_addr_table2_loc: DevAddrTable2Loc,
    dev_addr_table3_loc: DevAddrTable3Loc,
    dev_addr_table4_loc: DevAddrTable4Loc,
    dev_addr_table5_loc: DevAddrTable5Loc,
    dev_addr_table6_loc: DevAddrTable6Loc,
    dev_addr_table7_loc: DevAddrTable7Loc,
    dev_addr_table8_loc: DevAddrTable8Loc,
    dev_addr_table9_loc: DevAddrTable9Loc,
    dev_addr_table10_loc: DevAddrTable10Loc,
    dev_addr_table11_loc: DevAddrTable11Loc,
    dev_addr_table12_loc: DevAddrTable12Loc,
    _reserved18: [u8; 0x10],
    dev_char_table1_loc1: DevCharTable1Loc1,
    dev_char_table1_loc2: DevCharTable1Loc2,
    dev_char_table1_loc3: DevCharTable1Loc3,
    dev_char_table1_loc4: DevCharTable1Loc4,
    dev_char_table2_loc1: DevCharTable2Loc1,
    dev_char_table2_loc2: DevCharTable2Loc2,
    dev_char_table2_loc3: DevCharTable2Loc3,
    dev_char_table2_loc4: DevCharTable2Loc4,
    dev_char_table3_loc1: DevCharTable3Loc1,
    dev_char_table3_loc2: DevCharTable3Loc2,
    dev_char_table3_loc3: DevCharTable3Loc3,
    dev_char_table3_loc4: DevCharTable3Loc4,
    dev_char_table4_loc1: DevCharTable4Loc1,
    dev_char_table4_loc2: DevCharTable4Loc2,
    dev_char_table4_loc3: DevCharTable4Loc3,
    dev_char_table4_loc4: DevCharTable4Loc4,
    dev_char_table5_loc1: DevCharTable5Loc1,
    dev_char_table5_loc2: DevCharTable5Loc2,
    dev_char_table5_loc3: DevCharTable5Loc3,
    dev_char_table5_loc4: DevCharTable5Loc4,
    dev_char_table6_loc1: DevCharTable6Loc1,
    dev_char_table6_loc2: DevCharTable6Loc2,
    dev_char_table6_loc3: DevCharTable6Loc3,
    dev_char_table6_loc4: DevCharTable6Loc4,
    dev_char_table7_loc1: DevCharTable7Loc1,
    dev_char_table7_loc2: DevCharTable7Loc2,
    dev_char_table7_loc3: DevCharTable7Loc3,
    dev_char_table7_loc4: DevCharTable7Loc4,
    dev_char_table8_loc1: DevCharTable8Loc1,
    dev_char_table8_loc2: DevCharTable8Loc2,
    dev_char_table8_loc3: DevCharTable8Loc3,
    dev_char_table8_loc4: DevCharTable8Loc4,
    dev_char_table9_loc1: DevCharTable9Loc1,
    dev_char_table9_loc2: DevCharTable9Loc2,
    dev_char_table9_loc3: DevCharTable9Loc3,
    dev_char_table9_loc4: DevCharTable9Loc4,
    dev_char_table10_loc1: DevCharTable10Loc1,
    dev_char_table10_loc2: DevCharTable10Loc2,
    dev_char_table10_loc3: DevCharTable10Loc3,
    dev_char_table10_loc4: DevCharTable10Loc4,
    dev_char_table11_loc1: DevCharTable11Loc1,
    dev_char_table11_loc2: DevCharTable11Loc2,
    dev_char_table11_loc3: DevCharTable11Loc3,
    dev_char_table11_loc4: DevCharTable11Loc4,
    dev_char_table12_loc1: DevCharTable12Loc1,
    dev_char_table12_loc2: DevCharTable12Loc2,
    dev_char_table12_loc3: DevCharTable12Loc3,
    dev_char_table12_loc4: DevCharTable12Loc4,
}
impl RegisterBlock {
    #[doc = "0x08 - NA"]
    #[inline(always)]
    pub const fn command_buf_port(&self) -> &CommandBufPort {
        &self.command_buf_port
    }
    #[doc = "0x0c - NA"]
    #[inline(always)]
    pub const fn response_buf_port(&self) -> &ResponseBufPort {
        &self.response_buf_port
    }
    #[doc = "0x10 - NA"]
    #[inline(always)]
    pub const fn rx_data_port(&self) -> &RxDataPort {
        &self.rx_data_port
    }
    #[doc = "0x14 - NA"]
    #[inline(always)]
    pub const fn tx_data_port(&self) -> &TxDataPort {
        &self.tx_data_port
    }
    #[doc = "0x18 - In-Band Interrupt Buffer Status/Data Register. When receiving an IBI, IBI_PORT is used to both: Read the IBI Status Read the IBI Data(which is raw/opaque data)"]
    #[inline(always)]
    pub const fn ibi_status_buf(&self) -> &IbiStatusBuf {
        &self.ibi_status_buf
    }
    #[doc = "0x40 - NA"]
    #[inline(always)]
    pub const fn ibi_data_buf(&self) -> &IbiDataBuf {
        &self.ibi_data_buf
    }
    #[doc = "0xc0 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table1_loc(&self) -> &DevAddrTable1Loc {
        &self.dev_addr_table1_loc
    }
    #[doc = "0xc4 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table2_loc(&self) -> &DevAddrTable2Loc {
        &self.dev_addr_table2_loc
    }
    #[doc = "0xc8 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table3_loc(&self) -> &DevAddrTable3Loc {
        &self.dev_addr_table3_loc
    }
    #[doc = "0xcc - NA"]
    #[inline(always)]
    pub const fn dev_addr_table4_loc(&self) -> &DevAddrTable4Loc {
        &self.dev_addr_table4_loc
    }
    #[doc = "0xd0 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table5_loc(&self) -> &DevAddrTable5Loc {
        &self.dev_addr_table5_loc
    }
    #[doc = "0xd4 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table6_loc(&self) -> &DevAddrTable6Loc {
        &self.dev_addr_table6_loc
    }
    #[doc = "0xd8 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table7_loc(&self) -> &DevAddrTable7Loc {
        &self.dev_addr_table7_loc
    }
    #[doc = "0xdc - NA"]
    #[inline(always)]
    pub const fn dev_addr_table8_loc(&self) -> &DevAddrTable8Loc {
        &self.dev_addr_table8_loc
    }
    #[doc = "0xe0 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table9_loc(&self) -> &DevAddrTable9Loc {
        &self.dev_addr_table9_loc
    }
    #[doc = "0xe4 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table10_loc(&self) -> &DevAddrTable10Loc {
        &self.dev_addr_table10_loc
    }
    #[doc = "0xe8 - NA"]
    #[inline(always)]
    pub const fn dev_addr_table11_loc(&self) -> &DevAddrTable11Loc {
        &self.dev_addr_table11_loc
    }
    #[doc = "0xec - NA"]
    #[inline(always)]
    pub const fn dev_addr_table12_loc(&self) -> &DevAddrTable12Loc {
        &self.dev_addr_table12_loc
    }
    #[doc = "0x100 - NA"]
    #[inline(always)]
    pub const fn dev_char_table1_loc1(&self) -> &DevCharTable1Loc1 {
        &self.dev_char_table1_loc1
    }
    #[doc = "0x104 - NA"]
    #[inline(always)]
    pub const fn dev_char_table1_loc2(&self) -> &DevCharTable1Loc2 {
        &self.dev_char_table1_loc2
    }
    #[doc = "0x108 - NA"]
    #[inline(always)]
    pub const fn dev_char_table1_loc3(&self) -> &DevCharTable1Loc3 {
        &self.dev_char_table1_loc3
    }
    #[doc = "0x10c - NA"]
    #[inline(always)]
    pub const fn dev_char_table1_loc4(&self) -> &DevCharTable1Loc4 {
        &self.dev_char_table1_loc4
    }
    #[doc = "0x110 - NA"]
    #[inline(always)]
    pub const fn dev_char_table2_loc1(&self) -> &DevCharTable2Loc1 {
        &self.dev_char_table2_loc1
    }
    #[doc = "0x114 - NA"]
    #[inline(always)]
    pub const fn dev_char_table2_loc2(&self) -> &DevCharTable2Loc2 {
        &self.dev_char_table2_loc2
    }
    #[doc = "0x118 - NA"]
    #[inline(always)]
    pub const fn dev_char_table2_loc3(&self) -> &DevCharTable2Loc3 {
        &self.dev_char_table2_loc3
    }
    #[doc = "0x11c - NA"]
    #[inline(always)]
    pub const fn dev_char_table2_loc4(&self) -> &DevCharTable2Loc4 {
        &self.dev_char_table2_loc4
    }
    #[doc = "0x120 - NA"]
    #[inline(always)]
    pub const fn dev_char_table3_loc1(&self) -> &DevCharTable3Loc1 {
        &self.dev_char_table3_loc1
    }
    #[doc = "0x124 - NA"]
    #[inline(always)]
    pub const fn dev_char_table3_loc2(&self) -> &DevCharTable3Loc2 {
        &self.dev_char_table3_loc2
    }
    #[doc = "0x128 - NA"]
    #[inline(always)]
    pub const fn dev_char_table3_loc3(&self) -> &DevCharTable3Loc3 {
        &self.dev_char_table3_loc3
    }
    #[doc = "0x12c - NA"]
    #[inline(always)]
    pub const fn dev_char_table3_loc4(&self) -> &DevCharTable3Loc4 {
        &self.dev_char_table3_loc4
    }
    #[doc = "0x130 - NA"]
    #[inline(always)]
    pub const fn dev_char_table4_loc1(&self) -> &DevCharTable4Loc1 {
        &self.dev_char_table4_loc1
    }
    #[doc = "0x134 - NA"]
    #[inline(always)]
    pub const fn dev_char_table4_loc2(&self) -> &DevCharTable4Loc2 {
        &self.dev_char_table4_loc2
    }
    #[doc = "0x138 - NA"]
    #[inline(always)]
    pub const fn dev_char_table4_loc3(&self) -> &DevCharTable4Loc3 {
        &self.dev_char_table4_loc3
    }
    #[doc = "0x13c - NA"]
    #[inline(always)]
    pub const fn dev_char_table4_loc4(&self) -> &DevCharTable4Loc4 {
        &self.dev_char_table4_loc4
    }
    #[doc = "0x140 - NA"]
    #[inline(always)]
    pub const fn dev_char_table5_loc1(&self) -> &DevCharTable5Loc1 {
        &self.dev_char_table5_loc1
    }
    #[doc = "0x144 - NA"]
    #[inline(always)]
    pub const fn dev_char_table5_loc2(&self) -> &DevCharTable5Loc2 {
        &self.dev_char_table5_loc2
    }
    #[doc = "0x148 - NA"]
    #[inline(always)]
    pub const fn dev_char_table5_loc3(&self) -> &DevCharTable5Loc3 {
        &self.dev_char_table5_loc3
    }
    #[doc = "0x14c - NA"]
    #[inline(always)]
    pub const fn dev_char_table5_loc4(&self) -> &DevCharTable5Loc4 {
        &self.dev_char_table5_loc4
    }
    #[doc = "0x150 - NA"]
    #[inline(always)]
    pub const fn dev_char_table6_loc1(&self) -> &DevCharTable6Loc1 {
        &self.dev_char_table6_loc1
    }
    #[doc = "0x154 - NA"]
    #[inline(always)]
    pub const fn dev_char_table6_loc2(&self) -> &DevCharTable6Loc2 {
        &self.dev_char_table6_loc2
    }
    #[doc = "0x158 - NA"]
    #[inline(always)]
    pub const fn dev_char_table6_loc3(&self) -> &DevCharTable6Loc3 {
        &self.dev_char_table6_loc3
    }
    #[doc = "0x15c - NA"]
    #[inline(always)]
    pub const fn dev_char_table6_loc4(&self) -> &DevCharTable6Loc4 {
        &self.dev_char_table6_loc4
    }
    #[doc = "0x160 - NA"]
    #[inline(always)]
    pub const fn dev_char_table7_loc1(&self) -> &DevCharTable7Loc1 {
        &self.dev_char_table7_loc1
    }
    #[doc = "0x164 - NA"]
    #[inline(always)]
    pub const fn dev_char_table7_loc2(&self) -> &DevCharTable7Loc2 {
        &self.dev_char_table7_loc2
    }
    #[doc = "0x168 - NA"]
    #[inline(always)]
    pub const fn dev_char_table7_loc3(&self) -> &DevCharTable7Loc3 {
        &self.dev_char_table7_loc3
    }
    #[doc = "0x16c - NA"]
    #[inline(always)]
    pub const fn dev_char_table7_loc4(&self) -> &DevCharTable7Loc4 {
        &self.dev_char_table7_loc4
    }
    #[doc = "0x170 - NA"]
    #[inline(always)]
    pub const fn dev_char_table8_loc1(&self) -> &DevCharTable8Loc1 {
        &self.dev_char_table8_loc1
    }
    #[doc = "0x174 - NA"]
    #[inline(always)]
    pub const fn dev_char_table8_loc2(&self) -> &DevCharTable8Loc2 {
        &self.dev_char_table8_loc2
    }
    #[doc = "0x178 - NA"]
    #[inline(always)]
    pub const fn dev_char_table8_loc3(&self) -> &DevCharTable8Loc3 {
        &self.dev_char_table8_loc3
    }
    #[doc = "0x17c - NA"]
    #[inline(always)]
    pub const fn dev_char_table8_loc4(&self) -> &DevCharTable8Loc4 {
        &self.dev_char_table8_loc4
    }
    #[doc = "0x180 - NA"]
    #[inline(always)]
    pub const fn dev_char_table9_loc1(&self) -> &DevCharTable9Loc1 {
        &self.dev_char_table9_loc1
    }
    #[doc = "0x184 - NA"]
    #[inline(always)]
    pub const fn dev_char_table9_loc2(&self) -> &DevCharTable9Loc2 {
        &self.dev_char_table9_loc2
    }
    #[doc = "0x188 - NA"]
    #[inline(always)]
    pub const fn dev_char_table9_loc3(&self) -> &DevCharTable9Loc3 {
        &self.dev_char_table9_loc3
    }
    #[doc = "0x18c - NA"]
    #[inline(always)]
    pub const fn dev_char_table9_loc4(&self) -> &DevCharTable9Loc4 {
        &self.dev_char_table9_loc4
    }
    #[doc = "0x190 - NA"]
    #[inline(always)]
    pub const fn dev_char_table10_loc1(&self) -> &DevCharTable10Loc1 {
        &self.dev_char_table10_loc1
    }
    #[doc = "0x194 - NA"]
    #[inline(always)]
    pub const fn dev_char_table10_loc2(&self) -> &DevCharTable10Loc2 {
        &self.dev_char_table10_loc2
    }
    #[doc = "0x198 - NA"]
    #[inline(always)]
    pub const fn dev_char_table10_loc3(&self) -> &DevCharTable10Loc3 {
        &self.dev_char_table10_loc3
    }
    #[doc = "0x19c - NA"]
    #[inline(always)]
    pub const fn dev_char_table10_loc4(&self) -> &DevCharTable10Loc4 {
        &self.dev_char_table10_loc4
    }
    #[doc = "0x1a0 - NA"]
    #[inline(always)]
    pub const fn dev_char_table11_loc1(&self) -> &DevCharTable11Loc1 {
        &self.dev_char_table11_loc1
    }
    #[doc = "0x1a4 - NA"]
    #[inline(always)]
    pub const fn dev_char_table11_loc2(&self) -> &DevCharTable11Loc2 {
        &self.dev_char_table11_loc2
    }
    #[doc = "0x1a8 - NA"]
    #[inline(always)]
    pub const fn dev_char_table11_loc3(&self) -> &DevCharTable11Loc3 {
        &self.dev_char_table11_loc3
    }
    #[doc = "0x1ac - NA"]
    #[inline(always)]
    pub const fn dev_char_table11_loc4(&self) -> &DevCharTable11Loc4 {
        &self.dev_char_table11_loc4
    }
    #[doc = "0x1b0 - NA"]
    #[inline(always)]
    pub const fn dev_char_table12_loc1(&self) -> &DevCharTable12Loc1 {
        &self.dev_char_table12_loc1
    }
    #[doc = "0x1b4 - NA"]
    #[inline(always)]
    pub const fn dev_char_table12_loc2(&self) -> &DevCharTable12Loc2 {
        &self.dev_char_table12_loc2
    }
    #[doc = "0x1b8 - NA"]
    #[inline(always)]
    pub const fn dev_char_table12_loc3(&self) -> &DevCharTable12Loc3 {
        &self.dev_char_table12_loc3
    }
    #[doc = "0x1bc - NA"]
    #[inline(always)]
    pub const fn dev_char_table12_loc4(&self) -> &DevCharTable12Loc4 {
        &self.dev_char_table12_loc4
    }
}
#[doc = "COMMAND_BUF_PORT (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`command_buf_port::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command_buf_port::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command_buf_port`] module"]
#[doc(alias = "COMMAND_BUF_PORT")]
pub type CommandBufPort = crate::Reg<command_buf_port::CommandBufPortSpec>;
#[doc = "NA"]
pub mod command_buf_port;
#[doc = "RESPONSE_BUF_PORT (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`response_buf_port::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@response_buf_port`] module"]
#[doc(alias = "RESPONSE_BUF_PORT")]
pub type ResponseBufPort = crate::Reg<response_buf_port::ResponseBufPortSpec>;
#[doc = "NA"]
pub mod response_buf_port;
#[doc = "RX_DATA_PORT (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_data_port::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data_port`] module"]
#[doc(alias = "RX_DATA_PORT")]
pub type RxDataPort = crate::Reg<rx_data_port::RxDataPortSpec>;
#[doc = "NA"]
pub mod rx_data_port;
#[doc = "TX_DATA_PORT (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_data_port::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_data_port::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_data_port`] module"]
#[doc(alias = "TX_DATA_PORT")]
pub type TxDataPort = crate::Reg<tx_data_port::TxDataPortSpec>;
#[doc = "NA"]
pub mod tx_data_port;
#[doc = "IBI_STATUS_BUF (r) register accessor: In-Band Interrupt Buffer Status/Data Register. When receiving an IBI, IBI_PORT is used to both: Read the IBI Status Read the IBI Data(which is raw/opaque data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ibi_status_buf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibi_status_buf`] module"]
#[doc(alias = "IBI_STATUS_BUF")]
pub type IbiStatusBuf = crate::Reg<ibi_status_buf::IbiStatusBufSpec>;
#[doc = "In-Band Interrupt Buffer Status/Data Register. When receiving an IBI, IBI_PORT is used to both: Read the IBI Status Read the IBI Data(which is raw/opaque data)"]
pub mod ibi_status_buf;
#[doc = "IBI_DATA_BUF (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`ibi_data_buf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibi_data_buf`] module"]
#[doc(alias = "IBI_DATA_BUF")]
pub type IbiDataBuf = crate::Reg<ibi_data_buf::IbiDataBufSpec>;
#[doc = "NA"]
pub mod ibi_data_buf;
#[doc = "DEV_ADDR_TABLE1_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_addr_table1_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_addr_table1_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table1_loc`] module"]
#[doc(alias = "DEV_ADDR_TABLE1_LOC")]
pub type DevAddrTable1Loc = crate::Reg<dev_addr_table1_loc::DevAddrTable1LocSpec>;
#[doc = "NA"]
pub mod dev_addr_table1_loc;
#[doc = "DEV_ADDR_TABLE2_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_addr_table2_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_addr_table2_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table2_loc`] module"]
#[doc(alias = "DEV_ADDR_TABLE2_LOC")]
pub type DevAddrTable2Loc = crate::Reg<dev_addr_table2_loc::DevAddrTable2LocSpec>;
#[doc = "NA"]
pub mod dev_addr_table2_loc;
#[doc = "DEV_ADDR_TABLE3_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_addr_table3_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_addr_table3_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table3_loc`] module"]
#[doc(alias = "DEV_ADDR_TABLE3_LOC")]
pub type DevAddrTable3Loc = crate::Reg<dev_addr_table3_loc::DevAddrTable3LocSpec>;
#[doc = "NA"]
pub mod dev_addr_table3_loc;
#[doc = "DEV_ADDR_TABLE4_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_addr_table4_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_addr_table4_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table4_loc`] module"]
#[doc(alias = "DEV_ADDR_TABLE4_LOC")]
pub type DevAddrTable4Loc = crate::Reg<dev_addr_table4_loc::DevAddrTable4LocSpec>;
#[doc = "NA"]
pub mod dev_addr_table4_loc;
#[doc = "DEV_ADDR_TABLE5_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_addr_table5_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_addr_table5_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table5_loc`] module"]
#[doc(alias = "DEV_ADDR_TABLE5_LOC")]
pub type DevAddrTable5Loc = crate::Reg<dev_addr_table5_loc::DevAddrTable5LocSpec>;
#[doc = "NA"]
pub mod dev_addr_table5_loc;
#[doc = "DEV_ADDR_TABLE6_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_addr_table6_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_addr_table6_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table6_loc`] module"]
#[doc(alias = "DEV_ADDR_TABLE6_LOC")]
pub type DevAddrTable6Loc = crate::Reg<dev_addr_table6_loc::DevAddrTable6LocSpec>;
#[doc = "NA"]
pub mod dev_addr_table6_loc;
#[doc = "DEV_ADDR_TABLE7_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_addr_table7_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_addr_table7_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table7_loc`] module"]
#[doc(alias = "DEV_ADDR_TABLE7_LOC")]
pub type DevAddrTable7Loc = crate::Reg<dev_addr_table7_loc::DevAddrTable7LocSpec>;
#[doc = "NA"]
pub mod dev_addr_table7_loc;
#[doc = "DEV_ADDR_TABLE8_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_addr_table8_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_addr_table8_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table8_loc`] module"]
#[doc(alias = "DEV_ADDR_TABLE8_LOC")]
pub type DevAddrTable8Loc = crate::Reg<dev_addr_table8_loc::DevAddrTable8LocSpec>;
#[doc = "NA"]
pub mod dev_addr_table8_loc;
#[doc = "DEV_ADDR_TABLE9_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_addr_table9_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_addr_table9_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table9_loc`] module"]
#[doc(alias = "DEV_ADDR_TABLE9_LOC")]
pub type DevAddrTable9Loc = crate::Reg<dev_addr_table9_loc::DevAddrTable9LocSpec>;
#[doc = "NA"]
pub mod dev_addr_table9_loc;
#[doc = "DEV_ADDR_TABLE10_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_addr_table10_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_addr_table10_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table10_loc`] module"]
#[doc(alias = "DEV_ADDR_TABLE10_LOC")]
pub type DevAddrTable10Loc = crate::Reg<dev_addr_table10_loc::DevAddrTable10LocSpec>;
#[doc = "NA"]
pub mod dev_addr_table10_loc;
#[doc = "DEV_ADDR_TABLE11_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_addr_table11_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_addr_table11_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table11_loc`] module"]
#[doc(alias = "DEV_ADDR_TABLE11_LOC")]
pub type DevAddrTable11Loc = crate::Reg<dev_addr_table11_loc::DevAddrTable11LocSpec>;
#[doc = "NA"]
pub mod dev_addr_table11_loc;
#[doc = "DEV_ADDR_TABLE12_LOC (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_addr_table12_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_addr_table12_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_addr_table12_loc`] module"]
#[doc(alias = "DEV_ADDR_TABLE12_LOC")]
pub type DevAddrTable12Loc = crate::Reg<dev_addr_table12_loc::DevAddrTable12LocSpec>;
#[doc = "NA"]
pub mod dev_addr_table12_loc;
#[doc = "DEV_CHAR_TABLE1_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table1_loc1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table1_loc1`] module"]
#[doc(alias = "DEV_CHAR_TABLE1_LOC1")]
pub type DevCharTable1Loc1 = crate::Reg<dev_char_table1_loc1::DevCharTable1Loc1Spec>;
#[doc = "NA"]
pub mod dev_char_table1_loc1;
#[doc = "DEV_CHAR_TABLE1_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table1_loc2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table1_loc2`] module"]
#[doc(alias = "DEV_CHAR_TABLE1_LOC2")]
pub type DevCharTable1Loc2 = crate::Reg<dev_char_table1_loc2::DevCharTable1Loc2Spec>;
#[doc = "NA"]
pub mod dev_char_table1_loc2;
#[doc = "DEV_CHAR_TABLE1_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table1_loc3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table1_loc3`] module"]
#[doc(alias = "DEV_CHAR_TABLE1_LOC3")]
pub type DevCharTable1Loc3 = crate::Reg<dev_char_table1_loc3::DevCharTable1Loc3Spec>;
#[doc = "NA"]
pub mod dev_char_table1_loc3;
#[doc = "DEV_CHAR_TABLE1_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table1_loc4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table1_loc4`] module"]
#[doc(alias = "DEV_CHAR_TABLE1_LOC4")]
pub type DevCharTable1Loc4 = crate::Reg<dev_char_table1_loc4::DevCharTable1Loc4Spec>;
#[doc = "NA"]
pub mod dev_char_table1_loc4;
#[doc = "DEV_CHAR_TABLE2_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table2_loc1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table2_loc1`] module"]
#[doc(alias = "DEV_CHAR_TABLE2_LOC1")]
pub type DevCharTable2Loc1 = crate::Reg<dev_char_table2_loc1::DevCharTable2Loc1Spec>;
#[doc = "NA"]
pub mod dev_char_table2_loc1;
#[doc = "DEV_CHAR_TABLE2_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table2_loc2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table2_loc2`] module"]
#[doc(alias = "DEV_CHAR_TABLE2_LOC2")]
pub type DevCharTable2Loc2 = crate::Reg<dev_char_table2_loc2::DevCharTable2Loc2Spec>;
#[doc = "NA"]
pub mod dev_char_table2_loc2;
#[doc = "DEV_CHAR_TABLE2_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table2_loc3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table2_loc3`] module"]
#[doc(alias = "DEV_CHAR_TABLE2_LOC3")]
pub type DevCharTable2Loc3 = crate::Reg<dev_char_table2_loc3::DevCharTable2Loc3Spec>;
#[doc = "NA"]
pub mod dev_char_table2_loc3;
#[doc = "DEV_CHAR_TABLE2_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table2_loc4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table2_loc4`] module"]
#[doc(alias = "DEV_CHAR_TABLE2_LOC4")]
pub type DevCharTable2Loc4 = crate::Reg<dev_char_table2_loc4::DevCharTable2Loc4Spec>;
#[doc = "NA"]
pub mod dev_char_table2_loc4;
#[doc = "DEV_CHAR_TABLE3_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table3_loc1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table3_loc1`] module"]
#[doc(alias = "DEV_CHAR_TABLE3_LOC1")]
pub type DevCharTable3Loc1 = crate::Reg<dev_char_table3_loc1::DevCharTable3Loc1Spec>;
#[doc = "NA"]
pub mod dev_char_table3_loc1;
#[doc = "DEV_CHAR_TABLE3_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table3_loc2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table3_loc2`] module"]
#[doc(alias = "DEV_CHAR_TABLE3_LOC2")]
pub type DevCharTable3Loc2 = crate::Reg<dev_char_table3_loc2::DevCharTable3Loc2Spec>;
#[doc = "NA"]
pub mod dev_char_table3_loc2;
#[doc = "DEV_CHAR_TABLE3_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table3_loc3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table3_loc3`] module"]
#[doc(alias = "DEV_CHAR_TABLE3_LOC3")]
pub type DevCharTable3Loc3 = crate::Reg<dev_char_table3_loc3::DevCharTable3Loc3Spec>;
#[doc = "NA"]
pub mod dev_char_table3_loc3;
#[doc = "DEV_CHAR_TABLE3_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table3_loc4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table3_loc4`] module"]
#[doc(alias = "DEV_CHAR_TABLE3_LOC4")]
pub type DevCharTable3Loc4 = crate::Reg<dev_char_table3_loc4::DevCharTable3Loc4Spec>;
#[doc = "NA"]
pub mod dev_char_table3_loc4;
#[doc = "DEV_CHAR_TABLE4_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table4_loc1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table4_loc1`] module"]
#[doc(alias = "DEV_CHAR_TABLE4_LOC1")]
pub type DevCharTable4Loc1 = crate::Reg<dev_char_table4_loc1::DevCharTable4Loc1Spec>;
#[doc = "NA"]
pub mod dev_char_table4_loc1;
#[doc = "DEV_CHAR_TABLE4_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table4_loc2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table4_loc2`] module"]
#[doc(alias = "DEV_CHAR_TABLE4_LOC2")]
pub type DevCharTable4Loc2 = crate::Reg<dev_char_table4_loc2::DevCharTable4Loc2Spec>;
#[doc = "NA"]
pub mod dev_char_table4_loc2;
#[doc = "DEV_CHAR_TABLE4_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table4_loc3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table4_loc3`] module"]
#[doc(alias = "DEV_CHAR_TABLE4_LOC3")]
pub type DevCharTable4Loc3 = crate::Reg<dev_char_table4_loc3::DevCharTable4Loc3Spec>;
#[doc = "NA"]
pub mod dev_char_table4_loc3;
#[doc = "DEV_CHAR_TABLE4_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table4_loc4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table4_loc4`] module"]
#[doc(alias = "DEV_CHAR_TABLE4_LOC4")]
pub type DevCharTable4Loc4 = crate::Reg<dev_char_table4_loc4::DevCharTable4Loc4Spec>;
#[doc = "NA"]
pub mod dev_char_table4_loc4;
#[doc = "DEV_CHAR_TABLE5_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table5_loc1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table5_loc1`] module"]
#[doc(alias = "DEV_CHAR_TABLE5_LOC1")]
pub type DevCharTable5Loc1 = crate::Reg<dev_char_table5_loc1::DevCharTable5Loc1Spec>;
#[doc = "NA"]
pub mod dev_char_table5_loc1;
#[doc = "DEV_CHAR_TABLE5_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table5_loc2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table5_loc2`] module"]
#[doc(alias = "DEV_CHAR_TABLE5_LOC2")]
pub type DevCharTable5Loc2 = crate::Reg<dev_char_table5_loc2::DevCharTable5Loc2Spec>;
#[doc = "NA"]
pub mod dev_char_table5_loc2;
#[doc = "DEV_CHAR_TABLE5_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table5_loc3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table5_loc3`] module"]
#[doc(alias = "DEV_CHAR_TABLE5_LOC3")]
pub type DevCharTable5Loc3 = crate::Reg<dev_char_table5_loc3::DevCharTable5Loc3Spec>;
#[doc = "NA"]
pub mod dev_char_table5_loc3;
#[doc = "DEV_CHAR_TABLE5_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table5_loc4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table5_loc4`] module"]
#[doc(alias = "DEV_CHAR_TABLE5_LOC4")]
pub type DevCharTable5Loc4 = crate::Reg<dev_char_table5_loc4::DevCharTable5Loc4Spec>;
#[doc = "NA"]
pub mod dev_char_table5_loc4;
#[doc = "DEV_CHAR_TABLE6_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table6_loc1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table6_loc1`] module"]
#[doc(alias = "DEV_CHAR_TABLE6_LOC1")]
pub type DevCharTable6Loc1 = crate::Reg<dev_char_table6_loc1::DevCharTable6Loc1Spec>;
#[doc = "NA"]
pub mod dev_char_table6_loc1;
#[doc = "DEV_CHAR_TABLE6_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table6_loc2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table6_loc2`] module"]
#[doc(alias = "DEV_CHAR_TABLE6_LOC2")]
pub type DevCharTable6Loc2 = crate::Reg<dev_char_table6_loc2::DevCharTable6Loc2Spec>;
#[doc = "NA"]
pub mod dev_char_table6_loc2;
#[doc = "DEV_CHAR_TABLE6_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table6_loc3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table6_loc3`] module"]
#[doc(alias = "DEV_CHAR_TABLE6_LOC3")]
pub type DevCharTable6Loc3 = crate::Reg<dev_char_table6_loc3::DevCharTable6Loc3Spec>;
#[doc = "NA"]
pub mod dev_char_table6_loc3;
#[doc = "DEV_CHAR_TABLE6_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table6_loc4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table6_loc4`] module"]
#[doc(alias = "DEV_CHAR_TABLE6_LOC4")]
pub type DevCharTable6Loc4 = crate::Reg<dev_char_table6_loc4::DevCharTable6Loc4Spec>;
#[doc = "NA"]
pub mod dev_char_table6_loc4;
#[doc = "DEV_CHAR_TABLE7_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table7_loc1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table7_loc1`] module"]
#[doc(alias = "DEV_CHAR_TABLE7_LOC1")]
pub type DevCharTable7Loc1 = crate::Reg<dev_char_table7_loc1::DevCharTable7Loc1Spec>;
#[doc = "NA"]
pub mod dev_char_table7_loc1;
#[doc = "DEV_CHAR_TABLE7_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table7_loc2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table7_loc2`] module"]
#[doc(alias = "DEV_CHAR_TABLE7_LOC2")]
pub type DevCharTable7Loc2 = crate::Reg<dev_char_table7_loc2::DevCharTable7Loc2Spec>;
#[doc = "NA"]
pub mod dev_char_table7_loc2;
#[doc = "DEV_CHAR_TABLE7_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table7_loc3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table7_loc3`] module"]
#[doc(alias = "DEV_CHAR_TABLE7_LOC3")]
pub type DevCharTable7Loc3 = crate::Reg<dev_char_table7_loc3::DevCharTable7Loc3Spec>;
#[doc = "NA"]
pub mod dev_char_table7_loc3;
#[doc = "DEV_CHAR_TABLE7_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table7_loc4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table7_loc4`] module"]
#[doc(alias = "DEV_CHAR_TABLE7_LOC4")]
pub type DevCharTable7Loc4 = crate::Reg<dev_char_table7_loc4::DevCharTable7Loc4Spec>;
#[doc = "NA"]
pub mod dev_char_table7_loc4;
#[doc = "DEV_CHAR_TABLE8_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table8_loc1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table8_loc1`] module"]
#[doc(alias = "DEV_CHAR_TABLE8_LOC1")]
pub type DevCharTable8Loc1 = crate::Reg<dev_char_table8_loc1::DevCharTable8Loc1Spec>;
#[doc = "NA"]
pub mod dev_char_table8_loc1;
#[doc = "DEV_CHAR_TABLE8_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table8_loc2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table8_loc2`] module"]
#[doc(alias = "DEV_CHAR_TABLE8_LOC2")]
pub type DevCharTable8Loc2 = crate::Reg<dev_char_table8_loc2::DevCharTable8Loc2Spec>;
#[doc = "NA"]
pub mod dev_char_table8_loc2;
#[doc = "DEV_CHAR_TABLE8_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table8_loc3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table8_loc3`] module"]
#[doc(alias = "DEV_CHAR_TABLE8_LOC3")]
pub type DevCharTable8Loc3 = crate::Reg<dev_char_table8_loc3::DevCharTable8Loc3Spec>;
#[doc = "NA"]
pub mod dev_char_table8_loc3;
#[doc = "DEV_CHAR_TABLE8_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table8_loc4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table8_loc4`] module"]
#[doc(alias = "DEV_CHAR_TABLE8_LOC4")]
pub type DevCharTable8Loc4 = crate::Reg<dev_char_table8_loc4::DevCharTable8Loc4Spec>;
#[doc = "NA"]
pub mod dev_char_table8_loc4;
#[doc = "DEV_CHAR_TABLE9_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table9_loc1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table9_loc1`] module"]
#[doc(alias = "DEV_CHAR_TABLE9_LOC1")]
pub type DevCharTable9Loc1 = crate::Reg<dev_char_table9_loc1::DevCharTable9Loc1Spec>;
#[doc = "NA"]
pub mod dev_char_table9_loc1;
#[doc = "DEV_CHAR_TABLE9_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table9_loc2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table9_loc2`] module"]
#[doc(alias = "DEV_CHAR_TABLE9_LOC2")]
pub type DevCharTable9Loc2 = crate::Reg<dev_char_table9_loc2::DevCharTable9Loc2Spec>;
#[doc = "NA"]
pub mod dev_char_table9_loc2;
#[doc = "DEV_CHAR_TABLE9_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table9_loc3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table9_loc3`] module"]
#[doc(alias = "DEV_CHAR_TABLE9_LOC3")]
pub type DevCharTable9Loc3 = crate::Reg<dev_char_table9_loc3::DevCharTable9Loc3Spec>;
#[doc = "NA"]
pub mod dev_char_table9_loc3;
#[doc = "DEV_CHAR_TABLE9_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table9_loc4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table9_loc4`] module"]
#[doc(alias = "DEV_CHAR_TABLE9_LOC4")]
pub type DevCharTable9Loc4 = crate::Reg<dev_char_table9_loc4::DevCharTable9Loc4Spec>;
#[doc = "NA"]
pub mod dev_char_table9_loc4;
#[doc = "DEV_CHAR_TABLE10_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table10_loc1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table10_loc1`] module"]
#[doc(alias = "DEV_CHAR_TABLE10_LOC1")]
pub type DevCharTable10Loc1 = crate::Reg<dev_char_table10_loc1::DevCharTable10Loc1Spec>;
#[doc = "NA"]
pub mod dev_char_table10_loc1;
#[doc = "DEV_CHAR_TABLE10_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table10_loc2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table10_loc2`] module"]
#[doc(alias = "DEV_CHAR_TABLE10_LOC2")]
pub type DevCharTable10Loc2 = crate::Reg<dev_char_table10_loc2::DevCharTable10Loc2Spec>;
#[doc = "NA"]
pub mod dev_char_table10_loc2;
#[doc = "DEV_CHAR_TABLE10_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table10_loc3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table10_loc3`] module"]
#[doc(alias = "DEV_CHAR_TABLE10_LOC3")]
pub type DevCharTable10Loc3 = crate::Reg<dev_char_table10_loc3::DevCharTable10Loc3Spec>;
#[doc = "NA"]
pub mod dev_char_table10_loc3;
#[doc = "DEV_CHAR_TABLE10_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table10_loc4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table10_loc4`] module"]
#[doc(alias = "DEV_CHAR_TABLE10_LOC4")]
pub type DevCharTable10Loc4 = crate::Reg<dev_char_table10_loc4::DevCharTable10Loc4Spec>;
#[doc = "NA"]
pub mod dev_char_table10_loc4;
#[doc = "DEV_CHAR_TABLE11_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table11_loc1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table11_loc1`] module"]
#[doc(alias = "DEV_CHAR_TABLE11_LOC1")]
pub type DevCharTable11Loc1 = crate::Reg<dev_char_table11_loc1::DevCharTable11Loc1Spec>;
#[doc = "NA"]
pub mod dev_char_table11_loc1;
#[doc = "DEV_CHAR_TABLE11_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table11_loc2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table11_loc2`] module"]
#[doc(alias = "DEV_CHAR_TABLE11_LOC2")]
pub type DevCharTable11Loc2 = crate::Reg<dev_char_table11_loc2::DevCharTable11Loc2Spec>;
#[doc = "NA"]
pub mod dev_char_table11_loc2;
#[doc = "DEV_CHAR_TABLE11_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table11_loc3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table11_loc3`] module"]
#[doc(alias = "DEV_CHAR_TABLE11_LOC3")]
pub type DevCharTable11Loc3 = crate::Reg<dev_char_table11_loc3::DevCharTable11Loc3Spec>;
#[doc = "NA"]
pub mod dev_char_table11_loc3;
#[doc = "DEV_CHAR_TABLE11_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table11_loc4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table11_loc4`] module"]
#[doc(alias = "DEV_CHAR_TABLE11_LOC4")]
pub type DevCharTable11Loc4 = crate::Reg<dev_char_table11_loc4::DevCharTable11Loc4Spec>;
#[doc = "NA"]
pub mod dev_char_table11_loc4;
#[doc = "DEV_CHAR_TABLE12_LOC1 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table12_loc1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table12_loc1`] module"]
#[doc(alias = "DEV_CHAR_TABLE12_LOC1")]
pub type DevCharTable12Loc1 = crate::Reg<dev_char_table12_loc1::DevCharTable12Loc1Spec>;
#[doc = "NA"]
pub mod dev_char_table12_loc1;
#[doc = "DEV_CHAR_TABLE12_LOC2 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table12_loc2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table12_loc2`] module"]
#[doc(alias = "DEV_CHAR_TABLE12_LOC2")]
pub type DevCharTable12Loc2 = crate::Reg<dev_char_table12_loc2::DevCharTable12Loc2Spec>;
#[doc = "NA"]
pub mod dev_char_table12_loc2;
#[doc = "DEV_CHAR_TABLE12_LOC3 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table12_loc3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table12_loc3`] module"]
#[doc(alias = "DEV_CHAR_TABLE12_LOC3")]
pub type DevCharTable12Loc3 = crate::Reg<dev_char_table12_loc3::DevCharTable12Loc3Spec>;
#[doc = "NA"]
pub mod dev_char_table12_loc3;
#[doc = "DEV_CHAR_TABLE12_LOC4 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table12_loc4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_char_table12_loc4`] module"]
#[doc(alias = "DEV_CHAR_TABLE12_LOC4")]
pub type DevCharTable12Loc4 = crate::Reg<dev_char_table12_loc4::DevCharTable12Loc4Spec>;
#[doc = "NA"]
pub mod dev_char_table12_loc4;
