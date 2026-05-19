#[doc = "Register `COMMONREG_INTCLEAR0` writer"]
pub type W = crate::W<CommonregIntclear0Spec>;
#[doc = "Field `CLEAR_SLVIF_COMMONREG_DEC_ERR_INTSTAT` writer - NA"]
pub type ClearSlvifCommonregDecErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_SLVIF_COMMONREG_WR2RO_ERR_INTSTAT` writer - NA"]
pub type ClearSlvifCommonregWr2roErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_SLVIF_COMMONREG_RD2WO_ERR_INTSTAT` writer - NA"]
pub type ClearSlvifCommonregRd2woErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT` writer - NA"]
pub type ClearSlvifCommonregWronholdErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_SLVIF_COMMONREG_WRPARITY_ERR_INTSTAT` writer - NA"]
pub type ClearSlvifCommonregWrparityErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT` writer - NA"]
pub type ClearSlvifUndefinedregDecErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF1_RCH0_ECCPROT_CORRERR_INTSTAT` writer - NA"]
pub type ClearMxif1Rch0EccprotCorrerrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF1_RCH0_ECCPROT_UNCORRERR_INTSTAT` writer - NA"]
pub type ClearMxif1Rch0EccprotUncorrerrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF1_RCH1_ECCPROT_CORRERR_INTSTAT` writer - NA"]
pub type ClearMxif1Rch1EccprotCorrerrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF1_RCH1_ECCPROT_UNCORRERR_INTSTAT` writer - NA"]
pub type ClearMxif1Rch1EccprotUncorrerrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF1_BCH_ECCPROT_CORRERR_INTSTAT` writer - NA"]
pub type ClearMxif1BchEccprotCorrerrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF1_BCH_ECCPROT_UNCORRERR_INTSTAT` writer - NA"]
pub type ClearMxif1BchEccprotUncorrerrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF2_RCH0_ECCPROT_CORRERR_INTSTAT` writer - NA"]
pub type ClearMxif2Rch0EccprotCorrerrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF2_RCH0_ECCPROT_UNCORRERR_INTSTAT` writer - NA"]
pub type ClearMxif2Rch0EccprotUncorrerrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF2_RCH1_ECCPROT_CORRERR_INTSTAT` writer - NA"]
pub type ClearMxif2Rch1EccprotCorrerrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF2_RCH1_ECCPROT_UNCORRERR_INTSTAT` writer - NA"]
pub type ClearMxif2Rch1EccprotUncorrerrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF2_BCH_ECCPROT_CORRERR_INTSTAT` writer - NA"]
pub type ClearMxif2BchEccprotCorrerrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_MXIF2_BCH_ECCPROT_UNCORRERR_INTSTAT` writer - NA"]
pub type ClearMxif2BchEccprotUncorrerrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn clear_slvif_commonreg_dec_err_intstat(
        &mut self,
    ) -> ClearSlvifCommonregDecErrIntstatW<'_, CommonregIntclear0Spec> {
        ClearSlvifCommonregDecErrIntstatW::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn clear_slvif_commonreg_wr2ro_err_intstat(
        &mut self,
    ) -> ClearSlvifCommonregWr2roErrIntstatW<'_, CommonregIntclear0Spec> {
        ClearSlvifCommonregWr2roErrIntstatW::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn clear_slvif_commonreg_rd2wo_err_intstat(
        &mut self,
    ) -> ClearSlvifCommonregRd2woErrIntstatW<'_, CommonregIntclear0Spec> {
        ClearSlvifCommonregRd2woErrIntstatW::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn clear_slvif_commonreg_wronhold_err_intstat(
        &mut self,
    ) -> ClearSlvifCommonregWronholdErrIntstatW<'_, CommonregIntclear0Spec> {
        ClearSlvifCommonregWronholdErrIntstatW::new(self, 3)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn clear_slvif_commonreg_wrparity_err_intstat(
        &mut self,
    ) -> ClearSlvifCommonregWrparityErrIntstatW<'_, CommonregIntclear0Spec> {
        ClearSlvifCommonregWrparityErrIntstatW::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn clear_slvif_undefinedreg_dec_err_intstat(
        &mut self,
    ) -> ClearSlvifUndefinedregDecErrIntstatW<'_, CommonregIntclear0Spec> {
        ClearSlvifUndefinedregDecErrIntstatW::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn clear_mxif1_rch0_eccprot_correrr_intstat(
        &mut self,
    ) -> ClearMxif1Rch0EccprotCorrerrIntstatW<'_, CommonregIntclear0Spec> {
        ClearMxif1Rch0EccprotCorrerrIntstatW::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn clear_mxif1_rch0_eccprot_uncorrerr_intstat(
        &mut self,
    ) -> ClearMxif1Rch0EccprotUncorrerrIntstatW<'_, CommonregIntclear0Spec> {
        ClearMxif1Rch0EccprotUncorrerrIntstatW::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn clear_mxif1_rch1_eccprot_correrr_intstat(
        &mut self,
    ) -> ClearMxif1Rch1EccprotCorrerrIntstatW<'_, CommonregIntclear0Spec> {
        ClearMxif1Rch1EccprotCorrerrIntstatW::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn clear_mxif1_rch1_eccprot_uncorrerr_intstat(
        &mut self,
    ) -> ClearMxif1Rch1EccprotUncorrerrIntstatW<'_, CommonregIntclear0Spec> {
        ClearMxif1Rch1EccprotUncorrerrIntstatW::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn clear_mxif1_bch_eccprot_correrr_intstat(
        &mut self,
    ) -> ClearMxif1BchEccprotCorrerrIntstatW<'_, CommonregIntclear0Spec> {
        ClearMxif1BchEccprotCorrerrIntstatW::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn clear_mxif1_bch_eccprot_uncorrerr_intstat(
        &mut self,
    ) -> ClearMxif1BchEccprotUncorrerrIntstatW<'_, CommonregIntclear0Spec> {
        ClearMxif1BchEccprotUncorrerrIntstatW::new(self, 14)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn clear_mxif2_rch0_eccprot_correrr_intstat(
        &mut self,
    ) -> ClearMxif2Rch0EccprotCorrerrIntstatW<'_, CommonregIntclear0Spec> {
        ClearMxif2Rch0EccprotCorrerrIntstatW::new(self, 15)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn clear_mxif2_rch0_eccprot_uncorrerr_intstat(
        &mut self,
    ) -> ClearMxif2Rch0EccprotUncorrerrIntstatW<'_, CommonregIntclear0Spec> {
        ClearMxif2Rch0EccprotUncorrerrIntstatW::new(self, 16)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn clear_mxif2_rch1_eccprot_correrr_intstat(
        &mut self,
    ) -> ClearMxif2Rch1EccprotCorrerrIntstatW<'_, CommonregIntclear0Spec> {
        ClearMxif2Rch1EccprotCorrerrIntstatW::new(self, 17)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    pub fn clear_mxif2_rch1_eccprot_uncorrerr_intstat(
        &mut self,
    ) -> ClearMxif2Rch1EccprotUncorrerrIntstatW<'_, CommonregIntclear0Spec> {
        ClearMxif2Rch1EccprotUncorrerrIntstatW::new(self, 18)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn clear_mxif2_bch_eccprot_correrr_intstat(
        &mut self,
    ) -> ClearMxif2BchEccprotCorrerrIntstatW<'_, CommonregIntclear0Spec> {
        ClearMxif2BchEccprotCorrerrIntstatW::new(self, 19)
    }
    #[doc = "Bit 20 - NA"]
    #[inline(always)]
    pub fn clear_mxif2_bch_eccprot_uncorrerr_intstat(
        &mut self,
    ) -> ClearMxif2BchEccprotUncorrerrIntstatW<'_, CommonregIntclear0Spec> {
        ClearMxif2BchEccprotUncorrerrIntstatW::new(self, 20)
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`commonreg_intclear0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommonregIntclear0Spec;
impl crate::RegisterSpec for CommonregIntclear0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`commonreg_intclear0::W`](W) writer structure"]
impl crate::Writable for CommonregIntclear0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMMONREG_INTCLEAR0 to value 0"]
impl crate::Resettable for CommonregIntclear0Spec {}
