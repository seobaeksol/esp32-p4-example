#[doc = "Register `IN_DONE_DES_ADDR_CH1` reader"]
pub type R = crate::R<InDoneDesAddrCh1Spec>;
#[doc = "Field `IN_DONE_DES_ADDR_CH1` reader - Represents the address of the inlink descriptor when this descriptor is completed ."]
pub type InDoneDesAddrCh1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the inlink descriptor when this descriptor is completed ."]
    #[inline(always)]
    pub fn in_done_des_addr_ch1(&self) -> InDoneDesAddrCh1R {
        InDoneDesAddrCh1R::new(self.bits)
    }
}
#[doc = "RX_done Inlink descriptor address of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_done_des_addr_ch1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InDoneDesAddrCh1Spec;
impl crate::RegisterSpec for InDoneDesAddrCh1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_done_des_addr_ch1::R`](R) reader structure"]
impl crate::Readable for InDoneDesAddrCh1Spec {}
#[doc = "`reset()` method sets IN_DONE_DES_ADDR_CH1 to value 0"]
impl crate::Resettable for InDoneDesAddrCh1Spec {}
