#[doc = "Register `HIST_BIN3` reader"]
pub type R = crate::R<HistBin3Spec>;
#[doc = "Field `HIST_BIN_3` reader - this field represents result of histogram bin 3"]
pub type HistBin3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - this field represents result of histogram bin 3"]
    #[inline(always)]
    pub fn hist_bin_3(&self) -> HistBin3R {
        HistBin3R::new(self.bits & 0x0001_ffff)
    }
}
#[doc = "result of histogram bin 3\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistBin3Spec;
impl crate::RegisterSpec for HistBin3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_bin3::R`](R) reader structure"]
impl crate::Readable for HistBin3Spec {}
#[doc = "`reset()` method sets HIST_BIN3 to value 0"]
impl crate::Resettable for HistBin3Spec {}
