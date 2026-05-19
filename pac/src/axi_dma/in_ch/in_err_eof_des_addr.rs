#[doc = "Register `IN_ERR_EOF_DES_ADDR` reader"]
pub type R = crate::R<InErrEofDesAddrSpec>;
#[doc = "Field `IN_ERR_EOF_DES_ADDR` reader - This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0."]
pub type InErrEofDesAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0."]
    #[inline(always)]
    pub fn in_err_eof_des_addr(&self) -> InErrEofDesAddrR {
        InErrEofDesAddrR::new(self.bits)
    }
}
#[doc = "Inlink descriptor address when errors occur of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_err_eof_des_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InErrEofDesAddrSpec;
impl crate::RegisterSpec for InErrEofDesAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_err_eof_des_addr::R`](R) reader structure"]
impl crate::Readable for InErrEofDesAddrSpec {}
#[doc = "`reset()` method sets IN_ERR_EOF_DES_ADDR to value 0"]
impl crate::Resettable for InErrEofDesAddrSpec {}
