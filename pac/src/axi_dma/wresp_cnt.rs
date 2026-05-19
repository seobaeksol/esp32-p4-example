#[doc = "Register `WRESP_CNT` reader"]
pub type R = crate::R<WrespCntSpec>;
#[doc = "Field `WRESP_CNT` reader - axi wr responce cnt reg."]
pub type WrespCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - axi wr responce cnt reg."]
    #[inline(always)]
    pub fn wresp_cnt(&self) -> WrespCntR {
        WrespCntR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "AXI wr responce cnt register.\n\nYou can [`read`](crate::Reg::read) this register and get [`wresp_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrespCntSpec;
impl crate::RegisterSpec for WrespCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wresp_cnt::R`](R) reader structure"]
impl crate::Readable for WrespCntSpec {}
#[doc = "`reset()` method sets WRESP_CNT to value 0"]
impl crate::Resettable for WrespCntSpec {}
