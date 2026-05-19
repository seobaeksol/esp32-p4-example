#[doc = "Register `LP_CPU_PWR4` reader"]
pub type R = crate::R<LpCpuPwr4Spec>;
#[doc = "Register `LP_CPU_PWR4` writer"]
pub type W = crate::W<LpCpuPwr4Spec>;
#[doc = "Field `LP_CPU_REJECT_EN` reader - need_des"]
pub type LpCpuRejectEnR = crate::FieldReader<u32>;
#[doc = "Field `LP_CPU_REJECT_EN` writer - need_des"]
pub type LpCpuRejectEnW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_reject_en(&self) -> LpCpuRejectEnR {
        LpCpuRejectEnR::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_reject_en(&mut self) -> LpCpuRejectEnW<'_, LpCpuPwr4Spec> {
        LpCpuRejectEnW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_pwr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_cpu_pwr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpCpuPwr4Spec;
impl crate::RegisterSpec for LpCpuPwr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_cpu_pwr4::R`](R) reader structure"]
impl crate::Readable for LpCpuPwr4Spec {}
#[doc = "`write(|w| ..)` method takes [`lp_cpu_pwr4::W`](W) writer structure"]
impl crate::Writable for LpCpuPwr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_CPU_PWR4 to value 0"]
impl crate::Resettable for LpCpuPwr4Spec {}
