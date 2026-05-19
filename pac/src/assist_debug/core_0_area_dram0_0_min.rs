#[doc = "Register `CORE_0_AREA_DRAM0_0_MIN` reader"]
pub type R = crate::R<Core0AreaDram0_0MinSpec>;
#[doc = "Register `CORE_0_AREA_DRAM0_0_MIN` writer"]
pub type W = crate::W<Core0AreaDram0_0MinSpec>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_MIN` reader - Core0 dram0 region0 start addr"]
pub type Core0AreaDram0_0MinR = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_MIN` writer - Core0 dram0 region0 start addr"]
pub type Core0AreaDram0_0MinW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Core0 dram0 region0 start addr"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_min(&self) -> Core0AreaDram0_0MinR {
        Core0AreaDram0_0MinR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Core0 dram0 region0 start addr"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_min(&mut self) -> Core0AreaDram0_0MinW<'_, Core0AreaDram0_0MinSpec> {
        Core0AreaDram0_0MinW::new(self, 0)
    }
}
#[doc = "core0 dram0 region0 addr configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_area_dram0_0_min::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_area_dram0_0_min::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core0AreaDram0_0MinSpec;
impl crate::RegisterSpec for Core0AreaDram0_0MinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_area_dram0_0_min::R`](R) reader structure"]
impl crate::Readable for Core0AreaDram0_0MinSpec {}
#[doc = "`write(|w| ..)` method takes [`core_0_area_dram0_0_min::W`](W) writer structure"]
impl crate::Writable for Core0AreaDram0_0MinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_0_AREA_DRAM0_0_MIN to value 0xffff_ffff"]
impl crate::Resettable for Core0AreaDram0_0MinSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
