#[doc = "Register `IMM_PAD_HOLD_ALL` reader"]
pub type R = crate::R<ImmPadHoldAllSpec>;
#[doc = "Register `IMM_PAD_HOLD_ALL` writer"]
pub type W = crate::W<ImmPadHoldAllSpec>;
#[doc = "Field `PAD_SLP_SEL` reader - need_des"]
pub type PadSlpSelR = crate::BitReader;
#[doc = "Field `LP_PAD_HOLD_ALL` reader - need_des"]
pub type LpPadHoldAllR = crate::BitReader;
#[doc = "Field `HP_PAD_HOLD_ALL` reader - need_des"]
pub type HpPadHoldAllR = crate::BitReader;
#[doc = "Field `TIE_HIGH_PAD_SLP_SEL` writer - need_des"]
pub type TieHighPadSlpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_PAD_SLP_SEL` writer - need_des"]
pub type TieLowPadSlpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_LP_PAD_HOLD_ALL` writer - need_des"]
pub type TieHighLpPadHoldAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_LP_PAD_HOLD_ALL` writer - need_des"]
pub type TieLowLpPadHoldAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_HP_PAD_HOLD_ALL` writer - need_des"]
pub type TieHighHpPadHoldAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_HP_PAD_HOLD_ALL` writer - need_des"]
pub type TieLowHpPadHoldAllW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn pad_slp_sel(&self) -> PadSlpSelR {
        PadSlpSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn lp_pad_hold_all(&self) -> LpPadHoldAllR {
        LpPadHoldAllR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn hp_pad_hold_all(&self) -> HpPadHoldAllR {
        HpPadHoldAllR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn tie_high_pad_slp_sel(&mut self) -> TieHighPadSlpSelW<'_, ImmPadHoldAllSpec> {
        TieHighPadSlpSelW::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn tie_low_pad_slp_sel(&mut self) -> TieLowPadSlpSelW<'_, ImmPadHoldAllSpec> {
        TieLowPadSlpSelW::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn tie_high_lp_pad_hold_all(&mut self) -> TieHighLpPadHoldAllW<'_, ImmPadHoldAllSpec> {
        TieHighLpPadHoldAllW::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn tie_low_lp_pad_hold_all(&mut self) -> TieLowLpPadHoldAllW<'_, ImmPadHoldAllSpec> {
        TieLowLpPadHoldAllW::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn tie_high_hp_pad_hold_all(&mut self) -> TieHighHpPadHoldAllW<'_, ImmPadHoldAllSpec> {
        TieHighHpPadHoldAllW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn tie_low_hp_pad_hold_all(&mut self) -> TieLowHpPadHoldAllW<'_, ImmPadHoldAllSpec> {
        TieLowHpPadHoldAllW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`imm_pad_hold_all::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_pad_hold_all::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImmPadHoldAllSpec;
impl crate::RegisterSpec for ImmPadHoldAllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imm_pad_hold_all::R`](R) reader structure"]
impl crate::Readable for ImmPadHoldAllSpec {}
#[doc = "`write(|w| ..)` method takes [`imm_pad_hold_all::W`](W) writer structure"]
impl crate::Writable for ImmPadHoldAllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMM_PAD_HOLD_ALL to value 0"]
impl crate::Resettable for ImmPadHoldAllSpec {}
