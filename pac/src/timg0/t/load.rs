#[doc = "Register `LOAD` writer"]
pub type W = crate::W<LoadSpec>;
#[doc = "Field `LOAD` writer - Write any value to trigger a timer %s time-base counter reload."]
pub type LoadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Write any value to trigger a timer %s time-base counter reload."]
    #[inline(always)]
    pub fn load(&mut self) -> LoadW<'_, LoadSpec> {
        LoadW::new(self, 0)
    }
}
#[doc = "Write to reload timer from TIMG_T0_(LOADLOLOADHI)_REG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoadSpec;
impl crate::RegisterSpec for LoadSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`load::W`](W) writer structure"]
impl crate::Writable for LoadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOAD to value 0"]
impl crate::Resettable for LoadSpec {}
