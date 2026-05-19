#[doc = "Register `CORE_0_SP_MAX` reader"]
pub type R = crate::R<Core0SpMaxSpec>;
#[doc = "Register `CORE_0_SP_MAX` writer"]
pub type W = crate::W<Core0SpMaxSpec>;
#[doc = "Field `CORE_0_SP_MAX` reader - core0 sp pc status register"]
pub type Core0SpMaxR = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_SP_MAX` writer - core0 sp pc status register"]
pub type Core0SpMaxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - core0 sp pc status register"]
    #[inline(always)]
    pub fn core_0_sp_max(&self) -> Core0SpMaxR {
        Core0SpMaxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - core0 sp pc status register"]
    #[inline(always)]
    pub fn core_0_sp_max(&mut self) -> Core0SpMaxW<'_, Core0SpMaxSpec> {
        Core0SpMaxW::new(self, 0)
    }
}
#[doc = "stack max value\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_sp_max::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_sp_max::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core0SpMaxSpec;
impl crate::RegisterSpec for Core0SpMaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_sp_max::R`](R) reader structure"]
impl crate::Readable for Core0SpMaxSpec {}
#[doc = "`write(|w| ..)` method takes [`core_0_sp_max::W`](W) writer structure"]
impl crate::Writable for Core0SpMaxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_0_SP_MAX to value 0xffff_ffff"]
impl crate::Resettable for Core0SpMaxSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
