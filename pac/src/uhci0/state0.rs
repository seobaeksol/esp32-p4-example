#[doc = "Register `STATE0` reader"]
pub type R = crate::R<State0Spec>;
#[doc = "Field `RX_ERR_CAUSE` reader - Indicates the error types when DMA receives the error frame. 3'b001: UHCI packet checksum error. 3'b010: UHCI packet sequence number error. 3'b011: UHCI packet CRC bit error. 3'b100: find 0xC0, but received packet is uncompleted. 3'b101: 0xC0 is not found, but received packet is completed. 3'b110: CRC check error."]
pub type RxErrCauseR = crate::FieldReader;
#[doc = "Field `DECODE_STATE` reader - Indicates UHCI decoder status."]
pub type DecodeStateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Indicates the error types when DMA receives the error frame. 3'b001: UHCI packet checksum error. 3'b010: UHCI packet sequence number error. 3'b011: UHCI packet CRC bit error. 3'b100: find 0xC0, but received packet is uncompleted. 3'b101: 0xC0 is not found, but received packet is completed. 3'b110: CRC check error."]
    #[inline(always)]
    pub fn rx_err_cause(&self) -> RxErrCauseR {
        RxErrCauseR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Indicates UHCI decoder status."]
    #[inline(always)]
    pub fn decode_state(&self) -> DecodeStateR {
        DecodeStateR::new(((self.bits >> 3) & 7) as u8)
    }
}
#[doc = "UHCI Receive Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`state0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct State0Spec;
impl crate::RegisterSpec for State0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state0::R`](R) reader structure"]
impl crate::Readable for State0Spec {}
#[doc = "`reset()` method sets STATE0 to value 0"]
impl crate::Resettable for State0Spec {}
