#[doc = "Register `AMP_CTRL2` reader"]
pub type R = crate::R<AmpCtrl2Spec>;
#[doc = "Register `AMP_CTRL2` writer"]
pub type W = crate::W<AmpCtrl2Spec>;
#[doc = "Field `SAR1_DAC_XPD_FSM_IDLE` reader - N/A"]
pub type Sar1DacXpdFsmIdleR = crate::BitReader;
#[doc = "Field `SAR1_DAC_XPD_FSM_IDLE` writer - N/A"]
pub type Sar1DacXpdFsmIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_SAR_AMP_FSM_IDLE` reader - N/A"]
pub type XpdSarAmpFsmIdleR = crate::BitReader;
#[doc = "Field `XPD_SAR_AMP_FSM_IDLE` writer - N/A"]
pub type XpdSarAmpFsmIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMP_RST_FB_FSM_IDLE` reader - N/A"]
pub type AmpRstFbFsmIdleR = crate::BitReader;
#[doc = "Field `AMP_RST_FB_FSM_IDLE` writer - N/A"]
pub type AmpRstFbFsmIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMP_SHORT_REF_FSM_IDLE` reader - N/A"]
pub type AmpShortRefFsmIdleR = crate::BitReader;
#[doc = "Field `AMP_SHORT_REF_FSM_IDLE` writer - N/A"]
pub type AmpShortRefFsmIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMP_SHORT_REF_GND_FSM_IDLE` reader - N/A"]
pub type AmpShortRefGndFsmIdleR = crate::BitReader;
#[doc = "Field `AMP_SHORT_REF_GND_FSM_IDLE` writer - N/A"]
pub type AmpShortRefGndFsmIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_SAR_FSM_IDLE` reader - N/A"]
pub type XpdSarFsmIdleR = crate::BitReader;
#[doc = "Field `XPD_SAR_FSM_IDLE` writer - N/A"]
pub type XpdSarFsmIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_RSTB_FSM_IDLE` reader - N/A"]
pub type SarRstbFsmIdleR = crate::BitReader;
#[doc = "Field `SAR_RSTB_FSM_IDLE` writer - N/A"]
pub type SarRstbFsmIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_AMP_WAIT3` reader - N/A"]
pub type SarAmpWait3R = crate::FieldReader<u16>;
#[doc = "Field `SAR_AMP_WAIT3` writer - N/A"]
pub type SarAmpWait3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm_idle(&self) -> Sar1DacXpdFsmIdleR {
        Sar1DacXpdFsmIdleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm_idle(&self) -> XpdSarAmpFsmIdleR {
        XpdSarAmpFsmIdleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm_idle(&self) -> AmpRstFbFsmIdleR {
        AmpRstFbFsmIdleR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn amp_short_ref_fsm_idle(&self) -> AmpShortRefFsmIdleR {
        AmpShortRefFsmIdleR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm_idle(&self) -> AmpShortRefGndFsmIdleR {
        AmpShortRefGndFsmIdleR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn xpd_sar_fsm_idle(&self) -> XpdSarFsmIdleR {
        XpdSarFsmIdleR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&self) -> SarRstbFsmIdleR {
        SarRstbFsmIdleR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:31 - N/A"]
    #[inline(always)]
    pub fn sar_amp_wait3(&self) -> SarAmpWait3R {
        SarAmpWait3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm_idle(&mut self) -> Sar1DacXpdFsmIdleW<'_, AmpCtrl2Spec> {
        Sar1DacXpdFsmIdleW::new(self, 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm_idle(&mut self) -> XpdSarAmpFsmIdleW<'_, AmpCtrl2Spec> {
        XpdSarAmpFsmIdleW::new(self, 1)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm_idle(&mut self) -> AmpRstFbFsmIdleW<'_, AmpCtrl2Spec> {
        AmpRstFbFsmIdleW::new(self, 2)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn amp_short_ref_fsm_idle(&mut self) -> AmpShortRefFsmIdleW<'_, AmpCtrl2Spec> {
        AmpShortRefFsmIdleW::new(self, 3)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm_idle(&mut self) -> AmpShortRefGndFsmIdleW<'_, AmpCtrl2Spec> {
        AmpShortRefGndFsmIdleW::new(self, 4)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn xpd_sar_fsm_idle(&mut self) -> XpdSarFsmIdleW<'_, AmpCtrl2Spec> {
        XpdSarFsmIdleW::new(self, 5)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&mut self) -> SarRstbFsmIdleW<'_, AmpCtrl2Spec> {
        SarRstbFsmIdleW::new(self, 6)
    }
    #[doc = "Bits 16:31 - N/A"]
    #[inline(always)]
    pub fn sar_amp_wait3(&mut self) -> SarAmpWait3W<'_, AmpCtrl2Spec> {
        SarAmpWait3W::new(self, 16)
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`amp_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amp_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmpCtrl2Spec;
impl crate::RegisterSpec for AmpCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amp_ctrl2::R`](R) reader structure"]
impl crate::Readable for AmpCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`amp_ctrl2::W`](W) writer structure"]
impl crate::Writable for AmpCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AMP_CTRL2 to value 0x000a_0000"]
impl crate::Resettable for AmpCtrl2Spec {
    const RESET_VALUE: u32 = 0x000a_0000;
}
