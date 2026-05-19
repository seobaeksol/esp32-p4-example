#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `RXFIFO_CNT` reader - Stores the byte number of valid data in Rx-FIFO."]
pub type RxfifoCntR = crate::FieldReader;
#[doc = "Field `DSRN` reader - The register represent the level value of the internal uart dsr signal."]
pub type DsrnR = crate::BitReader;
#[doc = "Field `CTSN` reader - This register represent the level value of the internal uart cts signal."]
pub type CtsnR = crate::BitReader;
#[doc = "Field `RXD` reader - This register represent the level value of the internal uart rxd signal."]
pub type RxdR = crate::BitReader;
#[doc = "Field `TXFIFO_CNT` reader - Stores the byte number of data in Tx-FIFO."]
pub type TxfifoCntR = crate::FieldReader;
#[doc = "Field `DTRN` reader - This bit represents the level of the internal uart dtr signal."]
pub type DtrnR = crate::BitReader;
#[doc = "Field `RTSN` reader - This bit represents the level of the internal uart rts signal."]
pub type RtsnR = crate::BitReader;
#[doc = "Field `TXD` reader - This bit represents the level of the internal uart txd signal."]
pub type TxdR = crate::BitReader;
impl R {
    #[doc = "Bits 3:7 - Stores the byte number of valid data in Rx-FIFO."]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RxfifoCntR {
        RxfifoCntR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - The register represent the level value of the internal uart dsr signal."]
    #[inline(always)]
    pub fn dsrn(&self) -> DsrnR {
        DsrnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This register represent the level value of the internal uart cts signal."]
    #[inline(always)]
    pub fn ctsn(&self) -> CtsnR {
        CtsnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This register represent the level value of the internal uart rxd signal."]
    #[inline(always)]
    pub fn rxd(&self) -> RxdR {
        RxdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 19:23 - Stores the byte number of data in Tx-FIFO."]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TxfifoCntR {
        TxfifoCntR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - This bit represents the level of the internal uart dtr signal."]
    #[inline(always)]
    pub fn dtrn(&self) -> DtrnR {
        DtrnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit represents the level of the internal uart rts signal."]
    #[inline(always)]
    pub fn rtsn(&self) -> RtsnR {
        RtsnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit represents the level of the internal uart txd signal."]
    #[inline(always)]
    pub fn txd(&self) -> TxdR {
        TxdR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "UART status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0xe000_c000"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0xe000_c000;
}
