#[doc = "Register `PMUP_DRV_CFG` reader"]
pub type R = crate::R<PmupDrvCfgSpec>;
#[doc = "Register `PMUP_DRV_CFG` writer"]
pub type W = crate::W<PmupDrvCfgSpec>;
#[doc = "Field `BYPASS_EFUSE_CTRL` reader - needs desc"]
pub type BypassEfuseCtrlR = crate::BitReader;
#[doc = "Field `BYPASS_EFUSE_CTRL` writer - needs desc"]
pub type BypassEfuseCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUMP_EN` reader - configure pvt charge xpd"]
pub type PumpEnR = crate::BitReader;
#[doc = "Field `PUMP_EN` writer - configure pvt charge xpd"]
pub type PumpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - force register clken"]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - force register clken"]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUMP_DRV4` reader - configure cmd4 drv"]
pub type PumpDrv4R = crate::FieldReader;
#[doc = "Field `PUMP_DRV4` writer - configure cmd4 drv"]
pub type PumpDrv4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PUMP_DRV3` reader - configure cmd3 drv"]
pub type PumpDrv3R = crate::FieldReader;
#[doc = "Field `PUMP_DRV3` writer - configure cmd3 drv"]
pub type PumpDrv3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PUMP_DRV2` reader - configure cmd2 drv"]
pub type PumpDrv2R = crate::FieldReader;
#[doc = "Field `PUMP_DRV2` writer - configure cmd2 drv"]
pub type PumpDrv2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PUMP_DRV1` reader - configure cmd1 drv"]
pub type PumpDrv1R = crate::FieldReader;
#[doc = "Field `PUMP_DRV1` writer - configure cmd1 drv"]
pub type PumpDrv1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PUMP_DRV0` reader - configure cmd0 drv"]
pub type PumpDrv0R = crate::FieldReader;
#[doc = "Field `PUMP_DRV0` writer - configure cmd0 drv"]
pub type PumpDrv0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 8 - needs desc"]
    #[inline(always)]
    pub fn bypass_efuse_ctrl(&self) -> BypassEfuseCtrlR {
        BypassEfuseCtrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - configure pvt charge xpd"]
    #[inline(always)]
    pub fn pump_en(&self) -> PumpEnR {
        PumpEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - force register clken"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - configure cmd4 drv"]
    #[inline(always)]
    pub fn pump_drv4(&self) -> PumpDrv4R {
        PumpDrv4R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 15:18 - configure cmd3 drv"]
    #[inline(always)]
    pub fn pump_drv3(&self) -> PumpDrv3R {
        PumpDrv3R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:22 - configure cmd2 drv"]
    #[inline(always)]
    pub fn pump_drv2(&self) -> PumpDrv2R {
        PumpDrv2R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 23:26 - configure cmd1 drv"]
    #[inline(always)]
    pub fn pump_drv1(&self) -> PumpDrv1R {
        PumpDrv1R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 27:30 - configure cmd0 drv"]
    #[inline(always)]
    pub fn pump_drv0(&self) -> PumpDrv0R {
        PumpDrv0R::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - needs desc"]
    #[inline(always)]
    pub fn bypass_efuse_ctrl(&mut self) -> BypassEfuseCtrlW<'_, PmupDrvCfgSpec> {
        BypassEfuseCtrlW::new(self, 8)
    }
    #[doc = "Bit 9 - configure pvt charge xpd"]
    #[inline(always)]
    pub fn pump_en(&mut self) -> PumpEnW<'_, PmupDrvCfgSpec> {
        PumpEnW::new(self, 9)
    }
    #[doc = "Bit 10 - force register clken"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, PmupDrvCfgSpec> {
        ClkEnW::new(self, 10)
    }
    #[doc = "Bits 11:14 - configure cmd4 drv"]
    #[inline(always)]
    pub fn pump_drv4(&mut self) -> PumpDrv4W<'_, PmupDrvCfgSpec> {
        PumpDrv4W::new(self, 11)
    }
    #[doc = "Bits 15:18 - configure cmd3 drv"]
    #[inline(always)]
    pub fn pump_drv3(&mut self) -> PumpDrv3W<'_, PmupDrvCfgSpec> {
        PumpDrv3W::new(self, 15)
    }
    #[doc = "Bits 19:22 - configure cmd2 drv"]
    #[inline(always)]
    pub fn pump_drv2(&mut self) -> PumpDrv2W<'_, PmupDrvCfgSpec> {
        PumpDrv2W::new(self, 19)
    }
    #[doc = "Bits 23:26 - configure cmd1 drv"]
    #[inline(always)]
    pub fn pump_drv1(&mut self) -> PumpDrv1W<'_, PmupDrvCfgSpec> {
        PumpDrv1W::new(self, 23)
    }
    #[doc = "Bits 27:30 - configure cmd0 drv"]
    #[inline(always)]
    pub fn pump_drv0(&mut self) -> PumpDrv0W<'_, PmupDrvCfgSpec> {
        PumpDrv0W::new(self, 27)
    }
}
#[doc = "configure pump drv\n\nYou can [`read`](crate::Reg::read) this register and get [`pmup_drv_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmup_drv_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmupDrvCfgSpec;
impl crate::RegisterSpec for PmupDrvCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmup_drv_cfg::R`](R) reader structure"]
impl crate::Readable for PmupDrvCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pmup_drv_cfg::W`](W) writer structure"]
impl crate::Writable for PmupDrvCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMUP_DRV_CFG to value 0x0100"]
impl crate::Resettable for PmupDrvCfgSpec {
    const RESET_VALUE: u32 = 0x0100;
}
