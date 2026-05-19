#[doc = "Register `HIST_BIN14` reader"]
pub type R = crate::R<HistBin14Spec>;
#[doc = "Field `HIST_BIN_14` reader - this field represents result of histogram bin 14"]
pub type HistBin14R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - this field represents result of histogram bin 14"]
    #[inline(always)]
    pub fn hist_bin_14(&self) -> HistBin14R {
        HistBin14R::new(self.bits & 0x0001_ffff)
    }
}
#[doc = "result of histogram bin 14\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin14::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistBin14Spec;
impl crate::RegisterSpec for HistBin14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_bin14::R`](R) reader structure"]
impl crate::Readable for HistBin14Spec {}
#[doc = "`reset()` method sets HIST_BIN14 to value 0"]
impl crate::Resettable for HistBin14Spec {}
