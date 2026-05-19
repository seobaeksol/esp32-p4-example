#[doc = "Register `STATUS_W1TC` writer"]
pub type W = crate::W<StatusW1tcSpec>;
#[doc = "Field `STATUS_W1TC` writer - GPIO interrupt status clear register for GPIO0-31"]
pub type StatusW1tcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - GPIO interrupt status clear register for GPIO0-31"]
    #[inline(always)]
    pub fn status_w1tc(&mut self) -> StatusW1tcW<'_, StatusW1tcSpec> {
        StatusW1tcW::new(self, 0)
    }
}
#[doc = "GPIO interrupt status clear register for GPIO0-31\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
