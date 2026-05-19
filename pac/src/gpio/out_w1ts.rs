#[doc = "Register `OUT_W1TS` writer"]
pub type W = crate::W<OutW1tsSpec>;
#[doc = "Field `OUT_W1TS` writer - GPIO output set register for GPIO0-31"]
pub type OutW1tsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - GPIO output set register for GPIO0-31"]
    #[inline(always)]
    pub fn out_w1ts(&mut self) -> OutW1tsW<'_, OutW1tsSpec> {
        OutW1tsW::new(self, 0)
    }
}
#[doc = "GPIO output set register for GPIO0-31\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutW1tsSpec;
impl crate::RegisterSpec for OutW1tsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_w1ts::W`](W) writer structure"]
impl crate::Writable for OutW1tsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_W1TS to value 0"]
impl crate::Resettable for OutW1tsSpec {}
