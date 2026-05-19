#[doc = "Register `VAD_PARAM0` reader"]
pub type R = crate::R<VadParam0Spec>;
#[doc = "Register `VAD_PARAM0` writer"]
pub type W = crate::W<VadParam0Spec>;
#[doc = "Field `PARAM_MIN_ENERGY` reader - VAD parameter"]
pub type ParamMinEnergyR = crate::FieldReader<u16>;
#[doc = "Field `PARAM_MIN_ENERGY` writer - VAD parameter"]
pub type ParamMinEnergyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PARAM_INIT_FRAME_NUM` reader - VAD parameter"]
pub type ParamInitFrameNumR = crate::FieldReader<u16>;
#[doc = "Field `PARAM_INIT_FRAME_NUM` writer - VAD parameter"]
pub type ParamInitFrameNumW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    pub fn param_min_energy(&self) -> ParamMinEnergyR {
        ParamMinEnergyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:24 - VAD parameter"]
    #[inline(always)]
    pub fn param_init_frame_num(&self) -> ParamInitFrameNumR {
        ParamInitFrameNumR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    pub fn param_min_energy(&mut self) -> ParamMinEnergyW<'_, VadParam0Spec> {
        ParamMinEnergyW::new(self, 0)
    }
    #[doc = "Bits 16:24 - VAD parameter"]
    #[inline(always)]
    pub fn param_init_frame_num(&mut self) -> ParamInitFrameNumW<'_, VadParam0Spec> {
        ParamInitFrameNumW::new(self, 16)
    }
}
#[doc = "I2S VAD Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_param0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_param0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VadParam0Spec;
impl crate::RegisterSpec for VadParam0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_param0::R`](R) reader structure"]
impl crate::Readable for VadParam0Spec {}
#[doc = "`write(|w| ..)` method takes [`vad_param0::W`](W) writer structure"]
impl crate::Writable for VadParam0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VAD_PARAM0 to value 0x00c8_1388"]
impl crate::Resettable for VadParam0Spec {
    const RESET_VALUE: u32 = 0x00c8_1388;
}
