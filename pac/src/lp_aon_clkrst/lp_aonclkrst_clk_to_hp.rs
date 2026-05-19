#[doc = "Register `LP_AONCLKRST_CLK_TO_HP` reader"]
pub type R = crate::R<LpAonclkrstClkToHpSpec>;
#[doc = "Register `LP_AONCLKRST_CLK_TO_HP` writer"]
pub type W = crate::W<LpAonclkrstClkToHpSpec>;
#[doc = "Field `LP_AONCLKRST_ICG_HP_XTAL32K` reader - reserved"]
pub type LpAonclkrstIcgHpXtal32kR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_ICG_HP_XTAL32K` writer - reserved"]
pub type LpAonclkrstIcgHpXtal32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_ICG_HP_SOSC` reader - reserved"]
pub type LpAonclkrstIcgHpSoscR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_ICG_HP_SOSC` writer - reserved"]
pub type LpAonclkrstIcgHpSoscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_ICG_HP_OSC32K` reader - reserved"]
pub type LpAonclkrstIcgHpOsc32kR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_ICG_HP_OSC32K` writer - reserved"]
pub type LpAonclkrstIcgHpOsc32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_ICG_HP_FOSC` reader - reserved"]
pub type LpAonclkrstIcgHpFoscR = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_ICG_HP_FOSC` writer - reserved"]
pub type LpAonclkrstIcgHpFoscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_icg_hp_xtal32k(&self) -> LpAonclkrstIcgHpXtal32kR {
        LpAonclkrstIcgHpXtal32kR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_icg_hp_sosc(&self) -> LpAonclkrstIcgHpSoscR {
        LpAonclkrstIcgHpSoscR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_icg_hp_osc32k(&self) -> LpAonclkrstIcgHpOsc32kR {
        LpAonclkrstIcgHpOsc32kR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_icg_hp_fosc(&self) -> LpAonclkrstIcgHpFoscR {
        LpAonclkrstIcgHpFoscR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_icg_hp_xtal32k(
        &mut self,
    ) -> LpAonclkrstIcgHpXtal32kW<'_, LpAonclkrstClkToHpSpec> {
        LpAonclkrstIcgHpXtal32kW::new(self, 28)
    }
    #[doc = "Bit 29 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_icg_hp_sosc(
        &mut self,
    ) -> LpAonclkrstIcgHpSoscW<'_, LpAonclkrstClkToHpSpec> {
        LpAonclkrstIcgHpSoscW::new(self, 29)
    }
    #[doc = "Bit 30 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_icg_hp_osc32k(
        &mut self,
    ) -> LpAonclkrstIcgHpOsc32kW<'_, LpAonclkrstClkToHpSpec> {
        LpAonclkrstIcgHpOsc32kW::new(self, 30)
    }
    #[doc = "Bit 31 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_icg_hp_fosc(
        &mut self,
    ) -> LpAonclkrstIcgHpFoscW<'_, LpAonclkrstClkToHpSpec> {
        LpAonclkrstIcgHpFoscW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_clk_to_hp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_clk_to_hp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpAonclkrstClkToHpSpec;
impl crate::RegisterSpec for LpAonclkrstClkToHpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_clk_to_hp::R`](R) reader structure"]
impl crate::Readable for LpAonclkrstClkToHpSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_clk_to_hp::W`](W) writer structure"]
impl crate::Writable for LpAonclkrstClkToHpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_CLK_TO_HP to value 0xf000_0000"]
impl crate::Resettable for LpAonclkrstClkToHpSpec {
    const RESET_VALUE: u32 = 0xf000_0000;
}
