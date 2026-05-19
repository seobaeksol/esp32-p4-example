#[doc = "Register `CARRIER_CFG` reader"]
pub type R = crate::R<CarrierCfgSpec>;
#[doc = "Register `CARRIER_CFG` writer"]
pub type W = crate::W<CarrierCfgSpec>;
#[doc = "Field `EN` reader - Configures whether or not to enable carrier%s.\\\\0: Bypassed\\\\1: Enabled"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Configures whether or not to enable carrier%s.\\\\0: Bypassed\\\\1: Enabled"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESCALE` reader - Configures the prescale value of PWM carrier%s clock (PC_clk), so that period of PC_clk = period of PWM_clk * (PWM_CARRIER%s_PRESCALE + 1)"]
pub type PrescaleR = crate::FieldReader;
#[doc = "Field `PRESCALE` writer - Configures the prescale value of PWM carrier%s clock (PC_clk), so that period of PC_clk = period of PWM_clk * (PWM_CARRIER%s_PRESCALE + 1)"]
pub type PrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DUTY` reader - Configures carrier duty. Duty = PWM_CARRIER%s_DUTY / 8"]
pub type DutyR = crate::FieldReader;
#[doc = "Field `DUTY` writer - Configures carrier duty. Duty = PWM_CARRIER%s_DUTY / 8"]
pub type DutyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OSHTWTH` reader - Configures width of the first pulse. Measurement unit: Periods of the carrier."]
pub type OshtwthR = crate::FieldReader;
#[doc = "Field `OSHTWTH` writer - Configures width of the first pulse. Measurement unit: Periods of the carrier."]
pub type OshtwthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OUT_INVERT` reader - Configures whether or not to invert the output of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
pub type OutInvertR = crate::BitReader;
#[doc = "Field `OUT_INVERT` writer - Configures whether or not to invert the output of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
pub type OutInvertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_INVERT` reader - Configures whether or not to invert the input of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
pub type InInvertR = crate::BitReader;
#[doc = "Field `IN_INVERT` writer - Configures whether or not to invert the input of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
pub type InInvertW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable carrier%s.\\\\0: Bypassed\\\\1: Enabled"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Configures the prescale value of PWM carrier%s clock (PC_clk), so that period of PC_clk = period of PWM_clk * (PWM_CARRIER%s_PRESCALE + 1)"]
    #[inline(always)]
    pub fn prescale(&self) -> PrescaleR {
        PrescaleR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - Configures carrier duty. Duty = PWM_CARRIER%s_DUTY / 8"]
    #[inline(always)]
    pub fn duty(&self) -> DutyR {
        DutyR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Configures width of the first pulse. Measurement unit: Periods of the carrier."]
    #[inline(always)]
    pub fn oshtwth(&self) -> OshtwthR {
        OshtwthR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Configures whether or not to invert the output of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    pub fn out_invert(&self) -> OutInvertR {
        OutInvertR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures whether or not to invert the input of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    pub fn in_invert(&self) -> InInvertR {
        InInvertR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable carrier%s.\\\\0: Bypassed\\\\1: Enabled"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CarrierCfgSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:4 - Configures the prescale value of PWM carrier%s clock (PC_clk), so that period of PC_clk = period of PWM_clk * (PWM_CARRIER%s_PRESCALE + 1)"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PrescaleW<'_, CarrierCfgSpec> {
        PrescaleW::new(self, 1)
    }
    #[doc = "Bits 5:7 - Configures carrier duty. Duty = PWM_CARRIER%s_DUTY / 8"]
    #[inline(always)]
    pub fn duty(&mut self) -> DutyW<'_, CarrierCfgSpec> {
        DutyW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Configures width of the first pulse. Measurement unit: Periods of the carrier."]
    #[inline(always)]
    pub fn oshtwth(&mut self) -> OshtwthW<'_, CarrierCfgSpec> {
        OshtwthW::new(self, 8)
    }
    #[doc = "Bit 12 - Configures whether or not to invert the output of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    pub fn out_invert(&mut self) -> OutInvertW<'_, CarrierCfgSpec> {
        OutInvertW::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to invert the input of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    pub fn in_invert(&mut self) -> InInvertW<'_, CarrierCfgSpec> {
        InInvertW::new(self, 13)
    }
}
#[doc = "Carrier0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`carrier_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`carrier_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CarrierCfgSpec;
impl crate::RegisterSpec for CarrierCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`carrier_cfg::R`](R) reader structure"]
impl crate::Readable for CarrierCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`carrier_cfg::W`](W) writer structure"]
impl crate::Writable for CarrierCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CARRIER_CFG to value 0"]
impl crate::Resettable for CarrierCfgSpec {}
