#[doc = "Register `U3_CHANGE_CONF` reader"]
pub type R = crate::R<U3ChangeConfSpec>;
#[doc = "Register `U3_CHANGE_CONF` writer"]
pub type W = crate::W<U3ChangeConfSpec>;
#[doc = "Field `CNT_STEP_U3` reader - Configures the step value for unit 3."]
pub type CntStepU3R = crate::FieldReader<u16>;
#[doc = "Field `CNT_STEP_U3` writer - Configures the step value for unit 3."]
pub type CntStepU3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CNT_STEP_LIM_U3` reader - Configures the step limit value for unit 3."]
pub type CntStepLimU3R = crate::FieldReader<u16>;
#[doc = "Field `CNT_STEP_LIM_U3` writer - Configures the step limit value for unit 3."]
pub type CntStepLimU3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures the step value for unit 3."]
    #[inline(always)]
    pub fn cnt_step_u3(&self) -> CntStepU3R {
        CntStepU3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Configures the step limit value for unit 3."]
    #[inline(always)]
    pub fn cnt_step_lim_u3(&self) -> CntStepLimU3R {
        CntStepLimU3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures the step value for unit 3."]
    #[inline(always)]
    pub fn cnt_step_u3(&mut self) -> CntStepU3W<'_, U3ChangeConfSpec> {
        CntStepU3W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Configures the step limit value for unit 3."]
    #[inline(always)]
    pub fn cnt_step_lim_u3(&mut self) -> CntStepLimU3W<'_, U3ChangeConfSpec> {
        CntStepLimU3W::new(self, 16)
    }
}
#[doc = "Configuration register for unit $n's step value.\n\nYou can [`read`](crate::Reg::read) this register and get [`u3_change_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u3_change_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U3ChangeConfSpec;
impl crate::RegisterSpec for U3ChangeConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`u3_change_conf::R`](R) reader structure"]
impl crate::Readable for U3ChangeConfSpec {}
#[doc = "`write(|w| ..)` method takes [`u3_change_conf::W`](W) writer structure"]
impl crate::Writable for U3ChangeConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets U3_CHANGE_CONF to value 0"]
impl crate::Resettable for U3ChangeConfSpec {}
