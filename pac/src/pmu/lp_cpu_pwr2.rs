#[doc = "Register `LP_CPU_PWR2` reader"]
pub type R = crate::R<LpCpuPwr2Spec>;
#[doc = "Register `LP_CPU_PWR2` writer"]
pub type W = crate::W<LpCpuPwr2Spec>;
#[doc = "Field `LP_CPU_WAKEUP_EN` reader - need_des"]
pub type LpCpuWakeupEnR = crate::FieldReader<u32>;
#[doc = "Field `LP_CPU_WAKEUP_EN` writer - need_des"]
pub type LpCpuWakeupEnW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_wakeup_en(&self) -> LpCpuWakeupEnR {
        LpCpuWakeupEnR::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_wakeup_en(&mut self) -> LpCpuWakeupEnW<'_, LpCpuPwr2Spec> {
        LpCpuWakeupEnW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_pwr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_cpu_pwr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpCpuPwr2Spec;
impl crate::RegisterSpec for LpCpuPwr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_cpu_pwr2::R`](R) reader structure"]
impl crate::Readable for LpCpuPwr2Spec {}
#[doc = "`write(|w| ..)` method takes [`lp_cpu_pwr2::W`](W) writer structure"]
impl crate::Writable for LpCpuPwr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_CPU_PWR2 to value 0"]
impl crate::Resettable for LpCpuPwr2Spec {}
