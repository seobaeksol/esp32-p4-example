#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dmabusmode: Dmabusmode,
    dmatxpolldemand: Dmatxpolldemand,
    dmarxpolldemand: Dmarxpolldemand,
    dmarxbaseaddr: Dmarxbaseaddr,
    dmatxbaseaddr: Dmatxbaseaddr,
    dmastatus: Dmastatus,
    dmaoperation_mode: DmaoperationMode,
    dmain_en: DmainEn,
    dmamissedfr: Dmamissedfr,
    dmarintwdtimer: Dmarintwdtimer,
    _reserved10: [u8; 0x20],
    dmatxcurrdesc: Dmatxcurrdesc,
    dmarxcurrdesc: Dmarxcurrdesc,
    dmatxcurraddr_buf: DmatxcurraddrBuf,
    dmarxcurraddr_buf: DmarxcurraddrBuf,
}
impl RegisterBlock {
    #[doc = "0x00 - Bus mode configuration"]
    #[inline(always)]
    pub const fn dmabusmode(&self) -> &Dmabusmode {
        &self.dmabusmode
    }
    #[doc = "0x04 - When these bits are written with any value the DMA reads the current descriptor to which the Register (Current Host Transmit Descriptor Register) is pointing. If that descriptor is not available (owned by the Host) the transmission returns to the suspend state and Bit\\[2\\] (TU) of Status Register is asserted. If the descriptor is available the transmission resumes."]
    #[inline(always)]
    pub const fn dmatxpolldemand(&self) -> &Dmatxpolldemand {
        &self.dmatxpolldemand
    }
    #[doc = "0x08 - When these bits are written with any value the DMA reads the current descriptor to which the Current Host Receive Descriptor Register is pointing. If that descriptor is not available (owned by the Host) the reception returns to the Suspended state and Bit\\[7\\] (RU) of Status Register is asserted. If the descriptor is available the Rx DMA returns to the active state."]
    #[inline(always)]
    pub const fn dmarxpolldemand(&self) -> &Dmarxpolldemand {
        &self.dmarxpolldemand
    }
    #[doc = "0x0c - This field contains the base address of the first descriptor in the Receive Descriptor list. The LSB Bits\\[1:0\\] are ignored and internally taken as all-zero by the DMA. Therefore these LSB bits are read-only."]
    #[inline(always)]
    pub const fn dmarxbaseaddr(&self) -> &Dmarxbaseaddr {
        &self.dmarxbaseaddr
    }
    #[doc = "0x10 - This field contains the base address of the first descriptor in the Transmit Descriptor list. The LSB Bits\\[1:0\\] are ignored and are internally taken as all-zero by the DMA.Therefore these LSB bits are read-only."]
    #[inline(always)]
    pub const fn dmatxbaseaddr(&self) -> &Dmatxbaseaddr {
        &self.dmatxbaseaddr
    }
    #[doc = "0x14 - State of interrupts, errors and other events"]
    #[inline(always)]
    pub const fn dmastatus(&self) -> &Dmastatus {
        &self.dmastatus
    }
    #[doc = "0x18 - Receive and Transmit operating modes and command"]
    #[inline(always)]
    pub const fn dmaoperation_mode(&self) -> &DmaoperationMode {
        &self.dmaoperation_mode
    }
    #[doc = "0x1c - Enable / disable interrupts"]
    #[inline(always)]
    pub const fn dmain_en(&self) -> &DmainEn {
        &self.dmain_en
    }
    #[doc = "0x20 - Missed Frame and Buffer Overflow Counter Register"]
    #[inline(always)]
    pub const fn dmamissedfr(&self) -> &Dmamissedfr {
        &self.dmamissedfr
    }
    #[doc = "0x24 - Watchdog timer count on receive"]
    #[inline(always)]
    pub const fn dmarintwdtimer(&self) -> &Dmarintwdtimer {
        &self.dmarintwdtimer
    }
    #[doc = "0x48 - The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub const fn dmatxcurrdesc(&self) -> &Dmatxcurrdesc {
        &self.dmatxcurrdesc
    }
    #[doc = "0x4c - The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub const fn dmarxcurrdesc(&self) -> &Dmarxcurrdesc {
        &self.dmarxcurrdesc
    }
    #[doc = "0x50 - The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub const fn dmatxcurraddr_buf(&self) -> &DmatxcurraddrBuf {
        &self.dmatxcurraddr_buf
    }
    #[doc = "0x54 - The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub const fn dmarxcurraddr_buf(&self) -> &DmarxcurraddrBuf {
        &self.dmarxcurraddr_buf
    }
}
#[doc = "DMABUSMODE (rw) register accessor: Bus mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dmabusmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmabusmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmabusmode`] module"]
#[doc(alias = "DMABUSMODE")]
pub type Dmabusmode = crate::Reg<dmabusmode::DmabusmodeSpec>;
#[doc = "Bus mode configuration"]
pub mod dmabusmode;
#[doc = "DMATXPOLLDEMAND (r) register accessor: When these bits are written with any value the DMA reads the current descriptor to which the Register (Current Host Transmit Descriptor Register) is pointing. If that descriptor is not available (owned by the Host) the transmission returns to the suspend state and Bit\\[2\\] (TU) of Status Register is asserted. If the descriptor is available the transmission resumes.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatxpolldemand::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatxpolldemand`] module"]
#[doc(alias = "DMATXPOLLDEMAND")]
pub type Dmatxpolldemand = crate::Reg<dmatxpolldemand::DmatxpolldemandSpec>;
#[doc = "When these bits are written with any value the DMA reads the current descriptor to which the Register (Current Host Transmit Descriptor Register) is pointing. If that descriptor is not available (owned by the Host) the transmission returns to the suspend state and Bit\\[2\\] (TU) of Status Register is asserted. If the descriptor is available the transmission resumes."]
pub mod dmatxpolldemand;
#[doc = "DMARXPOLLDEMAND (r) register accessor: When these bits are written with any value the DMA reads the current descriptor to which the Current Host Receive Descriptor Register is pointing. If that descriptor is not available (owned by the Host) the reception returns to the Suspended state and Bit\\[7\\] (RU) of Status Register is asserted. If the descriptor is available the Rx DMA returns to the active state.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmarxpolldemand::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmarxpolldemand`] module"]
#[doc(alias = "DMARXPOLLDEMAND")]
pub type Dmarxpolldemand = crate::Reg<dmarxpolldemand::DmarxpolldemandSpec>;
#[doc = "When these bits are written with any value the DMA reads the current descriptor to which the Current Host Receive Descriptor Register is pointing. If that descriptor is not available (owned by the Host) the reception returns to the Suspended state and Bit\\[7\\] (RU) of Status Register is asserted. If the descriptor is available the Rx DMA returns to the active state."]
pub mod dmarxpolldemand;
#[doc = "DMARXBASEADDR (rw) register accessor: This field contains the base address of the first descriptor in the Receive Descriptor list. The LSB Bits\\[1:0\\] are ignored and internally taken as all-zero by the DMA. Therefore these LSB bits are read-only.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmarxbaseaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmarxbaseaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmarxbaseaddr`] module"]
#[doc(alias = "DMARXBASEADDR")]
pub type Dmarxbaseaddr = crate::Reg<dmarxbaseaddr::DmarxbaseaddrSpec>;
#[doc = "This field contains the base address of the first descriptor in the Receive Descriptor list. The LSB Bits\\[1:0\\] are ignored and internally taken as all-zero by the DMA. Therefore these LSB bits are read-only."]
pub mod dmarxbaseaddr;
#[doc = "DMATXBASEADDR (rw) register accessor: This field contains the base address of the first descriptor in the Transmit Descriptor list. The LSB Bits\\[1:0\\] are ignored and are internally taken as all-zero by the DMA.Therefore these LSB bits are read-only.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatxbaseaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatxbaseaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatxbaseaddr`] module"]
#[doc(alias = "DMATXBASEADDR")]
pub type Dmatxbaseaddr = crate::Reg<dmatxbaseaddr::DmatxbaseaddrSpec>;
#[doc = "This field contains the base address of the first descriptor in the Transmit Descriptor list. The LSB Bits\\[1:0\\] are ignored and are internally taken as all-zero by the DMA.Therefore these LSB bits are read-only."]
pub mod dmatxbaseaddr;
#[doc = "DMASTATUS (rw) register accessor: State of interrupts, errors and other events\n\nYou can [`read`](crate::Reg::read) this register and get [`dmastatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmastatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastatus`] module"]
#[doc(alias = "DMASTATUS")]
pub type Dmastatus = crate::Reg<dmastatus::DmastatusSpec>;
#[doc = "State of interrupts, errors and other events"]
pub mod dmastatus;
#[doc = "DMAOPERATION_MODE (rw) register accessor: Receive and Transmit operating modes and command\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaoperation_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaoperation_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaoperation_mode`] module"]
#[doc(alias = "DMAOPERATION_MODE")]
pub type DmaoperationMode = crate::Reg<dmaoperation_mode::DmaoperationModeSpec>;
#[doc = "Receive and Transmit operating modes and command"]
pub mod dmaoperation_mode;
#[doc = "DMAIN_EN (rw) register accessor: Enable / disable interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`dmain_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmain_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmain_en`] module"]
#[doc(alias = "DMAIN_EN")]
pub type DmainEn = crate::Reg<dmain_en::DmainEnSpec>;
#[doc = "Enable / disable interrupts"]
pub mod dmain_en;
#[doc = "DMAMISSEDFR (rw) register accessor: Missed Frame and Buffer Overflow Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmamissedfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamissedfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamissedfr`] module"]
#[doc(alias = "DMAMISSEDFR")]
pub type Dmamissedfr = crate::Reg<dmamissedfr::DmamissedfrSpec>;
#[doc = "Missed Frame and Buffer Overflow Counter Register"]
pub mod dmamissedfr;
#[doc = "DMARINTWDTIMER (rw) register accessor: Watchdog timer count on receive\n\nYou can [`read`](crate::Reg::read) this register and get [`dmarintwdtimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmarintwdtimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmarintwdtimer`] module"]
#[doc(alias = "DMARINTWDTIMER")]
pub type Dmarintwdtimer = crate::Reg<dmarintwdtimer::DmarintwdtimerSpec>;
#[doc = "Watchdog timer count on receive"]
pub mod dmarintwdtimer;
#[doc = "DMATXCURRDESC (r) register accessor: The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatxcurrdesc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatxcurrdesc`] module"]
#[doc(alias = "DMATXCURRDESC")]
pub type Dmatxcurrdesc = crate::Reg<dmatxcurrdesc::DmatxcurrdescSpec>;
#[doc = "The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation."]
pub mod dmatxcurrdesc;
#[doc = "DMARXCURRDESC (r) register accessor: The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmarxcurrdesc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmarxcurrdesc`] module"]
#[doc(alias = "DMARXCURRDESC")]
pub type Dmarxcurrdesc = crate::Reg<dmarxcurrdesc::DmarxcurrdescSpec>;
#[doc = "The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation."]
pub mod dmarxcurrdesc;
#[doc = "DMATXCURRADDR_BUF (r) register accessor: The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatxcurraddr_buf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatxcurraddr_buf`] module"]
#[doc(alias = "DMATXCURRADDR_BUF")]
pub type DmatxcurraddrBuf = crate::Reg<dmatxcurraddr_buf::DmatxcurraddrBufSpec>;
#[doc = "The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation."]
pub mod dmatxcurraddr_buf;
#[doc = "DMARXCURRADDR_BUF (r) register accessor: The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmarxcurraddr_buf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmarxcurraddr_buf`] module"]
#[doc(alias = "DMARXCURRADDR_BUF")]
pub type DmarxcurraddrBuf = crate::Reg<dmarxcurraddr_buf::DmarxcurraddrBufSpec>;
#[doc = "The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation."]
pub mod dmarxcurraddr_buf;
