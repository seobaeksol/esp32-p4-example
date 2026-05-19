#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<IntEnaSpec>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<IntEnaSpec>;
#[doc = "Field `JTAG_IN_FLUSH` reader - The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
pub type JtagInFlushR = crate::BitReader;
#[doc = "Field `JTAG_IN_FLUSH` writer - The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
pub type JtagInFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF` reader - The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt."]
pub type SofR = crate::BitReader;
#[doc = "Field `SOF` writer - The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt."]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERIAL_OUT_RECV_PKT` reader - The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub type SerialOutRecvPktR = crate::BitReader;
#[doc = "Field `SERIAL_OUT_RECV_PKT` writer - The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub type SerialOutRecvPktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERIAL_IN_EMPTY` reader - The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
pub type SerialInEmptyR = crate::BitReader;
#[doc = "Field `SERIAL_IN_EMPTY` writer - The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
pub type SerialInEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID_ERR` reader - The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt."]
pub type PidErrR = crate::BitReader;
#[doc = "Field `PID_ERR` writer - The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt."]
pub type PidErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC5_ERR` reader - The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
pub type Crc5ErrR = crate::BitReader;
#[doc = "Field `CRC5_ERR` writer - The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
pub type Crc5ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC16_ERR` reader - The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
pub type Crc16ErrR = crate::BitReader;
#[doc = "Field `CRC16_ERR` writer - The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
pub type Crc16ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUFF_ERR` reader - The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
pub type StuffErrR = crate::BitReader;
#[doc = "Field `STUFF_ERR` writer - The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
pub type StuffErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_TOKEN_REC_IN_EP1` reader - The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
pub type InTokenRecInEp1R = crate::BitReader;
#[doc = "Field `IN_TOKEN_REC_IN_EP1` writer - The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
pub type InTokenRecInEp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_BUS_RESET` reader - The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
pub type UsbBusResetR = crate::BitReader;
#[doc = "Field `USB_BUS_RESET` writer - The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
pub type UsbBusResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EP1_ZERO_PAYLOAD` reader - The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub type OutEp1ZeroPayloadR = crate::BitReader;
#[doc = "Field `OUT_EP1_ZERO_PAYLOAD` writer - The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub type OutEp1ZeroPayloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EP2_ZERO_PAYLOAD` reader - The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub type OutEp2ZeroPayloadR = crate::BitReader;
#[doc = "Field `OUT_EP2_ZERO_PAYLOAD` writer - The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub type OutEp2ZeroPayloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS_CHG` reader - The interrupt enable bit for the USB_DEVICE_RTS_CHG_INT interrupt."]
pub type RtsChgR = crate::BitReader;
#[doc = "Field `RTS_CHG` writer - The interrupt enable bit for the USB_DEVICE_RTS_CHG_INT interrupt."]
pub type RtsChgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTR_CHG` reader - The interrupt enable bit for the USB_DEVICE_DTR_CHG_INT interrupt."]
pub type DtrChgR = crate::BitReader;
#[doc = "Field `DTR_CHG` writer - The interrupt enable bit for the USB_DEVICE_DTR_CHG_INT interrupt."]
pub type DtrChgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GET_LINE_CODE` reader - The interrupt enable bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
pub type GetLineCodeR = crate::BitReader;
#[doc = "Field `GET_LINE_CODE` writer - The interrupt enable bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
pub type GetLineCodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_LINE_CODE` reader - The interrupt enable bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
pub type SetLineCodeR = crate::BitReader;
#[doc = "Field `SET_LINE_CODE` writer - The interrupt enable bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
pub type SetLineCodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    pub fn jtag_in_flush(&self) -> JtagInFlushR {
        JtagInFlushR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt."]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    pub fn serial_out_recv_pkt(&self) -> SerialOutRecvPktR {
        SerialOutRecvPktR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn serial_in_empty(&self) -> SerialInEmptyR {
        SerialInEmptyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt."]
    #[inline(always)]
    pub fn pid_err(&self) -> PidErrR {
        PidErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc5_err(&self) -> Crc5ErrR {
        Crc5ErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc16_err(&self) -> Crc16ErrR {
        Crc16ErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn stuff_err(&self) -> StuffErrR {
        StuffErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
    #[inline(always)]
    pub fn in_token_rec_in_ep1(&self) -> InTokenRecInEp1R {
        InTokenRecInEp1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    pub fn usb_bus_reset(&self) -> UsbBusResetR {
        UsbBusResetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep1_zero_payload(&self) -> OutEp1ZeroPayloadR {
        OutEp1ZeroPayloadR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep2_zero_payload(&self) -> OutEp2ZeroPayloadR {
        OutEp2ZeroPayloadR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the USB_DEVICE_RTS_CHG_INT interrupt."]
    #[inline(always)]
    pub fn rts_chg(&self) -> RtsChgR {
        RtsChgR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for the USB_DEVICE_DTR_CHG_INT interrupt."]
    #[inline(always)]
    pub fn dtr_chg(&self) -> DtrChgR {
        DtrChgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn get_line_code(&self) -> GetLineCodeR {
        GetLineCodeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn set_line_code(&self) -> SetLineCodeR {
        SetLineCodeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    pub fn jtag_in_flush(&mut self) -> JtagInFlushW<'_, IntEnaSpec> {
        JtagInFlushW::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt."]
    #[inline(always)]
    pub fn sof(&mut self) -> SofW<'_, IntEnaSpec> {
        SofW::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    pub fn serial_out_recv_pkt(&mut self) -> SerialOutRecvPktW<'_, IntEnaSpec> {
        SerialOutRecvPktW::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn serial_in_empty(&mut self) -> SerialInEmptyW<'_, IntEnaSpec> {
        SerialInEmptyW::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt."]
    #[inline(always)]
    pub fn pid_err(&mut self) -> PidErrW<'_, IntEnaSpec> {
        PidErrW::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc5_err(&mut self) -> Crc5ErrW<'_, IntEnaSpec> {
        Crc5ErrW::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc16_err(&mut self) -> Crc16ErrW<'_, IntEnaSpec> {
        Crc16ErrW::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn stuff_err(&mut self) -> StuffErrW<'_, IntEnaSpec> {
        StuffErrW::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
    #[inline(always)]
    pub fn in_token_rec_in_ep1(&mut self) -> InTokenRecInEp1W<'_, IntEnaSpec> {
        InTokenRecInEp1W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    pub fn usb_bus_reset(&mut self) -> UsbBusResetW<'_, IntEnaSpec> {
        UsbBusResetW::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep1_zero_payload(&mut self) -> OutEp1ZeroPayloadW<'_, IntEnaSpec> {
        OutEp1ZeroPayloadW::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep2_zero_payload(&mut self) -> OutEp2ZeroPayloadW<'_, IntEnaSpec> {
        OutEp2ZeroPayloadW::new(self, 11)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the USB_DEVICE_RTS_CHG_INT interrupt."]
    #[inline(always)]
    pub fn rts_chg(&mut self) -> RtsChgW<'_, IntEnaSpec> {
        RtsChgW::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt enable bit for the USB_DEVICE_DTR_CHG_INT interrupt."]
    #[inline(always)]
    pub fn dtr_chg(&mut self) -> DtrChgW<'_, IntEnaSpec> {
        DtrChgW::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt enable bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn get_line_code(&mut self) -> GetLineCodeW<'_, IntEnaSpec> {
        GetLineCodeW::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt enable bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn set_line_code(&mut self) -> SetLineCodeW<'_, IntEnaSpec> {
        SetLineCodeW::new(self, 15)
    }
}
#[doc = "Interrupt enable status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnaSpec;
impl crate::RegisterSpec for IntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for IntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for IntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for IntEnaSpec {}
