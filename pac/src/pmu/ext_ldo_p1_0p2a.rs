#[doc = "Register `EXT_LDO_P1_0P2A` reader"]
pub type R = crate::R<ExtLdoP1_0p2aSpec>;
#[doc = "Register `EXT_LDO_P1_0P2A` writer"]
pub type W = crate::W<ExtLdoP1_0p2aSpec>;
#[doc = "Field `_0P2A_CNT_CLR_1` writer - need_des"]
pub type _0p2aCntClr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_0P2A_FORCE_TIEH_SEL_1` reader - need_des"]
pub type _0p2aForceTiehSel1R = crate::BitReader;
#[doc = "Field `_0P2A_FORCE_TIEH_SEL_1` writer - need_des"]
pub type _0p2aForceTiehSel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_0P2A_XPD_1` reader - need_des"]
pub type _0p2aXpd1R = crate::BitReader;
#[doc = "Field `_0P2A_XPD_1` writer - need_des"]
pub type _0p2aXpd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_0P2A_TIEH_SEL_1` reader - need_des"]
pub type _0p2aTiehSel1R = crate::FieldReader;
#[doc = "Field `_0P2A_TIEH_SEL_1` writer - need_des"]
pub type _0p2aTiehSel1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `_0P2A_TIEH_POS_EN_1` reader - need_des"]
pub type _0p2aTiehPosEn1R = crate::BitReader;
#[doc = "Field `_0P2A_TIEH_POS_EN_1` writer - need_des"]
pub type _0p2aTiehPosEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_0P2A_TIEH_NEG_EN_1` reader - need_des"]
pub type _0p2aTiehNegEn1R = crate::BitReader;
#[doc = "Field `_0P2A_TIEH_NEG_EN_1` writer - need_des"]
pub type _0p2aTiehNegEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_0P2A_TIEH_1` reader - need_des"]
pub type _0p2aTieh1R = crate::BitReader;
#[doc = "Field `_0P2A_TIEH_1` writer - need_des"]
pub type _0p2aTieh1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_0P2A_TARGET1_1` reader - need_des"]
pub type _0p2aTarget1_1R = crate::FieldReader;
#[doc = "Field `_0P2A_TARGET1_1` writer - need_des"]
pub type _0p2aTarget1_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `_0P2A_TARGET0_1` reader - need_des"]
pub type _0p2aTarget0_1R = crate::FieldReader;
#[doc = "Field `_0P2A_TARGET0_1` writer - need_des"]
pub type _0p2aTarget0_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `_0P2A_LDO_CNT_PRESCALER_SEL_1` reader - need_des"]
pub type _0p2aLdoCntPrescalerSel1R = crate::BitReader;
#[doc = "Field `_0P2A_LDO_CNT_PRESCALER_SEL_1` writer - need_des"]
pub type _0p2aLdoCntPrescalerSel1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn _0p2a_force_tieh_sel_1(&self) -> _0p2aForceTiehSel1R {
        _0p2aForceTiehSel1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn _0p2a_xpd_1(&self) -> _0p2aXpd1R {
        _0p2aXpd1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - need_des"]
    #[inline(always)]
    pub fn _0p2a_tieh_sel_1(&self) -> _0p2aTiehSel1R {
        _0p2aTiehSel1R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn _0p2a_tieh_pos_en_1(&self) -> _0p2aTiehPosEn1R {
        _0p2aTiehPosEn1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    pub fn _0p2a_tieh_neg_en_1(&self) -> _0p2aTiehNegEn1R {
        _0p2aTiehNegEn1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    pub fn _0p2a_tieh_1(&self) -> _0p2aTieh1R {
        _0p2aTieh1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:22 - need_des"]
    #[inline(always)]
    pub fn _0p2a_target1_1(&self) -> _0p2aTarget1_1R {
        _0p2aTarget1_1R::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bits 23:30 - need_des"]
    #[inline(always)]
    pub fn _0p2a_target0_1(&self) -> _0p2aTarget0_1R {
        _0p2aTarget0_1R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn _0p2a_ldo_cnt_prescaler_sel_1(&self) -> _0p2aLdoCntPrescalerSel1R {
        _0p2aLdoCntPrescalerSel1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn _0p2a_cnt_clr_1(&mut self) -> _0p2aCntClr1W<'_, ExtLdoP1_0p2aSpec> {
        _0p2aCntClr1W::new(self, 6)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn _0p2a_force_tieh_sel_1(&mut self) -> _0p2aForceTiehSel1W<'_, ExtLdoP1_0p2aSpec> {
        _0p2aForceTiehSel1W::new(self, 7)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn _0p2a_xpd_1(&mut self) -> _0p2aXpd1W<'_, ExtLdoP1_0p2aSpec> {
        _0p2aXpd1W::new(self, 8)
    }
    #[doc = "Bits 9:11 - need_des"]
    #[inline(always)]
    pub fn _0p2a_tieh_sel_1(&mut self) -> _0p2aTiehSel1W<'_, ExtLdoP1_0p2aSpec> {
        _0p2aTiehSel1W::new(self, 9)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn _0p2a_tieh_pos_en_1(&mut self) -> _0p2aTiehPosEn1W<'_, ExtLdoP1_0p2aSpec> {
        _0p2aTiehPosEn1W::new(self, 12)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    pub fn _0p2a_tieh_neg_en_1(&mut self) -> _0p2aTiehNegEn1W<'_, ExtLdoP1_0p2aSpec> {
        _0p2aTiehNegEn1W::new(self, 13)
    }
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    pub fn _0p2a_tieh_1(&mut self) -> _0p2aTieh1W<'_, ExtLdoP1_0p2aSpec> {
        _0p2aTieh1W::new(self, 14)
    }
    #[doc = "Bits 15:22 - need_des"]
    #[inline(always)]
    pub fn _0p2a_target1_1(&mut self) -> _0p2aTarget1_1W<'_, ExtLdoP1_0p2aSpec> {
        _0p2aTarget1_1W::new(self, 15)
    }
    #[doc = "Bits 23:30 - need_des"]
    #[inline(always)]
    pub fn _0p2a_target0_1(&mut self) -> _0p2aTarget0_1W<'_, ExtLdoP1_0p2aSpec> {
        _0p2aTarget0_1W::new(self, 23)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn _0p2a_ldo_cnt_prescaler_sel_1(
        &mut self,
    ) -> _0p2aLdoCntPrescalerSel1W<'_, ExtLdoP1_0p2aSpec> {
        _0p2aLdoCntPrescalerSel1W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p1_0p2a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p1_0p2a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtLdoP1_0p2aSpec;
impl crate::RegisterSpec for ExtLdoP1_0p2aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_ldo_p1_0p2a::R`](R) reader structure"]
impl crate::Readable for ExtLdoP1_0p2aSpec {}
#[doc = "`write(|w| ..)` method takes [`ext_ldo_p1_0p2a::W`](W) writer structure"]
impl crate::Writable for ExtLdoP1_0p2aSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT_LDO_P1_0P2A to value 0x4020_0000"]
impl crate::Resettable for ExtLdoP1_0p2aSpec {
    const RESET_VALUE: u32 = 0x4020_0000;
}
