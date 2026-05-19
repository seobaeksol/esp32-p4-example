#[doc = "Register `AF_ENV_USER_TH_LUM` reader"]
pub type R = crate::R<AfEnvUserThLumSpec>;
#[doc = "Register `AF_ENV_USER_TH_LUM` writer"]
pub type W = crate::W<AfEnvUserThLumSpec>;
#[doc = "Field `AF_ENV_USER_THRESHOLD_LUM` reader - this field configures user setup env detect lum threshold"]
pub type AfEnvUserThresholdLumR = crate::FieldReader<u32>;
#[doc = "Field `AF_ENV_USER_THRESHOLD_LUM` writer - this field configures user setup env detect lum threshold"]
pub type AfEnvUserThresholdLumW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - this field configures user setup env detect lum threshold"]
    #[inline(always)]
    pub fn af_env_user_threshold_lum(&self) -> AfEnvUserThresholdLumR {
        AfEnvUserThresholdLumR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - this field configures user setup env detect lum threshold"]
    #[inline(always)]
    pub fn af_env_user_threshold_lum(&mut self) -> AfEnvUserThresholdLumW<'_, AfEnvUserThLumSpec> {
        AfEnvUserThresholdLumW::new(self, 0)
    }
}
#[doc = "af monitor user lum threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_env_user_th_lum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_env_user_th_lum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfEnvUserThLumSpec;
impl crate::RegisterSpec for AfEnvUserThLumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_env_user_th_lum::R`](R) reader structure"]
impl crate::Readable for AfEnvUserThLumSpec {}
#[doc = "`write(|w| ..)` method takes [`af_env_user_th_lum::W`](W) writer structure"]
impl crate::Writable for AfEnvUserThLumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AF_ENV_USER_TH_LUM to value 0"]
impl crate::Resettable for AfEnvUserThLumSpec {}
