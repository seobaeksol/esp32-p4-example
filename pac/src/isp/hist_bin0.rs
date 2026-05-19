#[doc = "Register `HIST_BIN0` reader"]
pub type R = crate::R<HistBin0Spec>;
#[doc = "Field `HIST_BIN_0` reader - this field represents result of histogram bin 0"]
pub type HistBin0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - this field represents result of histogram bin 0"]
    #[inline(always)]
    pub fn hist_bin_0(&self) -> HistBin0R {
        HistBin0R::new(self.bits & 0x0001_ffff)
    }
}
#[doc = "result of histogram bin 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistBin0Spec;
impl crate::RegisterSpec for HistBin0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_bin0::R`](R) reader structure"]
impl crate::Readable for HistBin0Spec {}
#[doc = "`reset()` method sets HIST_BIN0 to value 0"]
impl crate::Resettable for HistBin0Spec {}
