#[doc = "Register `CAP_CH%s_CFG` reader"]
pub type R = crate::R<CapChCfgSpec>;
#[doc = "Register `CAP_CH%s_CFG` writer"]
pub type W = crate::W<CapChCfgSpec>;
#[doc = "Field `EN` reader - Configures whether or not to enable capture on channel %s.\\\\0: Disable\\\\1: Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Configures whether or not to enable capture on channel %s.\\\\0: Disable\\\\1: Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - Configures which edge of capture on channel %s after prescaling is used.\\\\0: None\\\\Bit0 is set to 1: Rnable capture on the negative edge\\\\Bit1 is set to 1: Enable capture on the positive edge"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - Configures which edge of capture on channel %s after prescaling is used.\\\\0: None\\\\Bit0 is set to 1: Rnable capture on the negative edge\\\\Bit1 is set to 1: Enable capture on the positive edge"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRESCALE` reader - Configures prescale value on possitive edge of CAP%s. Prescale value = PWM_CAP%s_PRESCALE + 1"]
pub type PrescaleR = crate::FieldReader;
#[doc = "Field `PRESCALE` writer - Configures prescale value on possitive edge of CAP%s. Prescale value = PWM_CAP%s_PRESCALE + 1"]
pub type PrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IN_INVERT` reader - Configures whether or not to invert CAP%s from GPIO matrix before prescale.\\\\0: Normal\\\\1: Invert"]
pub type InInvertR = crate::BitReader;
#[doc = "Field `IN_INVERT` writer - Configures whether or not to invert CAP%s from GPIO matrix before prescale.\\\\0: Normal\\\\1: Invert"]
pub type InInvertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW` writer - Configures the generation of software capture.\\\\0: Invalid, No effect\\\\1: Trigger a software forced capture on channel %s"]
pub type SwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable capture on channel %s.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Configures which edge of capture on channel %s after prescaling is used.\\\\0: None\\\\Bit0 is set to 1: Rnable capture on the negative edge\\\\Bit1 is set to 1: Enable capture on the positive edge"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:10 - Configures prescale value on possitive edge of CAP%s. Prescale value = PWM_CAP%s_PRESCALE + 1"]
    #[inline(always)]
    pub fn prescale(&self) -> PrescaleR {
        PrescaleR::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - Configures whether or not to invert CAP%s from GPIO matrix before prescale.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    pub fn in_invert(&self) -> InInvertR {
        InInvertR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable capture on channel %s.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CapChCfgSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Configures which edge of capture on channel %s after prescaling is used.\\\\0: None\\\\Bit0 is set to 1: Rnable capture on the negative edge\\\\Bit1 is set to 1: Enable capture on the positive edge"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CapChCfgSpec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bits 3:10 - Configures prescale value on possitive edge of CAP%s. Prescale value = PWM_CAP%s_PRESCALE + 1"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PrescaleW<'_, CapChCfgSpec> {
        PrescaleW::new(self, 3)
    }
    #[doc = "Bit 11 - Configures whether or not to invert CAP%s from GPIO matrix before prescale.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    pub fn in_invert(&mut self) -> InInvertW<'_, CapChCfgSpec> {
        InInvertW::new(self, 11)
    }
    #[doc = "Bit 12 - Configures the generation of software capture.\\\\0: Invalid, No effect\\\\1: Trigger a software forced capture on channel %s"]
    #[inline(always)]
    pub fn sw(&mut self) -> SwW<'_, CapChCfgSpec> {
        SwW::new(self, 12)
    }
}
#[doc = "Capture channel %s configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_ch_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_ch_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapChCfgSpec;
impl crate::RegisterSpec for CapChCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_ch_cfg::R`](R) reader structure"]
impl crate::Readable for CapChCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cap_ch_cfg::W`](W) writer structure"]
impl crate::Writable for CapChCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAP_CH%s_CFG to value 0"]
impl crate::Resettable for CapChCfgSpec {}
