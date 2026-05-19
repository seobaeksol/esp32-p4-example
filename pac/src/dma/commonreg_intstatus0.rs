#[doc = "Register `COMMONREG_INTSTATUS0` reader"]
pub type R = crate::R<CommonregIntstatus0Spec>;
#[doc = "Field `SLVIF_COMMONREG_DEC_ERR_INTSTAT` reader - NA"]
pub type SlvifCommonregDecErrIntstatR = crate::BitReader;
#[doc = "Field `SLVIF_COMMONREG_WR2RO_ERR_INTSTAT` reader - NA"]
pub type SlvifCommonregWr2roErrIntstatR = crate::BitReader;
#[doc = "Field `SLVIF_COMMONREG_RD2WO_ERR_INTSTAT` reader - NA"]
pub type SlvifCommonregRd2woErrIntstatR = crate::BitReader;
#[doc = "Field `SLVIF_COMMONREG_WRONHOLD_ERR_INTSTAT` reader - NA"]
pub type SlvifCommonregWronholdErrIntstatR = crate::BitReader;
#[doc = "Field `SLVIF_COMMONREG_WRPARITY_ERR_INTSTAT` reader - NA"]
pub type SlvifCommonregWrparityErrIntstatR = crate::BitReader;
#[doc = "Field `SLVIF_UNDEFINEDREG_DEC_ERR_INTSTAT` reader - NA"]
pub type SlvifUndefinedregDecErrIntstatR = crate::BitReader;
#[doc = "Field `MXIF1_RCH0_ECCPROT_CORRERR_INTSTAT` reader - NA"]
pub type Mxif1Rch0EccprotCorrerrIntstatR = crate::BitReader;
#[doc = "Field `MXIF1_RCH0_ECCPROT_UNCORRERR_INTSTAT` reader - NA"]
pub type Mxif1Rch0EccprotUncorrerrIntstatR = crate::BitReader;
#[doc = "Field `MXIF1_RCH1_ECCPROT_CORRERR_INTSTAT` reader - NA"]
pub type Mxif1Rch1EccprotCorrerrIntstatR = crate::BitReader;
#[doc = "Field `MXIF1_RCH1_ECCPROT_UNCORRERR_INTSTAT` reader - NA"]
pub type Mxif1Rch1EccprotUncorrerrIntstatR = crate::BitReader;
#[doc = "Field `MXIF1_BCH_ECCPROT_CORRERR_INTSTAT` reader - NA"]
pub type Mxif1BchEccprotCorrerrIntstatR = crate::BitReader;
#[doc = "Field `MXIF1_BCH_ECCPROT_UNCORRERR_INTSTAT` reader - NA"]
pub type Mxif1BchEccprotUncorrerrIntstatR = crate::BitReader;
#[doc = "Field `MXIF2_RCH0_ECCPROT_CORRERR_INTSTAT` reader - NA"]
pub type Mxif2Rch0EccprotCorrerrIntstatR = crate::BitReader;
#[doc = "Field `MXIF2_RCH0_ECCPROT_UNCORRERR_INTSTAT` reader - NA"]
pub type Mxif2Rch0EccprotUncorrerrIntstatR = crate::BitReader;
#[doc = "Field `MXIF2_RCH1_ECCPROT_CORRERR_INTSTAT` reader - NA"]
pub type Mxif2Rch1EccprotCorrerrIntstatR = crate::BitReader;
#[doc = "Field `MXIF2_RCH1_ECCPROT_UNCORRERR_INTSTAT` reader - NA"]
pub type Mxif2Rch1EccprotUncorrerrIntstatR = crate::BitReader;
#[doc = "Field `MXIF2_BCH_ECCPROT_CORRERR_INTSTAT` reader - NA"]
pub type Mxif2BchEccprotCorrerrIntstatR = crate::BitReader;
#[doc = "Field `MXIF2_BCH_ECCPROT_UNCORRERR_INTSTAT` reader - NA"]
pub type Mxif2BchEccprotUncorrerrIntstatR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn slvif_commonreg_dec_err_intstat(&self) -> SlvifCommonregDecErrIntstatR {
        SlvifCommonregDecErrIntstatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn slvif_commonreg_wr2ro_err_intstat(&self) -> SlvifCommonregWr2roErrIntstatR {
        SlvifCommonregWr2roErrIntstatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn slvif_commonreg_rd2wo_err_intstat(&self) -> SlvifCommonregRd2woErrIntstatR {
        SlvifCommonregRd2woErrIntstatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn slvif_commonreg_wronhold_err_intstat(&self) -> SlvifCommonregWronholdErrIntstatR {
        SlvifCommonregWronholdErrIntstatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn slvif_commonreg_wrparity_err_intstat(&self) -> SlvifCommonregWrparityErrIntstatR {
        SlvifCommonregWrparityErrIntstatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn slvif_undefinedreg_dec_err_intstat(&self) -> SlvifUndefinedregDecErrIntstatR {
        SlvifUndefinedregDecErrIntstatR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn mxif1_rch0_eccprot_correrr_intstat(&self) -> Mxif1Rch0EccprotCorrerrIntstatR {
        Mxif1Rch0EccprotCorrerrIntstatR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn mxif1_rch0_eccprot_uncorrerr_intstat(&self) -> Mxif1Rch0EccprotUncorrerrIntstatR {
        Mxif1Rch0EccprotUncorrerrIntstatR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn mxif1_rch1_eccprot_correrr_intstat(&self) -> Mxif1Rch1EccprotCorrerrIntstatR {
        Mxif1Rch1EccprotCorrerrIntstatR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn mxif1_rch1_eccprot_uncorrerr_intstat(&self) -> Mxif1Rch1EccprotUncorrerrIntstatR {
        Mxif1Rch1EccprotUncorrerrIntstatR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn mxif1_bch_eccprot_correrr_intstat(&self) -> Mxif1BchEccprotCorrerrIntstatR {
        Mxif1BchEccprotCorrerrIntstatR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn mxif1_bch_eccprot_uncorrerr_intstat(&self) -> Mxif1BchEccprotUncorrerrIntstatR {
        Mxif1BchEccprotUncorrerrIntstatR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn mxif2_rch0_eccprot_correrr_intstat(&self) -> Mxif2Rch0EccprotCorrerrIntstatR {
        Mxif2Rch0EccprotCorrerrIntstatR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn mxif2_rch0_eccprot_uncorrerr_intstat(&self) -> Mxif2Rch0EccprotUncorrerrIntstatR {
        Mxif2Rch0EccprotUncorrerrIntstatR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn mxif2_rch1_eccprot_correrr_intstat(&self) -> Mxif2Rch1EccprotCorrerrIntstatR {
        Mxif2Rch1EccprotCorrerrIntstatR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    pub fn mxif2_rch1_eccprot_uncorrerr_intstat(&self) -> Mxif2Rch1EccprotUncorrerrIntstatR {
        Mxif2Rch1EccprotUncorrerrIntstatR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn mxif2_bch_eccprot_correrr_intstat(&self) -> Mxif2BchEccprotCorrerrIntstatR {
        Mxif2BchEccprotCorrerrIntstatR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - NA"]
    #[inline(always)]
    pub fn mxif2_bch_eccprot_uncorrerr_intstat(&self) -> Mxif2BchEccprotUncorrerrIntstatR {
        Mxif2BchEccprotUncorrerrIntstatR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`commonreg_intstatus0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommonregIntstatus0Spec;
impl crate::RegisterSpec for CommonregIntstatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`commonreg_intstatus0::R`](R) reader structure"]
impl crate::Readable for CommonregIntstatus0Spec {}
#[doc = "`reset()` method sets COMMONREG_INTSTATUS0 to value 0"]
impl crate::Resettable for CommonregIntstatus0Spec {}
