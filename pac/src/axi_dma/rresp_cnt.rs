#[doc = "Register `RRESP_CNT` reader"]
pub type R = crate::R<RrespCntSpec>;
#[doc = "Field `RRESP_CNT` reader - axi rd responce cnt reg."]
pub type RrespCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - axi rd responce cnt reg."]
    #[inline(always)]
    pub fn rresp_cnt(&self) -> RrespCntR {
        RrespCntR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "AXI wr responce cnt register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rresp_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrespCntSpec;
impl crate::RegisterSpec for RrespCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rresp_cnt::R`](R) reader structure"]
impl crate::Readable for RrespCntSpec {}
#[doc = "`reset()` method sets RRESP_CNT to value 0"]
impl crate::Resettable for RrespCntSpec {}
