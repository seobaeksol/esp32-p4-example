#[doc = "Register `OUT_PERI_SEL` reader"]
pub type R = crate::R<OutPeriSelSpec>;
#[doc = "Register `OUT_PERI_SEL` writer"]
pub type W = crate::W<OutPeriSelSpec>;
#[doc = "Field `PERI_OUT_SEL_CH0` reader - Configures the peripheral connected to TX channel 0.\\\\ 0: I3C\\\\ 1: Dummy\\\\ 2: UHCI0\\\\ 3: I2S0\\\\ 4: I2S1\\\\ 5: I2S2\\\\ 6: Dummy\\\\ 7: Dummy\\\\ 8: ADC_DAC\\\\ 9: Dummy\\\\ 10: RMT\\\\ 11~15: Dummy\\\\"]
pub type PeriOutSelCh0R = crate::FieldReader;
#[doc = "Field `PERI_OUT_SEL_CH0` writer - Configures the peripheral connected to TX channel 0.\\\\ 0: I3C\\\\ 1: Dummy\\\\ 2: UHCI0\\\\ 3: I2S0\\\\ 4: I2S1\\\\ 5: I2S2\\\\ 6: Dummy\\\\ 7: Dummy\\\\ 8: ADC_DAC\\\\ 9: Dummy\\\\ 10: RMT\\\\ 11~15: Dummy\\\\"]
pub type PeriOutSelCh0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Configures the peripheral connected to TX channel 0.\\\\ 0: I3C\\\\ 1: Dummy\\\\ 2: UHCI0\\\\ 3: I2S0\\\\ 4: I2S1\\\\ 5: I2S2\\\\ 6: Dummy\\\\ 7: Dummy\\\\ 8: ADC_DAC\\\\ 9: Dummy\\\\ 10: RMT\\\\ 11~15: Dummy\\\\"]
    #[inline(always)]
    pub fn peri_out_sel_ch0(&self) -> PeriOutSelCh0R {
        PeriOutSelCh0R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures the peripheral connected to TX channel 0.\\\\ 0: I3C\\\\ 1: Dummy\\\\ 2: UHCI0\\\\ 3: I2S0\\\\ 4: I2S1\\\\ 5: I2S2\\\\ 6: Dummy\\\\ 7: Dummy\\\\ 8: ADC_DAC\\\\ 9: Dummy\\\\ 10: RMT\\\\ 11~15: Dummy\\\\"]
    #[inline(always)]
    pub fn peri_out_sel_ch0(&mut self) -> PeriOutSelCh0W<'_, OutPeriSelSpec> {
        PeriOutSelCh0W::new(self, 0)
    }
}
#[doc = "Peripheral selection register of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_peri_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_peri_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutPeriSelSpec;
impl crate::RegisterSpec for OutPeriSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_peri_sel::R`](R) reader structure"]
impl crate::Readable for OutPeriSelSpec {}
#[doc = "`write(|w| ..)` method takes [`out_peri_sel::W`](W) writer structure"]
impl crate::Writable for OutPeriSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_PERI_SEL to value 0x3f"]
impl crate::Resettable for OutPeriSelSpec {
    const RESET_VALUE: u32 = 0x3f;
}
