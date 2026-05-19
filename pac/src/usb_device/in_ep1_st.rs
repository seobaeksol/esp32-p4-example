#[doc = "Register `IN_EP1_ST` reader"]
pub type R = crate::R<InEp1StSpec>;
#[doc = "Field `IN_EP1_STATE` reader - State of IN Endpoint 1."]
pub type InEp1StateR = crate::FieldReader;
#[doc = "Field `IN_EP1_WR_ADDR` reader - Write data address of IN endpoint 1."]
pub type InEp1WrAddrR = crate::FieldReader;
#[doc = "Field `IN_EP1_RD_ADDR` reader - Read data address of IN endpoint 1."]
pub type InEp1RdAddrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - State of IN Endpoint 1."]
    #[inline(always)]
    pub fn in_ep1_state(&self) -> InEp1StateR {
        InEp1StateR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:8 - Write data address of IN endpoint 1."]
    #[inline(always)]
    pub fn in_ep1_wr_addr(&self) -> InEp1WrAddrR {
        InEp1WrAddrR::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - Read data address of IN endpoint 1."]
    #[inline(always)]
    pub fn in_ep1_rd_addr(&self) -> InEp1RdAddrR {
        InEp1RdAddrR::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
#[doc = "CDC-ACM IN endpoint status information.\n\nYou can [`read`](crate::Reg::read) this register and get [`in_ep1_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InEp1StSpec;
impl crate::RegisterSpec for InEp1StSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_ep1_st::R`](R) reader structure"]
impl crate::Readable for InEp1StSpec {}
#[doc = "`reset()` method sets IN_EP1_ST to value 0x01"]
impl crate::Resettable for InEp1StSpec {
    const RESET_VALUE: u32 = 0x01;
}
