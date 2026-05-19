#[doc = "Register `EVT_EN2` reader"]
pub type R = crate::R<EvtEn2Spec>;
#[doc = "Register `EVT_EN2` writer"]
pub type W = crate::W<EvtEn2Spec>;
#[doc = "Field `OP0_TEE1` reader - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op0Tee1R = crate::BitReader;
#[doc = "Field `OP0_TEE1` writer - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op0Tee1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP1_TEE1` reader - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op1Tee1R = crate::BitReader;
#[doc = "Field `OP1_TEE1` writer - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op1Tee1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP2_TEE1` reader - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op2Tee1R = crate::BitReader;
#[doc = "Field `OP2_TEE1` writer - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op2Tee1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP0_TEE2` reader - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op0Tee2R = crate::BitReader;
#[doc = "Field `OP0_TEE2` writer - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op0Tee2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP1_TEE2` reader - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op1Tee2R = crate::BitReader;
#[doc = "Field `OP1_TEE2` writer - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op1Tee2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP2_TEE2` reader - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op2Tee2R = crate::BitReader;
#[doc = "Field `OP2_TEE2` writer - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op2Tee2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op0_tee1(&self) -> Op0Tee1R {
        Op0Tee1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op1_tee1(&self) -> Op1Tee1R {
        Op1Tee1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op2_tee1(&self) -> Op2Tee1R {
        Op2Tee1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op0_tee2(&self) -> Op0Tee2R {
        Op0Tee2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op1_tee2(&self) -> Op1Tee2R {
        Op1Tee2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op2_tee2(&self) -> Op2Tee2R {
        Op2Tee2R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op0_tee1(&mut self) -> Op0Tee1W<'_, EvtEn2Spec> {
        Op0Tee1W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op1_tee1(&mut self) -> Op1Tee1W<'_, EvtEn2Spec> {
        Op1Tee1W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op2_tee1(&mut self) -> Op2Tee1W<'_, EvtEn2Spec> {
        Op2Tee1W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op0_tee2(&mut self) -> Op0Tee2W<'_, EvtEn2Spec> {
        Op0Tee2W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op1_tee2(&mut self) -> Op1Tee2W<'_, EvtEn2Spec> {
        Op1Tee2W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op2_tee2(&mut self) -> Op2Tee2W<'_, EvtEn2Spec> {
        Op2Tee2W::new(self, 5)
    }
}
#[doc = "Event enable register2\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_en2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_en2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtEn2Spec;
impl crate::RegisterSpec for EvtEn2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_en2::R`](R) reader structure"]
impl crate::Readable for EvtEn2Spec {}
#[doc = "`write(|w| ..)` method takes [`evt_en2::W`](W) writer structure"]
impl crate::Writable for EvtEn2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_EN2 to value 0"]
impl crate::Resettable for EvtEn2Spec {}
