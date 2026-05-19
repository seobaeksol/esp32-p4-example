#[doc = "Register `SWFC_CONF1` reader"]
pub type R = crate::R<SwfcConf1Spec>;
#[doc = "Register `SWFC_CONF1` writer"]
pub type W = crate::W<SwfcConf1Spec>;
#[doc = "Field `XON_THRESHOLD` reader - When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1 it will send a Xon char."]
pub type XonThresholdR = crate::FieldReader;
#[doc = "Field `XON_THRESHOLD` writer - When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1 it will send a Xon char."]
pub type XonThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `XOFF_THRESHOLD` reader - When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1 it will send a Xoff char."]
pub type XoffThresholdR = crate::FieldReader;
#[doc = "Field `XOFF_THRESHOLD` writer - When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1 it will send a Xoff char."]
pub type XoffThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 3:7 - When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1 it will send a Xon char."]
    #[inline(always)]
    pub fn xon_threshold(&self) -> XonThresholdR {
        XonThresholdR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1 it will send a Xoff char."]
    #[inline(always)]
    pub fn xoff_threshold(&self) -> XoffThresholdR {
        XoffThresholdR::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1 it will send a Xon char."]
    #[inline(always)]
    pub fn xon_threshold(&mut self) -> XonThresholdW<'_, SwfcConf1Spec> {
        XonThresholdW::new(self, 3)
    }
    #[doc = "Bits 11:15 - When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1 it will send a Xoff char."]
    #[inline(always)]
    pub fn xoff_threshold(&mut self) -> XoffThresholdW<'_, SwfcConf1Spec> {
        XoffThresholdW::new(self, 11)
    }
}
#[doc = "Software flow-control character configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`swfc_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swfc_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwfcConf1Spec;
impl crate::RegisterSpec for SwfcConf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swfc_conf1::R`](R) reader structure"]
impl crate::Readable for SwfcConf1Spec {}
#[doc = "`write(|w| ..)` method takes [`swfc_conf1::W`](W) writer structure"]
impl crate::Writable for SwfcConf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWFC_CONF1 to value 0x6000"]
impl crate::Resettable for SwfcConf1Spec {
    const RESET_VALUE: u32 = 0x6000;
}
