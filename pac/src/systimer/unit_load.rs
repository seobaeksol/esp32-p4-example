#[doc = "Register `UNIT%s_LOAD` writer"]
pub type W = crate::W<UnitLoadSpec>;
#[doc = "Field `LOAD` writer - timer unit0 sync enable signal"]
pub type LoadW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - timer unit0 sync enable signal"]
    #[inline(always)]
    pub fn load(&mut self) -> LoadW<'_, UnitLoadSpec> {
        LoadW::new(self, 0)
    }
}
#[doc = "system timer unit%s conf sync register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit_load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UnitLoadSpec;
impl crate::RegisterSpec for UnitLoadSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`unit_load::W`](W) writer structure"]
impl crate::Writable for UnitLoadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNIT%s_LOAD to value 0"]
impl crate::Resettable for UnitLoadSpec {}
