#[doc = "Register `UPDATE_CFG` reader"]
pub type R = crate::R<UpdateCfgSpec>;
#[doc = "Register `UPDATE_CFG` writer"]
pub type W = crate::W<UpdateCfgSpec>;
#[doc = "Field `GLOBAL_UP_EN` reader - Configures whether or not to enable global update for all active registers in MCPWM module.\\\\0: Disable\\\\1: Enable"]
pub type GlobalUpEnR = crate::BitReader;
#[doc = "Field `GLOBAL_UP_EN` writer - Configures whether or not to enable global update for all active registers in MCPWM module.\\\\0: Disable\\\\1: Enable"]
pub type GlobalUpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLOBAL_FORCE_UP` reader - Configures the generation of global forced update for all active registers in MCPWM module. A toggle (software invert its value) will trigger a global forced update. Valid only when MCPWM_GLOBAL_UP_EN and MCPWM_OP0/1/2_UP_EN are both set to 1."]
pub type GlobalForceUpR = crate::BitReader;
#[doc = "Field `GLOBAL_FORCE_UP` writer - Configures the generation of global forced update for all active registers in MCPWM module. A toggle (software invert its value) will trigger a global forced update. Valid only when MCPWM_GLOBAL_UP_EN and MCPWM_OP0/1/2_UP_EN are both set to 1."]
pub type GlobalForceUpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP0_UP_EN` reader - Configures whether or not to enable update of active registers in PWM operator0. Valid only when PWM_GLOBAL_UP_EN is set to 1.\\\\0: Disable\\\\1: Enable"]
pub type Op0UpEnR = crate::BitReader;
#[doc = "Field `OP0_UP_EN` writer - Configures whether or not to enable update of active registers in PWM operator0. Valid only when PWM_GLOBAL_UP_EN is set to 1.\\\\0: Disable\\\\1: Enable"]
pub type Op0UpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP0_FORCE_UP` reader - Configures the generation of forced update for active registers in PWM operator0. A toggle (software invert its value) will trigger a forced update. Valid only when MCPWM_GLOBAL_UP_EN and MCPWM_OP0_UP_EN are both set to 1."]
pub type Op0ForceUpR = crate::BitReader;
#[doc = "Field `OP0_FORCE_UP` writer - Configures the generation of forced update for active registers in PWM operator0. A toggle (software invert its value) will trigger a forced update. Valid only when MCPWM_GLOBAL_UP_EN and MCPWM_OP0_UP_EN are both set to 1."]
pub type Op0ForceUpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP1_UP_EN` reader - Configures whether or not to enable update of active registers in PWM operator1. Valid only when PWM_GLOBAL_UP_EN is set to 1.\\\\0: Disable\\\\1: Enable"]
pub type Op1UpEnR = crate::BitReader;
#[doc = "Field `OP1_UP_EN` writer - Configures whether or not to enable update of active registers in PWM operator1. Valid only when PWM_GLOBAL_UP_EN is set to 1.\\\\0: Disable\\\\1: Enable"]
pub type Op1UpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP1_FORCE_UP` reader - Configures the generation of forced update for active registers in PWM operator1. A toggle (software invert its value) will trigger a forced update. Valid only when MCPWM_GLOBAL_UP_EN and MCPWM_OP1_UP_EN are both set to 1."]
pub type Op1ForceUpR = crate::BitReader;
#[doc = "Field `OP1_FORCE_UP` writer - Configures the generation of forced update for active registers in PWM operator1. A toggle (software invert its value) will trigger a forced update. Valid only when MCPWM_GLOBAL_UP_EN and MCPWM_OP1_UP_EN are both set to 1."]
pub type Op1ForceUpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP2_UP_EN` reader - Configures whether or not to enable update of active registers in PWM operator2. Valid only when PWM_GLOBAL_UP_EN is set to 1.\\\\0: Disable\\\\1: Enable"]
pub type Op2UpEnR = crate::BitReader;
#[doc = "Field `OP2_UP_EN` writer - Configures whether or not to enable update of active registers in PWM operator2. Valid only when PWM_GLOBAL_UP_EN is set to 1.\\\\0: Disable\\\\1: Enable"]
pub type Op2UpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP2_FORCE_UP` reader - Configures the generation of forced update for active registers in PWM operator2. A toggle (software invert its value) will trigger a forced update. Valid only when MCPWM_GLOBAL_UP_EN and MCPWM_OP2_UP_EN are both set to 1."]
pub type Op2ForceUpR = crate::BitReader;
#[doc = "Field `OP2_FORCE_UP` writer - Configures the generation of forced update for active registers in PWM operator2. A toggle (software invert its value) will trigger a forced update. Valid only when MCPWM_GLOBAL_UP_EN and MCPWM_OP2_UP_EN are both set to 1."]
pub type Op2ForceUpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable global update for all active registers in MCPWM module.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn global_up_en(&self) -> GlobalUpEnR {
        GlobalUpEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures the generation of global forced update for all active registers in MCPWM module. A toggle (software invert its value) will trigger a global forced update. Valid only when MCPWM_GLOBAL_UP_EN and MCPWM_OP0/1/2_UP_EN are both set to 1."]
    #[inline(always)]
    pub fn global_force_up(&self) -> GlobalForceUpR {
        GlobalForceUpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to enable update of active registers in PWM operator0. Valid only when PWM_GLOBAL_UP_EN is set to 1.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op0_up_en(&self) -> Op0UpEnR {
        Op0UpEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures the generation of forced update for active registers in PWM operator0. A toggle (software invert its value) will trigger a forced update. Valid only when MCPWM_GLOBAL_UP_EN and MCPWM_OP0_UP_EN are both set to 1."]
    #[inline(always)]
    pub fn op0_force_up(&self) -> Op0ForceUpR {
        Op0ForceUpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to enable update of active registers in PWM operator1. Valid only when PWM_GLOBAL_UP_EN is set to 1.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op1_up_en(&self) -> Op1UpEnR {
        Op1UpEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures the generation of forced update for active registers in PWM operator1. A toggle (software invert its value) will trigger a forced update. Valid only when MCPWM_GLOBAL_UP_EN and MCPWM_OP1_UP_EN are both set to 1."]
    #[inline(always)]
    pub fn op1_force_up(&self) -> Op1ForceUpR {
        Op1ForceUpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether or not to enable update of active registers in PWM operator2. Valid only when PWM_GLOBAL_UP_EN is set to 1.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op2_up_en(&self) -> Op2UpEnR {
        Op2UpEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures the generation of forced update for active registers in PWM operator2. A toggle (software invert its value) will trigger a forced update. Valid only when MCPWM_GLOBAL_UP_EN and MCPWM_OP2_UP_EN are both set to 1."]
    #[inline(always)]
    pub fn op2_force_up(&self) -> Op2ForceUpR {
        Op2ForceUpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable global update for all active registers in MCPWM module.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn global_up_en(&mut self) -> GlobalUpEnW<'_, UpdateCfgSpec> {
        GlobalUpEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Configures the generation of global forced update for all active registers in MCPWM module. A toggle (software invert its value) will trigger a global forced update. Valid only when MCPWM_GLOBAL_UP_EN and MCPWM_OP0/1/2_UP_EN are both set to 1."]
    #[inline(always)]
    pub fn global_force_up(&mut self) -> GlobalForceUpW<'_, UpdateCfgSpec> {
        GlobalForceUpW::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to enable update of active registers in PWM operator0. Valid only when PWM_GLOBAL_UP_EN is set to 1.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op0_up_en(&mut self) -> Op0UpEnW<'_, UpdateCfgSpec> {
        Op0UpEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Configures the generation of forced update for active registers in PWM operator0. A toggle (software invert its value) will trigger a forced update. Valid only when MCPWM_GLOBAL_UP_EN and MCPWM_OP0_UP_EN are both set to 1."]
    #[inline(always)]
    pub fn op0_force_up(&mut self) -> Op0ForceUpW<'_, UpdateCfgSpec> {
        Op0ForceUpW::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to enable update of active registers in PWM operator1. Valid only when PWM_GLOBAL_UP_EN is set to 1.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op1_up_en(&mut self) -> Op1UpEnW<'_, UpdateCfgSpec> {
        Op1UpEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Configures the generation of forced update for active registers in PWM operator1. A toggle (software invert its value) will trigger a forced update. Valid only when MCPWM_GLOBAL_UP_EN and MCPWM_OP1_UP_EN are both set to 1."]
    #[inline(always)]
    pub fn op1_force_up(&mut self) -> Op1ForceUpW<'_, UpdateCfgSpec> {
        Op1ForceUpW::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to enable update of active registers in PWM operator2. Valid only when PWM_GLOBAL_UP_EN is set to 1.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op2_up_en(&mut self) -> Op2UpEnW<'_, UpdateCfgSpec> {
        Op2UpEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Configures the generation of forced update for active registers in PWM operator2. A toggle (software invert its value) will trigger a forced update. Valid only when MCPWM_GLOBAL_UP_EN and MCPWM_OP2_UP_EN are both set to 1."]
    #[inline(always)]
    pub fn op2_force_up(&mut self) -> Op2ForceUpW<'_, UpdateCfgSpec> {
        Op2ForceUpW::new(self, 7)
    }
}
#[doc = "Generator Update configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`update_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`update_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UpdateCfgSpec;
impl crate::RegisterSpec for UpdateCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`update_cfg::R`](R) reader structure"]
impl crate::Readable for UpdateCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`update_cfg::W`](W) writer structure"]
impl crate::Writable for UpdateCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UPDATE_CFG to value 0x05"]
impl crate::Resettable for UpdateCfgSpec {
    const RESET_VALUE: u32 = 0x05;
}
