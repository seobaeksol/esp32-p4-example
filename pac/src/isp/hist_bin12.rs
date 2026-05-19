#[doc = "Register `HIST_BIN12` reader"]
pub type R = crate::R<HistBin12Spec>;
#[doc = "Field `HIST_BIN_12` reader - this field represents result of histogram bin 12"]
pub type HistBin12R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - this field represents result of histogram bin 12"]
    #[inline(always)]
    pub fn hist_bin_12(&self) -> HistBin12R {
        HistBin12R::new(self.bits & 0x0001_ffff)
    }
}
#[doc = "result of histogram bin 12\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistBin12Spec;
impl crate::RegisterSpec for HistBin12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_bin12::R`](R) reader structure"]
impl crate::Readable for HistBin12Spec {}
#[doc = "`reset()` method sets HIST_BIN12 to value 0"]
impl crate::Resettable for HistBin12Spec {}
