#[doc = "Register `HPCORE_WDT_RESET_SOURCE0` reader"]
pub type R = crate::R<HpcoreWdtResetSource0Spec>;
#[doc = "Register `HPCORE_WDT_RESET_SOURCE0` writer"]
pub type W = crate::W<HpcoreWdtResetSource0Spec>;
#[doc = "Field `HPCORE0_WDT_RESET_SOURCE_SEL` reader - 1'b0: use wdt0 to reset hpcore0, 1'b1: use wdt1 to reset hpcore0"]
pub type Hpcore0WdtResetSourceSelR = crate::BitReader;
#[doc = "Field `HPCORE0_WDT_RESET_SOURCE_SEL` writer - 1'b0: use wdt0 to reset hpcore0, 1'b1: use wdt1 to reset hpcore0"]
pub type Hpcore0WdtResetSourceSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPCORE1_WDT_RESET_SOURCE_SEL` reader - 1'b0: use wdt0 to reset hpcore1, 1'b1: use wdt1 to reset hpcore1"]
pub type Hpcore1WdtResetSourceSelR = crate::BitReader;
#[doc = "Field `HPCORE1_WDT_RESET_SOURCE_SEL` writer - 1'b0: use wdt0 to reset hpcore1, 1'b1: use wdt1 to reset hpcore1"]
pub type Hpcore1WdtResetSourceSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1'b0: use wdt0 to reset hpcore0, 1'b1: use wdt1 to reset hpcore0"]
    #[inline(always)]
    pub fn hpcore0_wdt_reset_source_sel(&self) -> Hpcore0WdtResetSourceSelR {
        Hpcore0WdtResetSourceSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1'b0: use wdt0 to reset hpcore1, 1'b1: use wdt1 to reset hpcore1"]
    #[inline(always)]
    pub fn hpcore1_wdt_reset_source_sel(&self) -> Hpcore1WdtResetSourceSelR {
        Hpcore1WdtResetSourceSelR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1'b0: use wdt0 to reset hpcore0, 1'b1: use wdt1 to reset hpcore0"]
    #[inline(always)]
    pub fn hpcore0_wdt_reset_source_sel(
        &mut self,
    ) -> Hpcore0WdtResetSourceSelW<'_, HpcoreWdtResetSource0Spec> {
        Hpcore0WdtResetSourceSelW::new(self, 0)
    }
    #[doc = "Bit 1 - 1'b0: use wdt0 to reset hpcore1, 1'b1: use wdt1 to reset hpcore1"]
    #[inline(always)]
    pub fn hpcore1_wdt_reset_source_sel(
        &mut self,
    ) -> Hpcore1WdtResetSourceSelW<'_, HpcoreWdtResetSource0Spec> {
        Hpcore1WdtResetSourceSelW::new(self, 1)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcore_wdt_reset_source0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcore_wdt_reset_source0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpcoreWdtResetSource0Spec;
impl crate::RegisterSpec for HpcoreWdtResetSource0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpcore_wdt_reset_source0::R`](R) reader structure"]
impl crate::Readable for HpcoreWdtResetSource0Spec {}
#[doc = "`write(|w| ..)` method takes [`hpcore_wdt_reset_source0::W`](W) writer structure"]
impl crate::Writable for HpcoreWdtResetSource0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HPCORE_WDT_RESET_SOURCE0 to value 0x02"]
impl crate::Resettable for HpcoreWdtResetSource0Spec {
    const RESET_VALUE: u32 = 0x02;
}
