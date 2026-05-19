#[doc = "Register `AF_GEN_TH_CTRL` reader"]
pub type R = crate::R<AfGenThCtrlSpec>;
#[doc = "Register `AF_GEN_TH_CTRL` writer"]
pub type W = crate::W<AfGenThCtrlSpec>;
#[doc = "Field `AF_GEN_THRESHOLD_MIN` reader - this field configures min threshold when use auto_threshold"]
pub type AfGenThresholdMinR = crate::FieldReader<u16>;
#[doc = "Field `AF_GEN_THRESHOLD_MIN` writer - this field configures min threshold when use auto_threshold"]
pub type AfGenThresholdMinW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `AF_GEN_THRESHOLD_MAX` reader - this field configures max threshold when use auto_threshold"]
pub type AfGenThresholdMaxR = crate::FieldReader<u16>;
#[doc = "Field `AF_GEN_THRESHOLD_MAX` writer - this field configures max threshold when use auto_threshold"]
pub type AfGenThresholdMaxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - this field configures min threshold when use auto_threshold"]
    #[inline(always)]
    pub fn af_gen_threshold_min(&self) -> AfGenThresholdMinR {
        AfGenThresholdMinR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - this field configures max threshold when use auto_threshold"]
    #[inline(always)]
    pub fn af_gen_threshold_max(&self) -> AfGenThresholdMaxR {
        AfGenThresholdMaxR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - this field configures min threshold when use auto_threshold"]
    #[inline(always)]
    pub fn af_gen_threshold_min(&mut self) -> AfGenThresholdMinW<'_, AfGenThCtrlSpec> {
        AfGenThresholdMinW::new(self, 0)
    }
    #[doc = "Bits 16:31 - this field configures max threshold when use auto_threshold"]
    #[inline(always)]
    pub fn af_gen_threshold_max(&mut self) -> AfGenThresholdMaxW<'_, AfGenThCtrlSpec> {
        AfGenThresholdMaxW::new(self, 16)
    }
}
#[doc = "af gen threshold control register\n\nYou can [`read`](crate::Reg::read) this register and get [`af_gen_th_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_gen_th_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfGenThCtrlSpec;
impl crate::RegisterSpec for AfGenThCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_gen_th_ctrl::R`](R) reader structure"]
impl crate::Readable for AfGenThCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`af_gen_th_ctrl::W`](W) writer structure"]
impl crate::Writable for AfGenThCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AF_GEN_TH_CTRL to value 0x0440_0080"]
impl crate::Resettable for AfGenThCtrlSpec {
    const RESET_VALUE: u32 = 0x0440_0080;
}
