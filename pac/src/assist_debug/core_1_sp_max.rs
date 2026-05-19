#[doc = "Register `CORE_1_SP_MAX` reader"]
pub type R = crate::R<Core1SpMaxSpec>;
#[doc = "Register `CORE_1_SP_MAX` writer"]
pub type W = crate::W<Core1SpMaxSpec>;
#[doc = "Field `CORE_1_SP_MAX` reader - core1 sp pc status register"]
pub type Core1SpMaxR = crate::FieldReader<u32>;
#[doc = "Field `CORE_1_SP_MAX` writer - core1 sp pc status register"]
pub type Core1SpMaxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - core1 sp pc status register"]
    #[inline(always)]
    pub fn core_1_sp_max(&self) -> Core1SpMaxR {
        Core1SpMaxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - core1 sp pc status register"]
    #[inline(always)]
    pub fn core_1_sp_max(&mut self) -> Core1SpMaxW<'_, Core1SpMaxSpec> {
        Core1SpMaxW::new(self, 0)
    }
}
#[doc = "stack max value\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_sp_max::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_sp_max::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1SpMaxSpec;
impl crate::RegisterSpec for Core1SpMaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_sp_max::R`](R) reader structure"]
impl crate::Readable for Core1SpMaxSpec {}
#[doc = "`write(|w| ..)` method takes [`core_1_sp_max::W`](W) writer structure"]
impl crate::Writable for Core1SpMaxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_1_SP_MAX to value 0xffff_ffff"]
impl crate::Resettable for Core1SpMaxSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
