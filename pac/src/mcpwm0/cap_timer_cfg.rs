#[doc = "Register `CAP_TIMER_CFG` reader"]
pub type R = crate::R<CapTimerCfgSpec>;
#[doc = "Register `CAP_TIMER_CFG` writer"]
pub type W = crate::W<CapTimerCfgSpec>;
#[doc = "Field `CAP_TIMER_EN` reader - Configures whether or not to enable capture timer increment.\\\\0: Disable\\\\1: Enable"]
pub type CapTimerEnR = crate::BitReader;
#[doc = "Field `CAP_TIMER_EN` writer - Configures whether or not to enable capture timer increment.\\\\0: Disable\\\\1: Enable"]
pub type CapTimerEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP_SYNCI_EN` reader - Configures whether or not to enable capture timer sync.\\\\0: Disable\\\\1: Enable"]
pub type CapSynciEnR = crate::BitReader;
#[doc = "Field `CAP_SYNCI_EN` writer - Configures whether or not to enable capture timer sync.\\\\0: Disable\\\\1: Enable"]
pub type CapSynciEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP_SYNCI_SEL` reader - Configures the selection of capture module sync input.\\\\0: None\\\\1: Timer0 sync_out\\\\2: Timer1 sync_out\\\\3: Timer2 sync_out\\\\4: SYNC0 from GPIO matrix\\\\5: SYNC1 from GPIO matrix\\\\6: SYNC2 from GPIO matrix\\\\7: None"]
pub type CapSynciSelR = crate::FieldReader;
#[doc = "Field `CAP_SYNCI_SEL` writer - Configures the selection of capture module sync input.\\\\0: None\\\\1: Timer0 sync_out\\\\2: Timer1 sync_out\\\\3: Timer2 sync_out\\\\4: SYNC0 from GPIO matrix\\\\5: SYNC1 from GPIO matrix\\\\6: SYNC2 from GPIO matrix\\\\7: None"]
pub type CapSynciSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CAP_SYNC_SW` writer - Configures the generation of a capture timer sync when reg_cap_synci_en is 1.\\\\0: Invalid, No effect\\\\1: Trigger a capture timer sync, capture timer is loaded with value in phase register"]
pub type CapSyncSwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable capture timer increment.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap_timer_en(&self) -> CapTimerEnR {
        CapTimerEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable capture timer sync.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap_synci_en(&self) -> CapSynciEnR {
        CapSynciEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Configures the selection of capture module sync input.\\\\0: None\\\\1: Timer0 sync_out\\\\2: Timer1 sync_out\\\\3: Timer2 sync_out\\\\4: SYNC0 from GPIO matrix\\\\5: SYNC1 from GPIO matrix\\\\6: SYNC2 from GPIO matrix\\\\7: None"]
    #[inline(always)]
    pub fn cap_synci_sel(&self) -> CapSynciSelR {
        CapSynciSelR::new(((self.bits >> 2) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable capture timer increment.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap_timer_en(&mut self) -> CapTimerEnW<'_, CapTimerCfgSpec> {
        CapTimerEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable capture timer sync.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap_synci_en(&mut self) -> CapSynciEnW<'_, CapTimerCfgSpec> {
        CapSynciEnW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Configures the selection of capture module sync input.\\\\0: None\\\\1: Timer0 sync_out\\\\2: Timer1 sync_out\\\\3: Timer2 sync_out\\\\4: SYNC0 from GPIO matrix\\\\5: SYNC1 from GPIO matrix\\\\6: SYNC2 from GPIO matrix\\\\7: None"]
    #[inline(always)]
    pub fn cap_synci_sel(&mut self) -> CapSynciSelW<'_, CapTimerCfgSpec> {
        CapSynciSelW::new(self, 2)
    }
    #[doc = "Bit 5 - Configures the generation of a capture timer sync when reg_cap_synci_en is 1.\\\\0: Invalid, No effect\\\\1: Trigger a capture timer sync, capture timer is loaded with value in phase register"]
    #[inline(always)]
    pub fn cap_sync_sw(&mut self) -> CapSyncSwW<'_, CapTimerCfgSpec> {
        CapSyncSwW::new(self, 5)
    }
}
#[doc = "Capture timer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_timer_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_timer_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapTimerCfgSpec;
impl crate::RegisterSpec for CapTimerCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_timer_cfg::R`](R) reader structure"]
impl crate::Readable for CapTimerCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cap_timer_cfg::W`](W) writer structure"]
impl crate::Writable for CapTimerCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAP_TIMER_CFG to value 0"]
impl crate::Resettable for CapTimerCfgSpec {}
