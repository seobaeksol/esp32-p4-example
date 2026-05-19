#[doc = "Register `OUT_W1TC` writer"]
pub type W = crate::W<OutW1tcSpec>;
#[doc = "Field `OUT_W1TC` writer - GPIO output clear register for GPIO0-31"]
pub type OutW1tcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - GPIO output clear register for GPIO0-31"]
    #[inline(always)]
    pub fn out_w1tc(&mut self) -> OutW1tcW<'_, OutW1tcSpec> {
        OutW1tcW::new(self, 0)
    }
}
#[doc = "GPIO output clear register for GPIO0-31\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutW1tcSpec;
impl crate::RegisterSpec for OutW1tcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_w1tc::W`](W) writer structure"]
impl crate::Writable for OutW1tcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_W1TC to value 0"]
impl crate::Resettable for OutW1tcSpec {}
