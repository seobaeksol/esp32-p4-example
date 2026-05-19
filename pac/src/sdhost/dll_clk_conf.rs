#[doc = "Register `DLL_CLK_CONF` reader"]
pub type R = crate::R<DllClkConfSpec>;
#[doc = "Register `DLL_CLK_CONF` writer"]
pub type W = crate::W<DllClkConfSpec>;
#[doc = "Field `DLL_CCLK_IN_SLF_EN` reader - Clock enable of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1."]
pub type DllCclkInSlfEnR = crate::BitReader;
#[doc = "Field `DLL_CCLK_IN_SLF_EN` writer - Clock enable of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1."]
pub type DllCclkInSlfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLL_CCLK_IN_DRV_EN` reader - Clock enable of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1."]
pub type DllCclkInDrvEnR = crate::BitReader;
#[doc = "Field `DLL_CCLK_IN_DRV_EN` writer - Clock enable of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1."]
pub type DllCclkInDrvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLL_CCLK_IN_SAM_EN` reader - Clock enable of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1."]
pub type DllCclkInSamEnR = crate::BitReader;
#[doc = "Field `DLL_CCLK_IN_SAM_EN` writer - Clock enable of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1."]
pub type DllCclkInSamEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLL_CCLK_IN_SLF_PHASE` reader - It's used to control the phase of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1."]
pub type DllCclkInSlfPhaseR = crate::FieldReader;
#[doc = "Field `DLL_CCLK_IN_SLF_PHASE` writer - It's used to control the phase of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1."]
pub type DllCclkInSlfPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DLL_CCLK_IN_DRV_PHASE` reader - It's used to control the phase of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1."]
pub type DllCclkInDrvPhaseR = crate::FieldReader;
#[doc = "Field `DLL_CCLK_IN_DRV_PHASE` writer - It's used to control the phase of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1."]
pub type DllCclkInDrvPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DLL_CCLK_IN_SAM_PHASE` reader - It's used to control the phase of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1."]
pub type DllCclkInSamPhaseR = crate::FieldReader;
#[doc = "Field `DLL_CCLK_IN_SAM_PHASE` writer - It's used to control the phase of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1."]
pub type DllCclkInSamPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Clock enable of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_slf_en(&self) -> DllCclkInSlfEnR {
        DllCclkInSlfEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock enable of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_drv_en(&self) -> DllCclkInDrvEnR {
        DllCclkInDrvEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock enable of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_sam_en(&self) -> DllCclkInSamEnR {
        DllCclkInSamEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - It's used to control the phase of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_slf_phase(&self) -> DllCclkInSlfPhaseR {
        DllCclkInSlfPhaseR::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bits 9:14 - It's used to control the phase of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_drv_phase(&self) -> DllCclkInDrvPhaseR {
        DllCclkInDrvPhaseR::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bits 15:20 - It's used to control the phase of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_sam_phase(&self) -> DllCclkInSamPhaseR {
        DllCclkInSamPhaseR::new(((self.bits >> 15) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock enable of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_slf_en(&mut self) -> DllCclkInSlfEnW<'_, DllClkConfSpec> {
        DllCclkInSlfEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock enable of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_drv_en(&mut self) -> DllCclkInDrvEnW<'_, DllClkConfSpec> {
        DllCclkInDrvEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Clock enable of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_sam_en(&mut self) -> DllCclkInSamEnW<'_, DllClkConfSpec> {
        DllCclkInSamEnW::new(self, 2)
    }
    #[doc = "Bits 3:8 - It's used to control the phase of cclk_in_slf when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_slf_phase(&mut self) -> DllCclkInSlfPhaseW<'_, DllClkConfSpec> {
        DllCclkInSlfPhaseW::new(self, 3)
    }
    #[doc = "Bits 9:14 - It's used to control the phase of cclk_in_drv when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_drv_phase(&mut self) -> DllCclkInDrvPhaseW<'_, DllClkConfSpec> {
        DllCclkInDrvPhaseW::new(self, 9)
    }
    #[doc = "Bits 15:20 - It's used to control the phase of cclk_in_sam when ULTRA_HIGH_SPEED_MODE==1."]
    #[inline(always)]
    pub fn dll_cclk_in_sam_phase(&mut self) -> DllCclkInSamPhaseW<'_, DllClkConfSpec> {
        DllCclkInSamPhaseW::new(self, 15)
    }
}
#[doc = "SDIO DLL clock control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dll_clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll_clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DllClkConfSpec;
impl crate::RegisterSpec for DllClkConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll_clk_conf::R`](R) reader structure"]
impl crate::Readable for DllClkConfSpec {}
#[doc = "`write(|w| ..)` method takes [`dll_clk_conf::W`](W) writer structure"]
impl crate::Writable for DllClkConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DLL_CLK_CONF to value 0"]
impl crate::Resettable for DllClkConfSpec {}
