#[doc = "Register `CROP_CTRL` writer"]
pub type W = crate::W<CropCtrlSpec>;
#[doc = "Field `CROP_SFT_RST` writer - Write 1 to clear err st"]
pub type CropSftRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 to clear err st"]
    #[inline(always)]
    pub fn crop_sft_rst(&mut self) -> CropSftRstW<'_, CropCtrlSpec> {
        CropSftRstW::new(self, 0)
    }
}
#[doc = "isp_crop ctrl register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crop_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CropCtrlSpec;
impl crate::RegisterSpec for CropCtrlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crop_ctrl::W`](W) writer structure"]
impl crate::Writable for CropCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CROP_CTRL to value 0"]
impl crate::Resettable for CropCtrlSpec {}
