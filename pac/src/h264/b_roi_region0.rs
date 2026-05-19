#[doc = "Register `B_ROI_REGION0` reader"]
pub type R = crate::R<BRoiRegion0Spec>;
#[doc = "Register `B_ROI_REGION0` writer"]
pub type W = crate::W<BRoiRegion0Spec>;
#[doc = "Field `X` reader - Configures the horizontal start macroblocks of region 0 in Video B."]
pub type XR = crate::FieldReader;
#[doc = "Field `X` writer - Configures the horizontal start macroblocks of region 0 in Video B."]
pub type XW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Y` reader - Configures the vertical start macroblocks of region 0 in Video B."]
pub type YR = crate::FieldReader;
#[doc = "Field `Y` writer - Configures the vertical start macroblocks of region 0 in Video B."]
pub type YW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `X_LEN` reader - Configures the number of macroblocks in horizontal direction of the region 0 in Video B."]
pub type XLenR = crate::FieldReader;
#[doc = "Field `X_LEN` writer - Configures the number of macroblocks in horizontal direction of the region 0 in Video B."]
pub type XLenW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Y_LEN` reader - Configures the number of macroblocks in vertical direction of the region 0 in Video B."]
pub type YLenR = crate::FieldReader;
#[doc = "Field `Y_LEN` writer - Configures the number of macroblocks in vertical direction of the region 0 in Video B."]
pub type YLenW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EN` reader - Configures whether or not to open Video B ROI of region 0 .\\\\0:Close ROI\\\\1:Open ROI."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Configures whether or not to open Video B ROI of region 0 .\\\\0:Close ROI\\\\1:Open ROI."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Configures the horizontal start macroblocks of region 0 in Video B."]
    #[inline(always)]
    pub fn x(&self) -> XR {
        XR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Configures the vertical start macroblocks of region 0 in Video B."]
    #[inline(always)]
    pub fn y(&self) -> YR {
        YR::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:20 - Configures the number of macroblocks in horizontal direction of the region 0 in Video B."]
    #[inline(always)]
    pub fn x_len(&self) -> XLenR {
        XLenR::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bits 21:27 - Configures the number of macroblocks in vertical direction of the region 0 in Video B."]
    #[inline(always)]
    pub fn y_len(&self) -> YLenR {
        YLenR::new(((self.bits >> 21) & 0x7f) as u8)
    }
    #[doc = "Bit 28 - Configures whether or not to open Video B ROI of region 0 .\\\\0:Close ROI\\\\1:Open ROI."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Configures the horizontal start macroblocks of region 0 in Video B."]
    #[inline(always)]
    pub fn x(&mut self) -> XW<'_, BRoiRegion0Spec> {
        XW::new(self, 0)
    }
    #[doc = "Bits 7:13 - Configures the vertical start macroblocks of region 0 in Video B."]
    #[inline(always)]
    pub fn y(&mut self) -> YW<'_, BRoiRegion0Spec> {
        YW::new(self, 7)
    }
    #[doc = "Bits 14:20 - Configures the number of macroblocks in horizontal direction of the region 0 in Video B."]
    #[inline(always)]
    pub fn x_len(&mut self) -> XLenW<'_, BRoiRegion0Spec> {
        XLenW::new(self, 14)
    }
    #[doc = "Bits 21:27 - Configures the number of macroblocks in vertical direction of the region 0 in Video B."]
    #[inline(always)]
    pub fn y_len(&mut self) -> YLenW<'_, BRoiRegion0Spec> {
        YLenW::new(self, 21)
    }
    #[doc = "Bit 28 - Configures whether or not to open Video B ROI of region 0 .\\\\0:Close ROI\\\\1:Open ROI."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, BRoiRegion0Spec> {
        EnW::new(self, 28)
    }
}
#[doc = "Video B H264 ROI region0 range configure register.\n\nYou can [`read`](crate::Reg::read) this register and get [`b_roi_region0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b_roi_region0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRoiRegion0Spec;
impl crate::RegisterSpec for BRoiRegion0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b_roi_region0::R`](R) reader structure"]
impl crate::Readable for BRoiRegion0Spec {}
#[doc = "`write(|w| ..)` method takes [`b_roi_region0::W`](W) writer structure"]
impl crate::Writable for BRoiRegion0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets B_ROI_REGION0 to value 0"]
impl crate::Resettable for BRoiRegion0Spec {}
