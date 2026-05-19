#[doc = "Register `LP_CPU_PWR1` writer"]
pub type W = crate::W<LpCpuPwr1Spec>;
#[doc = "Field `LP_CPU_SLEEP_REQ` writer - need_des"]
pub type LpCpuSleepReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_sleep_req(&mut self) -> LpCpuSleepReqW<'_, LpCpuPwr1Spec> {
        LpCpuSleepReqW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_cpu_pwr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpCpuPwr1Spec;
impl crate::RegisterSpec for LpCpuPwr1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lp_cpu_pwr1::W`](W) writer structure"]
impl crate::Writable for LpCpuPwr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_CPU_PWR1 to value 0"]
impl crate::Resettable for LpCpuPwr1Spec {}
