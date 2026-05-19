#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `RX_DONE` reader - The masked interrupt status bit for the i2s_rx_done_int interrupt"]
pub type RxDoneR = crate::BitReader;
#[doc = "Field `RX_HUNG` reader - The masked interrupt status bit for the i2s_rx_hung_int interrupt"]
pub type RxHungR = crate::BitReader;
#[doc = "Field `RX_FIFOMEM_UDF` reader - The masked interrupt status bit for the i2s_rx_fifomem_udf_int interrupt"]
pub type RxFifomemUdfR = crate::BitReader;
#[doc = "Field `LP_VAD_DONE` reader - The masked interrupt status bit for the vad_done_int interrupt"]
pub type LpVadDoneR = crate::BitReader;
#[doc = "Field `LP_VAD_RESET_DONE` reader - The masked interrupt status bit for the vad_reset_done_int interrupt"]
pub type LpVadResetDoneR = crate::BitReader;
#[doc = "Field `RX_MEM_THRESHOLD` reader - The masked interrupt status bit for the rx_mem_threshold_int interrupt"]
pub type RxMemThresholdR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn rx_done(&self) -> RxDoneR {
        RxDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn rx_hung(&self) -> RxHungR {
        RxHungR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status bit for the i2s_rx_fifomem_udf_int interrupt"]
    #[inline(always)]
    pub fn rx_fifomem_udf(&self) -> RxFifomemUdfR {
        RxFifomemUdfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status bit for the vad_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_done(&self) -> LpVadDoneR {
        LpVadDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status bit for the vad_reset_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_reset_done(&self) -> LpVadResetDoneR {
        LpVadResetDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status bit for the rx_mem_threshold_int interrupt"]
    #[inline(always)]
    pub fn rx_mem_threshold(&self) -> RxMemThresholdR {
        RxMemThresholdR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "I2S interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}
