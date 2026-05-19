#[doc = "Register `LP_CORE_BOOT_ADDR` reader"]
pub type R = crate::R<LpCoreBootAddrSpec>;
#[doc = "Register `LP_CORE_BOOT_ADDR` writer"]
pub type W = crate::W<LpCoreBootAddrSpec>;
#[doc = "Field `LP_CPU_BOOT_ADDR` reader - need_des"]
pub type LpCpuBootAddrR = crate::FieldReader<u32>;
#[doc = "Field `LP_CPU_BOOT_ADDR` writer - need_des"]
pub type LpCpuBootAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_boot_addr(&self) -> LpCpuBootAddrR {
        LpCpuBootAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_boot_addr(&mut self) -> LpCpuBootAddrW<'_, LpCoreBootAddrSpec> {
        LpCpuBootAddrW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_core_boot_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_core_boot_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpCoreBootAddrSpec;
impl crate::RegisterSpec for LpCoreBootAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_core_boot_addr::R`](R) reader structure"]
impl crate::Readable for LpCoreBootAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_core_boot_addr::W`](W) writer structure"]
impl crate::Writable for LpCoreBootAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_CORE_BOOT_ADDR to value 0x5010_0000"]
impl crate::Resettable for LpCoreBootAddrSpec {
    const RESET_VALUE: u32 = 0x5010_0000;
}
