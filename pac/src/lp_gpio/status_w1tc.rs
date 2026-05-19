#[doc = "Register `STATUS_W1TC` writer"]
pub type W = crate::W<StatusW1tcSpec>;
#[doc = "Field `REG_GPIO_STATUS_DATA_W1TC` writer - Reserved"]
pub type RegGpioStatusDataW1tcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_status_data_w1tc(&mut self) -> RegGpioStatusDataW1tcW<'_, StatusW1tcSpec> {
        RegGpioStatusDataW1tcW::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusW1tcSpec;
impl crate::RegisterSpec for StatusW1tcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`status_w1tc::W`](W) writer structure"]
impl crate::Writable for StatusW1tcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS_W1TC to value 0"]
impl crate::Resettable for StatusW1tcSpec {}
