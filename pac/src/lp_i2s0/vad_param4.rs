#[doc = "Register `VAD_PARAM4` reader"]
pub type R = crate::R<VadParam4Spec>;
#[doc = "Register `VAD_PARAM4` writer"]
pub type W = crate::W<VadParam4Spec>;
#[doc = "Field `PARAM_NOISE_SPE_DOWN` reader - VAD parameter"]
pub type ParamNoiseSpeDownR = crate::FieldReader<u16>;
#[doc = "Field `PARAM_NOISE_SPE_DOWN` writer - VAD parameter"]
pub type ParamNoiseSpeDownW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PARAM_NOISE_MEAN_DOWN` reader - VAD parameter"]
pub type ParamNoiseMeanDownR = crate::FieldReader<u16>;
#[doc = "Field `PARAM_NOISE_MEAN_DOWN` writer - VAD parameter"]
pub type ParamNoiseMeanDownW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_spe_down(&self) -> ParamNoiseSpeDownR {
        ParamNoiseSpeDownR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_mean_down(&self) -> ParamNoiseMeanDownR {
        ParamNoiseMeanDownR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_spe_down(&mut self) -> ParamNoiseSpeDownW<'_, VadParam4Spec> {
        ParamNoiseSpeDownW::new(self, 0)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_mean_down(&mut self) -> ParamNoiseMeanDownW<'_, VadParam4Spec> {
        ParamNoiseMeanDownW::new(self, 16)
    }
}
#[doc = "I2S VAD Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_param4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_param4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VadParam4Spec;
impl crate::RegisterSpec for VadParam4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_param4::R`](R) reader structure"]
impl crate::Readable for VadParam4Spec {}
#[doc = "`write(|w| ..)` method takes [`vad_param4::W`](W) writer structure"]
impl crate::Writable for VadParam4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VAD_PARAM4 to value 0x799a_6666"]
impl crate::Resettable for VadParam4Spec {
    const RESET_VALUE: u32 = 0x799a_6666;
}
