#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<IntEnaSpec>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<IntEnaSpec>;
#[doc = "Field `RX_DONE` reader - The interrupt enable bit for the i2s_rx_done_int interrupt"]
pub type RxDoneR = crate::BitReader;
#[doc = "Field `RX_DONE` writer - The interrupt enable bit for the i2s_rx_done_int interrupt"]
pub type RxDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HUNG` reader - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
pub type RxHungR = crate::BitReader;
#[doc = "Field `RX_HUNG` writer - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
pub type RxHungW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFOMEM_UDF` reader - The interrupt enable bit for the i2s_rx_fifomem_udf_int interrupt"]
pub type RxFifomemUdfR = crate::BitReader;
#[doc = "Field `RX_FIFOMEM_UDF` writer - The interrupt enable bit for the i2s_rx_fifomem_udf_int interrupt"]
pub type RxFifomemUdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_VAD_DONE` reader - The interrupt enable bit for the vad_done_int interrupt"]
pub type LpVadDoneR = crate::BitReader;
#[doc = "Field `LP_VAD_DONE` writer - The interrupt enable bit for the vad_done_int interrupt"]
pub type LpVadDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_VAD_RESET_DONE` reader - The interrupt enable bit for the vad_reset_done_int interrupt"]
pub type LpVadResetDoneR = crate::BitReader;
#[doc = "Field `LP_VAD_RESET_DONE` writer - The interrupt enable bit for the vad_reset_done_int interrupt"]
pub type LpVadResetDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MEM_THRESHOLD` reader - The interrupt enable bit for the rx_mem_threshold_int interrupt"]
pub type RxMemThresholdR = crate::BitReader;
#[doc = "Field `RX_MEM_THRESHOLD` writer - The interrupt enable bit for the rx_mem_threshold_int interrupt"]
pub type RxMemThresholdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn rx_done(&self) -> RxDoneR {
        RxDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn rx_hung(&self) -> RxHungR {
        RxHungR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the i2s_rx_fifomem_udf_int interrupt"]
    #[inline(always)]
    pub fn rx_fifomem_udf(&self) -> RxFifomemUdfR {
        RxFifomemUdfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the vad_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_done(&self) -> LpVadDoneR {
        LpVadDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the vad_reset_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_reset_done(&self) -> LpVadResetDoneR {
        LpVadResetDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the rx_mem_threshold_int interrupt"]
    #[inline(always)]
    pub fn rx_mem_threshold(&self) -> RxMemThresholdR {
        RxMemThresholdR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn rx_done(&mut self) -> RxDoneW<'_, IntEnaSpec> {
        RxDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn rx_hung(&mut self) -> RxHungW<'_, IntEnaSpec> {
        RxHungW::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the i2s_rx_fifomem_udf_int interrupt"]
    #[inline(always)]
    pub fn rx_fifomem_udf(&mut self) -> RxFifomemUdfW<'_, IntEnaSpec> {
        RxFifomemUdfW::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the vad_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_done(&mut self) -> LpVadDoneW<'_, IntEnaSpec> {
        LpVadDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the vad_reset_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_reset_done(&mut self) -> LpVadResetDoneW<'_, IntEnaSpec> {
        LpVadResetDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the rx_mem_threshold_int interrupt"]
    #[inline(always)]
    pub fn rx_mem_threshold(&mut self) -> RxMemThresholdW<'_, IntEnaSpec> {
        RxMemThresholdW::new(self, 5)
    }
}
#[doc = "I2S interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnaSpec;
impl crate::RegisterSpec for IntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for IntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for IntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for IntEnaSpec {}
