#[doc = "Register `HP_ACTIVE_DIG_POWER` reader"]
pub type R = crate::R<HpActiveDigPowerSpec>;
#[doc = "Register `HP_ACTIVE_DIG_POWER` writer"]
pub type W = crate::W<HpActiveDigPowerSpec>;
#[doc = "Field `HP_ACTIVE_DCDC_SWITCH_PD_EN` reader - need_des"]
pub type HpActiveDcdcSwitchPdEnR = crate::BitReader;
#[doc = "Field `HP_ACTIVE_DCDC_SWITCH_PD_EN` writer - need_des"]
pub type HpActiveDcdcSwitchPdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_HP_MEM_DSLP` reader - need_des"]
pub type HpActiveHpMemDslpR = crate::BitReader;
#[doc = "Field `HP_ACTIVE_HP_MEM_DSLP` writer - need_des"]
pub type HpActiveHpMemDslpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_PD_HP_MEM_PD_EN` reader - need_des"]
pub type HpActivePdHpMemPdEnR = crate::BitReader;
#[doc = "Field `HP_ACTIVE_PD_HP_MEM_PD_EN` writer - need_des"]
pub type HpActivePdHpMemPdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_PD_HP_CPU_PD_EN` reader - need_des"]
pub type HpActivePdHpCpuPdEnR = crate::BitReader;
#[doc = "Field `HP_ACTIVE_PD_HP_CPU_PD_EN` writer - need_des"]
pub type HpActivePdHpCpuPdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_PD_CNNT_PD_EN` reader - need_des"]
pub type HpActivePdCnntPdEnR = crate::BitReader;
#[doc = "Field `HP_ACTIVE_PD_CNNT_PD_EN` writer - need_des"]
pub type HpActivePdCnntPdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_PD_TOP_PD_EN` reader - need_des"]
pub type HpActivePdTopPdEnR = crate::BitReader;
#[doc = "Field `HP_ACTIVE_PD_TOP_PD_EN` writer - need_des"]
pub type HpActivePdTopPdEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn hp_active_dcdc_switch_pd_en(&self) -> HpActiveDcdcSwitchPdEnR {
        HpActiveDcdcSwitchPdEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn hp_active_hp_mem_dslp(&self) -> HpActiveHpMemDslpR {
        HpActiveHpMemDslpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn hp_active_pd_hp_mem_pd_en(&self) -> HpActivePdHpMemPdEnR {
        HpActivePdHpMemPdEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_active_pd_hp_cpu_pd_en(&self) -> HpActivePdHpCpuPdEnR {
        HpActivePdHpCpuPdEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_active_pd_cnnt_pd_en(&self) -> HpActivePdCnntPdEnR {
        HpActivePdCnntPdEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_active_pd_top_pd_en(&self) -> HpActivePdTopPdEnR {
        HpActivePdTopPdEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn hp_active_dcdc_switch_pd_en(
        &mut self,
    ) -> HpActiveDcdcSwitchPdEnW<'_, HpActiveDigPowerSpec> {
        HpActiveDcdcSwitchPdEnW::new(self, 21)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn hp_active_hp_mem_dslp(&mut self) -> HpActiveHpMemDslpW<'_, HpActiveDigPowerSpec> {
        HpActiveHpMemDslpW::new(self, 22)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn hp_active_pd_hp_mem_pd_en(&mut self) -> HpActivePdHpMemPdEnW<'_, HpActiveDigPowerSpec> {
        HpActivePdHpMemPdEnW::new(self, 23)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_active_pd_hp_cpu_pd_en(&mut self) -> HpActivePdHpCpuPdEnW<'_, HpActiveDigPowerSpec> {
        HpActivePdHpCpuPdEnW::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_active_pd_cnnt_pd_en(&mut self) -> HpActivePdCnntPdEnW<'_, HpActiveDigPowerSpec> {
        HpActivePdCnntPdEnW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_active_pd_top_pd_en(&mut self) -> HpActivePdTopPdEnW<'_, HpActiveDigPowerSpec> {
        HpActivePdTopPdEnW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_dig_power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_dig_power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpActiveDigPowerSpec;
impl crate::RegisterSpec for HpActiveDigPowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_active_dig_power::R`](R) reader structure"]
impl crate::Readable for HpActiveDigPowerSpec {}
#[doc = "`write(|w| ..)` method takes [`hp_active_dig_power::W`](W) writer structure"]
impl crate::Writable for HpActiveDigPowerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_ACTIVE_DIG_POWER to value 0"]
impl crate::Resettable for HpActiveDigPowerSpec {}
