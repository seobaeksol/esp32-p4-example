#[doc = "Register `YUV_FORMAT` reader"]
pub type R = crate::R<YuvFormatSpec>;
#[doc = "Register `YUV_FORMAT` writer"]
pub type W = crate::W<YuvFormatSpec>;
#[doc = "Field `YUV_MODE` reader - this bit configures the yuv mode. 0: ITU-R BT.601, 1: ITU-R BT.709"]
pub type YuvModeR = crate::BitReader;
#[doc = "Field `YUV_MODE` writer - this bit configures the yuv mode. 0: ITU-R BT.601, 1: ITU-R BT.709"]
pub type YuvModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YUV_RANGE` reader - this bit configures the yuv range. 0: full range, 1: limit range"]
pub type YuvRangeR = crate::BitReader;
#[doc = "Field `YUV_RANGE` writer - this bit configures the yuv range. 0: full range, 1: limit range"]
pub type YuvRangeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this bit configures the yuv mode. 0: ITU-R BT.601, 1: ITU-R BT.709"]
    #[inline(always)]
    pub fn yuv_mode(&self) -> YuvModeR {
        YuvModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - this bit configures the yuv range. 0: full range, 1: limit range"]
    #[inline(always)]
    pub fn yuv_range(&self) -> YuvRangeR {
        YuvRangeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - this bit configures the yuv mode. 0: ITU-R BT.601, 1: ITU-R BT.709"]
    #[inline(always)]
    pub fn yuv_mode(&mut self) -> YuvModeW<'_, YuvFormatSpec> {
        YuvModeW::new(self, 0)
    }
    #[doc = "Bit 1 - this bit configures the yuv range. 0: full range, 1: limit range"]
    #[inline(always)]
    pub fn yuv_range(&mut self) -> YuvRangeW<'_, YuvFormatSpec> {
        YuvRangeW::new(self, 1)
    }
}
#[doc = "yuv format control register\n\nYou can [`read`](crate::Reg::read) this register and get [`yuv_format::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`yuv_format::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct YuvFormatSpec;
impl crate::RegisterSpec for YuvFormatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`yuv_format::R`](R) reader structure"]
impl crate::Readable for YuvFormatSpec {}
#[doc = "`write(|w| ..)` method takes [`yuv_format::W`](W) writer structure"]
impl crate::Writable for YuvFormatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets YUV_FORMAT to value 0"]
impl crate::Resettable for YuvFormatSpec {}
