#[doc = "Register `DEBUG_INFO2` reader"]
pub type R = crate::R<DebugInfo2Spec>;
#[doc = "Field `P_RC_DONE_DEBUG_FLAG` reader - Represents p rate ctrl done status.\\\\0: not done\\\\1: done."]
pub type PRcDoneDebugFlagR = crate::BitReader;
#[doc = "Field `P_P_I_CMP_DONE_DEBUG_FLAG` reader - Represents p p_i_cmp done status.\\\\0: not done\\\\1: done."]
pub type PPICmpDoneDebugFlagR = crate::BitReader;
#[doc = "Field `P_MV_MERGE_DONE_DEBUG_FLAG` reader - Represents p mv merge done status.\\\\0: not done\\\\1: done."]
pub type PMvMergeDoneDebugFlagR = crate::BitReader;
#[doc = "Field `P_MOVE_ORI_DONE_DEBUG_FLAG` reader - Represents p move origin done status.\\\\0: not done\\\\1: done."]
pub type PMoveOriDoneDebugFlagR = crate::BitReader;
#[doc = "Field `P_MC_DONE_DEBUG_FLAG` reader - Represents p mc done status.\\\\0: not done\\\\1: done."]
pub type PMcDoneDebugFlagR = crate::BitReader;
#[doc = "Field `P_IME_DONE_DEBUG_FLAG` reader - Represents p ime done status.\\\\0: not done\\\\1: done."]
pub type PImeDoneDebugFlagR = crate::BitReader;
#[doc = "Field `P_GET_ORI_DONE_DEBUG_FLAG` reader - Represents p get origin done status.\\\\0: not done\\\\1: done."]
pub type PGetOriDoneDebugFlagR = crate::BitReader;
#[doc = "Field `P_FME_DONE_DEBUG_FLAG` reader - Represents p fme done status.\\\\0: not done\\\\1: done."]
pub type PFmeDoneDebugFlagR = crate::BitReader;
#[doc = "Field `P_FETCH_DONE_DEBUG_FLAG` reader - Represents p fetch done status.\\\\0: not done\\\\1: done."]
pub type PFetchDoneDebugFlagR = crate::BitReader;
#[doc = "Field `P_DB_DONE_DEBUG_FLAG` reader - Represents p deblocking done status.\\\\0: not done\\\\1: done."]
pub type PDbDoneDebugFlagR = crate::BitReader;
#[doc = "Field `P_BS_BUF_DONE_DEBUG_FLAG` reader - Represents p bitstream buffer done status.\\\\0: not done\\\\1: done."]
pub type PBsBufDoneDebugFlagR = crate::BitReader;
#[doc = "Field `REF_MOVE_2MB_LINE_DONE_DEBUG_FLAG` reader - Represents dma move 2 ref mb line done status.\\\\0: not done\\\\1: done."]
pub type RefMove2mbLineDoneDebugFlagR = crate::BitReader;
#[doc = "Field `I_P_I_CMP_DONE_DEBUG_FLAG` reader - Represents I p_i_cmp done status.\\\\0: not done\\\\1: done."]
pub type IPICmpDoneDebugFlagR = crate::BitReader;
#[doc = "Field `I_MOVE_ORI_DONE_DEBUG_FLAG` reader - Represents I move origin done status.\\\\0: not done\\\\1: done."]
pub type IMoveOriDoneDebugFlagR = crate::BitReader;
#[doc = "Field `I_GET_ORI_DONE_DEBUG_FLAG` reader - Represents I get origin done status.\\\\0: not done\\\\1: done."]
pub type IGetOriDoneDebugFlagR = crate::BitReader;
#[doc = "Field `I_EC_DONE_DEBUG_FLAG` reader - Represents I encoder done status.\\\\0: not done\\\\1: done."]
pub type IEcDoneDebugFlagR = crate::BitReader;
#[doc = "Field `I_DB_DONE_DEBUG_FLAG` reader - Represents I deblocking done status.\\\\0: not done\\\\1: done."]
pub type IDbDoneDebugFlagR = crate::BitReader;
#[doc = "Field `I_BS_BUF_DONE_DEBUG_FLAG` reader - Represents I bitstream buffer done status.\\\\0: not done\\\\1: done."]
pub type IBsBufDoneDebugFlagR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents p rate ctrl done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_rc_done_debug_flag(&self) -> PRcDoneDebugFlagR {
        PRcDoneDebugFlagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents p p_i_cmp done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_p_i_cmp_done_debug_flag(&self) -> PPICmpDoneDebugFlagR {
        PPICmpDoneDebugFlagR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents p mv merge done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_mv_merge_done_debug_flag(&self) -> PMvMergeDoneDebugFlagR {
        PMvMergeDoneDebugFlagR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents p move origin done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_move_ori_done_debug_flag(&self) -> PMoveOriDoneDebugFlagR {
        PMoveOriDoneDebugFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents p mc done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_mc_done_debug_flag(&self) -> PMcDoneDebugFlagR {
        PMcDoneDebugFlagR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents p ime done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_ime_done_debug_flag(&self) -> PImeDoneDebugFlagR {
        PImeDoneDebugFlagR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents p get origin done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_get_ori_done_debug_flag(&self) -> PGetOriDoneDebugFlagR {
        PGetOriDoneDebugFlagR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents p fme done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_fme_done_debug_flag(&self) -> PFmeDoneDebugFlagR {
        PFmeDoneDebugFlagR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents p fetch done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_fetch_done_debug_flag(&self) -> PFetchDoneDebugFlagR {
        PFetchDoneDebugFlagR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents p deblocking done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_db_done_debug_flag(&self) -> PDbDoneDebugFlagR {
        PDbDoneDebugFlagR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents p bitstream buffer done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_bs_buf_done_debug_flag(&self) -> PBsBufDoneDebugFlagR {
        PBsBufDoneDebugFlagR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents dma move 2 ref mb line done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn ref_move_2mb_line_done_debug_flag(&self) -> RefMove2mbLineDoneDebugFlagR {
        RefMove2mbLineDoneDebugFlagR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents I p_i_cmp done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn i_p_i_cmp_done_debug_flag(&self) -> IPICmpDoneDebugFlagR {
        IPICmpDoneDebugFlagR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents I move origin done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn i_move_ori_done_debug_flag(&self) -> IMoveOriDoneDebugFlagR {
        IMoveOriDoneDebugFlagR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents I get origin done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn i_get_ori_done_debug_flag(&self) -> IGetOriDoneDebugFlagR {
        IGetOriDoneDebugFlagR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents I encoder done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn i_ec_done_debug_flag(&self) -> IEcDoneDebugFlagR {
        IEcDoneDebugFlagR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents I deblocking done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn i_db_done_debug_flag(&self) -> IDbDoneDebugFlagR {
        IDbDoneDebugFlagR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents I bitstream buffer done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn i_bs_buf_done_debug_flag(&self) -> IBsBufDoneDebugFlagR {
        IBsBufDoneDebugFlagR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Debug information register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_info2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugInfo2Spec;
impl crate::RegisterSpec for DebugInfo2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_info2::R`](R) reader structure"]
impl crate::Readable for DebugInfo2Spec {}
#[doc = "`reset()` method sets DEBUG_INFO2 to value 0"]
impl crate::Resettable for DebugInfo2Spec {}
