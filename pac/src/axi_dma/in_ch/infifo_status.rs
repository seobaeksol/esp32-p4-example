#[doc = "Register `INFIFO_STATUS` reader"]
pub type R = crate::R<InfifoStatusSpec>;
#[doc = "Field `INFIFO_L3_FULL` reader - L3 Rx FIFO full signal for Rx channel 0."]
pub type InfifoL3FullR = crate::BitReader;
#[doc = "Field `INFIFO_L3_EMPTY` reader - L3 Rx FIFO empty signal for Rx channel 0."]
pub type InfifoL3EmptyR = crate::BitReader;
#[doc = "Field `INFIFO_L3_CNT` reader - The register stores the byte number of the data in L3 Rx FIFO for Rx channel 0."]
pub type InfifoL3CntR = crate::FieldReader;
#[doc = "Field `INFIFO_L3_UDF` reader - L3 Rx FIFO under flow signal for Rx channel 0."]
pub type InfifoL3UdfR = crate::BitReader;
#[doc = "Field `INFIFO_L3_OVF` reader - L3 Rx FIFO over flow signal for Rx channel 0."]
pub type InfifoL3OvfR = crate::BitReader;
#[doc = "Field `INFIFO_L1_FULL` reader - L1 Rx FIFO full signal for Rx channel 0."]
pub type InfifoL1FullR = crate::BitReader;
#[doc = "Field `INFIFO_L1_EMPTY` reader - L1 Rx FIFO empty signal for Rx channel 0."]
pub type InfifoL1EmptyR = crate::BitReader;
#[doc = "Field `INFIFO_L1_UDF` reader - L1 Rx FIFO under flow signal for Rx channel 0."]
pub type InfifoL1UdfR = crate::BitReader;
#[doc = "Field `INFIFO_L1_OVF` reader - L1 Rx FIFO over flow signal for Rx channel 0."]
pub type InfifoL1OvfR = crate::BitReader;
#[doc = "Field `INFIFO_L2_FULL` reader - L2 Rx RAM full signal for Rx channel 0."]
pub type InfifoL2FullR = crate::BitReader;
#[doc = "Field `INFIFO_L2_EMPTY` reader - L2 Rx RAM empty signal for Rx channel 0."]
pub type InfifoL2EmptyR = crate::BitReader;
#[doc = "Field `INFIFO_L2_UDF` reader - L2 Rx FIFO under flow signal for Rx channel 0."]
pub type InfifoL2UdfR = crate::BitReader;
#[doc = "Field `INFIFO_L2_OVF` reader - L2 Rx FIFO over flow signal for Rx channel 0."]
pub type InfifoL2OvfR = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_1B` reader - reserved"]
pub type InRemainUnder1bR = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_2B` reader - reserved"]
pub type InRemainUnder2bR = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_3B` reader - reserved"]
pub type InRemainUnder3bR = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_4B` reader - reserved"]
pub type InRemainUnder4bR = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_5B` reader - reserved"]
pub type InRemainUnder5bR = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_6B` reader - reserved"]
pub type InRemainUnder6bR = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_7B` reader - reserved"]
pub type InRemainUnder7bR = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_8B` reader - reserved"]
pub type InRemainUnder8bR = crate::BitReader;
#[doc = "Field `IN_BUF_HUNGRY` reader - reserved"]
pub type InBufHungryR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - L3 Rx FIFO full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l3_full(&self) -> InfifoL3FullR {
        InfifoL3FullR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - L3 Rx FIFO empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l3_empty(&self) -> InfifoL3EmptyR {
        InfifoL3EmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - The register stores the byte number of the data in L3 Rx FIFO for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l3_cnt(&self) -> InfifoL3CntR {
        InfifoL3CntR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - L3 Rx FIFO under flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l3_udf(&self) -> InfifoL3UdfR {
        InfifoL3UdfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - L3 Rx FIFO over flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l3_ovf(&self) -> InfifoL3OvfR {
        InfifoL3OvfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - L1 Rx FIFO full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l1_full(&self) -> InfifoL1FullR {
        InfifoL1FullR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - L1 Rx FIFO empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l1_empty(&self) -> InfifoL1EmptyR {
        InfifoL1EmptyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - L1 Rx FIFO under flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l1_udf(&self) -> InfifoL1UdfR {
        InfifoL1UdfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - L1 Rx FIFO over flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l1_ovf(&self) -> InfifoL1OvfR {
        InfifoL1OvfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - L2 Rx RAM full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l2_full(&self) -> InfifoL2FullR {
        InfifoL2FullR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - L2 Rx RAM empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l2_empty(&self) -> InfifoL2EmptyR {
        InfifoL2EmptyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - L2 Rx FIFO under flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l2_udf(&self) -> InfifoL2UdfR {
        InfifoL2UdfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - L2 Rx FIFO over flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l2_ovf(&self) -> InfifoL2OvfR {
        InfifoL2OvfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_1b(&self) -> InRemainUnder1bR {
        InRemainUnder1bR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_2b(&self) -> InRemainUnder2bR {
        InRemainUnder2bR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_3b(&self) -> InRemainUnder3bR {
        InRemainUnder3bR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_4b(&self) -> InRemainUnder4bR {
        InRemainUnder4bR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_5b(&self) -> InRemainUnder5bR {
        InRemainUnder5bR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_6b(&self) -> InRemainUnder6bR {
        InRemainUnder6bR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_7b(&self) -> InRemainUnder7bR {
        InRemainUnder7bR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_8b(&self) -> InRemainUnder8bR {
        InRemainUnder8bR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - reserved"]
    #[inline(always)]
    pub fn in_buf_hungry(&self) -> InBufHungryR {
        InBufHungryR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Receive FIFO status of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`infifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InfifoStatusSpec;
impl crate::RegisterSpec for InfifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`infifo_status::R`](R) reader structure"]
impl crate::Readable for InfifoStatusSpec {}
#[doc = "`reset()` method sets INFIFO_STATUS to value 0x8803"]
impl crate::Resettable for InfifoStatusSpec {
    const RESET_VALUE: u32 = 0x8803;
}
