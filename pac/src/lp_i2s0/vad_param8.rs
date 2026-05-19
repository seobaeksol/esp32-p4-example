#[doc = "Register `VAD_PARAM8` reader"]
pub type R = crate::R<VadParam8Spec>;
#[doc = "Register `VAD_PARAM8` writer"]
pub type W = crate::W<VadParam8Spec>;
#[doc = "Field `PARAM_THRES_UPD_BDL` reader - Noise_std boundary low when updating threshold."]
pub type ParamThresUpdBdlR = crate::FieldReader;
#[doc = "Field `PARAM_THRES_UPD_BDL` writer - Noise_std boundary low when updating threshold."]
pub type ParamThresUpdBdlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PARAM_THRES_UPD_BDH` reader - Noise_std boundary high when updating threshold."]
pub type ParamThresUpdBdhR = crate::FieldReader;
#[doc = "Field `PARAM_THRES_UPD_BDH` writer - Noise_std boundary high when updating threshold."]
pub type ParamThresUpdBdhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PARAM_FEATURE_BURST` reader - VAD parameter"]
pub type ParamFeatureBurstR = crate::FieldReader<u16>;
#[doc = "Field `PARAM_FEATURE_BURST` writer - VAD parameter"]
pub type ParamFeatureBurstW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Noise_std boundary low when updating threshold."]
    #[inline(always)]
    pub fn param_thres_upd_bdl(&self) -> ParamThresUpdBdlR {
        ParamThresUpdBdlR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Noise_std boundary high when updating threshold."]
    #[inline(always)]
    pub fn param_thres_upd_bdh(&self) -> ParamThresUpdBdhR {
        ParamThresUpdBdhR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    pub fn param_feature_burst(&self) -> ParamFeatureBurstR {
        ParamFeatureBurstR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Noise_std boundary low when updating threshold."]
    #[inline(always)]
    pub fn param_thres_upd_bdl(&mut self) -> ParamThresUpdBdlW<'_, VadParam8Spec> {
        ParamThresUpdBdlW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Noise_std boundary high when updating threshold."]
    #[inline(always)]
    pub fn param_thres_upd_bdh(&mut self) -> ParamThresUpdBdhW<'_, VadParam8Spec> {
        ParamThresUpdBdhW::new(self, 8)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    pub fn param_feature_burst(&mut self) -> ParamFeatureBurstW<'_, VadParam8Spec> {
        ParamFeatureBurstW::new(self, 16)
    }
}
#[doc = "I2S VAD Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_param8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_param8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VadParam8Spec;
impl crate::RegisterSpec for VadParam8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_param8::R`](R) reader structure"]
impl crate::Readable for VadParam8Spec {}
#[doc = "`write(|w| ..)` method takes [`vad_param8::W`](W) writer structure"]
impl crate::Writable for VadParam8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VAD_PARAM8 to value 0x2000_5040"]
impl crate::Resettable for VadParam8Spec {
    const RESET_VALUE: u32 = 0x2000_5040;
}
