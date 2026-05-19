#[doc = "Register `FAULT_DETECT` reader"]
pub type R = crate::R<FaultDetectSpec>;
#[doc = "Register `FAULT_DETECT` writer"]
pub type W = crate::W<FaultDetectSpec>;
#[doc = "Field `F0_EN` reader - Configures whether or not to enable event_f0 generation.\\\\0: Disable\\\\1: Enable"]
pub type F0EnR = crate::BitReader;
#[doc = "Field `F0_EN` writer - Configures whether or not to enable event_f0 generation.\\\\0: Disable\\\\1: Enable"]
pub type F0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F1_EN` reader - Configures whether or not to enable event_f1 generation.\\\\0: Disable\\\\1: Enable"]
pub type F1EnR = crate::BitReader;
#[doc = "Field `F1_EN` writer - Configures whether or not to enable event_f1 generation.\\\\0: Disable\\\\1: Enable"]
pub type F1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F2_EN` reader - Configures whether or not to enable event_f2 generation.\\\\0: Disable\\\\1: Enable"]
pub type F2EnR = crate::BitReader;
#[doc = "Field `F2_EN` writer - Configures whether or not to enable event_f2 generation.\\\\0: Disable\\\\1: Enable"]
pub type F2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F0_POLE` reader - Configures event_f0 trigger polarity on FAULT0 source from GPIO matrix.\\\\0: Level low\\\\1: Level high"]
pub type F0PoleR = crate::BitReader;
#[doc = "Field `F0_POLE` writer - Configures event_f0 trigger polarity on FAULT0 source from GPIO matrix.\\\\0: Level low\\\\1: Level high"]
pub type F0PoleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F1_POLE` reader - Configures event_f1 trigger polarity on FAULT1 source from GPIO matrix.\\\\0: Level low\\\\1: Level high"]
pub type F1PoleR = crate::BitReader;
#[doc = "Field `F1_POLE` writer - Configures event_f1 trigger polarity on FAULT1 source from GPIO matrix.\\\\0: Level low\\\\1: Level high"]
pub type F1PoleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F2_POLE` reader - Configures event_f2 trigger polarity on FAULT2 source from GPIO matrix.\\\\0: Level low\\\\1: Level high"]
pub type F2PoleR = crate::BitReader;
#[doc = "Field `F2_POLE` writer - Configures event_f2 trigger polarity on FAULT2 source from GPIO matrix.\\\\0: Level low\\\\1: Level high"]
pub type F2PoleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENT_F0` reader - Represents whether or not an event_f0 is on going.\\\\0: No action\\\\1: On going"]
pub type EventF0R = crate::BitReader;
#[doc = "Field `EVENT_F1` reader - Represents whether or not an event_f1 is on going.\\\\0: No action\\\\1: On going"]
pub type EventF1R = crate::BitReader;
#[doc = "Field `EVENT_F2` reader - Represents whether or not an event_f2 is on going.\\\\0: No action\\\\1: On going"]
pub type EventF2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable event_f0 generation.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f0_en(&self) -> F0EnR {
        F0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable event_f1 generation.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f1_en(&self) -> F1EnR {
        F1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to enable event_f2 generation.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f2_en(&self) -> F2EnR {
        F2EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures event_f0 trigger polarity on FAULT0 source from GPIO matrix.\\\\0: Level low\\\\1: Level high"]
    #[inline(always)]
    pub fn f0_pole(&self) -> F0PoleR {
        F0PoleR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures event_f1 trigger polarity on FAULT1 source from GPIO matrix.\\\\0: Level low\\\\1: Level high"]
    #[inline(always)]
    pub fn f1_pole(&self) -> F1PoleR {
        F1PoleR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures event_f2 trigger polarity on FAULT2 source from GPIO matrix.\\\\0: Level low\\\\1: Level high"]
    #[inline(always)]
    pub fn f2_pole(&self) -> F2PoleR {
        F2PoleR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents whether or not an event_f0 is on going.\\\\0: No action\\\\1: On going"]
    #[inline(always)]
    pub fn event_f0(&self) -> EventF0R {
        EventF0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents whether or not an event_f1 is on going.\\\\0: No action\\\\1: On going"]
    #[inline(always)]
    pub fn event_f1(&self) -> EventF1R {
        EventF1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents whether or not an event_f2 is on going.\\\\0: No action\\\\1: On going"]
    #[inline(always)]
    pub fn event_f2(&self) -> EventF2R {
        EventF2R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable event_f0 generation.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f0_en(&mut self) -> F0EnW<'_, FaultDetectSpec> {
        F0EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable event_f1 generation.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f1_en(&mut self) -> F1EnW<'_, FaultDetectSpec> {
        F1EnW::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to enable event_f2 generation.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f2_en(&mut self) -> F2EnW<'_, FaultDetectSpec> {
        F2EnW::new(self, 2)
    }
    #[doc = "Bit 3 - Configures event_f0 trigger polarity on FAULT0 source from GPIO matrix.\\\\0: Level low\\\\1: Level high"]
    #[inline(always)]
    pub fn f0_pole(&mut self) -> F0PoleW<'_, FaultDetectSpec> {
        F0PoleW::new(self, 3)
    }
    #[doc = "Bit 4 - Configures event_f1 trigger polarity on FAULT1 source from GPIO matrix.\\\\0: Level low\\\\1: Level high"]
    #[inline(always)]
    pub fn f1_pole(&mut self) -> F1PoleW<'_, FaultDetectSpec> {
        F1PoleW::new(self, 4)
    }
    #[doc = "Bit 5 - Configures event_f2 trigger polarity on FAULT2 source from GPIO matrix.\\\\0: Level low\\\\1: Level high"]
    #[inline(always)]
    pub fn f2_pole(&mut self) -> F2PoleW<'_, FaultDetectSpec> {
        F2PoleW::new(self, 5)
    }
}
#[doc = "Fault detection configuration and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_detect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_detect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaultDetectSpec;
impl crate::RegisterSpec for FaultDetectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fault_detect::R`](R) reader structure"]
impl crate::Readable for FaultDetectSpec {}
#[doc = "`write(|w| ..)` method takes [`fault_detect::W`](W) writer structure"]
impl crate::Writable for FaultDetectSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FAULT_DETECT to value 0"]
impl crate::Resettable for FaultDetectSpec {}
