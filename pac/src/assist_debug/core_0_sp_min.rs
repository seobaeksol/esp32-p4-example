#[doc = "Register `CORE_0_SP_MIN` reader"]
pub type R = crate::R<Core0SpMinSpec>;
#[doc = "Register `CORE_0_SP_MIN` writer"]
pub type W = crate::W<Core0SpMinSpec>;
#[doc = "Field `CORE_0_SP_MIN` reader - core0 sp region configuration regsiter"]
pub type Core0SpMinR = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_SP_MIN` writer - core0 sp region configuration regsiter"]
pub type Core0SpMinW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - core0 sp region configuration regsiter"]
    #[inline(always)]
    pub fn core_0_sp_min(&self) -> Core0SpMinR {
        Core0SpMinR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - core0 sp region configuration regsiter"]
    #[inline(always)]
    pub fn core_0_sp_min(&mut self) -> Core0SpMinW<'_, Core0SpMinSpec> {
        Core0SpMinW::new(self, 0)
    }
}
#[doc = "stack min value\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_sp_min::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_sp_min::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core0SpMinSpec;
impl crate::RegisterSpec for Core0SpMinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_sp_min::R`](R) reader structure"]
impl crate::Readable for Core0SpMinSpec {}
#[doc = "`write(|w| ..)` method takes [`core_0_sp_min::W`](W) writer structure"]
impl crate::Writable for Core0SpMinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_0_SP_MIN to value 0"]
impl crate::Resettable for Core0SpMinSpec {}
