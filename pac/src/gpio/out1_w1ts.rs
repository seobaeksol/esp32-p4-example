#[doc = "Register `OUT1_W1TS` writer"]
pub type W = crate::W<Out1W1tsSpec>;
#[doc = "Field `OUT1_W1TS` writer - GPIO output set register for GPIO32-56"]
pub type Out1W1tsW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl W {
    #[doc = "Bits 0:24 - GPIO output set register for GPIO32-56"]
    #[inline(always)]
    pub fn out1_w1ts(&mut self) -> Out1W1tsW<'_, Out1W1tsSpec> {
        Out1W1tsW::new(self, 0)
    }
}
#[doc = "GPIO output set register for GPIO32-56\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out1_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Out1W1tsSpec;
impl crate::RegisterSpec for Out1W1tsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out1_w1ts::W`](W) writer structure"]
impl crate::Writable for Out1W1tsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT1_W1TS to value 0"]
impl crate::Resettable for Out1W1tsSpec {}
