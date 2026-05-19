#[doc = "Register `POWER_PD_HP_CPU_MASK` reader"]
pub type R = crate::R<PowerPdHpCpuMaskSpec>;
#[doc = "Register `POWER_PD_HP_CPU_MASK` writer"]
pub type W = crate::W<PowerPdHpCpuMaskSpec>;
#[doc = "Field `XPD_HP_CPU_MASK` reader - need_des"]
pub type XpdHpCpuMaskR = crate::FieldReader;
#[doc = "Field `XPD_HP_CPU_MASK` writer - need_des"]
pub type XpdHpCpuMaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PD_HP_CPU_MASK` reader - need_des"]
pub type PdHpCpuMaskR = crate::FieldReader;
#[doc = "Field `PD_HP_CPU_MASK` writer - need_des"]
pub type PdHpCpuMaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    pub fn xpd_hp_cpu_mask(&self) -> XpdHpCpuMaskR {
        XpdHpCpuMaskR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn pd_hp_cpu_mask(&self) -> PdHpCpuMaskR {
        PdHpCpuMaskR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    pub fn xpd_hp_cpu_mask(&mut self) -> XpdHpCpuMaskW<'_, PowerPdHpCpuMaskSpec> {
        XpdHpCpuMaskW::new(self, 0)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn pd_hp_cpu_mask(&mut self) -> PdHpCpuMaskW<'_, PowerPdHpCpuMaskSpec> {
        PdHpCpuMaskW::new(self, 27)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_hp_cpu_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_hp_cpu_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerPdHpCpuMaskSpec;
impl crate::RegisterSpec for PowerPdHpCpuMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_pd_hp_cpu_mask::R`](R) reader structure"]
impl crate::Readable for PowerPdHpCpuMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`power_pd_hp_cpu_mask::W`](W) writer structure"]
impl crate::Writable for PowerPdHpCpuMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER_PD_HP_CPU_MASK to value 0"]
impl crate::Resettable for PowerPdHpCpuMaskSpec {}
