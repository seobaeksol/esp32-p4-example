#[doc = "Register `SUC_EOF_DES_ADDR` reader"]
pub type R = crate::R<SucEofDesAddrSpec>;
#[doc = "Field `IN_SUC_EOF_DES_ADDR` reader - This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1."]
pub type InSucEofDesAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1."]
    #[inline(always)]
    pub fn in_suc_eof_des_addr(&self) -> InSucEofDesAddrR {
        InSucEofDesAddrR::new(self.bits)
    }
}
#[doc = "RX CHx eof des addr register\n\nYou can [`read`](crate::Reg::read) this register and get [`suc_eof_des_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SucEofDesAddrSpec;
impl crate::RegisterSpec for SucEofDesAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`suc_eof_des_addr::R`](R) reader structure"]
impl crate::Readable for SucEofDesAddrSpec {}
#[doc = "`reset()` method sets SUC_EOF_DES_ADDR to value 0"]
impl crate::Resettable for SucEofDesAddrSpec {}
