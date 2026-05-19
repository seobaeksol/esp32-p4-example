#[doc = "Register `A_ROI_REGION4_7_QP` reader"]
pub type R = crate::R<ARoiRegion4_7QpSpec>;
#[doc = "Register `A_ROI_REGION4_7_QP` writer"]
pub type W = crate::W<ARoiRegion4_7QpSpec>;
#[doc = "Field `A_ROI_REGION4_QP` reader - Configure H264 ROI region4 qp in video A,fixed qp or delta qp."]
pub type ARoiRegion4QpR = crate::FieldReader;
#[doc = "Field `A_ROI_REGION4_QP` writer - Configure H264 ROI region4 qp in video A,fixed qp or delta qp."]
pub type ARoiRegion4QpW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `A_ROI_REGION5_QP` reader - Configure H264 ROI region5 qp in video A,fixed qp or delta qp."]
pub type ARoiRegion5QpR = crate::FieldReader;
#[doc = "Field `A_ROI_REGION5_QP` writer - Configure H264 ROI region5 qp in video A,fixed qp or delta qp."]
pub type ARoiRegion5QpW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `A_ROI_REGION6_QP` reader - Configure H264 ROI region6 qp in video A,fixed qp or delta qp."]
pub type ARoiRegion6QpR = crate::FieldReader;
#[doc = "Field `A_ROI_REGION6_QP` writer - Configure H264 ROI region6 qp in video A,fixed qp or delta qp."]
pub type ARoiRegion6QpW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `A_ROI_REGION7_QP` reader - Configure H264 ROI region7 qp in video A,fixed qp or delta qp."]
pub type ARoiRegion7QpR = crate::FieldReader;
#[doc = "Field `A_ROI_REGION7_QP` writer - Configure H264 ROI region7 qp in video A,fixed qp or delta qp."]
pub type ARoiRegion7QpW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Configure H264 ROI region4 qp in video A,fixed qp or delta qp."]
    #[inline(always)]
    pub fn a_roi_region4_qp(&self) -> ARoiRegion4QpR {
        ARoiRegion4QpR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Configure H264 ROI region5 qp in video A,fixed qp or delta qp."]
    #[inline(always)]
    pub fn a_roi_region5_qp(&self) -> ARoiRegion5QpR {
        ARoiRegion5QpR::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:20 - Configure H264 ROI region6 qp in video A,fixed qp or delta qp."]
    #[inline(always)]
    pub fn a_roi_region6_qp(&self) -> ARoiRegion6QpR {
        ARoiRegion6QpR::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bits 21:27 - Configure H264 ROI region7 qp in video A,fixed qp or delta qp."]
    #[inline(always)]
    pub fn a_roi_region7_qp(&self) -> ARoiRegion7QpR {
        ARoiRegion7QpR::new(((self.bits >> 21) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Configure H264 ROI region4 qp in video A,fixed qp or delta qp."]
    #[inline(always)]
    pub fn a_roi_region4_qp(&mut self) -> ARoiRegion4QpW<'_, ARoiRegion4_7QpSpec> {
        ARoiRegion4QpW::new(self, 0)
    }
    #[doc = "Bits 7:13 - Configure H264 ROI region5 qp in video A,fixed qp or delta qp."]
    #[inline(always)]
    pub fn a_roi_region5_qp(&mut self) -> ARoiRegion5QpW<'_, ARoiRegion4_7QpSpec> {
        ARoiRegion5QpW::new(self, 7)
    }
    #[doc = "Bits 14:20 - Configure H264 ROI region6 qp in video A,fixed qp or delta qp."]
    #[inline(always)]
    pub fn a_roi_region6_qp(&mut self) -> ARoiRegion6QpW<'_, ARoiRegion4_7QpSpec> {
        ARoiRegion6QpW::new(self, 14)
    }
    #[doc = "Bits 21:27 - Configure H264 ROI region7 qp in video A,fixed qp or delta qp."]
    #[inline(always)]
    pub fn a_roi_region7_qp(&mut self) -> ARoiRegion7QpW<'_, ARoiRegion4_7QpSpec> {
        ARoiRegion7QpW::new(self, 21)
    }
}
#[doc = "Video A H264 ROI region4, region5,region6,region7 QP register.\n\nYou can [`read`](crate::Reg::read) this register and get [`a_roi_region4_7_qp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a_roi_region4_7_qp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARoiRegion4_7QpSpec;
impl crate::RegisterSpec for ARoiRegion4_7QpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a_roi_region4_7_qp::R`](R) reader structure"]
impl crate::Readable for ARoiRegion4_7QpSpec {}
#[doc = "`write(|w| ..)` method takes [`a_roi_region4_7_qp::W`](W) writer structure"]
impl crate::Writable for ARoiRegion4_7QpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets A_ROI_REGION4_7_QP to value 0"]
impl crate::Resettable for ARoiRegion4_7QpSpec {}
