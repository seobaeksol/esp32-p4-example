#[doc = "Register `STATUS1_W1TC` writer"]
pub type W = crate::W<Status1W1tcSpec>;
#[doc = "Field `STATUS1_W1TC` writer - GPIO interrupt status clear register for GPIO32-56"]
pub type Status1W1tcW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl W {
    #[doc = "Bits 0:24 - GPIO interrupt status clear register for GPIO32-56"]
    #[inline(always)]
    pub fn status1_w1tc(&mut self) -> Status1W1tcW<'_, Status1W1tcSpec> {
        Status1W1tcW::new(self, 0)
    }
}
#[doc = "GPIO interrupt status clear register for GPIO32-56\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status1_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status1W1tcSpec;
impl crate::RegisterSpec for Status1W1tcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`status1_w1tc::W`](W) writer structure"]
impl crate::Writable for Status1W1tcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS1_W1TC to value 0"]
impl crate::Resettable for Status1W1tcSpec {}
