#[doc = "Register `FIFO_STATUS` reader"]
pub type R = crate::R<FifoStatusSpec>;
#[doc = "Field `INFIFO_FULL_L1` reader - Tx FIFO full signal for Tx channel 1."]
pub type InfifoFullL1R = crate::BitReader;
#[doc = "Field `INFIFO_EMPTY_L1` reader - Tx FIFO empty signal for Tx channel 1."]
pub type InfifoEmptyL1R = crate::BitReader;
#[doc = "Field `INFIFO_CNT_L1` reader - The register stores the byte number of the data in Tx FIFO for Tx channel 1."]
pub type InfifoCntL1R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Tx FIFO full signal for Tx channel 1."]
    #[inline(always)]
    pub fn infifo_full_l1(&self) -> InfifoFullL1R {
        InfifoFullL1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx FIFO empty signal for Tx channel 1."]
    #[inline(always)]
    pub fn infifo_empty_l1(&self) -> InfifoEmptyL1R {
        InfifoEmptyL1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - The register stores the byte number of the data in Tx FIFO for Tx channel 1."]
    #[inline(always)]
    pub fn infifo_cnt_l1(&self) -> InfifoCntL1R {
        InfifoCntL1R::new(((self.bits >> 2) & 0x1f) as u8)
    }
}
#[doc = "RX CH5 INFIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoStatusSpec;
impl crate::RegisterSpec for FifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_status::R`](R) reader structure"]
impl crate::Readable for FifoStatusSpec {}
#[doc = "`reset()` method sets FIFO_STATUS to value 0x02"]
impl crate::Resettable for FifoStatusSpec {
    const RESET_VALUE: u32 = 0x02;
}
