#[doc = "Register `B_ROI_REGION0_3_QP` reader"]
pub type R = crate::R<BRoiRegion0_3QpSpec>;
#[doc = "Register `B_ROI_REGION0_3_QP` writer"]
pub type W = crate::W<BRoiRegion0_3QpSpec>;
#[doc = "Field `B_ROI_REGION0_QP` reader - Configure H264 ROI region0 qp in video B,fixed qp or delta qp."]
pub type BRoiRegion0QpR = crate::FieldReader;
#[doc = "Field `B_ROI_REGION0_QP` writer - Configure H264 ROI region0 qp in video B,fixed qp or delta qp."]
pub type BRoiRegion0QpW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `B_ROI_REGION1_QP` reader - Configure H264 ROI region1 qp in video B,fixed qp or delta qp."]
pub type BRoiRegion1QpR = crate::FieldReader;
#[doc = "Field `B_ROI_REGION1_QP` writer - Configure H264 ROI region1 qp in video B,fixed qp or delta qp."]
pub type BRoiRegion1QpW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `B_ROI_REGION2_QP` reader - Configure H264 ROI region2 qp in video B,fixed qp or delta qp."]
pub type BRoiRegion2QpR = crate::FieldReader;
#[doc = "Field `B_ROI_REGION2_QP` writer - Configure H264 ROI region2 qp in video B,fixed qp or delta qp."]
pub type BRoiRegion2QpW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `B_ROI_REGION3_QP` reader - Configure H264 ROI region3 qp in video B,fixed qp or delta qp."]
pub type BRoiRegion3QpR = crate::FieldReader;
#[doc = "Field `B_ROI_REGION3_QP` writer - Configure H264 ROI region3 qp in video B,fixed qp or delta qp."]
pub type BRoiRegion3QpW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Configure H264 ROI region0 qp in video B,fixed qp or delta qp."]
    #[inline(always)]
    pub fn b_roi_region0_qp(&self) -> BRoiRegion0QpR {
        BRoiRegion0QpR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Configure H264 ROI region1 qp in video B,fixed qp or delta qp."]
    #[inline(always)]
    pub fn b_roi_region1_qp(&self) -> BRoiRegion1QpR {
        BRoiRegion1QpR::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:20 - Configure H264 ROI region2 qp in video B,fixed qp or delta qp."]
    #[inline(always)]
    pub fn b_roi_region2_qp(&self) -> BRoiRegion2QpR {
        BRoiRegion2QpR::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bits 21:27 - Configure H264 ROI region3 qp in video B,fixed qp or delta qp."]
    #[inline(always)]
    pub fn b_roi_region3_qp(&self) -> BRoiRegion3QpR {
        BRoiRegion3QpR::new(((self.bits >> 21) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Configure H264 ROI region0 qp in video B,fixed qp or delta qp."]
    #[inline(always)]
    pub fn b_roi_region0_qp(&mut self) -> BRoiRegion0QpW<'_, BRoiRegion0_3QpSpec> {
        BRoiRegion0QpW::new(self, 0)
    }
    #[doc = "Bits 7:13 - Configure H264 ROI region1 qp in video B,fixed qp or delta qp."]
    #[inline(always)]
    pub fn b_roi_region1_qp(&mut self) -> BRoiRegion1QpW<'_, BRoiRegion0_3QpSpec> {
        BRoiRegion1QpW::new(self, 7)
    }
    #[doc = "Bits 14:20 - Configure H264 ROI region2 qp in video B,fixed qp or delta qp."]
    #[inline(always)]
    pub fn b_roi_region2_qp(&mut self) -> BRoiRegion2QpW<'_, BRoiRegion0_3QpSpec> {
        BRoiRegion2QpW::new(self, 14)
    }
    #[doc = "Bits 21:27 - Configure H264 ROI region3 qp in video B,fixed qp or delta qp."]
    #[inline(always)]
    pub fn b_roi_region3_qp(&mut self) -> BRoiRegion3QpW<'_, BRoiRegion0_3QpSpec> {
        BRoiRegion3QpW::new(self, 21)
    }
}
#[doc = "Video B H264 ROI region0, region1,region2,region3 QP register.\n\nYou can [`read`](crate::Reg::read) this register and get [`b_roi_region0_3_qp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b_roi_region0_3_qp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRoiRegion0_3QpSpec;
impl crate::RegisterSpec for BRoiRegion0_3QpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b_roi_region0_3_qp::R`](R) reader structure"]
impl crate::Readable for BRoiRegion0_3QpSpec {}
#[doc = "`write(|w| ..)` method takes [`b_roi_region0_3_qp::W`](W) writer structure"]
impl crate::Writable for BRoiRegion0_3QpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets B_ROI_REGION0_3_QP to value 0"]
impl crate::Resettable for BRoiRegion0_3QpSpec {}
