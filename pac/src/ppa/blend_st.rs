#[doc = "Register `BLEND_ST` reader"]
pub type R = crate::R<BlendStSpec>;
#[doc = "Field `BLEND_SIZE_DIFF_ST` reader - 1: indicate the size of two image is different."]
pub type BlendSizeDiffStR = crate::BitReader;
#[doc = "Field `BLEND_YUV_X_SCALE_ERR_ST` reader - Represents that x param is an odd num when enable yuv422 or yuv420"]
pub type BlendYuvXScaleErrStR = crate::BitReader;
#[doc = "Field `BLEND_YUV_Y_SCALE_ERR_ST` reader - Represents that y param is an odd num when enable yuv420"]
pub type BlendYuvYScaleErrStR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: indicate the size of two image is different."]
    #[inline(always)]
    pub fn blend_size_diff_st(&self) -> BlendSizeDiffStR {
        BlendSizeDiffStR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents that x param is an odd num when enable yuv422 or yuv420"]
    #[inline(always)]
    pub fn blend_yuv_x_scale_err_st(&self) -> BlendYuvXScaleErrStR {
        BlendYuvXScaleErrStR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents that y param is an odd num when enable yuv420"]
    #[inline(always)]
    pub fn blend_yuv_y_scale_err_st(&self) -> BlendYuvYScaleErrStR {
        BlendYuvYScaleErrStR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Blending engine status register\n\nYou can [`read`](crate::Reg::read) this register and get [`blend_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlendStSpec;
impl crate::RegisterSpec for BlendStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blend_st::R`](R) reader structure"]
impl crate::Readable for BlendStSpec {}
#[doc = "`reset()` method sets BLEND_ST to value 0"]
impl crate::Resettable for BlendStSpec {}
