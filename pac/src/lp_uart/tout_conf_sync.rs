#[doc = "Register `TOUT_CONF_SYNC` reader"]
pub type R = crate::R<ToutConfSyncSpec>;
#[doc = "Register `TOUT_CONF_SYNC` writer"]
pub type W = crate::W<ToutConfSyncSpec>;
#[doc = "Field `RX_TOUT_EN` reader - This is the enble bit for uart receiver's timeout function."]
pub type RxToutEnR = crate::BitReader;
#[doc = "Field `RX_TOUT_EN` writer - This is the enble bit for uart receiver's timeout function."]
pub type RxToutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TOUT_FLOW_DIS` reader - Set this bit to stop accumulating idle_cnt when hardware flow control works."]
pub type RxToutFlowDisR = crate::BitReader;
#[doc = "Field `RX_TOUT_FLOW_DIS` writer - Set this bit to stop accumulating idle_cnt when hardware flow control works."]
pub type RxToutFlowDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TOUT_THRHD` reader - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
pub type RxToutThrhdR = crate::FieldReader<u16>;
#[doc = "Field `RX_TOUT_THRHD` writer - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
pub type RxToutThrhdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - This is the enble bit for uart receiver's timeout function."]
    #[inline(always)]
    pub fn rx_tout_en(&self) -> RxToutEnR {
        RxToutEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to stop accumulating idle_cnt when hardware flow control works."]
    #[inline(always)]
    pub fn rx_tout_flow_dis(&self) -> RxToutFlowDisR {
        RxToutFlowDisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:11 - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
    #[inline(always)]
    pub fn rx_tout_thrhd(&self) -> RxToutThrhdR {
        RxToutThrhdR::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - This is the enble bit for uart receiver's timeout function."]
    #[inline(always)]
    pub fn rx_tout_en(&mut self) -> RxToutEnW<'_, ToutConfSyncSpec> {
        RxToutEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to stop accumulating idle_cnt when hardware flow control works."]
    #[inline(always)]
    pub fn rx_tout_flow_dis(&mut self) -> RxToutFlowDisW<'_, ToutConfSyncSpec> {
        RxToutFlowDisW::new(self, 1)
    }
    #[doc = "Bits 2:11 - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
    #[inline(always)]
    pub fn rx_tout_thrhd(&mut self) -> RxToutThrhdW<'_, ToutConfSyncSpec> {
        RxToutThrhdW::new(self, 2)
    }
}
#[doc = "UART threshold and allocation configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tout_conf_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tout_conf_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ToutConfSyncSpec;
impl crate::RegisterSpec for ToutConfSyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tout_conf_sync::R`](R) reader structure"]
impl crate::Readable for ToutConfSyncSpec {}
#[doc = "`write(|w| ..)` method takes [`tout_conf_sync::W`](W) writer structure"]
impl crate::Writable for ToutConfSyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUT_CONF_SYNC to value 0x28"]
impl crate::Resettable for ToutConfSyncSpec {
    const RESET_VALUE: u32 = 0x28;
}
