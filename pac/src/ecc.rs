#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    mult_int_raw: MultIntRaw,
    mult_int_st: MultIntSt,
    mult_int_ena: MultIntEna,
    mult_int_clr: MultIntClr,
    mult_conf: MultConf,
    _reserved5: [u8; 0xdc],
    mult_date: MultDate,
    k_mem: [KMem; 12],
    px_mem: [PxMem; 12],
    py_mem: [PyMem; 12],
    qx_mem: [QxMem; 12],
    qy_mem: [QyMem; 12],
    qz_mem: [QzMem; 12],
}
impl RegisterBlock {
    #[doc = "0x0c - ECC raw interrupt status register"]
    #[inline(always)]
    pub const fn mult_int_raw(&self) -> &MultIntRaw {
        &self.mult_int_raw
    }
    #[doc = "0x10 - ECC masked interrupt status register"]
    #[inline(always)]
    pub const fn mult_int_st(&self) -> &MultIntSt {
        &self.mult_int_st
    }
    #[doc = "0x14 - ECC interrupt enable register"]
    #[inline(always)]
    pub const fn mult_int_ena(&self) -> &MultIntEna {
        &self.mult_int_ena
    }
    #[doc = "0x18 - ECC interrupt clear register"]
    #[inline(always)]
    pub const fn mult_int_clr(&self) -> &MultIntClr {
        &self.mult_int_clr
    }
    #[doc = "0x1c - ECC configuration register"]
    #[inline(always)]
    pub const fn mult_conf(&self) -> &MultConf {
        &self.mult_conf
    }
    #[doc = "0xfc - Version control register"]
    #[inline(always)]
    pub const fn mult_date(&self) -> &MultDate {
        &self.mult_date
    }
    #[doc = "0x100..0x130 - The memory that stores k."]
    #[inline(always)]
    pub const fn k_mem(&self, n: usize) -> &KMem {
        &self.k_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x130 - The memory that stores k."]
    #[inline(always)]
    pub fn k_mem_iter(&self) -> impl Iterator<Item = &KMem> {
        self.k_mem.iter()
    }
    #[doc = "0x130..0x160 - The memory that stores Px."]
    #[inline(always)]
    pub const fn px_mem(&self, n: usize) -> &PxMem {
        &self.px_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x130..0x160 - The memory that stores Px."]
    #[inline(always)]
    pub fn px_mem_iter(&self) -> impl Iterator<Item = &PxMem> {
        self.px_mem.iter()
    }
    #[doc = "0x160..0x190 - The memory that stores Py."]
    #[inline(always)]
    pub const fn py_mem(&self, n: usize) -> &PyMem {
        &self.py_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x160..0x190 - The memory that stores Py."]
    #[inline(always)]
    pub fn py_mem_iter(&self) -> impl Iterator<Item = &PyMem> {
        self.py_mem.iter()
    }
    #[doc = "0x190..0x1c0 - The memory that stores Qx."]
    #[inline(always)]
    pub const fn qx_mem(&self, n: usize) -> &QxMem {
        &self.qx_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x190..0x1c0 - The memory that stores Qx."]
    #[inline(always)]
    pub fn qx_mem_iter(&self) -> impl Iterator<Item = &QxMem> {
        self.qx_mem.iter()
    }
    #[doc = "0x1c0..0x1f0 - The memory that stores Qy."]
    #[inline(always)]
    pub const fn qy_mem(&self, n: usize) -> &QyMem {
        &self.qy_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x1f0 - The memory that stores Qy."]
    #[inline(always)]
    pub fn qy_mem_iter(&self) -> impl Iterator<Item = &QyMem> {
        self.qy_mem.iter()
    }
    #[doc = "0x1f0..0x220 - The memory that stores Qz."]
    #[inline(always)]
    pub const fn qz_mem(&self, n: usize) -> &QzMem {
        &self.qz_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1f0..0x220 - The memory that stores Qz."]
    #[inline(always)]
    pub fn qz_mem_iter(&self) -> impl Iterator<Item = &QzMem> {
        self.qz_mem.iter()
    }
}
#[doc = "MULT_INT_RAW (r) register accessor: ECC raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_int_raw`] module"]
#[doc(alias = "MULT_INT_RAW")]
pub type MultIntRaw = crate::Reg<mult_int_raw::MultIntRawSpec>;
#[doc = "ECC raw interrupt status register"]
pub mod mult_int_raw;
#[doc = "MULT_INT_ST (r) register accessor: ECC masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_int_st`] module"]
#[doc(alias = "MULT_INT_ST")]
pub type MultIntSt = crate::Reg<mult_int_st::MultIntStSpec>;
#[doc = "ECC masked interrupt status register"]
pub mod mult_int_st;
#[doc = "MULT_INT_ENA (rw) register accessor: ECC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_int_ena`] module"]
#[doc(alias = "MULT_INT_ENA")]
pub type MultIntEna = crate::Reg<mult_int_ena::MultIntEnaSpec>;
#[doc = "ECC interrupt enable register"]
pub mod mult_int_ena;
#[doc = "MULT_INT_CLR (w) register accessor: ECC interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_int_clr`] module"]
#[doc(alias = "MULT_INT_CLR")]
pub type MultIntClr = crate::Reg<mult_int_clr::MultIntClrSpec>;
#[doc = "ECC interrupt clear register"]
pub mod mult_int_clr;
#[doc = "MULT_CONF (rw) register accessor: ECC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_conf`] module"]
#[doc(alias = "MULT_CONF")]
pub type MultConf = crate::Reg<mult_conf::MultConfSpec>;
#[doc = "ECC configuration register"]
pub mod mult_conf;
#[doc = "MULT_DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_date`] module"]
#[doc(alias = "MULT_DATE")]
pub type MultDate = crate::Reg<mult_date::MultDateSpec>;
#[doc = "Version control register"]
pub mod mult_date;
#[doc = "K_MEM (rw) register accessor: The memory that stores k.\n\nYou can [`read`](crate::Reg::read) this register and get [`k_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@k_mem`] module"]
#[doc(alias = "K_MEM")]
pub type KMem = crate::Reg<k_mem::KMemSpec>;
#[doc = "The memory that stores k."]
pub mod k_mem;
#[doc = "PX_MEM (rw) register accessor: The memory that stores Px.\n\nYou can [`read`](crate::Reg::read) this register and get [`px_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@px_mem`] module"]
#[doc(alias = "PX_MEM")]
pub type PxMem = crate::Reg<px_mem::PxMemSpec>;
#[doc = "The memory that stores Px."]
pub mod px_mem;
#[doc = "PY_MEM (rw) register accessor: The memory that stores Py.\n\nYou can [`read`](crate::Reg::read) this register and get [`py_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`py_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@py_mem`] module"]
#[doc(alias = "PY_MEM")]
pub type PyMem = crate::Reg<py_mem::PyMemSpec>;
#[doc = "The memory that stores Py."]
pub mod py_mem;
#[doc = "QX_MEM (rw) register accessor: The memory that stores Qx.\n\nYou can [`read`](crate::Reg::read) this register and get [`qx_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qx_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qx_mem`] module"]
#[doc(alias = "QX_MEM")]
pub type QxMem = crate::Reg<qx_mem::QxMemSpec>;
#[doc = "The memory that stores Qx."]
pub mod qx_mem;
#[doc = "QY_MEM (rw) register accessor: The memory that stores Qy.\n\nYou can [`read`](crate::Reg::read) this register and get [`qy_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qy_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qy_mem`] module"]
#[doc(alias = "QY_MEM")]
pub type QyMem = crate::Reg<qy_mem::QyMemSpec>;
#[doc = "The memory that stores Qy."]
pub mod qy_mem;
#[doc = "QZ_MEM (rw) register accessor: The memory that stores Qz.\n\nYou can [`read`](crate::Reg::read) this register and get [`qz_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qz_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qz_mem`] module"]
#[doc(alias = "QZ_MEM")]
pub type QzMem = crate::Reg<qz_mem::QzMemSpec>;
#[doc = "The memory that stores Qz."]
pub mod qz_mem;
