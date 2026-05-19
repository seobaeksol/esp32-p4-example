#[doc = "Register `EMACINTS` reader"]
pub type R = crate::R<EmacintsSpec>;
#[doc = "Field `PMTINTS` reader - This bit is set when a magic packet or remote wake-up frame is received in the power-down mode (see Bit\\[5\\] and Bit\\[6\\] in the PMT Control and Status Register). This bit is cleared when both Bits\\[6:5\\] are cleared because of a read operation to the PMT Control and Status register. This bit is valid only when you select the optional PMT module during core configuration."]
pub type PmtintsR = crate::BitReader;
#[doc = "Field `LPIIS` reader - When the Energy Efficient Ethernet feature is enabled this bit is set for any LPI state entry or exit in the MAC Transmitter or Receiver. This bit is cleared on reading Bit\\[0\\] of Register (LPI Control and Status Register)."]
pub type LpiisR = crate::BitReader;
impl R {
    #[doc = "Bit 3 - This bit is set when a magic packet or remote wake-up frame is received in the power-down mode (see Bit\\[5\\] and Bit\\[6\\] in the PMT Control and Status Register). This bit is cleared when both Bits\\[6:5\\] are cleared because of a read operation to the PMT Control and Status register. This bit is valid only when you select the optional PMT module during core configuration."]
    #[inline(always)]
    pub fn pmtints(&self) -> PmtintsR {
        PmtintsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 10 - When the Energy Efficient Ethernet feature is enabled this bit is set for any LPI state entry or exit in the MAC Transmitter or Receiver. This bit is cleared on reading Bit\\[0\\] of Register (LPI Control and Status Register)."]
    #[inline(always)]
    pub fn lpiis(&self) -> LpiisR {
        LpiisR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`emacints::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmacintsSpec;
impl crate::RegisterSpec for EmacintsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacints::R`](R) reader structure"]
impl crate::Readable for EmacintsSpec {}
#[doc = "`reset()` method sets EMACINTS to value 0"]
impl crate::Resettable for EmacintsSpec {}
