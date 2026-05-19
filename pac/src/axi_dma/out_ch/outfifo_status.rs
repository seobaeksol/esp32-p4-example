#[doc = "Register `OUTFIFO_STATUS` reader"]
pub type R = crate::R<OutfifoStatusSpec>;
#[doc = "Field `OUTFIFO_L3_FULL` reader - L3 Tx FIFO full signal for Tx channel0."]
pub type OutfifoL3FullR = crate::BitReader;
#[doc = "Field `OUTFIFO_L3_EMPTY` reader - L3 Tx FIFO empty signal for Tx channel0."]
pub type OutfifoL3EmptyR = crate::BitReader;
#[doc = "Field `OUTFIFO_L3_CNT` reader - The register stores the byte number of the data in L3 Tx FIFO for Tx channel0."]
pub type OutfifoL3CntR = crate::FieldReader;
#[doc = "Field `OUTFIFO_L3_UDF` reader - L3 Tx FIFO under flow signal for Tx channel0."]
pub type OutfifoL3UdfR = crate::BitReader;
#[doc = "Field `OUTFIFO_L3_OVF` reader - L3 Tx FIFO over flow signal for Tx channel0."]
pub type OutfifoL3OvfR = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_FULL` reader - L1 Tx FIFO full signal for Tx channel0."]
pub type OutfifoL1FullR = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_EMPTY` reader - L1 Tx FIFO empty signal for Tx channel0."]
pub type OutfifoL1EmptyR = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_UDF` reader - L1 Tx FIFO under flow signal for Tx channel0."]
pub type OutfifoL1UdfR = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_OVF` reader - L1 Tx FIFO over flow signal for Tx channel0."]
pub type OutfifoL1OvfR = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_FULL` reader - L2 Tx RAM full signal for Tx channel0."]
pub type OutfifoL2FullR = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_EMPTY` reader - L2 Tx RAM empty signal for Tx channel0."]
pub type OutfifoL2EmptyR = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_UDF` reader - L2 Tx FIFO under flow signal for Tx channel0."]
pub type OutfifoL2UdfR = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_OVF` reader - L2 Tx FIFO over flow signal for Tx channel0."]
pub type OutfifoL2OvfR = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_1B` reader - reserved"]
pub type OutRemainUnder1bR = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_2B` reader - reserved"]
pub type OutRemainUnder2bR = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_3B` reader - reserved"]
pub type OutRemainUnder3bR = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_4B` reader - reserved"]
pub type OutRemainUnder4bR = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_5B` reader - reserved"]
pub type OutRemainUnder5bR = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_6B` reader - reserved"]
pub type OutRemainUnder6bR = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_7B` reader - reserved"]
pub type OutRemainUnder7bR = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_8B` reader - reserved"]
pub type OutRemainUnder8bR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - L3 Tx FIFO full signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l3_full(&self) -> OutfifoL3FullR {
        OutfifoL3FullR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - L3 Tx FIFO empty signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l3_empty(&self) -> OutfifoL3EmptyR {
        OutfifoL3EmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - The register stores the byte number of the data in L3 Tx FIFO for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l3_cnt(&self) -> OutfifoL3CntR {
        OutfifoL3CntR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - L3 Tx FIFO under flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l3_udf(&self) -> OutfifoL3UdfR {
        OutfifoL3UdfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - L3 Tx FIFO over flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l3_ovf(&self) -> OutfifoL3OvfR {
        OutfifoL3OvfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - L1 Tx FIFO full signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l1_full(&self) -> OutfifoL1FullR {
        OutfifoL1FullR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - L1 Tx FIFO empty signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l1_empty(&self) -> OutfifoL1EmptyR {
        OutfifoL1EmptyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - L1 Tx FIFO under flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l1_udf(&self) -> OutfifoL1UdfR {
        OutfifoL1UdfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - L1 Tx FIFO over flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l1_ovf(&self) -> OutfifoL1OvfR {
        OutfifoL1OvfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - L2 Tx RAM full signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l2_full(&self) -> OutfifoL2FullR {
        OutfifoL2FullR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - L2 Tx RAM empty signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l2_empty(&self) -> OutfifoL2EmptyR {
        OutfifoL2EmptyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - L2 Tx FIFO under flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l2_udf(&self) -> OutfifoL2UdfR {
        OutfifoL2UdfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - L2 Tx FIFO over flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l2_ovf(&self) -> OutfifoL2OvfR {
        OutfifoL2OvfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_1b(&self) -> OutRemainUnder1bR {
        OutRemainUnder1bR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_2b(&self) -> OutRemainUnder2bR {
        OutRemainUnder2bR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_3b(&self) -> OutRemainUnder3bR {
        OutRemainUnder3bR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_4b(&self) -> OutRemainUnder4bR {
        OutRemainUnder4bR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_5b(&self) -> OutRemainUnder5bR {
        OutRemainUnder5bR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_6b(&self) -> OutRemainUnder6bR {
        OutRemainUnder6bR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_7b(&self) -> OutRemainUnder7bR {
        OutRemainUnder7bR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_8b(&self) -> OutRemainUnder8bR {
        OutRemainUnder8bR::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "Transmit FIFO status of Tx channel0\n\nYou can [`read`](crate::Reg::read) this register and get [`outfifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutfifoStatusSpec;
impl crate::RegisterSpec for OutfifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outfifo_status::R`](R) reader structure"]
impl crate::Readable for OutfifoStatusSpec {}
#[doc = "`reset()` method sets OUTFIFO_STATUS to value 0x7f80_8802"]
impl crate::Resettable for OutfifoStatusSpec {
    const RESET_VALUE: u32 = 0x7f80_8802;
}
