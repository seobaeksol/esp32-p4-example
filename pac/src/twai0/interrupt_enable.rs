#[doc = "Register `INTERRUPT_ENABLE` reader"]
pub type R = crate::R<InterruptEnableSpec>;
#[doc = "Register `INTERRUPT_ENABLE` writer"]
pub type W = crate::W<InterruptEnableSpec>;
#[doc = "Field `EXT_RECEIVE_INT_ENA` reader - 1: enabled, when the receive buffer status is 'full' the TWAI controller requests the respective interrupt. 0: disable"]
pub type ExtReceiveIntEnaR = crate::BitReader;
#[doc = "Field `EXT_RECEIVE_INT_ENA` writer - 1: enabled, when the receive buffer status is 'full' the TWAI controller requests the respective interrupt. 0: disable"]
pub type ExtReceiveIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_TRANSMIT_INT_ENA` reader - 1: enabled, when a message has been successfully transmitted or the transmit buffer is accessible again (e.g. after an abort transmission command), the TWAI controller requests the respective interrupt. 0: disable"]
pub type ExtTransmitIntEnaR = crate::BitReader;
#[doc = "Field `EXT_TRANSMIT_INT_ENA` writer - 1: enabled, when a message has been successfully transmitted or the transmit buffer is accessible again (e.g. after an abort transmission command), the TWAI controller requests the respective interrupt. 0: disable"]
pub type ExtTransmitIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_ERR_WARNING_INT_ENA` reader - 1: enabled, if the error or bus status change (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable"]
pub type ExtErrWarningIntEnaR = crate::BitReader;
#[doc = "Field `EXT_ERR_WARNING_INT_ENA` writer - 1: enabled, if the error or bus status change (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable"]
pub type ExtErrWarningIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_DATA_OVERRUN_INT_ENA` reader - 1: enabled, if the data overrun status bit is set (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable"]
pub type ExtDataOverrunIntEnaR = crate::BitReader;
#[doc = "Field `EXT_DATA_OVERRUN_INT_ENA` writer - 1: enabled, if the data overrun status bit is set (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable"]
pub type ExtDataOverrunIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_COUNTER_OVFL_INT_ENA` reader - enable the timestamp counter overflow interrupt request."]
pub type TsCounterOvflIntEnaR = crate::BitReader;
#[doc = "Field `TS_COUNTER_OVFL_INT_ENA` writer - enable the timestamp counter overflow interrupt request."]
pub type TsCounterOvflIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_PASSIVE_INT_ENA` reader - 1: enabled, if the error status of the TWAI controller changes from error active to error passive or vice versa, the respective interrupt is requested. 0: disable"]
pub type ErrPassiveIntEnaR = crate::BitReader;
#[doc = "Field `ERR_PASSIVE_INT_ENA` writer - 1: enabled, if the error status of the TWAI controller changes from error active to error passive or vice versa, the respective interrupt is requested. 0: disable"]
pub type ErrPassiveIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBITRATION_LOST_INT_ENA` reader - 1: enabled, if the TWAI controller has lost arbitration, the respective interrupt is requested. 0: disable"]
pub type ArbitrationLostIntEnaR = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST_INT_ENA` writer - 1: enabled, if the TWAI controller has lost arbitration, the respective interrupt is requested. 0: disable"]
pub type ArbitrationLostIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_ERR_INT_ENA` reader - 1: enabled, if an bus error has been detected, the TWAI controller requests the respective interrupt. 0: disable"]
pub type BusErrIntEnaR = crate::BitReader;
#[doc = "Field `BUS_ERR_INT_ENA` writer - 1: enabled, if an bus error has been detected, the TWAI controller requests the respective interrupt. 0: disable"]
pub type BusErrIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_INT_ENA` reader - 1: enabled, if state of TWAI become IDLE, the TWAI controller requests the respective interrupt. 0: disable"]
pub type IdleIntEnaR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: enabled, when the receive buffer status is 'full' the TWAI controller requests the respective interrupt. 0: disable"]
    #[inline(always)]
    pub fn ext_receive_int_ena(&self) -> ExtReceiveIntEnaR {
        ExtReceiveIntEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: enabled, when a message has been successfully transmitted or the transmit buffer is accessible again (e.g. after an abort transmission command), the TWAI controller requests the respective interrupt. 0: disable"]
    #[inline(always)]
    pub fn ext_transmit_int_ena(&self) -> ExtTransmitIntEnaR {
        ExtTransmitIntEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: enabled, if the error or bus status change (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable"]
    #[inline(always)]
    pub fn ext_err_warning_int_ena(&self) -> ExtErrWarningIntEnaR {
        ExtErrWarningIntEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: enabled, if the data overrun status bit is set (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable"]
    #[inline(always)]
    pub fn ext_data_overrun_int_ena(&self) -> ExtDataOverrunIntEnaR {
        ExtDataOverrunIntEnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable the timestamp counter overflow interrupt request."]
    #[inline(always)]
    pub fn ts_counter_ovfl_int_ena(&self) -> TsCounterOvflIntEnaR {
        TsCounterOvflIntEnaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: enabled, if the error status of the TWAI controller changes from error active to error passive or vice versa, the respective interrupt is requested. 0: disable"]
    #[inline(always)]
    pub fn err_passive_int_ena(&self) -> ErrPassiveIntEnaR {
        ErrPassiveIntEnaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: enabled, if the TWAI controller has lost arbitration, the respective interrupt is requested. 0: disable"]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&self) -> ArbitrationLostIntEnaR {
        ArbitrationLostIntEnaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: enabled, if an bus error has been detected, the TWAI controller requests the respective interrupt. 0: disable"]
    #[inline(always)]
    pub fn bus_err_int_ena(&self) -> BusErrIntEnaR {
        BusErrIntEnaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: enabled, if state of TWAI become IDLE, the TWAI controller requests the respective interrupt. 0: disable"]
    #[inline(always)]
    pub fn idle_int_ena(&self) -> IdleIntEnaR {
        IdleIntEnaR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: enabled, when the receive buffer status is 'full' the TWAI controller requests the respective interrupt. 0: disable"]
    #[inline(always)]
    pub fn ext_receive_int_ena(&mut self) -> ExtReceiveIntEnaW<'_, InterruptEnableSpec> {
        ExtReceiveIntEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: enabled, when a message has been successfully transmitted or the transmit buffer is accessible again (e.g. after an abort transmission command), the TWAI controller requests the respective interrupt. 0: disable"]
    #[inline(always)]
    pub fn ext_transmit_int_ena(&mut self) -> ExtTransmitIntEnaW<'_, InterruptEnableSpec> {
        ExtTransmitIntEnaW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: enabled, if the error or bus status change (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable"]
    #[inline(always)]
    pub fn ext_err_warning_int_ena(&mut self) -> ExtErrWarningIntEnaW<'_, InterruptEnableSpec> {
        ExtErrWarningIntEnaW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: enabled, if the data overrun status bit is set (see status register. Table 14), the TWAI controllerrequests the respective interrupt. 0: disable"]
    #[inline(always)]
    pub fn ext_data_overrun_int_ena(&mut self) -> ExtDataOverrunIntEnaW<'_, InterruptEnableSpec> {
        ExtDataOverrunIntEnaW::new(self, 3)
    }
    #[doc = "Bit 4 - enable the timestamp counter overflow interrupt request."]
    #[inline(always)]
    pub fn ts_counter_ovfl_int_ena(&mut self) -> TsCounterOvflIntEnaW<'_, InterruptEnableSpec> {
        TsCounterOvflIntEnaW::new(self, 4)
    }
    #[doc = "Bit 5 - 1: enabled, if the error status of the TWAI controller changes from error active to error passive or vice versa, the respective interrupt is requested. 0: disable"]
    #[inline(always)]
    pub fn err_passive_int_ena(&mut self) -> ErrPassiveIntEnaW<'_, InterruptEnableSpec> {
        ErrPassiveIntEnaW::new(self, 5)
    }
    #[doc = "Bit 6 - 1: enabled, if the TWAI controller has lost arbitration, the respective interrupt is requested. 0: disable"]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&mut self) -> ArbitrationLostIntEnaW<'_, InterruptEnableSpec> {
        ArbitrationLostIntEnaW::new(self, 6)
    }
    #[doc = "Bit 7 - 1: enabled, if an bus error has been detected, the TWAI controller requests the respective interrupt. 0: disable"]
    #[inline(always)]
    pub fn bus_err_int_ena(&mut self) -> BusErrIntEnaW<'_, InterruptEnableSpec> {
        BusErrIntEnaW::new(self, 7)
    }
}
#[doc = "Interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptEnableSpec;
impl crate::RegisterSpec for InterruptEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_enable::R`](R) reader structure"]
impl crate::Readable for InterruptEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_enable::W`](W) writer structure"]
impl crate::Writable for InterruptEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTERRUPT_ENABLE to value 0"]
impl crate::Resettable for InterruptEnableSpec {}
