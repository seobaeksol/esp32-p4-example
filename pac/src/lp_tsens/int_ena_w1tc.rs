#[doc = "Register `INT_ENA_W1TC` writer"]
pub type W = crate::W<IntEnaW1tcSpec>;
#[doc = "Field `COCPU_TSENS_WAKE_INT_ENA_W1TC` writer - Write 1 to this field to deassert interrupt enable."]
pub type CocpuTsensWakeIntEnaW1tcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 to this field to deassert interrupt enable."]
    #[inline(always)]
    pub fn cocpu_tsens_wake_int_ena_w1tc(
        &mut self,
    ) -> CocpuTsensWakeIntEnaW1tcW<'_, IntEnaW1tcSpec> {
        CocpuTsensWakeIntEnaW1tcW::new(self, 0)
    }
}
#[doc = "Tsens wakeup interrupt enable deassert.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnaW1tcSpec;
impl crate::RegisterSpec for IntEnaW1tcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_ena_w1tc::W`](W) writer structure"]
impl crate::Writable for IntEnaW1tcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA_W1TC to value 0"]
impl crate::Resettable for IntEnaW1tcSpec {}
