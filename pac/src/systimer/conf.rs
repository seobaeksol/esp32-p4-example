#[doc = "Register `CONF` reader"]
pub type R = crate::R<ConfSpec>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<ConfSpec>;
#[doc = "Field `SYSTIMER_CLK_FO` reader - systimer clock force on"]
pub type SystimerClkFoR = crate::BitReader;
#[doc = "Field `SYSTIMER_CLK_FO` writer - systimer clock force on"]
pub type SystimerClkFoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_EN` reader - enable systimer's etm task and event"]
pub type EtmEnR = crate::BitReader;
#[doc = "Field `ETM_EN` writer - enable systimer's etm task and event"]
pub type EtmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET2_WORK_EN` reader - target2 work enable"]
pub type Target2WorkEnR = crate::BitReader;
#[doc = "Field `TARGET2_WORK_EN` writer - target2 work enable"]
pub type Target2WorkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET1_WORK_EN` reader - target1 work enable"]
pub type Target1WorkEnR = crate::BitReader;
#[doc = "Field `TARGET1_WORK_EN` writer - target1 work enable"]
pub type Target1WorkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET0_WORK_EN` reader - target0 work enable"]
pub type Target0WorkEnR = crate::BitReader;
#[doc = "Field `TARGET0_WORK_EN` writer - target0 work enable"]
pub type Target0WorkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_UNIT1_CORE1_STALL_EN` reader - If timer unit1 is stalled when core1 stalled"]
pub type TimerUnit1Core1StallEnR = crate::BitReader;
#[doc = "Field `TIMER_UNIT1_CORE1_STALL_EN` writer - If timer unit1 is stalled when core1 stalled"]
pub type TimerUnit1Core1StallEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_UNIT1_CORE0_STALL_EN` reader - If timer unit1 is stalled when core0 stalled"]
pub type TimerUnit1Core0StallEnR = crate::BitReader;
#[doc = "Field `TIMER_UNIT1_CORE0_STALL_EN` writer - If timer unit1 is stalled when core0 stalled"]
pub type TimerUnit1Core0StallEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_UNIT0_CORE1_STALL_EN` reader - If timer unit0 is stalled when core1 stalled"]
pub type TimerUnit0Core1StallEnR = crate::BitReader;
#[doc = "Field `TIMER_UNIT0_CORE1_STALL_EN` writer - If timer unit0 is stalled when core1 stalled"]
pub type TimerUnit0Core1StallEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_UNIT0_CORE0_STALL_EN` reader - If timer unit0 is stalled when core0 stalled"]
pub type TimerUnit0Core0StallEnR = crate::BitReader;
#[doc = "Field `TIMER_UNIT0_CORE0_STALL_EN` writer - If timer unit0 is stalled when core0 stalled"]
pub type TimerUnit0Core0StallEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_UNIT1_WORK_EN` reader - timer unit1 work enable"]
pub type TimerUnit1WorkEnR = crate::BitReader;
#[doc = "Field `TIMER_UNIT1_WORK_EN` writer - timer unit1 work enable"]
pub type TimerUnit1WorkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_UNIT0_WORK_EN` reader - timer unit0 work enable"]
pub type TimerUnit0WorkEnR = crate::BitReader;
#[doc = "Field `TIMER_UNIT0_WORK_EN` writer - timer unit0 work enable"]
pub type TimerUnit0WorkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - register file clk gating"]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - register file clk gating"]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - systimer clock force on"]
    #[inline(always)]
    pub fn systimer_clk_fo(&self) -> SystimerClkFoR {
        SystimerClkFoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable systimer's etm task and event"]
    #[inline(always)]
    pub fn etm_en(&self) -> EtmEnR {
        EtmEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 22 - target2 work enable"]
    #[inline(always)]
    pub fn target2_work_en(&self) -> Target2WorkEnR {
        Target2WorkEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - target1 work enable"]
    #[inline(always)]
    pub fn target1_work_en(&self) -> Target1WorkEnR {
        Target1WorkEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - target0 work enable"]
    #[inline(always)]
    pub fn target0_work_en(&self) -> Target0WorkEnR {
        Target0WorkEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - If timer unit1 is stalled when core1 stalled"]
    #[inline(always)]
    pub fn timer_unit1_core1_stall_en(&self) -> TimerUnit1Core1StallEnR {
        TimerUnit1Core1StallEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - If timer unit1 is stalled when core0 stalled"]
    #[inline(always)]
    pub fn timer_unit1_core0_stall_en(&self) -> TimerUnit1Core0StallEnR {
        TimerUnit1Core0StallEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - If timer unit0 is stalled when core1 stalled"]
    #[inline(always)]
    pub fn timer_unit0_core1_stall_en(&self) -> TimerUnit0Core1StallEnR {
        TimerUnit0Core1StallEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - If timer unit0 is stalled when core0 stalled"]
    #[inline(always)]
    pub fn timer_unit0_core0_stall_en(&self) -> TimerUnit0Core0StallEnR {
        TimerUnit0Core0StallEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - timer unit1 work enable"]
    #[inline(always)]
    pub fn timer_unit1_work_en(&self) -> TimerUnit1WorkEnR {
        TimerUnit1WorkEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - timer unit0 work enable"]
    #[inline(always)]
    pub fn timer_unit0_work_en(&self) -> TimerUnit0WorkEnR {
        TimerUnit0WorkEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - register file clk gating"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - systimer clock force on"]
    #[inline(always)]
    pub fn systimer_clk_fo(&mut self) -> SystimerClkFoW<'_, ConfSpec> {
        SystimerClkFoW::new(self, 0)
    }
    #[doc = "Bit 1 - enable systimer's etm task and event"]
    #[inline(always)]
    pub fn etm_en(&mut self) -> EtmEnW<'_, ConfSpec> {
        EtmEnW::new(self, 1)
    }
    #[doc = "Bit 22 - target2 work enable"]
    #[inline(always)]
    pub fn target2_work_en(&mut self) -> Target2WorkEnW<'_, ConfSpec> {
        Target2WorkEnW::new(self, 22)
    }
    #[doc = "Bit 23 - target1 work enable"]
    #[inline(always)]
    pub fn target1_work_en(&mut self) -> Target1WorkEnW<'_, ConfSpec> {
        Target1WorkEnW::new(self, 23)
    }
    #[doc = "Bit 24 - target0 work enable"]
    #[inline(always)]
    pub fn target0_work_en(&mut self) -> Target0WorkEnW<'_, ConfSpec> {
        Target0WorkEnW::new(self, 24)
    }
    #[doc = "Bit 25 - If timer unit1 is stalled when core1 stalled"]
    #[inline(always)]
    pub fn timer_unit1_core1_stall_en(&mut self) -> TimerUnit1Core1StallEnW<'_, ConfSpec> {
        TimerUnit1Core1StallEnW::new(self, 25)
    }
    #[doc = "Bit 26 - If timer unit1 is stalled when core0 stalled"]
    #[inline(always)]
    pub fn timer_unit1_core0_stall_en(&mut self) -> TimerUnit1Core0StallEnW<'_, ConfSpec> {
        TimerUnit1Core0StallEnW::new(self, 26)
    }
    #[doc = "Bit 27 - If timer unit0 is stalled when core1 stalled"]
    #[inline(always)]
    pub fn timer_unit0_core1_stall_en(&mut self) -> TimerUnit0Core1StallEnW<'_, ConfSpec> {
        TimerUnit0Core1StallEnW::new(self, 27)
    }
    #[doc = "Bit 28 - If timer unit0 is stalled when core0 stalled"]
    #[inline(always)]
    pub fn timer_unit0_core0_stall_en(&mut self) -> TimerUnit0Core0StallEnW<'_, ConfSpec> {
        TimerUnit0Core0StallEnW::new(self, 28)
    }
    #[doc = "Bit 29 - timer unit1 work enable"]
    #[inline(always)]
    pub fn timer_unit1_work_en(&mut self) -> TimerUnit1WorkEnW<'_, ConfSpec> {
        TimerUnit1WorkEnW::new(self, 29)
    }
    #[doc = "Bit 30 - timer unit0 work enable"]
    #[inline(always)]
    pub fn timer_unit0_work_en(&mut self) -> TimerUnit0WorkEnW<'_, ConfSpec> {
        TimerUnit0WorkEnW::new(self, 30)
    }
    #[doc = "Bit 31 - register file clk gating"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, ConfSpec> {
        ClkEnW::new(self, 31)
    }
}
#[doc = "Configure system timer clock\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfSpec;
impl crate::RegisterSpec for ConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for ConfSpec {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for ConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF to value 0x4600_0000"]
impl crate::Resettable for ConfSpec {
    const RESET_VALUE: u32 = 0x4600_0000;
}
