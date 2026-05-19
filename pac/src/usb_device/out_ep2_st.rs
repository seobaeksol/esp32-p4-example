#[doc = "Register `OUT_EP2_ST` reader"]
pub type R = crate::R<OutEp2StSpec>;
#[doc = "Field `OUT_EP2_STATE` reader - State of OUT Endpoint 2."]
pub type OutEp2StateR = crate::FieldReader;
#[doc = "Field `OUT_EP2_WR_ADDR` reader - Write data address of OUT endpoint 2. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP2_WR_ADDR-2 bytes data in OUT EP2."]
pub type OutEp2WrAddrR = crate::FieldReader;
#[doc = "Field `OUT_EP2_RD_ADDR` reader - Read data address of OUT endpoint 2."]
pub type OutEp2RdAddrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - State of OUT Endpoint 2."]
    #[inline(always)]
    pub fn out_ep2_state(&self) -> OutEp2StateR {
        OutEp2StateR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:8 - Write data address of OUT endpoint 2. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP2_WR_ADDR-2 bytes data in OUT EP2."]
    #[inline(always)]
    pub fn out_ep2_wr_addr(&self) -> OutEp2WrAddrR {
        OutEp2WrAddrR::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - Read data address of OUT endpoint 2."]
    #[inline(always)]
    pub fn out_ep2_rd_addr(&self) -> OutEp2RdAddrR {
        OutEp2RdAddrR::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
#[doc = "JTAG OUT endpoint status information.\n\nYou can [`read`](crate::Reg::read) this register and get [`out_ep2_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutEp2StSpec;
impl crate::RegisterSpec for OutEp2StSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_ep2_st::R`](R) reader structure"]
impl crate::Readable for OutEp2StSpec {}
#[doc = "`reset()` method sets OUT_EP2_ST to value 0"]
impl crate::Resettable for OutEp2StSpec {}
