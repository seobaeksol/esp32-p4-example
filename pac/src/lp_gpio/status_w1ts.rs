#[doc = "Register `STATUS_W1TS` writer"]
pub type W = crate::W<StatusW1tsSpec>;
#[doc = "Field `REG_GPIO_STATUS_DATA_W1TS` writer - Reserved"]
pub type RegGpioStatusDataW1tsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_status_data_w1ts(&mut self) -> RegGpioStatusDataW1tsW<'_, StatusW1tsSpec> {
        RegGpioStatusDataW1tsW::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusW1tsSpec;
impl crate::RegisterSpec for StatusW1tsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`status_w1ts::W`](W) writer structure"]
impl crate::Writable for StatusW1tsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS_W1TS to value 0"]
impl crate::Resettable for StatusW1tsSpec {}
