#[doc = "Register `CROP_Y_CAPTURE` reader"]
pub type R = crate::R<CropYCaptureSpec>;
#[doc = "Register `CROP_Y_CAPTURE` writer"]
pub type W = crate::W<CropYCaptureSpec>;
#[doc = "Field `CROP_Y_START` reader - isp_crop capture row start index"]
pub type CropYStartR = crate::FieldReader<u16>;
#[doc = "Field `CROP_Y_START` writer - isp_crop capture row start index"]
pub type CropYStartW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CROP_Y_END` reader - isp_crop capture row end index"]
pub type CropYEndR = crate::FieldReader<u16>;
#[doc = "Field `CROP_Y_END` writer - isp_crop capture row end index"]
pub type CropYEndW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - isp_crop capture row start index"]
    #[inline(always)]
    pub fn crop_y_start(&self) -> CropYStartR {
        CropYStartR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - isp_crop capture row end index"]
    #[inline(always)]
    pub fn crop_y_end(&self) -> CropYEndR {
        CropYEndR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - isp_crop capture row start index"]
    #[inline(always)]
    pub fn crop_y_start(&mut self) -> CropYStartW<'_, CropYCaptureSpec> {
        CropYStartW::new(self, 0)
    }
    #[doc = "Bits 12:23 - isp_crop capture row end index"]
    #[inline(always)]
    pub fn crop_y_end(&mut self) -> CropYEndW<'_, CropYCaptureSpec> {
        CropYEndW::new(self, 12)
    }
}
#[doc = "isp_crop row capture range register\n\nYou can [`read`](crate::Reg::read) this register and get [`crop_y_capture::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crop_y_capture::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CropYCaptureSpec;
impl crate::RegisterSpec for CropYCaptureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crop_y_capture::R`](R) reader structure"]
impl crate::Readable for CropYCaptureSpec {}
#[doc = "`write(|w| ..)` method takes [`crop_y_capture::W`](W) writer structure"]
impl crate::Writable for CropYCaptureSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CROP_Y_CAPTURE to value 0"]
impl crate::Resettable for CropYCaptureSpec {}
