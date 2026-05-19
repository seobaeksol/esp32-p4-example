#[doc = "Register `HIST_BIN8` reader"]
pub type R = crate::R<HistBin8Spec>;
#[doc = "Field `HIST_BIN_8` reader - this field represents result of histogram bin 8"]
pub type HistBin8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - this field represents result of histogram bin 8"]
    #[inline(always)]
    pub fn hist_bin_8(&self) -> HistBin8R {
        HistBin8R::new(self.bits & 0x0001_ffff)
    }
}
#[doc = "result of histogram bin 8\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistBin8Spec;
impl crate::RegisterSpec for HistBin8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_bin8::R`](R) reader structure"]
impl crate::Readable for HistBin8Spec {}
#[doc = "`reset()` method sets HIST_BIN8 to value 0"]
impl crate::Resettable for HistBin8Spec {}
