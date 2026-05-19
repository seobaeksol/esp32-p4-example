#[doc = "Register `SHADOW_REG_CTRL` reader"]
pub type R = crate::R<ShadowRegCtrlSpec>;
#[doc = "Register `SHADOW_REG_CTRL` writer"]
pub type W = crate::W<ShadowRegCtrlSpec>;
#[doc = "Field `BLC_UPDATE` reader - Write 1 to update blc configuration register"]
pub type BlcUpdateR = crate::BitReader;
#[doc = "Field `BLC_UPDATE` writer - Write 1 to update blc configuration register"]
pub type BlcUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPC_UPDATE` reader - Write 1 to update dpc configuration register"]
pub type DpcUpdateR = crate::BitReader;
#[doc = "Field `DPC_UPDATE` writer - Write 1 to update dpc configuration register"]
pub type DpcUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF_UPDATE` reader - Write 1 to update bf configuration register"]
pub type BfUpdateR = crate::BitReader;
#[doc = "Field `BF_UPDATE` writer - Write 1 to update bf configuration register"]
pub type BfUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WBG_UPDATE` reader - Write 1 to update wbg configuration register"]
pub type WbgUpdateR = crate::BitReader;
#[doc = "Field `WBG_UPDATE` writer - Write 1 to update wbg configuration register"]
pub type WbgUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCM_UPDATE` reader - Write 1 to update ccm configuration register"]
pub type CcmUpdateR = crate::BitReader;
#[doc = "Field `CCM_UPDATE` writer - Write 1 to update ccm configuration register"]
pub type CcmUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHARP_UPDATE` reader - Write 1 to update sharp configuration register"]
pub type SharpUpdateR = crate::BitReader;
#[doc = "Field `SHARP_UPDATE` writer - Write 1 to update sharp configuration register"]
pub type SharpUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLOR_UPDATE` reader - Write 1 to update color configuration register"]
pub type ColorUpdateR = crate::BitReader;
#[doc = "Field `COLOR_UPDATE` writer - Write 1 to update color configuration register"]
pub type ColorUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHADOW_UPDATE_SEL` reader - Configures shadow register update type. 0: no shadow register. 1: update every vsyn. 2: update only the next vsync after write reg_xxx_update"]
pub type ShadowUpdateSelR = crate::FieldReader;
#[doc = "Field `SHADOW_UPDATE_SEL` writer - Configures shadow register update type. 0: no shadow register. 1: update every vsyn. 2: update only the next vsync after write reg_xxx_update"]
pub type ShadowUpdateSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Write 1 to update blc configuration register"]
    #[inline(always)]
    pub fn blc_update(&self) -> BlcUpdateR {
        BlcUpdateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to update dpc configuration register"]
    #[inline(always)]
    pub fn dpc_update(&self) -> DpcUpdateR {
        DpcUpdateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to update bf configuration register"]
    #[inline(always)]
    pub fn bf_update(&self) -> BfUpdateR {
        BfUpdateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to update wbg configuration register"]
    #[inline(always)]
    pub fn wbg_update(&self) -> WbgUpdateR {
        WbgUpdateR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to update ccm configuration register"]
    #[inline(always)]
    pub fn ccm_update(&self) -> CcmUpdateR {
        CcmUpdateR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Write 1 to update sharp configuration register"]
    #[inline(always)]
    pub fn sharp_update(&self) -> SharpUpdateR {
        SharpUpdateR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write 1 to update color configuration register"]
    #[inline(always)]
    pub fn color_update(&self) -> ColorUpdateR {
        ColorUpdateR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Configures shadow register update type. 0: no shadow register. 1: update every vsyn. 2: update only the next vsync after write reg_xxx_update"]
    #[inline(always)]
    pub fn shadow_update_sel(&self) -> ShadowUpdateSelR {
        ShadowUpdateSelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to update blc configuration register"]
    #[inline(always)]
    pub fn blc_update(&mut self) -> BlcUpdateW<'_, ShadowRegCtrlSpec> {
        BlcUpdateW::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to update dpc configuration register"]
    #[inline(always)]
    pub fn dpc_update(&mut self) -> DpcUpdateW<'_, ShadowRegCtrlSpec> {
        DpcUpdateW::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to update bf configuration register"]
    #[inline(always)]
    pub fn bf_update(&mut self) -> BfUpdateW<'_, ShadowRegCtrlSpec> {
        BfUpdateW::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to update wbg configuration register"]
    #[inline(always)]
    pub fn wbg_update(&mut self) -> WbgUpdateW<'_, ShadowRegCtrlSpec> {
        WbgUpdateW::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to update ccm configuration register"]
    #[inline(always)]
    pub fn ccm_update(&mut self) -> CcmUpdateW<'_, ShadowRegCtrlSpec> {
        CcmUpdateW::new(self, 4)
    }
    #[doc = "Bit 6 - Write 1 to update sharp configuration register"]
    #[inline(always)]
    pub fn sharp_update(&mut self) -> SharpUpdateW<'_, ShadowRegCtrlSpec> {
        SharpUpdateW::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 to update color configuration register"]
    #[inline(always)]
    pub fn color_update(&mut self) -> ColorUpdateW<'_, ShadowRegCtrlSpec> {
        ColorUpdateW::new(self, 7)
    }
    #[doc = "Bits 30:31 - Configures shadow register update type. 0: no shadow register. 1: update every vsyn. 2: update only the next vsync after write reg_xxx_update"]
    #[inline(always)]
    pub fn shadow_update_sel(&mut self) -> ShadowUpdateSelW<'_, ShadowRegCtrlSpec> {
        ShadowUpdateSelW::new(self, 30)
    }
}
#[doc = "shadow register ctrl register\n\nYou can [`read`](crate::Reg::read) this register and get [`shadow_reg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shadow_reg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShadowRegCtrlSpec;
impl crate::RegisterSpec for ShadowRegCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shadow_reg_ctrl::R`](R) reader structure"]
impl crate::Readable for ShadowRegCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`shadow_reg_ctrl::W`](W) writer structure"]
impl crate::Writable for ShadowRegCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHADOW_REG_CTRL to value 0x4000_0000"]
impl crate::Resettable for ShadowRegCtrlSpec {
    const RESET_VALUE: u32 = 0x4000_0000;
}
