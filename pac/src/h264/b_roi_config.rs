#[doc = "Register `B_ROI_CONFIG` reader"]
pub type R = crate::R<BRoiConfigSpec>;
#[doc = "Register `B_ROI_CONFIG` writer"]
pub type W = crate::W<BRoiConfigSpec>;
#[doc = "Field `B_ROI_EN` reader - Configure whether or not to enable ROI in video B.\\\\0:not enable ROI\\\\1:enable ROI."]
pub type BRoiEnR = crate::BitReader;
#[doc = "Field `B_ROI_EN` writer - Configure whether or not to enable ROI in video B.\\\\0:not enable ROI\\\\1:enable ROI."]
pub type BRoiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B_ROI_MODE` reader - Configure the mode of ROI in video B.\\\\0:fixed qp\\\\1:delta qp."]
pub type BRoiModeR = crate::BitReader;
#[doc = "Field `B_ROI_MODE` writer - Configure the mode of ROI in video B.\\\\0:fixed qp\\\\1:delta qp."]
pub type BRoiModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configure whether or not to enable ROI in video B.\\\\0:not enable ROI\\\\1:enable ROI."]
    #[inline(always)]
    pub fn b_roi_en(&self) -> BRoiEnR {
        BRoiEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configure the mode of ROI in video B.\\\\0:fixed qp\\\\1:delta qp."]
    #[inline(always)]
    pub fn b_roi_mode(&self) -> BRoiModeR {
        BRoiModeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure whether or not to enable ROI in video B.\\\\0:not enable ROI\\\\1:enable ROI."]
    #[inline(always)]
    pub fn b_roi_en(&mut self) -> BRoiEnW<'_, BRoiConfigSpec> {
        BRoiEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Configure the mode of ROI in video B.\\\\0:fixed qp\\\\1:delta qp."]
    #[inline(always)]
    pub fn b_roi_mode(&mut self) -> BRoiModeW<'_, BRoiConfigSpec> {
        BRoiModeW::new(self, 1)
    }
}
#[doc = "Video B H264 ROI configure register.\n\nYou can [`read`](crate::Reg::read) this register and get [`b_roi_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b_roi_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRoiConfigSpec;
impl crate::RegisterSpec for BRoiConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b_roi_config::R`](R) reader structure"]
impl crate::Readable for BRoiConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`b_roi_config::W`](W) writer structure"]
impl crate::Writable for BRoiConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets B_ROI_CONFIG to value 0"]
impl crate::Resettable for BRoiConfigSpec {}
