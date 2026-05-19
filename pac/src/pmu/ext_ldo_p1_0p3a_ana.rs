#[doc = "Register `EXT_LDO_P1_0P3A_ANA` reader"]
pub type R = crate::R<ExtLdoP1_0p3aAnaSpec>;
#[doc = "Register `EXT_LDO_P1_0P3A_ANA` writer"]
pub type W = crate::W<ExtLdoP1_0p3aAnaSpec>;
#[doc = "Field `ANA_0P3A_MUL_1` reader - need_des"]
pub type Ana0p3aMul1R = crate::FieldReader;
#[doc = "Field `ANA_0P3A_MUL_1` writer - need_des"]
pub type Ana0p3aMul1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ANA_0P3A_EN_VDET_1` reader - need_des"]
pub type Ana0p3aEnVdet1R = crate::BitReader;
#[doc = "Field `ANA_0P3A_EN_VDET_1` writer - need_des"]
pub type Ana0p3aEnVdet1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANA_0P3A_EN_CUR_LIM_1` reader - need_des"]
pub type Ana0p3aEnCurLim1R = crate::BitReader;
#[doc = "Field `ANA_0P3A_EN_CUR_LIM_1` writer - need_des"]
pub type Ana0p3aEnCurLim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANA_0P3A_DREF_1` reader - need_des"]
pub type Ana0p3aDref1R = crate::FieldReader;
#[doc = "Field `ANA_0P3A_DREF_1` writer - need_des"]
pub type Ana0p3aDref1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 23:25 - need_des"]
    #[inline(always)]
    pub fn ana_0p3a_mul_1(&self) -> Ana0p3aMul1R {
        Ana0p3aMul1R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn ana_0p3a_en_vdet_1(&self) -> Ana0p3aEnVdet1R {
        Ana0p3aEnVdet1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn ana_0p3a_en_cur_lim_1(&self) -> Ana0p3aEnCurLim1R {
        Ana0p3aEnCurLim1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - need_des"]
    #[inline(always)]
    pub fn ana_0p3a_dref_1(&self) -> Ana0p3aDref1R {
        Ana0p3aDref1R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 23:25 - need_des"]
    #[inline(always)]
    pub fn ana_0p3a_mul_1(&mut self) -> Ana0p3aMul1W<'_, ExtLdoP1_0p3aAnaSpec> {
        Ana0p3aMul1W::new(self, 23)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn ana_0p3a_en_vdet_1(&mut self) -> Ana0p3aEnVdet1W<'_, ExtLdoP1_0p3aAnaSpec> {
        Ana0p3aEnVdet1W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn ana_0p3a_en_cur_lim_1(&mut self) -> Ana0p3aEnCurLim1W<'_, ExtLdoP1_0p3aAnaSpec> {
        Ana0p3aEnCurLim1W::new(self, 27)
    }
    #[doc = "Bits 28:31 - need_des"]
    #[inline(always)]
    pub fn ana_0p3a_dref_1(&mut self) -> Ana0p3aDref1W<'_, ExtLdoP1_0p3aAnaSpec> {
        Ana0p3aDref1W::new(self, 28)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p1_0p3a_ana::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p1_0p3a_ana::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtLdoP1_0p3aAnaSpec;
impl crate::RegisterSpec for ExtLdoP1_0p3aAnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_ldo_p1_0p3a_ana::R`](R) reader structure"]
impl crate::Readable for ExtLdoP1_0p3aAnaSpec {}
#[doc = "`write(|w| ..)` method takes [`ext_ldo_p1_0p3a_ana::W`](W) writer structure"]
impl crate::Writable for ExtLdoP1_0p3aAnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT_LDO_P1_0P3A_ANA to value 0xa000_0000"]
impl crate::Resettable for ExtLdoP1_0p3aAnaSpec {
    const RESET_VALUE: u32 = 0xa000_0000;
}
