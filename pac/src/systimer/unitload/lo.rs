#[doc = "Register `LO` reader"]
pub type R = crate::R<LoSpec>;
#[doc = "Register `LO` writer"]
pub type W = crate::W<LoSpec>;
#[doc = "Field `LOAD_LO` reader - timer unit0 load low 32 bits"]
pub type LoadLoR = crate::FieldReader<u32>;
#[doc = "Field `LOAD_LO` writer - timer unit0 load low 32 bits"]
pub type LoadLoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    #[doc = "Bits 0:31 - timer unit0 load low 32 bits"]
    #[inline(always)]
    pub fn load_lo(&self) -> LoadLoR {
        LoadLoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - timer unit0 load low 32 bits"]
    #[inline(always)]
    pub fn load_lo(&mut self) -> LoadLoW<'_, LoSpec> {
        LoadLoW::new(self, 0)
    }
}
#[doc = "system timer unit0 value low load register\n\nYou can [`read`](crate::Reg::read) this register and get [`lo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoSpec;
impl crate::RegisterSpec for LoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo::R`](R) reader structure"]
impl crate::Readable for LoSpec {}
#[doc = "`write(|w| ..)` method takes [`lo::W`](W) writer structure"]
impl crate::Writable for LoSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets LO to value 0"]
impl crate::Resettable for LoSpec {}
