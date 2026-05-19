#[doc = "Register `INTCLEAR0` writer"]
pub type W = crate::W<Intclear0Spec>;
#[doc = "Field `CH1_CLEAR_BLOCK_TFR_DONE_INTSTAT` writer - NA"]
pub type Ch1ClearBlockTfrDoneIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_DMA_TFR_DONE_INTSTAT` writer - NA"]
pub type Ch1ClearDmaTfrDoneIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_SRC_TRANSCOMP_INTSTAT` writer - NA"]
pub type Ch1ClearSrcTranscompIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_DST_TRANSCOMP_INTSTAT` writer - NA"]
pub type Ch1ClearDstTranscompIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_SRC_DEC_ERR_INTSTAT` writer - NA"]
pub type Ch1ClearSrcDecErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_DST_DEC_ERR_INTSTAT` writer - NA"]
pub type Ch1ClearDstDecErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_SRC_SLV_ERR_INTSTAT` writer - NA"]
pub type Ch1ClearSrcSlvErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_DST_SLV_ERR_INTSTAT` writer - NA"]
pub type Ch1ClearDstSlvErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_LLI_RD_DEC_ERR_INTSTAT` writer - NA"]
pub type Ch1ClearLliRdDecErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_LLI_WR_DEC_ERR_INTSTAT` writer - NA"]
pub type Ch1ClearLliWrDecErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_LLI_RD_SLV_ERR_INTSTAT` writer - NA"]
pub type Ch1ClearLliRdSlvErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_LLI_WR_SLV_ERR_INTSTAT` writer - NA"]
pub type Ch1ClearLliWrSlvErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_SHADOWREG_OR_LLI_INVALID_ERR_INTSTAT` writer - NA"]
pub type Ch1ClearShadowregOrLliInvalidErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_SLVIF_MULTIBLKTYPE_ERR_INTSTAT` writer - NA"]
pub type Ch1ClearSlvifMultiblktypeErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_SLVIF_DEC_ERR_INTSTAT` writer - NA"]
pub type Ch1ClearSlvifDecErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_SLVIF_WR2RO_ERR_INTSTAT` writer - NA"]
pub type Ch1ClearSlvifWr2roErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_SLVIF_RD2RWO_ERR_INTSTAT` writer - NA"]
pub type Ch1ClearSlvifRd2rwoErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_SLVIF_WRONCHEN_ERR_INTSTAT` writer - NA"]
pub type Ch1ClearSlvifWronchenErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_SLVIF_SHADOWREG_WRON_VALID_ERR_INTSTAT` writer - NA"]
pub type Ch1ClearSlvifShadowregWronValidErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_SLVIF_WRONHOLD_ERR_INTSTAT` writer - NA"]
pub type Ch1ClearSlvifWronholdErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_SLVIF_WRPARITY_ERR_INTSTAT` writer - NA"]
pub type Ch1ClearSlvifWrparityErrIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_CH_LOCK_CLEARED_INTSTAT` writer - NA"]
pub type Ch1ClearChLockClearedIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_CH_SRC_SUSPENDED_INTSTAT` writer - NA"]
pub type Ch1ClearChSrcSuspendedIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_CH_SUSPENDED_INTSTAT` writer - NA"]
pub type Ch1ClearChSuspendedIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_CH_DISABLED_INTSTAT` writer - NA"]
pub type Ch1ClearChDisabledIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_CH_ABORTED_INTSTAT` writer - NA"]
pub type Ch1ClearChAbortedIntstatW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_clear_block_tfr_done_intstat(
        &mut self,
    ) -> Ch1ClearBlockTfrDoneIntstatW<'_, Intclear0Spec> {
        Ch1ClearBlockTfrDoneIntstatW::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ch1_clear_dma_tfr_done_intstat(
        &mut self,
    ) -> Ch1ClearDmaTfrDoneIntstatW<'_, Intclear0Spec> {
        Ch1ClearDmaTfrDoneIntstatW::new(self, 1)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ch1_clear_src_transcomp_intstat(
        &mut self,
    ) -> Ch1ClearSrcTranscompIntstatW<'_, Intclear0Spec> {
        Ch1ClearSrcTranscompIntstatW::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn ch1_clear_dst_transcomp_intstat(
        &mut self,
    ) -> Ch1ClearDstTranscompIntstatW<'_, Intclear0Spec> {
        Ch1ClearDstTranscompIntstatW::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn ch1_clear_src_dec_err_intstat(
        &mut self,
    ) -> Ch1ClearSrcDecErrIntstatW<'_, Intclear0Spec> {
        Ch1ClearSrcDecErrIntstatW::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn ch1_clear_dst_dec_err_intstat(
        &mut self,
    ) -> Ch1ClearDstDecErrIntstatW<'_, Intclear0Spec> {
        Ch1ClearDstDecErrIntstatW::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn ch1_clear_src_slv_err_intstat(
        &mut self,
    ) -> Ch1ClearSrcSlvErrIntstatW<'_, Intclear0Spec> {
        Ch1ClearSrcSlvErrIntstatW::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn ch1_clear_dst_slv_err_intstat(
        &mut self,
    ) -> Ch1ClearDstSlvErrIntstatW<'_, Intclear0Spec> {
        Ch1ClearDstSlvErrIntstatW::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn ch1_clear_lli_rd_dec_err_intstat(
        &mut self,
    ) -> Ch1ClearLliRdDecErrIntstatW<'_, Intclear0Spec> {
        Ch1ClearLliRdDecErrIntstatW::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn ch1_clear_lli_wr_dec_err_intstat(
        &mut self,
    ) -> Ch1ClearLliWrDecErrIntstatW<'_, Intclear0Spec> {
        Ch1ClearLliWrDecErrIntstatW::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn ch1_clear_lli_rd_slv_err_intstat(
        &mut self,
    ) -> Ch1ClearLliRdSlvErrIntstatW<'_, Intclear0Spec> {
        Ch1ClearLliRdSlvErrIntstatW::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn ch1_clear_lli_wr_slv_err_intstat(
        &mut self,
    ) -> Ch1ClearLliWrSlvErrIntstatW<'_, Intclear0Spec> {
        Ch1ClearLliWrSlvErrIntstatW::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn ch1_clear_shadowreg_or_lli_invalid_err_intstat(
        &mut self,
    ) -> Ch1ClearShadowregOrLliInvalidErrIntstatW<'_, Intclear0Spec> {
        Ch1ClearShadowregOrLliInvalidErrIntstatW::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn ch1_clear_slvif_multiblktype_err_intstat(
        &mut self,
    ) -> Ch1ClearSlvifMultiblktypeErrIntstatW<'_, Intclear0Spec> {
        Ch1ClearSlvifMultiblktypeErrIntstatW::new(self, 14)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn ch1_clear_slvif_dec_err_intstat(
        &mut self,
    ) -> Ch1ClearSlvifDecErrIntstatW<'_, Intclear0Spec> {
        Ch1ClearSlvifDecErrIntstatW::new(self, 16)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn ch1_clear_slvif_wr2ro_err_intstat(
        &mut self,
    ) -> Ch1ClearSlvifWr2roErrIntstatW<'_, Intclear0Spec> {
        Ch1ClearSlvifWr2roErrIntstatW::new(self, 17)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    pub fn ch1_clear_slvif_rd2rwo_err_intstat(
        &mut self,
    ) -> Ch1ClearSlvifRd2rwoErrIntstatW<'_, Intclear0Spec> {
        Ch1ClearSlvifRd2rwoErrIntstatW::new(self, 18)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn ch1_clear_slvif_wronchen_err_intstat(
        &mut self,
    ) -> Ch1ClearSlvifWronchenErrIntstatW<'_, Intclear0Spec> {
        Ch1ClearSlvifWronchenErrIntstatW::new(self, 19)
    }
    #[doc = "Bit 20 - NA"]
    #[inline(always)]
    pub fn ch1_clear_slvif_shadowreg_wron_valid_err_intstat(
        &mut self,
    ) -> Ch1ClearSlvifShadowregWronValidErrIntstatW<'_, Intclear0Spec> {
        Ch1ClearSlvifShadowregWronValidErrIntstatW::new(self, 20)
    }
    #[doc = "Bit 21 - NA"]
    #[inline(always)]
    pub fn ch1_clear_slvif_wronhold_err_intstat(
        &mut self,
    ) -> Ch1ClearSlvifWronholdErrIntstatW<'_, Intclear0Spec> {
        Ch1ClearSlvifWronholdErrIntstatW::new(self, 21)
    }
    #[doc = "Bit 25 - NA"]
    #[inline(always)]
    pub fn ch1_clear_slvif_wrparity_err_intstat(
        &mut self,
    ) -> Ch1ClearSlvifWrparityErrIntstatW<'_, Intclear0Spec> {
        Ch1ClearSlvifWrparityErrIntstatW::new(self, 25)
    }
    #[doc = "Bit 27 - NA"]
    #[inline(always)]
    pub fn ch1_clear_ch_lock_cleared_intstat(
        &mut self,
    ) -> Ch1ClearChLockClearedIntstatW<'_, Intclear0Spec> {
        Ch1ClearChLockClearedIntstatW::new(self, 27)
    }
    #[doc = "Bit 28 - NA"]
    #[inline(always)]
    pub fn ch1_clear_ch_src_suspended_intstat(
        &mut self,
    ) -> Ch1ClearChSrcSuspendedIntstatW<'_, Intclear0Spec> {
        Ch1ClearChSrcSuspendedIntstatW::new(self, 28)
    }
    #[doc = "Bit 29 - NA"]
    #[inline(always)]
    pub fn ch1_clear_ch_suspended_intstat(
        &mut self,
    ) -> Ch1ClearChSuspendedIntstatW<'_, Intclear0Spec> {
        Ch1ClearChSuspendedIntstatW::new(self, 29)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    pub fn ch1_clear_ch_disabled_intstat(
        &mut self,
    ) -> Ch1ClearChDisabledIntstatW<'_, Intclear0Spec> {
        Ch1ClearChDisabledIntstatW::new(self, 30)
    }
    #[doc = "Bit 31 - NA"]
    #[inline(always)]
    pub fn ch1_clear_ch_aborted_intstat(&mut self) -> Ch1ClearChAbortedIntstatW<'_, Intclear0Spec> {
        Ch1ClearChAbortedIntstatW::new(self, 31)
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclear0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intclear0Spec;
impl crate::RegisterSpec for Intclear0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intclear0::W`](W) writer structure"]
impl crate::Writable for Intclear0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTCLEAR0 to value 0"]
impl crate::Resettable for Intclear0Spec {}
