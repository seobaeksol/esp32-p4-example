#[doc = "Register `HIST_BIN11` reader"]
pub type R = crate::R<HistBin11Spec>;
#[doc = "Field `HIST_BIN_11` reader - this field represents result of histogram bin 11"]
pub type HistBin11R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - this field represents result of histogram bin 11"]
    #[inline(always)]
    pub fn hist_bin_11(&self) -> HistBin11R {
        HistBin11R::new(self.bits & 0x0001_ffff)
    }
}
#[doc = "result of histogram bin 11\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HistBin11Spec;
impl crate::RegisterSpec for HistBin11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_bin11::R`](R) reader structure"]
impl crate::Readable for HistBin11Spec {}
#[doc = "`reset()` method sets HIST_BIN11 to value 0"]
impl crate::Resettable for HistBin11Spec {}
