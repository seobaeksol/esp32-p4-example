#[doc = "Register `INT_ENA_W1TS` writer"]
pub type W = crate::W<IntEnaW1tsSpec>;
#[doc = "Field `COCPU_SARADC1_INT_ENA_W1TS` writer - ADC1 Conversion is done, write 1 to assert int enable."]
pub type CocpuSaradc1IntEnaW1tsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC2_INT_ENA_W1TS` writer - ADC2 Conversion is done, write 1 to assert int enable."]
pub type CocpuSaradc2IntEnaW1tsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC1_ERROR_INT_ENA_W1TS` writer - An errro occurs from ADC1, write 1 to assert int enable."]
pub type CocpuSaradc1ErrorIntEnaW1tsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC2_ERROR_INT_ENA_W1TS` writer - An errro occurs from ADC2, write 1 to assert int enable."]
pub type CocpuSaradc2ErrorIntEnaW1tsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC1_WAKE_INT_ENA_W1TS` writer - A wakeup event is triggered from ADC1, write 1 to assert int enable."]
pub type CocpuSaradc1WakeIntEnaW1tsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC2_WAKE_INT_ENA_W1TS` writer - A wakeup event is triggered from ADC2, write 1 to assert int enable."]
pub type CocpuSaradc2WakeIntEnaW1tsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - ADC1 Conversion is done, write 1 to assert int enable."]
    #[inline(always)]
    pub fn cocpu_saradc1_int_ena_w1ts(&mut self) -> CocpuSaradc1IntEnaW1tsW<'_, IntEnaW1tsSpec> {
        CocpuSaradc1IntEnaW1tsW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC2 Conversion is done, write 1 to assert int enable."]
    #[inline(always)]
    pub fn cocpu_saradc2_int_ena_w1ts(&mut self) -> CocpuSaradc2IntEnaW1tsW<'_, IntEnaW1tsSpec> {
        CocpuSaradc2IntEnaW1tsW::new(self, 1)
    }
    #[doc = "Bit 2 - An errro occurs from ADC1, write 1 to assert int enable."]
    #[inline(always)]
    pub fn cocpu_saradc1_error_int_ena_w1ts(
        &mut self,
    ) -> CocpuSaradc1ErrorIntEnaW1tsW<'_, IntEnaW1tsSpec> {
        CocpuSaradc1ErrorIntEnaW1tsW::new(self, 2)
    }
    #[doc = "Bit 3 - An errro occurs from ADC2, write 1 to assert int enable."]
    #[inline(always)]
    pub fn cocpu_saradc2_error_int_ena_w1ts(
        &mut self,
    ) -> CocpuSaradc2ErrorIntEnaW1tsW<'_, IntEnaW1tsSpec> {
        CocpuSaradc2ErrorIntEnaW1tsW::new(self, 3)
    }
    #[doc = "Bit 4 - A wakeup event is triggered from ADC1, write 1 to assert int enable."]
    #[inline(always)]
    pub fn cocpu_saradc1_wake_int_ena_w1ts(
        &mut self,
    ) -> CocpuSaradc1WakeIntEnaW1tsW<'_, IntEnaW1tsSpec> {
        CocpuSaradc1WakeIntEnaW1tsW::new(self, 4)
    }
    #[doc = "Bit 5 - A wakeup event is triggered from ADC2, write 1 to assert int enable."]
    #[inline(always)]
    pub fn cocpu_saradc2_wake_int_ena_w1ts(
        &mut self,
    ) -> CocpuSaradc2WakeIntEnaW1tsW<'_, IntEnaW1tsSpec> {
        CocpuSaradc2WakeIntEnaW1tsW::new(self, 5)
    }
}
#[doc = "Interrupt enable assert registers.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
