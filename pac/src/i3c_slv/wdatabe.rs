#[doc = "Register `WDATABE` writer"]
pub type W = crate::W<WdatabeSpec>;
#[doc = "Field `WDATABE` writer - NA"]
pub type WdatabeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn wdatabe(&mut self) -> WdatabeW<'_, WdatabeSpec> {
        WdatabeW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdatabe::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdatabeSpec;
impl crate::RegisterSpec for WdatabeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wdatabe::W`](W) writer structure"]
impl crate::Writable for WdatabeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDATABE to value 0"]
impl crate::Resettable for WdatabeSpec {}
