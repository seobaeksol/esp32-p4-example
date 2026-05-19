#[doc = "Register `COMMONREG_INTSIGNAL_ENABLE0` reader"]
pub type R = crate::R<CommonregIntsignalEnable0Spec>;
#[doc = "Register `COMMONREG_INTSIGNAL_ENABLE0` writer"]
pub type W = crate::W<CommonregIntsignalEnable0Spec>;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSIGNAL` reader - NA"]
pub type EnableSlvifCommonregDecErrIntsignalR = crate::BitReader;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_DEC_ERR_INTSIGNAL` writer - NA"]
pub type EnableSlvifCommonregDecErrIntsignalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSIGNAL` reader - NA"]
pub type EnableSlvifCommonregWr2roErrIntsignalR = crate::BitReader;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_WR2RO_ERR_INTSIGNAL` writer - NA"]
pub type EnableSlvifCommonregWr2roErrIntsignalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSIGNAL` reader - NA"]
pub type EnableSlvifCommonregRd2woErrIntsignalR = crate::BitReader;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_RD2WO_ERR_INTSIGNAL` writer - NA"]
pub type EnableSlvifCommonregRd2woErrIntsignalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSIGNAL` reader - NA"]
pub type EnableSlvifCommonregWronholdErrIntsignalR = crate::BitReader;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_WRONHOLD_ERR_INTSIGNAL` writer - NA"]
pub type EnableSlvifCommonregWronholdErrIntsignalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_SLVIF_COMMONREG_WRPARITY_ERR_INTSIGNAL` reader - NA"]
pub type EnableSlvifCommonregWrparityErrIntsignalR = crate::BitReader;
#[doc = "Field `ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSIGNAL` reader - NA"]
pub type EnableSlvifUndefinedregDecErrIntsignalR = crate::BitReader;
#[doc = "Field `ENABLE_SLVIF_UNDEFINEDREG_DEC_ERR_INTSIGNAL` writer - NA"]
pub type EnableSlvifUndefinedregDecErrIntsignalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_MXIF1_RCH0_ECCPROT_CORRERR_INTSIGNAL` reader - NA"]
pub type EnableMxif1Rch0EccprotCorrerrIntsignalR = crate::BitReader;
#[doc = "Field `ENABLE_MXIF1_RCH0_ECCPROT_UNCORRERR_INTSIGNAL` reader - NA"]
pub type EnableMxif1Rch0EccprotUncorrerrIntsignalR = crate::BitReader;
#[doc = "Field `ENABLE_MXIF1_RCH1_ECCPROT_CORRERR_INTSIGNAL` reader - NA"]
pub type EnableMxif1Rch1EccprotCorrerrIntsignalR = crate::BitReader;
#[doc = "Field `ENABLE_MXIF1_RCH1_ECCPROT_UNCORRERR_INTSIGNAL` reader - NA"]
pub type EnableMxif1Rch1EccprotUncorrerrIntsignalR = crate::BitReader;
#[doc = "Field `ENABLE_MXIF1_BCH_ECCPROT_CORRERR_INTSIGNAL` reader - NA"]
pub type EnableMxif1BchEccprotCorrerrIntsignalR = crate::BitReader;
#[doc = "Field `ENABLE_MXIF1_BCH_ECCPROT_UNCORRERR_INTSIGNAL` reader - NA"]
pub type EnableMxif1BchEccprotUncorrerrIntsignalR = crate::BitReader;
#[doc = "Field `ENABLE_MXIF2_RCH0_ECCPROT_CORRERR_INTSIGNAL` reader - NA"]
pub type EnableMxif2Rch0EccprotCorrerrIntsignalR = crate::BitReader;
#[doc = "Field `ENABLE_MXIF2_RCH0_ECCPROT_UNCORRERR_INTSIGNAL` reader - NA"]
pub type EnableMxif2Rch0EccprotUncorrerrIntsignalR = crate::BitReader;
#[doc = "Field `ENABLE_MXIF2_RCH1_ECCPROT_CORRERR_INTSIGNAL` reader - NA"]
pub type EnableMxif2Rch1EccprotCorrerrIntsignalR = crate::BitReader;
#[doc = "Field `ENABLE_MXIF2_RCH1_ECCPROT_UNCORRERR_INTSIGNAL` reader - NA"]
pub type EnableMxif2Rch1EccprotUncorrerrIntsignalR = crate::BitReader;
#[doc = "Field `ENABLE_MXIF2_BCH_ECCPROT_CORRERR_INTSIGNAL` reader - NA"]
pub type EnableMxif2BchEccprotCorrerrIntsignalR = crate::BitReader;
#[doc = "Field `ENABLE_MXIF2_BCH_ECCPROT_UNCORRERR_INTSIGNAL` reader - NA"]
pub type EnableMxif2BchEccprotUncorrerrIntsignalR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn enable_slvif_commonreg_dec_err_intsignal(&self) -> EnableSlvifCommonregDecErrIntsignalR {
        EnableSlvifCommonregDecErrIntsignalR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn enable_slvif_commonreg_wr2ro_err_intsignal(
        &self,
    ) -> EnableSlvifCommonregWr2roErrIntsignalR {
        EnableSlvifCommonregWr2roErrIntsignalR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn enable_slvif_commonreg_rd2wo_err_intsignal(
        &self,
    ) -> EnableSlvifCommonregRd2woErrIntsignalR {
        EnableSlvifCommonregRd2woErrIntsignalR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn enable_slvif_commonreg_wronhold_err_intsignal(
        &self,
    ) -> EnableSlvifCommonregWronholdErrIntsignalR {
        EnableSlvifCommonregWronholdErrIntsignalR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn enable_slvif_commonreg_wrparity_err_intsignal(
        &self,
    ) -> EnableSlvifCommonregWrparityErrIntsignalR {
        EnableSlvifCommonregWrparityErrIntsignalR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn enable_slvif_undefinedreg_dec_err_intsignal(
        &self,
    ) -> EnableSlvifUndefinedregDecErrIntsignalR {
        EnableSlvifUndefinedregDecErrIntsignalR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn enable_mxif1_rch0_eccprot_correrr_intsignal(
        &self,
    ) -> EnableMxif1Rch0EccprotCorrerrIntsignalR {
        EnableMxif1Rch0EccprotCorrerrIntsignalR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn enable_mxif1_rch0_eccprot_uncorrerr_intsignal(
        &self,
    ) -> EnableMxif1Rch0EccprotUncorrerrIntsignalR {
        EnableMxif1Rch0EccprotUncorrerrIntsignalR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn enable_mxif1_rch1_eccprot_correrr_intsignal(
        &self,
    ) -> EnableMxif1Rch1EccprotCorrerrIntsignalR {
        EnableMxif1Rch1EccprotCorrerrIntsignalR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn enable_mxif1_rch1_eccprot_uncorrerr_intsignal(
        &self,
    ) -> EnableMxif1Rch1EccprotUncorrerrIntsignalR {
        EnableMxif1Rch1EccprotUncorrerrIntsignalR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn enable_mxif1_bch_eccprot_correrr_intsignal(
        &self,
    ) -> EnableMxif1BchEccprotCorrerrIntsignalR {
        EnableMxif1BchEccprotCorrerrIntsignalR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn enable_mxif1_bch_eccprot_uncorrerr_intsignal(
        &self,
    ) -> EnableMxif1BchEccprotUncorrerrIntsignalR {
        EnableMxif1BchEccprotUncorrerrIntsignalR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn enable_mxif2_rch0_eccprot_correrr_intsignal(
        &self,
    ) -> EnableMxif2Rch0EccprotCorrerrIntsignalR {
        EnableMxif2Rch0EccprotCorrerrIntsignalR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn enable_mxif2_rch0_eccprot_uncorrerr_intsignal(
        &self,
    ) -> EnableMxif2Rch0EccprotUncorrerrIntsignalR {
        EnableMxif2Rch0EccprotUncorrerrIntsignalR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn enable_mxif2_rch1_eccprot_correrr_intsignal(
        &self,
    ) -> EnableMxif2Rch1EccprotCorrerrIntsignalR {
        EnableMxif2Rch1EccprotCorrerrIntsignalR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    pub fn enable_mxif2_rch1_eccprot_uncorrerr_intsignal(
        &self,
    ) -> EnableMxif2Rch1EccprotUncorrerrIntsignalR {
        EnableMxif2Rch1EccprotUncorrerrIntsignalR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn enable_mxif2_bch_eccprot_correrr_intsignal(
        &self,
    ) -> EnableMxif2BchEccprotCorrerrIntsignalR {
        EnableMxif2BchEccprotCorrerrIntsignalR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - NA"]
    #[inline(always)]
    pub fn enable_mxif2_bch_eccprot_uncorrerr_intsignal(
        &self,
    ) -> EnableMxif2BchEccprotUncorrerrIntsignalR {
        EnableMxif2BchEccprotUncorrerrIntsignalR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn enable_slvif_commonreg_dec_err_intsignal(
        &mut self,
    ) -> EnableSlvifCommonregDecErrIntsignalW<'_, CommonregIntsignalEnable0Spec> {
        EnableSlvifCommonregDecErrIntsignalW::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn enable_slvif_commonreg_wr2ro_err_intsignal(
        &mut self,
    ) -> EnableSlvifCommonregWr2roErrIntsignalW<'_, CommonregIntsignalEnable0Spec> {
        EnableSlvifCommonregWr2roErrIntsignalW::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn enable_slvif_commonreg_rd2wo_err_intsignal(
        &mut self,
    ) -> EnableSlvifCommonregRd2woErrIntsignalW<'_, CommonregIntsignalEnable0Spec> {
        EnableSlvifCommonregRd2woErrIntsignalW::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn enable_slvif_commonreg_wronhold_err_intsignal(
        &mut self,
    ) -> EnableSlvifCommonregWronholdErrIntsignalW<'_, CommonregIntsignalEnable0Spec> {
        EnableSlvifCommonregWronholdErrIntsignalW::new(self, 3)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn enable_slvif_undefinedreg_dec_err_intsignal(
        &mut self,
    ) -> EnableSlvifUndefinedregDecErrIntsignalW<'_, CommonregIntsignalEnable0Spec> {
        EnableSlvifUndefinedregDecErrIntsignalW::new(self, 8)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`commonreg_intsignal_enable0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`commonreg_intsignal_enable0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommonregIntsignalEnable0Spec;
impl crate::RegisterSpec for CommonregIntsignalEnable0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`commonreg_intsignal_enable0::R`](R) reader structure"]
impl crate::Readable for CommonregIntsignalEnable0Spec {}
#[doc = "`write(|w| ..)` method takes [`commonreg_intsignal_enable0::W`](W) writer structure"]
impl crate::Writable for CommonregIntsignalEnable0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMMONREG_INTSIGNAL_ENABLE0 to value 0x001f_ff8f"]
impl crate::Resettable for CommonregIntsignalEnable0Spec {
    const RESET_VALUE: u32 = 0x001f_ff8f;
}
