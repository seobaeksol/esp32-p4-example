#[doc = "Register `VAD_PARAM3` reader"]
pub type R = crate::R<VadParam3Spec>;
#[doc = "Register `VAD_PARAM3` writer"]
pub type W = crate::W<VadParam3Spec>;
#[doc = "Field `PARAM_NOISE_SPE_UP0` reader - VAD parameter"]
pub type ParamNoiseSpeUp0R = crate::FieldReader<u16>;
#[doc = "Field `PARAM_NOISE_SPE_UP0` writer - VAD parameter"]
pub type ParamNoiseSpeUp0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PARAM_NOISE_SPE_UP1` reader - VAD parameter"]
pub type ParamNoiseSpeUp1R = crate::FieldReader<u16>;
#[doc = "Field `PARAM_NOISE_SPE_UP1` writer - VAD parameter"]
pub type ParamNoiseSpeUp1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_spe_up0(&self) -> ParamNoiseSpeUp0R {
        ParamNoiseSpeUp0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_spe_up1(&self) -> ParamNoiseSpeUp1R {
        ParamNoiseSpeUp1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_spe_up0(&mut self) -> ParamNoiseSpeUp0W<'_, VadParam3Spec> {
        ParamNoiseSpeUp0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_spe_up1(&mut self) -> ParamNoiseSpeUp1W<'_, VadParam3Spec> {
        ParamNoiseSpeUp1W::new(self, 16)
    }
}
#[doc = "I2S VAD Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_param3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_param3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VadParam3Spec;
impl crate::RegisterSpec for VadParam3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_param3::R`](R) reader structure"]
impl crate::Readable for VadParam3Spec {}
#[doc = "`write(|w| ..)` method takes [`vad_param3::W`](W) writer structure"]
impl crate::Writable for VadParam3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VAD_PARAM3 to value 0x7d71_7fdf"]
impl crate::Resettable for VadParam3Spec {
    const RESET_VALUE: u32 = 0x7d71_7fdf;
}
