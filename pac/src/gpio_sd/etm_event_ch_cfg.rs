#[doc = "Register `ETM_EVENT_CH%s_CFG` reader"]
pub type R = crate::R<EtmEventChCfgSpec>;
#[doc = "Register `ETM_EVENT_CH%s_CFG` writer"]
pub type W = crate::W<EtmEventChCfgSpec>;
#[doc = "Field `EVENT_SEL` reader - Etm event channel select gpio."]
pub type EventSelR = crate::FieldReader;
#[doc = "Field `EVENT_SEL` writer - Etm event channel select gpio."]
pub type EventSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `EVENT_EN` reader - Etm event send enable bit."]
pub type EventEnR = crate::BitReader;
#[doc = "Field `EVENT_EN` writer - Etm event send enable bit."]
pub type EventEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Etm event channel select gpio."]
    #[inline(always)]
    pub fn event_sel(&self) -> EventSelR {
        EventSelR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Etm event send enable bit."]
    #[inline(always)]
    pub fn event_en(&self) -> EventEnR {
        EventEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Etm event channel select gpio."]
    #[inline(always)]
    pub fn event_sel(&mut self) -> EventSelW<'_, EtmEventChCfgSpec> {
        EventSelW::new(self, 0)
    }
    #[doc = "Bit 7 - Etm event send enable bit."]
    #[inline(always)]
    pub fn event_en(&mut self) -> EventEnW<'_, EtmEventChCfgSpec> {
        EventEnW::new(self, 7)
    }
}
#[doc = "Etm Config register of Channel%s\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_event_ch_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_event_ch_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmEventChCfgSpec;
impl crate::RegisterSpec for EtmEventChCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_event_ch_cfg::R`](R) reader structure"]
impl crate::Readable for EtmEventChCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`etm_event_ch_cfg::W`](W) writer structure"]
impl crate::Writable for EtmEventChCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_EVENT_CH%s_CFG to value 0"]
impl crate::Resettable for EtmEventChCfgSpec {}
