#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Field `RX_DONE` reader - The raw interrupt status bit for the i2s_rx_done_int interrupt"]
pub type RxDoneR = crate::BitReader;
#[doc = "Field `RX_HUNG` reader - The raw interrupt status bit for the i2s_rx_hung_int interrupt"]
pub type RxHungR = crate::BitReader;
#[doc = "Field `RX_FIFOMEM_UDF` reader - The raw interrupt status bit for the i2s_rx_fifomem_udf_int interrupt"]
pub type RxFifomemUdfR = crate::BitReader;
#[doc = "Field `VAD_DONE` reader - The raw interrupt status bit for the vad_done_int interrupt"]
pub type VadDoneR = crate::BitReader;
#[doc = "Field `VAD_RESET_DONE` reader - The raw interrupt status bit for the vad_reset_done_int interrupt"]
pub type VadResetDoneR = crate::BitReader;
#[doc = "Field `RX_MEM_THRESHOLD` reader - The raw interrupt status bit for the rx_mem_threshold_int interrupt"]
pub type RxMemThresholdR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn rx_done(&self) -> RxDoneR {
        RxDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn rx_hung(&self) -> RxHungR {
        RxHungR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the i2s_rx_fifomem_udf_int interrupt"]
    #[inline(always)]
    pub fn rx_fifomem_udf(&self) -> RxFifomemUdfR {
        RxFifomemUdfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the vad_done_int interrupt"]
    #[inline(always)]
    pub fn vad_done(&self) -> VadDoneR {
        VadDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for the vad_reset_done_int interrupt"]
    #[inline(always)]
    pub fn vad_reset_done(&self) -> VadResetDoneR {
        VadResetDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for the rx_mem_threshold_int interrupt"]
    #[inline(always)]
    pub fn rx_mem_threshold(&self) -> RxMemThresholdR {
        RxMemThresholdR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "I2S interrupt raw register, valid in level.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntRawSpec;
impl crate::RegisterSpec for IntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for IntRawSpec {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for IntRawSpec {}
