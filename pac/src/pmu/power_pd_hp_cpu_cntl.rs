#[doc = "Register `POWER_PD_HP_CPU_CNTL` reader"]
pub type R = crate::R<PowerPdHpCpuCntlSpec>;
#[doc = "Register `POWER_PD_HP_CPU_CNTL` writer"]
pub type W = crate::W<PowerPdHpCpuCntlSpec>;
#[doc = "Field `FORCE_HP_CPU_RESET` reader - need_des"]
pub type ForceHpCpuResetR = crate::BitReader;
#[doc = "Field `FORCE_HP_CPU_RESET` writer - need_des"]
pub type ForceHpCpuResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_HP_CPU_ISO` reader - need_des"]
pub type ForceHpCpuIsoR = crate::BitReader;
#[doc = "Field `FORCE_HP_CPU_ISO` writer - need_des"]
pub type ForceHpCpuIsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_HP_CPU_PU` reader - need_des"]
pub type ForceHpCpuPuR = crate::BitReader;
#[doc = "Field `FORCE_HP_CPU_PU` writer - need_des"]
pub type ForceHpCpuPuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_HP_CPU_NO_RESET` reader - need_des"]
pub type ForceHpCpuNoResetR = crate::BitReader;
#[doc = "Field `FORCE_HP_CPU_NO_RESET` writer - need_des"]
pub type ForceHpCpuNoResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_HP_CPU_NO_ISO` reader - need_des"]
pub type ForceHpCpuNoIsoR = crate::BitReader;
#[doc = "Field `FORCE_HP_CPU_NO_ISO` writer - need_des"]
pub type ForceHpCpuNoIsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_HP_CPU_PD` reader - need_des"]
pub type ForceHpCpuPdR = crate::BitReader;
#[doc = "Field `FORCE_HP_CPU_PD` writer - need_des"]
pub type ForceHpCpuPdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_reset(&self) -> ForceHpCpuResetR {
        ForceHpCpuResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_iso(&self) -> ForceHpCpuIsoR {
        ForceHpCpuIsoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_pu(&self) -> ForceHpCpuPuR {
        ForceHpCpuPuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_no_reset(&self) -> ForceHpCpuNoResetR {
        ForceHpCpuNoResetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_no_iso(&self) -> ForceHpCpuNoIsoR {
        ForceHpCpuNoIsoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_pd(&self) -> ForceHpCpuPdR {
        ForceHpCpuPdR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_reset(&mut self) -> ForceHpCpuResetW<'_, PowerPdHpCpuCntlSpec> {
        ForceHpCpuResetW::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_iso(&mut self) -> ForceHpCpuIsoW<'_, PowerPdHpCpuCntlSpec> {
        ForceHpCpuIsoW::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_pu(&mut self) -> ForceHpCpuPuW<'_, PowerPdHpCpuCntlSpec> {
        ForceHpCpuPuW::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_no_reset(&mut self) -> ForceHpCpuNoResetW<'_, PowerPdHpCpuCntlSpec> {
        ForceHpCpuNoResetW::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_no_iso(&mut self) -> ForceHpCpuNoIsoW<'_, PowerPdHpCpuCntlSpec> {
        ForceHpCpuNoIsoW::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn force_hp_cpu_pd(&mut self) -> ForceHpCpuPdW<'_, PowerPdHpCpuCntlSpec> {
        ForceHpCpuPdW::new(self, 5)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_hp_cpu_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_hp_cpu_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerPdHpCpuCntlSpec;
impl crate::RegisterSpec for PowerPdHpCpuCntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_pd_hp_cpu_cntl::R`](R) reader structure"]
impl crate::Readable for PowerPdHpCpuCntlSpec {}
#[doc = "`write(|w| ..)` method takes [`power_pd_hp_cpu_cntl::W`](W) writer structure"]
impl crate::Writable for PowerPdHpCpuCntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER_PD_HP_CPU_CNTL to value 0x1c"]
impl crate::Resettable for PowerPdHpCpuCntlSpec {
    const RESET_VALUE: u32 = 0x1c;
}
