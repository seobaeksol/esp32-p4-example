#[doc = "Register `AF_ENV_USER_TH_SUM` reader"]
pub type R = crate::R<AfEnvUserThSumSpec>;
#[doc = "Register `AF_ENV_USER_TH_SUM` writer"]
pub type W = crate::W<AfEnvUserThSumSpec>;
#[doc = "Field `AF_ENV_USER_THRESHOLD_SUM` reader - this field configures user setup env detect sum threshold"]
pub type AfEnvUserThresholdSumR = crate::FieldReader<u32>;
#[doc = "Field `AF_ENV_USER_THRESHOLD_SUM` writer - this field configures user setup env detect sum threshold"]
pub type AfEnvUserThresholdSumW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - this field configures user setup env detect sum threshold"]
    #[inline(always)]
    pub fn af_env_user_threshold_sum(&self) -> AfEnvUserThresholdSumR {
        AfEnvUserThresholdSumR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - this field configures user setup env detect sum threshold"]
    #[inline(always)]
    pub fn af_env_user_threshold_sum(&mut self) -> AfEnvUserThresholdSumW<'_, AfEnvUserThSumSpec> {
        AfEnvUserThresholdSumW::new(self, 0)
    }
}
#[doc = "af monitor user sum threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_env_user_th_sum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_env_user_th_sum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfEnvUserThSumSpec;
impl crate::RegisterSpec for AfEnvUserThSumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_env_user_th_sum::R`](R) reader structure"]
impl crate::Readable for AfEnvUserThSumSpec {}
#[doc = "`write(|w| ..)` method takes [`af_env_user_th_sum::W`](W) writer structure"]
impl crate::Writable for AfEnvUserThSumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AF_ENV_USER_TH_SUM to value 0"]
impl crate::Resettable for AfEnvUserThSumSpec {}
