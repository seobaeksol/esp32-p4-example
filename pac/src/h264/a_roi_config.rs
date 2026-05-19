#[doc = "Register `A_ROI_CONFIG` reader"]
pub type R = crate::R<ARoiConfigSpec>;
#[doc = "Register `A_ROI_CONFIG` writer"]
pub type W = crate::W<ARoiConfigSpec>;
#[doc = "Field `A_ROI_EN` reader - Configure whether or not to enable ROI in video A.\\\\0:not enable ROI\\\\1:enable ROI."]
pub type ARoiEnR = crate::BitReader;
#[doc = "Field `A_ROI_EN` writer - Configure whether or not to enable ROI in video A.\\\\0:not enable ROI\\\\1:enable ROI."]
pub type ARoiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A_ROI_MODE` reader - Configure the mode of ROI in video A.\\\\0:fixed qp\\\\1:delta qp."]
pub type ARoiModeR = crate::BitReader;
#[doc = "Field `A_ROI_MODE` writer - Configure the mode of ROI in video A.\\\\0:fixed qp\\\\1:delta qp."]
pub type ARoiModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configure whether or not to enable ROI in video A.\\\\0:not enable ROI\\\\1:enable ROI."]
    #[inline(always)]
    pub fn a_roi_en(&self) -> ARoiEnR {
        ARoiEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configure the mode of ROI in video A.\\\\0:fixed qp\\\\1:delta qp."]
    #[inline(always)]
    pub fn a_roi_mode(&self) -> ARoiModeR {
        ARoiModeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure whether or not to enable ROI in video A.\\\\0:not enable ROI\\\\1:enable ROI."]
    #[inline(always)]
    pub fn a_roi_en(&mut self) -> ARoiEnW<'_, ARoiConfigSpec> {
        ARoiEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Configure the mode of ROI in video A.\\\\0:fixed qp\\\\1:delta qp."]
    #[inline(always)]
    pub fn a_roi_mode(&mut self) -> ARoiModeW<'_, ARoiConfigSpec> {
        ARoiModeW::new(self, 1)
    }
}
#[doc = "Video A H264 ROI configure register.\n\nYou can [`read`](crate::Reg::read) this register and get [`a_roi_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a_roi_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARoiConfigSpec;
impl crate::RegisterSpec for ARoiConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a_roi_config::R`](R) reader structure"]
impl crate::Readable for ARoiConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`a_roi_config::W`](W) writer structure"]
impl crate::Writable for ARoiConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets A_ROI_CONFIG to value 0"]
impl crate::Resettable for ARoiConfigSpec {}
