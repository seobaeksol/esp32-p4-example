#[doc = "Register `CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1` reader"]
pub type R = crate::R<CoreXIram0Dram0ExceptionMonitor1Spec>;
#[doc = "Register `CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1` writer"]
pub type W = crate::W<CoreXIram0Dram0ExceptionMonitor1Spec>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1` reader - reg_core_x_iram0_dram0_limit_cycle_1"]
pub type CoreXIram0Dram0LimitCycle1R = crate::FieldReader<u32>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1` writer - reg_core_x_iram0_dram0_limit_cycle_1"]
pub type CoreXIram0Dram0LimitCycle1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - reg_core_x_iram0_dram0_limit_cycle_1"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_limit_cycle_1(&self) -> CoreXIram0Dram0LimitCycle1R {
        CoreXIram0Dram0LimitCycle1R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - reg_core_x_iram0_dram0_limit_cycle_1"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_limit_cycle_1(
        &mut self,
    ) -> CoreXIram0Dram0LimitCycle1W<'_, CoreXIram0Dram0ExceptionMonitor1Spec> {
        CoreXIram0Dram0LimitCycle1W::new(self, 0)
    }
}
#[doc = "exception monitor status register7\n\nYou can [`read`](crate::Reg::read) this register and get [`core_x_iram0_dram0_exception_monitor_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_x_iram0_dram0_exception_monitor_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreXIram0Dram0ExceptionMonitor1Spec;
impl crate::RegisterSpec for CoreXIram0Dram0ExceptionMonitor1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_x_iram0_dram0_exception_monitor_1::R`](R) reader structure"]
impl crate::Readable for CoreXIram0Dram0ExceptionMonitor1Spec {}
#[doc = "`write(|w| ..)` method takes [`core_x_iram0_dram0_exception_monitor_1::W`](W) writer structure"]
impl crate::Writable for CoreXIram0Dram0ExceptionMonitor1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 to value 0"]
impl crate::Resettable for CoreXIram0Dram0ExceptionMonitor1Spec {}
