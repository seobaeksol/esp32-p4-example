#[doc = "Register `FIFO_STATUS` reader"]
pub type R = crate::R<FifoStatusSpec>;
#[doc = "Field `INFIFO_FULL_L2` reader - Rx FIFO full signal for Rx channel."]
pub type InfifoFullL2R = crate::BitReader;
#[doc = "Field `INFIFO_EMPTY_L2` reader - Rx FIFO empty signal for Rx channel."]
pub type InfifoEmptyL2R = crate::BitReader;
#[doc = "Field `INFIFO_CNT_L2` reader - The register stores the byte number of the data in Rx FIFO for Rx channel."]
pub type InfifoCntL2R = crate::FieldReader;
#[doc = "Field `INFIFO_FULL_L1` reader - Tx FIFO full signal for Tx channel 0."]
pub type InfifoFullL1R = crate::BitReader;
#[doc = "Field `INFIFO_EMPTY_L1` reader - Tx FIFO empty signal for Tx channel 0."]
pub type InfifoEmptyL1R = crate::BitReader;
#[doc = "Field `INFIFO_CNT_L1` reader - The register stores the byte number of the data in Tx FIFO for Tx channel 0."]
pub type InfifoCntL1R = crate::FieldReader;
#[doc = "Field `INFIFO_FULL_L3` reader - Tx FIFO full signal for Tx channel 0."]
pub type InfifoFullL3R = crate::BitReader;
#[doc = "Field `INFIFO_EMPTY_L3` reader - Tx FIFO empty signal for Tx channel 0."]
pub type InfifoEmptyL3R = crate::BitReader;
#[doc = "Field `INFIFO_CNT_L3` reader - The register stores the 8byte number of the data in Tx FIFO for Tx channel 0."]
pub type InfifoCntL3R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Rx FIFO full signal for Rx channel."]
    #[inline(always)]
    pub fn infifo_full_l2(&self) -> InfifoFullL2R {
        InfifoFullL2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO empty signal for Rx channel."]
    #[inline(always)]
    pub fn infifo_empty_l2(&self) -> InfifoEmptyL2R {
        InfifoEmptyL2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - The register stores the byte number of the data in Rx FIFO for Rx channel."]
    #[inline(always)]
    pub fn infifo_cnt_l2(&self) -> InfifoCntL2R {
        InfifoCntL2R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Tx FIFO full signal for Tx channel 0."]
    #[inline(always)]
    pub fn infifo_full_l1(&self) -> InfifoFullL1R {
        InfifoFullL1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx FIFO empty signal for Tx channel 0."]
    #[inline(always)]
    pub fn infifo_empty_l1(&self) -> InfifoEmptyL1R {
        InfifoEmptyL1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - The register stores the byte number of the data in Tx FIFO for Tx channel 0."]
    #[inline(always)]
    pub fn infifo_cnt_l1(&self) -> InfifoCntL1R {
        InfifoCntL1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Tx FIFO full signal for Tx channel 0."]
    #[inline(always)]
    pub fn infifo_full_l3(&self) -> InfifoFullL3R {
        InfifoFullL3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tx FIFO empty signal for Tx channel 0."]
    #[inline(always)]
    pub fn infifo_empty_l3(&self) -> InfifoEmptyL3R {
        InfifoEmptyL3R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - The register stores the 8byte number of the data in Tx FIFO for Tx channel 0."]
    #[inline(always)]
    pub fn infifo_cnt_l3(&self) -> InfifoCntL3R {
        InfifoCntL3R::new(((self.bits >> 18) & 3) as u8)
    }
}
#[doc = "RX CHx INFIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoStatusSpec;
impl crate::RegisterSpec for FifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_status::R`](R) reader structure"]
impl crate::Readable for FifoStatusSpec {}
#[doc = "`reset()` method sets FIFO_STATUS to value 0x0002_0082"]
impl crate::Resettable for FifoStatusSpec {
    const RESET_VALUE: u32 = 0x0002_0082;
}
