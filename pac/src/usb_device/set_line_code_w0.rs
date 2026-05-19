#[doc = "Register `SET_LINE_CODE_W0` reader"]
pub type R = crate::R<SetLineCodeW0Spec>;
#[doc = "Field `DW_DTE_RATE` reader - The value of dwDTERate set by host through SET_LINE_CODING command."]
pub type DwDteRateR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The value of dwDTERate set by host through SET_LINE_CODING command."]
    #[inline(always)]
    pub fn dw_dte_rate(&self) -> DwDteRateR {
        DwDteRateR::new(self.bits)
    }
}
#[doc = "W0 of SET_LINE_CODING command.\n\nYou can [`read`](crate::Reg::read) this register and get [`set_line_code_w0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetLineCodeW0Spec;
impl crate::RegisterSpec for SetLineCodeW0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`set_line_code_w0::R`](R) reader structure"]
impl crate::Readable for SetLineCodeW0Spec {}
#[doc = "`reset()` method sets SET_LINE_CODE_W0 to value 0"]
impl crate::Resettable for SetLineCodeW0Spec {}
