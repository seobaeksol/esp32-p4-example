#[doc = "Register `CPU_INTR_FROM_CPU%s` reader"]
pub type R = crate::R<CpuIntrFromCpuSpec>;
#[doc = "Register `CPU_INTR_FROM_CPU%s` writer"]
pub type W = crate::W<CpuIntrFromCpuSpec>;
#[doc = "Field `CPU_INTR` reader - set 1 will triger a interrupt"]
pub type CpuIntrR = crate::BitReader;
#[doc = "Field `CPU_INTR` writer - set 1 will triger a interrupt"]
pub type CpuIntrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - set 1 will triger a interrupt"]
    #[inline(always)]
    pub fn cpu_intr(&self) -> CpuIntrR {
        CpuIntrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - set 1 will triger a interrupt"]
    #[inline(always)]
    pub fn cpu_intr(&mut self) -> CpuIntrW<'_, CpuIntrFromCpuSpec> {
        CpuIntrW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_intr_from_cpu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_intr_from_cpu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuIntrFromCpuSpec;
impl crate::RegisterSpec for CpuIntrFromCpuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_intr_from_cpu::R`](R) reader structure"]
impl crate::Readable for CpuIntrFromCpuSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu_intr_from_cpu::W`](W) writer structure"]
impl crate::Writable for CpuIntrFromCpuSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_INTR_FROM_CPU%s to value 0"]
impl crate::Resettable for CpuIntrFromCpuSpec {}
