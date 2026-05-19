#[doc = "Register `INT_ENA_W1TS` writer"]
pub type W = crate::W<IntEnaW1tsSpec>;
#[doc = "Field `COCPU_TSENS_WAKE_INT_ENA_W1TS` writer - Write 1 to this field to assert interrupt enable."]
pub type CocpuTsensWakeIntEnaW1tsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 to this field to assert interrupt enable."]
    #[inline(always)]
    pub fn cocpu_tsens_wake_int_ena_w1ts(
        &mut self,
    ) -> CocpuTsensWakeIntEnaW1tsW<'_, IntEnaW1tsSpec> {
        CocpuTsensWakeIntEnaW1tsW::new(self, 0)
    }
}
#[doc = "Tsens wakeup interrupt enable assert.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnaW1tsSpec;
impl crate::RegisterSpec for IntEnaW1tsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_ena_w1ts::W`](W) writer structure"]
impl crate::Writable for IntEnaW1tsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA_W1TS to value 0"]
impl crate::Resettable for IntEnaW1tsSpec {}
