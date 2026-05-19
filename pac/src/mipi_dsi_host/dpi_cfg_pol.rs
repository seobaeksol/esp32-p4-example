#[doc = "Register `DPI_CFG_POL` reader"]
pub type R = crate::R<DpiCfgPolSpec>;
#[doc = "Register `DPI_CFG_POL` writer"]
pub type W = crate::W<DpiCfgPolSpec>;
#[doc = "Field `DATAEN_ACTIVE_LOW` reader - NA"]
pub type DataenActiveLowR = crate::BitReader;
#[doc = "Field `DATAEN_ACTIVE_LOW` writer - NA"]
pub type DataenActiveLowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSYNC_ACTIVE_LOW` reader - NA"]
pub type VsyncActiveLowR = crate::BitReader;
#[doc = "Field `VSYNC_ACTIVE_LOW` writer - NA"]
pub type VsyncActiveLowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSYNC_ACTIVE_LOW` reader - NA"]
pub type HsyncActiveLowR = crate::BitReader;
#[doc = "Field `HSYNC_ACTIVE_LOW` writer - NA"]
pub type HsyncActiveLowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHUTD_ACTIVE_LOW` reader - NA"]
pub type ShutdActiveLowR = crate::BitReader;
#[doc = "Field `SHUTD_ACTIVE_LOW` writer - NA"]
pub type ShutdActiveLowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLORM_ACTIVE_LOW` reader - NA"]
pub type ColormActiveLowR = crate::BitReader;
#[doc = "Field `COLORM_ACTIVE_LOW` writer - NA"]
pub type ColormActiveLowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn dataen_active_low(&self) -> DataenActiveLowR {
        DataenActiveLowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn vsync_active_low(&self) -> VsyncActiveLowR {
        VsyncActiveLowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn hsync_active_low(&self) -> HsyncActiveLowR {
        HsyncActiveLowR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn shutd_active_low(&self) -> ShutdActiveLowR {
        ShutdActiveLowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn colorm_active_low(&self) -> ColormActiveLowR {
        ColormActiveLowR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn dataen_active_low(&mut self) -> DataenActiveLowW<'_, DpiCfgPolSpec> {
        DataenActiveLowW::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn vsync_active_low(&mut self) -> VsyncActiveLowW<'_, DpiCfgPolSpec> {
        VsyncActiveLowW::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn hsync_active_low(&mut self) -> HsyncActiveLowW<'_, DpiCfgPolSpec> {
        HsyncActiveLowW::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn shutd_active_low(&mut self) -> ShutdActiveLowW<'_, DpiCfgPolSpec> {
        ShutdActiveLowW::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn colorm_active_low(&mut self) -> ColormActiveLowW<'_, DpiCfgPolSpec> {
        ColormActiveLowW::new(self, 4)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_cfg_pol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_cfg_pol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiCfgPolSpec;
impl crate::RegisterSpec for DpiCfgPolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_cfg_pol::R`](R) reader structure"]
impl crate::Readable for DpiCfgPolSpec {}
#[doc = "`write(|w| ..)` method takes [`dpi_cfg_pol::W`](W) writer structure"]
impl crate::Writable for DpiCfgPolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPI_CFG_POL to value 0"]
impl crate::Resettable for DpiCfgPolSpec {}
