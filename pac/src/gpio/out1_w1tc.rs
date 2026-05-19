#[doc = "Register `OUT1_W1TC` writer"]
pub type W = crate::W<Out1W1tcSpec>;
#[doc = "Field `OUT1_W1TC` writer - GPIO output clear register for GPIO32-56"]
pub type Out1W1tcW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl W {
    #[doc = "Bits 0:24 - GPIO output clear register for GPIO32-56"]
    #[inline(always)]
    pub fn out1_w1tc(&mut self) -> Out1W1tcW<'_, Out1W1tcSpec> {
        Out1W1tcW::new(self, 0)
    }
}
#[doc = "GPIO output clear register for GPIO32-56\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out1_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Out1W1tcSpec;
impl crate::RegisterSpec for Out1W1tcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out1_w1tc::W`](W) writer structure"]
impl crate::Writable for Out1W1tcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT1_W1TC to value 0"]
impl crate::Resettable for Out1W1tcSpec {}
