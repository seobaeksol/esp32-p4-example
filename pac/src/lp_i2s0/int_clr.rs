#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<IntClrSpec>;
#[doc = "Field `RX_DONE` writer - Set this bit to clear the i2s_rx_done_int interrupt"]
pub type RxDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_HUNG` writer - Set this bit to clear the i2s_rx_hung_int interrupt"]
pub type RxHungW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_FIFOMEM_UDF` writer - Set this bit to clear the i2s_rx_fifomem_udf_int interrupt"]
pub type RxFifomemUdfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LP_VAD_DONE` writer - Set this bit to clear the vad_done_int interrupt"]
pub type LpVadDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LP_VAD_RESET_DONE` writer - Set this bit to clear the vad_reset_done_int interrupt"]
pub type LpVadResetDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_MEM_THRESHOLD` writer - Set this bit to clear the rx_mem_threshold_int interrupt"]
pub type RxMemThresholdW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn rx_done(&mut self) -> RxDoneW<'_, IntClrSpec> {
        RxDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn rx_hung(&mut self) -> RxHungW<'_, IntClrSpec> {
        RxHungW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the i2s_rx_fifomem_udf_int interrupt"]
    #[inline(always)]
    pub fn rx_fifomem_udf(&mut self) -> RxFifomemUdfW<'_, IntClrSpec> {
        RxFifomemUdfW::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the vad_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_done(&mut self) -> LpVadDoneW<'_, IntClrSpec> {
        LpVadDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear the vad_reset_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_reset_done(&mut self) -> LpVadResetDoneW<'_, IntClrSpec> {
        LpVadResetDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear the rx_mem_threshold_int interrupt"]
    #[inline(always)]
    pub fn rx_mem_threshold(&mut self) -> RxMemThresholdW<'_, IntClrSpec> {
        RxMemThresholdW::new(self, 5)
    }
}
#[doc = "I2S interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClrSpec;
impl crate::RegisterSpec for IntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for IntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for IntClrSpec {}
