#[doc = "Register `ENABLE_W1TS` writer"]
pub type W = crate::W<EnableW1tsSpec>;
#[doc = "Field `REG_GPIO_ENABLE_DATA_W1TS` writer - Reserved"]
pub type RegGpioEnableDataW1tsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_enable_data_w1ts(&mut self) -> RegGpioEnableDataW1tsW<'_, EnableW1tsSpec> {
        RegGpioEnableDataW1tsW::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableW1tsSpec;
impl crate::RegisterSpec for EnableW1tsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enable_w1ts::W`](W) writer structure"]
impl crate::Writable for EnableW1tsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLE_W1TS to value 0"]
impl crate::Resettable for EnableW1tsSpec {}
