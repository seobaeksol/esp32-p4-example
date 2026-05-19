#[doc = "Register `B_DECI_SCORE` reader"]
pub type R = crate::R<BDeciScoreSpec>;
#[doc = "Register `B_DECI_SCORE` writer"]
pub type W = crate::W<BDeciScoreSpec>;
#[doc = "Field `B_C_DECI_SCORE` reader - Configures video B chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable."]
pub type BCDeciScoreR = crate::FieldReader<u16>;
#[doc = "Field `B_C_DECI_SCORE` writer - Configures video B chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable."]
pub type BCDeciScoreW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `B_L_DECI_SCORE` reader - Configures video B luma MB decimate score. When luma score is smaller than it, luma decimate will be enable."]
pub type BLDeciScoreR = crate::FieldReader<u16>;
#[doc = "Field `B_L_DECI_SCORE` writer - Configures video B luma MB decimate score. When luma score is smaller than it, luma decimate will be enable."]
pub type BLDeciScoreW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Configures video B chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable."]
    #[inline(always)]
    pub fn b_c_deci_score(&self) -> BCDeciScoreR {
        BCDeciScoreR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Configures video B luma MB decimate score. When luma score is smaller than it, luma decimate will be enable."]
    #[inline(always)]
    pub fn b_l_deci_score(&self) -> BLDeciScoreR {
        BLDeciScoreR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Configures video B chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable."]
    #[inline(always)]
    pub fn b_c_deci_score(&mut self) -> BCDeciScoreW<'_, BDeciScoreSpec> {
        BCDeciScoreW::new(self, 0)
    }
    #[doc = "Bits 10:19 - Configures video B luma MB decimate score. When luma score is smaller than it, luma decimate will be enable."]
    #[inline(always)]
    pub fn b_l_deci_score(&mut self) -> BLDeciScoreW<'_, BDeciScoreSpec> {
        BLDeciScoreW::new(self, 10)
    }
}
#[doc = "Video B luma and chroma MB decimate score Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`b_deci_score::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b_deci_score::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDeciScoreSpec;
impl crate::RegisterSpec for BDeciScoreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b_deci_score::R`](R) reader structure"]
impl crate::Readable for BDeciScoreSpec {}
#[doc = "`write(|w| ..)` method takes [`b_deci_score::W`](W) writer structure"]
impl crate::Writable for BDeciScoreSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets B_DECI_SCORE to value 0"]
impl crate::Resettable for BDeciScoreSpec {}
