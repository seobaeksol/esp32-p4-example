#[doc = "Register `ATTEN1` reader"]
pub type R = crate::R<Atten1Spec>;
#[doc = "Register `ATTEN1` writer"]
pub type W = crate::W<Atten1Spec>;
#[doc = "Field `SAR1_ATTEN` reader - 2-bit attenuation for each pad."]
pub type Sar1AttenR = crate::FieldReader<u32>;
#[doc = "Field `SAR1_ATTEN` writer - 2-bit attenuation for each pad."]
pub type Sar1AttenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 2-bit attenuation for each pad."]
    #[inline(always)]
    pub fn sar1_atten(&self) -> Sar1AttenR {
        Sar1AttenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 2-bit attenuation for each pad."]
    #[inline(always)]
    pub fn sar1_atten(&mut self) -> Sar1AttenW<'_, Atten1Spec> {
        Sar1AttenW::new(self, 0)
    }
}
#[doc = "ADC1 attenuation registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`atten1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atten1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Atten1Spec;
impl crate::RegisterSpec for Atten1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atten1::R`](R) reader structure"]
impl crate::Readable for Atten1Spec {}
#[doc = "`write(|w| ..)` method takes [`atten1::W`](W) writer structure"]
impl crate::Writable for Atten1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ATTEN1 to value 0xffff_ffff"]
impl crate::Resettable for Atten1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
