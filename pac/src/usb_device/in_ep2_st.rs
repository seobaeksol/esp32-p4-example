#[doc = "Register `IN_EP2_ST` reader"]
pub type R = crate::R<InEp2StSpec>;
#[doc = "Field `IN_EP2_STATE` reader - State of IN Endpoint 2."]
pub type InEp2StateR = crate::FieldReader;
#[doc = "Field `IN_EP2_WR_ADDR` reader - Write data address of IN endpoint 2."]
pub type InEp2WrAddrR = crate::FieldReader;
#[doc = "Field `IN_EP2_RD_ADDR` reader - Read data address of IN endpoint 2."]
pub type InEp2RdAddrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - State of IN Endpoint 2."]
    #[inline(always)]
    pub fn in_ep2_state(&self) -> InEp2StateR {
        InEp2StateR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:8 - Write data address of IN endpoint 2."]
    #[inline(always)]
    pub fn in_ep2_wr_addr(&self) -> InEp2WrAddrR {
        InEp2WrAddrR::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - Read data address of IN endpoint 2."]
    #[inline(always)]
    pub fn in_ep2_rd_addr(&self) -> InEp2RdAddrR {
        InEp2RdAddrR::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
#[doc = "CDC-ACM interrupt IN endpoint status information.\n\nYou can [`read`](crate::Reg::read) this register and get [`in_ep2_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InEp2StSpec;
impl crate::RegisterSpec for InEp2StSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_ep2_st::R`](R) reader structure"]
impl crate::Readable for InEp2StSpec {}
#[doc = "`reset()` method sets IN_EP2_ST to value 0x01"]
impl crate::Resettable for InEp2StSpec {
    const RESET_VALUE: u32 = 0x01;
}
