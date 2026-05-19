#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `JTAG_IN_FLUSH` reader - The raw interrupt status bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
pub type JtagInFlushR = crate::BitReader;
#[doc = "Field `SOF` reader - The raw interrupt status bit for the USB_DEVICE_SOF_INT interrupt."]
pub type SofR = crate::BitReader;
#[doc = "Field `SERIAL_OUT_RECV_PKT` reader - The raw interrupt status bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub type SerialOutRecvPktR = crate::BitReader;
#[doc = "Field `SERIAL_IN_EMPTY` reader - The raw interrupt status bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
pub type SerialInEmptyR = crate::BitReader;
#[doc = "Field `PID_ERR` reader - The raw interrupt status bit for the USB_DEVICE_PID_ERR_INT interrupt."]
pub type PidErrR = crate::BitReader;
#[doc = "Field `CRC5_ERR` reader - The raw interrupt status bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
pub type Crc5ErrR = crate::BitReader;
#[doc = "Field `CRC16_ERR` reader - The raw interrupt status bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
pub type Crc16ErrR = crate::BitReader;
#[doc = "Field `STUFF_ERR` reader - The raw interrupt status bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
pub type StuffErrR = crate::BitReader;
#[doc = "Field `IN_TOKEN_REC_IN_EP1` reader - The raw interrupt status bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
pub type InTokenRecInEp1R = crate::BitReader;
#[doc = "Field `USB_BUS_RESET` reader - The raw interrupt status bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
pub type UsbBusResetR = crate::BitReader;
#[doc = "Field `OUT_EP1_ZERO_PAYLOAD` reader - The raw interrupt status bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub type OutEp1ZeroPayloadR = crate::BitReader;
#[doc = "Field `OUT_EP2_ZERO_PAYLOAD` reader - The raw interrupt status bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub type OutEp2ZeroPayloadR = crate::BitReader;
#[doc = "Field `RTS_CHG` reader - The raw interrupt status bit for the USB_DEVICE_RTS_CHG_INT interrupt."]
pub type RtsChgR = crate::BitReader;
#[doc = "Field `DTR_CHG` reader - The raw interrupt status bit for the USB_DEVICE_DTR_CHG_INT interrupt."]
pub type DtrChgR = crate::BitReader;
#[doc = "Field `GET_LINE_CODE` reader - The raw interrupt status bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
pub type GetLineCodeR = crate::BitReader;
#[doc = "Field `SET_LINE_CODE` reader - The raw interrupt status bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
pub type SetLineCodeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    pub fn jtag_in_flush(&self) -> JtagInFlushR {
        JtagInFlushR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the USB_DEVICE_SOF_INT interrupt."]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    pub fn serial_out_recv_pkt(&self) -> SerialOutRecvPktR {
        SerialOutRecvPktR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn serial_in_empty(&self) -> SerialInEmptyR {
        SerialInEmptyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for the USB_DEVICE_PID_ERR_INT interrupt."]
    #[inline(always)]
    pub fn pid_err(&self) -> PidErrR {
        PidErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc5_err(&self) -> Crc5ErrR {
        Crc5ErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc16_err(&self) -> Crc16ErrR {
        Crc16ErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn stuff_err(&self) -> StuffErrR {
        StuffErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
    #[inline(always)]
    pub fn in_token_rec_in_ep1(&self) -> InTokenRecInEp1R {
        InTokenRecInEp1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt status bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    pub fn usb_bus_reset(&self) -> UsbBusResetR {
        UsbBusResetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw interrupt status bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep1_zero_payload(&self) -> OutEp1ZeroPayloadR {
        OutEp1ZeroPayloadR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt status bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep2_zero_payload(&self) -> OutEp2ZeroPayloadR {
        OutEp2ZeroPayloadR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw interrupt status bit for the USB_DEVICE_RTS_CHG_INT interrupt."]
    #[inline(always)]
    pub fn rts_chg(&self) -> RtsChgR {
        RtsChgR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw interrupt status bit for the USB_DEVICE_DTR_CHG_INT interrupt."]
    #[inline(always)]
    pub fn dtr_chg(&self) -> DtrChgR {
        DtrChgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw interrupt status bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn get_line_code(&self) -> GetLineCodeR {
        GetLineCodeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw interrupt status bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn set_line_code(&self) -> SetLineCodeR {
        SetLineCodeR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}
