#[doc = "Register `CORE_1_AREA_PIF_0_MAX` reader"]
pub type R = crate::R<Core1AreaPif0MaxSpec>;
#[doc = "Register `CORE_1_AREA_PIF_0_MAX` writer"]
pub type W = crate::W<Core1AreaPif0MaxSpec>;
#[doc = "Field `CORE_1_AREA_PIF_0_MAX` reader - Core1 PIF region0 end addr"]
pub type Core1AreaPif0MaxR = crate::FieldReader<u32>;
#[doc = "Field `CORE_1_AREA_PIF_0_MAX` writer - Core1 PIF region0 end addr"]
pub type Core1AreaPif0MaxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Core1 PIF region0 end addr"]
    #[inline(always)]
    pub fn core_1_area_pif_0_max(&self) -> Core1AreaPif0MaxR {
        Core1AreaPif0MaxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Core1 PIF region0 end addr"]
    #[inline(always)]
    pub fn core_1_area_pif_0_max(&mut self) -> Core1AreaPif0MaxW<'_, Core1AreaPif0MaxSpec> {
        Core1AreaPif0MaxW::new(self, 0)
    }
}
#[doc = "core1 PIF region0 addr configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_area_pif_0_max::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_area_pif_0_max::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1AreaPif0MaxSpec;
impl crate::RegisterSpec for Core1AreaPif0MaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_area_pif_0_max::R`](R) reader structure"]
impl crate::Readable for Core1AreaPif0MaxSpec {}
#[doc = "`write(|w| ..)` method takes [`core_1_area_pif_0_max::W`](W) writer structure"]
impl crate::Writable for Core1AreaPif0MaxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_1_AREA_PIF_0_MAX to value 0"]
impl crate::Resettable for Core1AreaPif0MaxSpec {}
