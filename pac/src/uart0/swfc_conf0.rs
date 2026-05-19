#[doc = "Register `SWFC_CONF0` reader"]
pub type R = crate::R<SwfcConf0Spec>;
#[doc = "Register `SWFC_CONF0` writer"]
pub type W = crate::W<SwfcConf0Spec>;
#[doc = "Field `XON_CHAR` reader - This register stores the Xon flow control char."]
pub type XonCharR = crate::FieldReader;
#[doc = "Field `XON_CHAR` writer - This register stores the Xon flow control char."]
pub type XonCharW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `XOFF_CHAR` reader - This register stores the Xoff flow control char."]
pub type XoffCharR = crate::FieldReader;
#[doc = "Field `XOFF_CHAR` writer - This register stores the Xoff flow control char."]
pub type XoffCharW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `XON_XOFF_STILL_SEND` reader - In software flow control mode, UART Tx is disabled once UART Rx receives XOFF. In this status, UART Tx can not transmit XOFF even the received data number is larger than UART_XOFF_THRESHOLD. Set this bit to enable UART Tx can transmit XON/XOFF when UART Tx is disabled."]
pub type XonXoffStillSendR = crate::BitReader;
#[doc = "Field `XON_XOFF_STILL_SEND` writer - In software flow control mode, UART Tx is disabled once UART Rx receives XOFF. In this status, UART Tx can not transmit XOFF even the received data number is larger than UART_XOFF_THRESHOLD. Set this bit to enable UART Tx can transmit XON/XOFF when UART Tx is disabled."]
pub type XonXoffStillSendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_FLOW_CON_EN` reader - Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff."]
pub type SwFlowConEnR = crate::BitReader;
#[doc = "Field `SW_FLOW_CON_EN` writer - Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff."]
pub type SwFlowConEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XONOFF_DEL` reader - Set this bit to remove flow control char from the received data."]
pub type XonoffDelR = crate::BitReader;
#[doc = "Field `XONOFF_DEL` writer - Set this bit to remove flow control char from the received data."]
pub type XonoffDelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_XON` reader - Set this bit to enable the transmitter to go on sending data."]
pub type ForceXonR = crate::BitReader;
#[doc = "Field `FORCE_XON` writer - Set this bit to enable the transmitter to go on sending data."]
pub type ForceXonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_XOFF` reader - Set this bit to stop the transmitter from sending data."]
pub type ForceXoffR = crate::BitReader;
#[doc = "Field `FORCE_XOFF` writer - Set this bit to stop the transmitter from sending data."]
pub type ForceXoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_XON` reader - Set this bit to send Xon char. It is cleared by hardware automatically."]
pub type SendXonR = crate::BitReader;
#[doc = "Field `SEND_XON` writer - Set this bit to send Xon char. It is cleared by hardware automatically."]
pub type SendXonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_XOFF` reader - Set this bit to send Xoff char. It is cleared by hardware automatically."]
pub type SendXoffR = crate::BitReader;
#[doc = "Field `SEND_XOFF` writer - Set this bit to send Xoff char. It is cleared by hardware automatically."]
pub type SendXoffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - This register stores the Xon flow control char."]
    #[inline(always)]
    pub fn xon_char(&self) -> XonCharR {
        XonCharR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register stores the Xoff flow control char."]
    #[inline(always)]
    pub fn xoff_char(&self) -> XoffCharR {
        XoffCharR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - In software flow control mode, UART Tx is disabled once UART Rx receives XOFF. In this status, UART Tx can not transmit XOFF even the received data number is larger than UART_XOFF_THRESHOLD. Set this bit to enable UART Tx can transmit XON/XOFF when UART Tx is disabled."]
    #[inline(always)]
    pub fn xon_xoff_still_send(&self) -> XonXoffStillSendR {
        XonXoffStillSendR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff."]
    #[inline(always)]
    pub fn sw_flow_con_en(&self) -> SwFlowConEnR {
        SwFlowConEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set this bit to remove flow control char from the received data."]
    #[inline(always)]
    pub fn xonoff_del(&self) -> XonoffDelR {
        XonoffDelR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set this bit to enable the transmitter to go on sending data."]
    #[inline(always)]
    pub fn force_xon(&self) -> ForceXonR {
        ForceXonR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to stop the transmitter from sending data."]
    #[inline(always)]
    pub fn force_xoff(&self) -> ForceXoffR {
        ForceXoffR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to send Xon char. It is cleared by hardware automatically."]
    #[inline(always)]
    pub fn send_xon(&self) -> SendXonR {
        SendXonR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to send Xoff char. It is cleared by hardware automatically."]
    #[inline(always)]
    pub fn send_xoff(&self) -> SendXoffR {
        SendXoffR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register stores the Xon flow control char."]
    #[inline(always)]
    pub fn xon_char(&mut self) -> XonCharW<'_, SwfcConf0Spec> {
        XonCharW::new(self, 0)
    }
    #[doc = "Bits 8:15 - This register stores the Xoff flow control char."]
    #[inline(always)]
    pub fn xoff_char(&mut self) -> XoffCharW<'_, SwfcConf0Spec> {
        XoffCharW::new(self, 8)
    }
    #[doc = "Bit 16 - In software flow control mode, UART Tx is disabled once UART Rx receives XOFF. In this status, UART Tx can not transmit XOFF even the received data number is larger than UART_XOFF_THRESHOLD. Set this bit to enable UART Tx can transmit XON/XOFF when UART Tx is disabled."]
    #[inline(always)]
    pub fn xon_xoff_still_send(&mut self) -> XonXoffStillSendW<'_, SwfcConf0Spec> {
        XonXoffStillSendW::new(self, 16)
    }
    #[doc = "Bit 17 - Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff."]
    #[inline(always)]
    pub fn sw_flow_con_en(&mut self) -> SwFlowConEnW<'_, SwfcConf0Spec> {
        SwFlowConEnW::new(self, 17)
    }
    #[doc = "Bit 18 - Set this bit to remove flow control char from the received data."]
    #[inline(always)]
    pub fn xonoff_del(&mut self) -> XonoffDelW<'_, SwfcConf0Spec> {
        XonoffDelW::new(self, 18)
    }
    #[doc = "Bit 19 - Set this bit to enable the transmitter to go on sending data."]
    #[inline(always)]
    pub fn force_xon(&mut self) -> ForceXonW<'_, SwfcConf0Spec> {
        ForceXonW::new(self, 19)
    }
    #[doc = "Bit 20 - Set this bit to stop the transmitter from sending data."]
    #[inline(always)]
    pub fn force_xoff(&mut self) -> ForceXoffW<'_, SwfcConf0Spec> {
        ForceXoffW::new(self, 20)
    }
    #[doc = "Bit 21 - Set this bit to send Xon char. It is cleared by hardware automatically."]
    #[inline(always)]
    pub fn send_xon(&mut self) -> SendXonW<'_, SwfcConf0Spec> {
        SendXonW::new(self, 21)
    }
    #[doc = "Bit 22 - Set this bit to send Xoff char. It is cleared by hardware automatically."]
    #[inline(always)]
    pub fn send_xoff(&mut self) -> SendXoffW<'_, SwfcConf0Spec> {
        SendXoffW::new(self, 22)
    }
}
#[doc = "Software flow-control character configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`swfc_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swfc_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwfcConf0Spec;
impl crate::RegisterSpec for SwfcConf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swfc_conf0::R`](R) reader structure"]
impl crate::Readable for SwfcConf0Spec {}
#[doc = "`write(|w| ..)` method takes [`swfc_conf0::W`](W) writer structure"]
impl crate::Writable for SwfcConf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWFC_CONF0 to value 0x1311"]
impl crate::Resettable for SwfcConf0Spec {
    const RESET_VALUE: u32 = 0x1311;
}
