#[doc = "Register `WBG_COEF_R` reader"]
pub type R = crate::R<WbgCoefRSpec>;
#[doc = "Register `WBG_COEF_R` writer"]
pub type W = crate::W<WbgCoefRSpec>;
#[doc = "Field `WBG_R` reader - Configures the white balance red gain"]
pub type WbgRR = crate::FieldReader<u16>;
#[doc = "Field `WBG_R` writer - Configures the white balance red gain"]
pub type WbgRW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Configures the white balance red gain"]
    #[inline(always)]
    pub fn wbg_r(&self) -> WbgRR {
        WbgRR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Configures the white balance red gain"]
    #[inline(always)]
    pub fn wbg_r(&mut self) -> WbgRW<'_, WbgCoefRSpec> {
        WbgRW::new(self, 0)
    }
}
#[doc = "white balance red gain register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wbg_coef_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wbg_coef_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WbgCoefRSpec;
impl crate::RegisterSpec for WbgCoefRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wbg_coef_r::R`](R) reader structure"]
impl crate::Readable for WbgCoefRSpec {}
#[doc = "`write(|w| ..)` method takes [`wbg_coef_r::W`](W) writer structure"]
impl crate::Writable for WbgCoefRSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WBG_COEF_R to value 0x0100"]
impl crate::Resettable for WbgCoefRSpec {
    const RESET_VALUE: u32 = 0x0100;
}
