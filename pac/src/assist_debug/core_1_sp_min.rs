#[doc = "Register `CORE_1_SP_MIN` reader"]
pub type R = crate::R<Core1SpMinSpec>;
#[doc = "Register `CORE_1_SP_MIN` writer"]
pub type W = crate::W<Core1SpMinSpec>;
#[doc = "Field `CORE_1_SP_MIN` reader - core1 sp region configuration regsiter"]
pub type Core1SpMinR = crate::FieldReader<u32>;
#[doc = "Field `CORE_1_SP_MIN` writer - core1 sp region configuration regsiter"]
pub type Core1SpMinW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - core1 sp region configuration regsiter"]
    #[inline(always)]
    pub fn core_1_sp_min(&self) -> Core1SpMinR {
        Core1SpMinR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - core1 sp region configuration regsiter"]
    #[inline(always)]
    pub fn core_1_sp_min(&mut self) -> Core1SpMinW<'_, Core1SpMinSpec> {
        Core1SpMinW::new(self, 0)
    }
}
#[doc = "stack min value\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_sp_min::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_sp_min::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1SpMinSpec;
impl crate::RegisterSpec for Core1SpMinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_sp_min::R`](R) reader structure"]
impl crate::Readable for Core1SpMinSpec {}
#[doc = "`write(|w| ..)` method takes [`core_1_sp_min::W`](W) writer structure"]
impl crate::Writable for Core1SpMinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_1_SP_MIN to value 0"]
impl crate::Resettable for Core1SpMinSpec {}
