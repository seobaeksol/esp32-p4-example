#[doc = "Register `CORE_0_AREA_DRAM0_1_MAX` reader"]
pub type R = crate::R<Core0AreaDram0_1MaxSpec>;
#[doc = "Register `CORE_0_AREA_DRAM0_1_MAX` writer"]
pub type W = crate::W<Core0AreaDram0_1MaxSpec>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_MAX` reader - Core0 dram0 region1 end addr"]
pub type Core0AreaDram0_1MaxR = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_MAX` writer - Core0 dram0 region1 end addr"]
pub type Core0AreaDram0_1MaxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Core0 dram0 region1 end addr"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_max(&self) -> Core0AreaDram0_1MaxR {
        Core0AreaDram0_1MaxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Core0 dram0 region1 end addr"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_max(&mut self) -> Core0AreaDram0_1MaxW<'_, Core0AreaDram0_1MaxSpec> {
        Core0AreaDram0_1MaxW::new(self, 0)
    }
}
#[doc = "core0 dram0 region1 addr configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_area_dram0_1_max::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_area_dram0_1_max::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core0AreaDram0_1MaxSpec;
impl crate::RegisterSpec for Core0AreaDram0_1MaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_area_dram0_1_max::R`](R) reader structure"]
impl crate::Readable for Core0AreaDram0_1MaxSpec {}
#[doc = "`write(|w| ..)` method takes [`core_0_area_dram0_1_max::W`](W) writer structure"]
impl crate::Writable for Core0AreaDram0_1MaxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_0_AREA_DRAM0_1_MAX to value 0"]
impl crate::Resettable for Core0AreaDram0_1MaxSpec {}
