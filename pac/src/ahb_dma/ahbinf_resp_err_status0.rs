#[doc = "Register `AHBINF_RESP_ERR_STATUS0` reader"]
pub type R = crate::R<AhbinfRespErrStatus0Spec>;
#[doc = "Field `AHBINF_RESP_ERR_ADDR` reader - Represents the address of the AHB response error."]
pub type AhbinfRespErrAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the AHB response error."]
    #[inline(always)]
    pub fn ahbinf_resp_err_addr(&self) -> AhbinfRespErrAddrR {
        AhbinfRespErrAddrR::new(self.bits)
    }
}
#[doc = "AHB response error status 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbinf_resp_err_status0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbinfRespErrStatus0Spec;
impl crate::RegisterSpec for AhbinfRespErrStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbinf_resp_err_status0::R`](R) reader structure"]
impl crate::Readable for AhbinfRespErrStatus0Spec {}
#[doc = "`reset()` method sets AHBINF_RESP_ERR_STATUS0 to value 0"]
impl crate::Resettable for AhbinfRespErrStatus0Spec {}
