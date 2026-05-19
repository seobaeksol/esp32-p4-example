#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    emacconfig: Emacconfig,
    emacff: Emacff,
    _reserved2: [u8; 0x08],
    emacgmiiaddr: Emacgmiiaddr,
    emacmiidata: Emacmiidata,
    emacfc: Emacfc,
    _reserved5: [u8; 0x08],
    emacdebug: Emacdebug,
    pmt_rwuffr: PmtRwuffr,
    pmt_csr: PmtCsr,
    emaclpi_crs: EmaclpiCrs,
    emaclpitimerscontrol: Emaclpitimerscontrol,
    emacints: Emacints,
    emacintmask: Emacintmask,
    emacaddr0high: Emacaddr0high,
    emacaddr0low: Emacaddr0low,
    emacaddr1high: Emacaddr1high,
    emacaddr1low: Emacaddr1low,
    emacaddr2high: Emacaddr2high,
    emacaddr2low: Emacaddr2low,
    emacaddr3high: Emacaddr3high,
    emacaddr3low: Emacaddr3low,
    emacaddr4high: Emacaddr4high,
    emacaddr4low: Emacaddr4low,
    emacaddr5high: Emacaddr5high,
    emacaddr5low: Emacaddr5low,
    emacaddr6high: Emacaddr6high,
    emacaddr6low: Emacaddr6low,
    emacaddr7high: Emacaddr7high,
    emacaddr7low: Emacaddr7low,
    _reserved28: [u8; 0x58],
    emaccstatus: Emaccstatus,
    emacwdogto: Emacwdogto,
}
impl RegisterBlock {
    #[doc = "0x00 - MAC configuration"]
    #[inline(always)]
    pub const fn emacconfig(&self) -> &Emacconfig {
        &self.emacconfig
    }
    #[doc = "0x04 - Frame filter settings"]
    #[inline(always)]
    pub const fn emacff(&self) -> &Emacff {
        &self.emacff
    }
    #[doc = "0x10 - PHY configuration access"]
    #[inline(always)]
    pub const fn emacgmiiaddr(&self) -> &Emacgmiiaddr {
        &self.emacgmiiaddr
    }
    #[doc = "0x14 - PHY data read write"]
    #[inline(always)]
    pub const fn emacmiidata(&self) -> &Emacmiidata {
        &self.emacmiidata
    }
    #[doc = "0x18 - Frame flow control"]
    #[inline(always)]
    pub const fn emacfc(&self) -> &Emacfc {
        &self.emacfc
    }
    #[doc = "0x24 - Status debugging bits"]
    #[inline(always)]
    pub const fn emacdebug(&self) -> &Emacdebug {
        &self.emacdebug
    }
    #[doc = "0x28 - The MSB (31st bit) must be zero.Bit j\\[30:0\\] is the byte mask. If Bit 1/2/3/4 (byte number) of the byte mask is set the CRC block processes the Filter 1/2/3/4 Offset + j of the incoming packet(PWKPTR is 0/1/2/3).RWKPTR is 0:Filter 0 Byte Mask .RWKPTR is 1:Filter 1 Byte Mask RWKPTR is 2:Filter 2 Byte Mask RWKPTR is 3:Filter 3 Byte Mask RWKPTR is 4:Bit 3/11/19/27 specifies the address type defining the destination address type of the pattern.When the bit is set the pattern applies to only multicast packets"]
    #[inline(always)]
    pub const fn pmt_rwuffr(&self) -> &PmtRwuffr {
        &self.pmt_rwuffr
    }
    #[doc = "0x2c - PMT Control and Status"]
    #[inline(always)]
    pub const fn pmt_csr(&self) -> &PmtCsr {
        &self.pmt_csr
    }
    #[doc = "0x30 - LPI Control and Status"]
    #[inline(always)]
    pub const fn emaclpi_crs(&self) -> &EmaclpiCrs {
        &self.emaclpi_crs
    }
    #[doc = "0x34 - LPI Timers Control"]
    #[inline(always)]
    pub const fn emaclpitimerscontrol(&self) -> &Emaclpitimerscontrol {
        &self.emaclpitimerscontrol
    }
    #[doc = "0x38 - Interrupt status"]
    #[inline(always)]
    pub const fn emacints(&self) -> &Emacints {
        &self.emacints
    }
    #[doc = "0x3c - Interrupt mask"]
    #[inline(always)]
    pub const fn emacintmask(&self) -> &Emacintmask {
        &self.emacintmask
    }
    #[doc = "0x40 - Upper 16 bits of the first 6-byte MAC address"]
    #[inline(always)]
    pub const fn emacaddr0high(&self) -> &Emacaddr0high {
        &self.emacaddr0high
    }
    #[doc = "0x44 - This field contains the lower 32 bits of the first 6-byte MAC address. This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
    #[inline(always)]
    pub const fn emacaddr0low(&self) -> &Emacaddr0low {
        &self.emacaddr0low
    }
    #[doc = "0x48 - Upper 16 bits of the second 6-byte MAC address"]
    #[inline(always)]
    pub const fn emacaddr1high(&self) -> &Emacaddr1high {
        &self.emacaddr1high
    }
    #[doc = "0x4c - This field contains the lower 32 bits of the second 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
    #[inline(always)]
    pub const fn emacaddr1low(&self) -> &Emacaddr1low {
        &self.emacaddr1low
    }
    #[doc = "0x50 - Upper 16 bits of the third 6-byte MAC address"]
    #[inline(always)]
    pub const fn emacaddr2high(&self) -> &Emacaddr2high {
        &self.emacaddr2high
    }
    #[doc = "0x54 - This field contains the lower 32 bits of the third 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
    #[inline(always)]
    pub const fn emacaddr2low(&self) -> &Emacaddr2low {
        &self.emacaddr2low
    }
    #[doc = "0x58 - Upper 16 bits of the fourth 6-byte MAC address"]
    #[inline(always)]
    pub const fn emacaddr3high(&self) -> &Emacaddr3high {
        &self.emacaddr3high
    }
    #[doc = "0x5c - This field contains the lower 32 bits of the fourth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
    #[inline(always)]
    pub const fn emacaddr3low(&self) -> &Emacaddr3low {
        &self.emacaddr3low
    }
    #[doc = "0x60 - Upper 16 bits of the fifth 6-byte MAC address"]
    #[inline(always)]
    pub const fn emacaddr4high(&self) -> &Emacaddr4high {
        &self.emacaddr4high
    }
    #[doc = "0x64 - This field contains the lower 32 bits of the fifth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
    #[inline(always)]
    pub const fn emacaddr4low(&self) -> &Emacaddr4low {
        &self.emacaddr4low
    }
    #[doc = "0x68 - Upper 16 bits of the sixth 6-byte MAC address"]
    #[inline(always)]
    pub const fn emacaddr5high(&self) -> &Emacaddr5high {
        &self.emacaddr5high
    }
    #[doc = "0x6c - This field contains the lower 32 bits of the sixth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
    #[inline(always)]
    pub const fn emacaddr5low(&self) -> &Emacaddr5low {
        &self.emacaddr5low
    }
    #[doc = "0x70 - Upper 16 bits of the seventh 6-byte MAC address"]
    #[inline(always)]
    pub const fn emacaddr6high(&self) -> &Emacaddr6high {
        &self.emacaddr6high
    }
    #[doc = "0x74 - This field contains the lower 32 bits of the seventh 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
    #[inline(always)]
    pub const fn emacaddr6low(&self) -> &Emacaddr6low {
        &self.emacaddr6low
    }
    #[doc = "0x78 - Upper 16 bits of the eighth 6-byte MAC address"]
    #[inline(always)]
    pub const fn emacaddr7high(&self) -> &Emacaddr7high {
        &self.emacaddr7high
    }
    #[doc = "0x7c - This field contains the lower 32 bits of the eighth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
    #[inline(always)]
    pub const fn emacaddr7low(&self) -> &Emacaddr7low {
        &self.emacaddr7low
    }
    #[doc = "0xd8 - Link communication status"]
    #[inline(always)]
    pub const fn emaccstatus(&self) -> &Emaccstatus {
        &self.emaccstatus
    }
    #[doc = "0xdc - Watchdog timeout control"]
    #[inline(always)]
    pub const fn emacwdogto(&self) -> &Emacwdogto {
        &self.emacwdogto
    }
}
#[doc = "EMACCONFIG (rw) register accessor: MAC configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`emacconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacconfig`] module"]
#[doc(alias = "EMACCONFIG")]
pub type Emacconfig = crate::Reg<emacconfig::EmacconfigSpec>;
#[doc = "MAC configuration"]
pub mod emacconfig;
#[doc = "EMACFF (rw) register accessor: Frame filter settings\n\nYou can [`read`](crate::Reg::read) this register and get [`emacff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacff`] module"]
#[doc(alias = "EMACFF")]
pub type Emacff = crate::Reg<emacff::EmacffSpec>;
#[doc = "Frame filter settings"]
pub mod emacff;
#[doc = "EMACGMIIADDR (rw) register accessor: PHY configuration access\n\nYou can [`read`](crate::Reg::read) this register and get [`emacgmiiaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacgmiiaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacgmiiaddr`] module"]
#[doc(alias = "EMACGMIIADDR")]
pub type Emacgmiiaddr = crate::Reg<emacgmiiaddr::EmacgmiiaddrSpec>;
#[doc = "PHY configuration access"]
pub mod emacgmiiaddr;
#[doc = "EMACMIIDATA (rw) register accessor: PHY data read write\n\nYou can [`read`](crate::Reg::read) this register and get [`emacmiidata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacmiidata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacmiidata`] module"]
#[doc(alias = "EMACMIIDATA")]
pub type Emacmiidata = crate::Reg<emacmiidata::EmacmiidataSpec>;
#[doc = "PHY data read write"]
pub mod emacmiidata;
#[doc = "EMACFC (rw) register accessor: Frame flow control\n\nYou can [`read`](crate::Reg::read) this register and get [`emacfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacfc`] module"]
#[doc(alias = "EMACFC")]
pub type Emacfc = crate::Reg<emacfc::EmacfcSpec>;
#[doc = "Frame flow control"]
pub mod emacfc;
#[doc = "EMACDEBUG (r) register accessor: Status debugging bits\n\nYou can [`read`](crate::Reg::read) this register and get [`emacdebug::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacdebug`] module"]
#[doc(alias = "EMACDEBUG")]
pub type Emacdebug = crate::Reg<emacdebug::EmacdebugSpec>;
#[doc = "Status debugging bits"]
pub mod emacdebug;
#[doc = "PMT_RWUFFR (r) register accessor: The MSB (31st bit) must be zero.Bit j\\[30:0\\] is the byte mask. If Bit 1/2/3/4 (byte number) of the byte mask is set the CRC block processes the Filter 1/2/3/4 Offset + j of the incoming packet(PWKPTR is 0/1/2/3).RWKPTR is 0:Filter 0 Byte Mask .RWKPTR is 1:Filter 1 Byte Mask RWKPTR is 2:Filter 2 Byte Mask RWKPTR is 3:Filter 3 Byte Mask RWKPTR is 4:Bit 3/11/19/27 specifies the address type defining the destination address type of the pattern.When the bit is set the pattern applies to only multicast packets\n\nYou can [`read`](crate::Reg::read) this register and get [`pmt_rwuffr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmt_rwuffr`] module"]
#[doc(alias = "PMT_RWUFFR")]
pub type PmtRwuffr = crate::Reg<pmt_rwuffr::PmtRwuffrSpec>;
#[doc = "The MSB (31st bit) must be zero.Bit j\\[30:0\\] is the byte mask. If Bit 1/2/3/4 (byte number) of the byte mask is set the CRC block processes the Filter 1/2/3/4 Offset + j of the incoming packet(PWKPTR is 0/1/2/3).RWKPTR is 0:Filter 0 Byte Mask .RWKPTR is 1:Filter 1 Byte Mask RWKPTR is 2:Filter 2 Byte Mask RWKPTR is 3:Filter 3 Byte Mask RWKPTR is 4:Bit 3/11/19/27 specifies the address type defining the destination address type of the pattern.When the bit is set the pattern applies to only multicast packets"]
pub mod pmt_rwuffr;
#[doc = "PMT_CSR (r) register accessor: PMT Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`pmt_csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmt_csr`] module"]
#[doc(alias = "PMT_CSR")]
pub type PmtCsr = crate::Reg<pmt_csr::PmtCsrSpec>;
#[doc = "PMT Control and Status"]
pub mod pmt_csr;
#[doc = "EMACLPI_CRS (r) register accessor: LPI Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`emaclpi_crs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emaclpi_crs`] module"]
#[doc(alias = "EMACLPI_CRS")]
pub type EmaclpiCrs = crate::Reg<emaclpi_crs::EmaclpiCrsSpec>;
#[doc = "LPI Control and Status"]
pub mod emaclpi_crs;
#[doc = "EMACLPITIMERSCONTROL (r) register accessor: LPI Timers Control\n\nYou can [`read`](crate::Reg::read) this register and get [`emaclpitimerscontrol::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emaclpitimerscontrol`] module"]
#[doc(alias = "EMACLPITIMERSCONTROL")]
pub type Emaclpitimerscontrol = crate::Reg<emaclpitimerscontrol::EmaclpitimerscontrolSpec>;
#[doc = "LPI Timers Control"]
pub mod emaclpitimerscontrol;
#[doc = "EMACINTS (r) register accessor: Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`emacints::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacints`] module"]
#[doc(alias = "EMACINTS")]
pub type Emacints = crate::Reg<emacints::EmacintsSpec>;
#[doc = "Interrupt status"]
pub mod emacints;
#[doc = "EMACINTMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`emacintmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacintmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacintmask`] module"]
#[doc(alias = "EMACINTMASK")]
pub type Emacintmask = crate::Reg<emacintmask::EmacintmaskSpec>;
#[doc = "Interrupt mask"]
pub mod emacintmask;
#[doc = "EMACADDR0HIGH (rw) register accessor: Upper 16 bits of the first 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr0high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr0high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacaddr0high`] module"]
#[doc(alias = "EMACADDR0HIGH")]
pub type Emacaddr0high = crate::Reg<emacaddr0high::Emacaddr0highSpec>;
#[doc = "Upper 16 bits of the first 6-byte MAC address"]
pub mod emacaddr0high;
#[doc = "EMACADDR0LOW (rw) register accessor: This field contains the lower 32 bits of the first 6-byte MAC address. This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames.\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr0low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr0low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacaddr0low`] module"]
#[doc(alias = "EMACADDR0LOW")]
pub type Emacaddr0low = crate::Reg<emacaddr0low::Emacaddr0lowSpec>;
#[doc = "This field contains the lower 32 bits of the first 6-byte MAC address. This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
pub mod emacaddr0low;
#[doc = "EMACADDR1HIGH (rw) register accessor: Upper 16 bits of the second 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr1high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr1high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacaddr1high`] module"]
#[doc(alias = "EMACADDR1HIGH")]
pub type Emacaddr1high = crate::Reg<emacaddr1high::Emacaddr1highSpec>;
#[doc = "Upper 16 bits of the second 6-byte MAC address"]
pub mod emacaddr1high;
#[doc = "EMACADDR1LOW (rw) register accessor: This field contains the lower 32 bits of the second 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr1low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr1low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacaddr1low`] module"]
#[doc(alias = "EMACADDR1LOW")]
pub type Emacaddr1low = crate::Reg<emacaddr1low::Emacaddr1lowSpec>;
#[doc = "This field contains the lower 32 bits of the second 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
pub mod emacaddr1low;
#[doc = "EMACADDR2HIGH (rw) register accessor: Upper 16 bits of the third 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr2high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr2high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacaddr2high`] module"]
#[doc(alias = "EMACADDR2HIGH")]
pub type Emacaddr2high = crate::Reg<emacaddr2high::Emacaddr2highSpec>;
#[doc = "Upper 16 bits of the third 6-byte MAC address"]
pub mod emacaddr2high;
#[doc = "EMACADDR2LOW (rw) register accessor: This field contains the lower 32 bits of the third 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr2low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr2low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacaddr2low`] module"]
#[doc(alias = "EMACADDR2LOW")]
pub type Emacaddr2low = crate::Reg<emacaddr2low::Emacaddr2lowSpec>;
#[doc = "This field contains the lower 32 bits of the third 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
pub mod emacaddr2low;
#[doc = "EMACADDR3HIGH (rw) register accessor: Upper 16 bits of the fourth 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr3high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr3high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacaddr3high`] module"]
#[doc(alias = "EMACADDR3HIGH")]
pub type Emacaddr3high = crate::Reg<emacaddr3high::Emacaddr3highSpec>;
#[doc = "Upper 16 bits of the fourth 6-byte MAC address"]
pub mod emacaddr3high;
#[doc = "EMACADDR3LOW (rw) register accessor: This field contains the lower 32 bits of the fourth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr3low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr3low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacaddr3low`] module"]
#[doc(alias = "EMACADDR3LOW")]
pub type Emacaddr3low = crate::Reg<emacaddr3low::Emacaddr3lowSpec>;
#[doc = "This field contains the lower 32 bits of the fourth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
pub mod emacaddr3low;
#[doc = "EMACADDR4HIGH (rw) register accessor: Upper 16 bits of the fifth 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr4high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr4high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacaddr4high`] module"]
#[doc(alias = "EMACADDR4HIGH")]
pub type Emacaddr4high = crate::Reg<emacaddr4high::Emacaddr4highSpec>;
#[doc = "Upper 16 bits of the fifth 6-byte MAC address"]
pub mod emacaddr4high;
#[doc = "EMACADDR4LOW (rw) register accessor: This field contains the lower 32 bits of the fifth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr4low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr4low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacaddr4low`] module"]
#[doc(alias = "EMACADDR4LOW")]
pub type Emacaddr4low = crate::Reg<emacaddr4low::Emacaddr4lowSpec>;
#[doc = "This field contains the lower 32 bits of the fifth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
pub mod emacaddr4low;
#[doc = "EMACADDR5HIGH (rw) register accessor: Upper 16 bits of the sixth 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr5high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr5high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacaddr5high`] module"]
#[doc(alias = "EMACADDR5HIGH")]
pub type Emacaddr5high = crate::Reg<emacaddr5high::Emacaddr5highSpec>;
#[doc = "Upper 16 bits of the sixth 6-byte MAC address"]
pub mod emacaddr5high;
#[doc = "EMACADDR5LOW (rw) register accessor: This field contains the lower 32 bits of the sixth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr5low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr5low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacaddr5low`] module"]
#[doc(alias = "EMACADDR5LOW")]
pub type Emacaddr5low = crate::Reg<emacaddr5low::Emacaddr5lowSpec>;
#[doc = "This field contains the lower 32 bits of the sixth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
pub mod emacaddr5low;
#[doc = "EMACADDR6HIGH (rw) register accessor: Upper 16 bits of the seventh 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr6high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr6high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacaddr6high`] module"]
#[doc(alias = "EMACADDR6HIGH")]
pub type Emacaddr6high = crate::Reg<emacaddr6high::Emacaddr6highSpec>;
#[doc = "Upper 16 bits of the seventh 6-byte MAC address"]
pub mod emacaddr6high;
#[doc = "EMACADDR6LOW (rw) register accessor: This field contains the lower 32 bits of the seventh 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr6low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr6low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacaddr6low`] module"]
#[doc(alias = "EMACADDR6LOW")]
pub type Emacaddr6low = crate::Reg<emacaddr6low::Emacaddr6lowSpec>;
#[doc = "This field contains the lower 32 bits of the seventh 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
pub mod emacaddr6low;
#[doc = "EMACADDR7HIGH (rw) register accessor: Upper 16 bits of the eighth 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr7high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr7high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacaddr7high`] module"]
#[doc(alias = "EMACADDR7HIGH")]
pub type Emacaddr7high = crate::Reg<emacaddr7high::Emacaddr7highSpec>;
#[doc = "Upper 16 bits of the eighth 6-byte MAC address"]
pub mod emacaddr7high;
#[doc = "EMACADDR7LOW (rw) register accessor: This field contains the lower 32 bits of the eighth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr7low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr7low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacaddr7low`] module"]
#[doc(alias = "EMACADDR7LOW")]
pub type Emacaddr7low = crate::Reg<emacaddr7low::Emacaddr7lowSpec>;
#[doc = "This field contains the lower 32 bits of the eighth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
pub mod emacaddr7low;
#[doc = "EMACCSTATUS (r) register accessor: Link communication status\n\nYou can [`read`](crate::Reg::read) this register and get [`emaccstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emaccstatus`] module"]
#[doc(alias = "EMACCSTATUS")]
pub type Emaccstatus = crate::Reg<emaccstatus::EmaccstatusSpec>;
#[doc = "Link communication status"]
pub mod emaccstatus;
#[doc = "EMACWDOGTO (rw) register accessor: Watchdog timeout control\n\nYou can [`read`](crate::Reg::read) this register and get [`emacwdogto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacwdogto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emacwdogto`] module"]
#[doc(alias = "EMACWDOGTO")]
pub type Emacwdogto = crate::Reg<emacwdogto::EmacwdogtoSpec>;
#[doc = "Watchdog timeout control"]
pub mod emacwdogto;
