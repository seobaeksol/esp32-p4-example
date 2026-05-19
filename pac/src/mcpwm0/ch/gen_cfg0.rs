#[doc = "Register `GEN_CFG0` reader"]
pub type R = crate::R<GenCfg0Spec>;
#[doc = "Register `GEN_CFG0` writer"]
pub type W = crate::W<GenCfg0Spec>;
#[doc = "Field `CFG_UPMETHOD` reader - Configures update method for PWM generator %s's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type CfgUpmethodR = crate::FieldReader;
#[doc = "Field `CFG_UPMETHOD` writer - Configures update method for PWM generator %s's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type CfgUpmethodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `T0_SEL` reader - Configures source selection for PWM generator %s event_t0, take effect immediately.\\\\0: fault_event0\\\\1: fault_event1\\\\2: fault_event2\\\\3: sync_taken\\\\4: Invalid, Select nothing"]
pub type T0SelR = crate::FieldReader;
#[doc = "Field `T0_SEL` writer - Configures source selection for PWM generator %s event_t0, take effect immediately.\\\\0: fault_event0\\\\1: fault_event1\\\\2: fault_event2\\\\3: sync_taken\\\\4: Invalid, Select nothing"]
pub type T0SelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `T1_SEL` reader - Configures source selection for PWM generator %s event_t1, take effect immediately.\\\\0: fault_event0\\\\1: fault_event1\\\\2: fault_event2\\\\3: sync_taken\\\\4: Invalid, Select nothing"]
pub type T1SelR = crate::FieldReader;
#[doc = "Field `T1_SEL` writer - Configures source selection for PWM generator %s event_t1, take effect immediately.\\\\0: fault_event0\\\\1: fault_event1\\\\2: fault_event2\\\\3: sync_taken\\\\4: Invalid, Select nothing"]
pub type T1SelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Configures update method for PWM generator %s's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn cfg_upmethod(&self) -> CfgUpmethodR {
        CfgUpmethodR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Configures source selection for PWM generator %s event_t0, take effect immediately.\\\\0: fault_event0\\\\1: fault_event1\\\\2: fault_event2\\\\3: sync_taken\\\\4: Invalid, Select nothing"]
    #[inline(always)]
    pub fn t0_sel(&self) -> T0SelR {
        T0SelR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Configures source selection for PWM generator %s event_t1, take effect immediately.\\\\0: fault_event0\\\\1: fault_event1\\\\2: fault_event2\\\\3: sync_taken\\\\4: Invalid, Select nothing"]
    #[inline(always)]
    pub fn t1_sel(&self) -> T1SelR {
        T1SelR::new(((self.bits >> 7) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures update method for PWM generator %s's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn cfg_upmethod(&mut self) -> CfgUpmethodW<'_, GenCfg0Spec> {
        CfgUpmethodW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Configures source selection for PWM generator %s event_t0, take effect immediately.\\\\0: fault_event0\\\\1: fault_event1\\\\2: fault_event2\\\\3: sync_taken\\\\4: Invalid, Select nothing"]
    #[inline(always)]
    pub fn t0_sel(&mut self) -> T0SelW<'_, GenCfg0Spec> {
        T0SelW::new(self, 4)
    }
    #[doc = "Bits 7:9 - Configures source selection for PWM generator %s event_t1, take effect immediately.\\\\0: fault_event0\\\\1: fault_event1\\\\2: fault_event2\\\\3: sync_taken\\\\4: Invalid, Select nothing"]
    #[inline(always)]
    pub fn t1_sel(&mut self) -> T1SelW<'_, GenCfg0Spec> {
        T1SelW::new(self, 7)
    }
}
#[doc = "Generator0 fault event T0 and T1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`gen_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GenCfg0Spec;
impl crate::RegisterSpec for GenCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen_cfg0::R`](R) reader structure"]
impl crate::Readable for GenCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`gen_cfg0::W`](W) writer structure"]
impl crate::Writable for GenCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GEN_CFG0 to value 0"]
impl crate::Resettable for GenCfg0Spec {}
