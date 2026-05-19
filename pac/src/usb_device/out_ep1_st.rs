#[doc = "Register `OUT_EP1_ST` reader"]
pub type R = crate::R<OutEp1StSpec>;
#[doc = "Field `OUT_EP1_STATE` reader - State of OUT Endpoint 1."]
pub type OutEp1StateR = crate::FieldReader;
#[doc = "Field `OUT_EP1_WR_ADDR` reader - Write data address of OUT endpoint 1. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP1_WR_ADDR-2 bytes data in OUT EP1."]
pub type OutEp1WrAddrR = crate::FieldReader;
#[doc = "Field `OUT_EP1_RD_ADDR` reader - Read data address of OUT endpoint 1."]
pub type OutEp1RdAddrR = crate::FieldReader;
#[doc = "Field `OUT_EP1_REC_DATA_CNT` reader - Data count in OUT endpoint 1 when one packet is received."]
pub type OutEp1RecDataCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - State of OUT Endpoint 1."]
    #[inline(always)]
    pub fn out_ep1_state(&self) -> OutEp1StateR {
        OutEp1StateR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:8 - Write data address of OUT endpoint 1. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP1_WR_ADDR-2 bytes data in OUT EP1."]
    #[inline(always)]
    pub fn out_ep1_wr_addr(&self) -> OutEp1WrAddrR {
        OutEp1WrAddrR::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - Read data address of OUT endpoint 1."]
    #[inline(always)]
    pub fn out_ep1_rd_addr(&self) -> OutEp1RdAddrR {
        OutEp1RdAddrR::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Data count in OUT endpoint 1 when one packet is received."]
    #[inline(always)]
    pub fn out_ep1_rec_data_cnt(&self) -> OutEp1RecDataCntR {
        OutEp1RecDataCntR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "CDC-ACM OUT endpoint status information.\n\nYou can [`read`](crate::Reg::read) this register and get [`out_ep1_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutEp1StSpec;
impl crate::RegisterSpec for OutEp1StSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_ep1_st::R`](R) reader structure"]
impl crate::Readable for OutEp1StSpec {}
#[doc = "`reset()` method sets OUT_EP1_ST to value 0"]
impl crate::Resettable for OutEp1StSpec {}
