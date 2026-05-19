#[doc = "Register `IN_SUC_EOF_DES_ADDR` reader"]
pub type R = crate::R<InSucEofDesAddrSpec>;
#[doc = "Field `IN_SUC_EOF_DES_ADDR_CH0` reader - Represents the address of the receive descriptor when the EOF bit in this descriptor is 1."]
pub type InSucEofDesAddrCh0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the receive descriptor when the EOF bit in this descriptor is 1."]
    #[inline(always)]
    pub fn in_suc_eof_des_addr_ch0(&self) -> InSucEofDesAddrCh0R {
        InSucEofDesAddrCh0R::new(self.bits)
    }
}
#[doc = "Receive descriptor address when EOF occurs on RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_suc_eof_des_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InSucEofDesAddrSpec;
impl crate::RegisterSpec for InSucEofDesAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_suc_eof_des_addr::R`](R) reader structure"]
impl crate::Readable for InSucEofDesAddrSpec {}
#[doc = "`reset()` method sets IN_SUC_EOF_DES_ADDR to value 0"]
impl crate::Resettable for InSucEofDesAddrSpec {}
