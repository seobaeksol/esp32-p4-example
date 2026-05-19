#[doc = "Register `DEBUG_INFO0` reader"]
pub type R = crate::R<DebugInfo0Spec>;
#[doc = "Field `TOP_CTRL_INTER_DEBUG_STATE` reader - Represents top_ctrl_inter module FSM info."]
pub type TopCtrlInterDebugStateR = crate::FieldReader;
#[doc = "Field `TOP_CTRL_INTRA_DEBUG_STATE` reader - Represents top_ctrl_intra module FSM info."]
pub type TopCtrlIntraDebugStateR = crate::FieldReader;
#[doc = "Field `P_I_CMP_DEBUG_STATE` reader - Represents p_i_cmp module FSM info."]
pub type PICmpDebugStateR = crate::FieldReader;
#[doc = "Field `MVD_DEBUG_STATE` reader - Represents mvd module FSM info."]
pub type MvdDebugStateR = crate::FieldReader;
#[doc = "Field `MC_CHROMA_IP_DEBUG_STATE` reader - Represents mc_chroma_ip module FSM info."]
pub type McChromaIpDebugStateR = crate::BitReader;
#[doc = "Field `INTRA_16X16_CHROMA_CTRL_DEBUG_STATE` reader - Represents intra_16x16_chroma_ctrl module FSM info."]
pub type Intra16x16ChromaCtrlDebugStateR = crate::FieldReader;
#[doc = "Field `INTRA_4X4_CTRL_DEBUG_STATE` reader - Represents intra_4x4_ctrl module FSM info."]
pub type Intra4x4CtrlDebugStateR = crate::FieldReader;
#[doc = "Field `INTRA_TOP_CTRL_DEBUG_STATE` reader - Represents intra_top_ctrl module FSM info."]
pub type IntraTopCtrlDebugStateR = crate::FieldReader;
#[doc = "Field `IME_CTRL_DEBUG_STATE` reader - Represents ime_ctrl module FSM info."]
pub type ImeCtrlDebugStateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Represents top_ctrl_inter module FSM info."]
    #[inline(always)]
    pub fn top_ctrl_inter_debug_state(&self) -> TopCtrlInterDebugStateR {
        TopCtrlInterDebugStateR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Represents top_ctrl_intra module FSM info."]
    #[inline(always)]
    pub fn top_ctrl_intra_debug_state(&self) -> TopCtrlIntraDebugStateR {
        TopCtrlIntraDebugStateR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Represents p_i_cmp module FSM info."]
    #[inline(always)]
    pub fn p_i_cmp_debug_state(&self) -> PICmpDebugStateR {
        PICmpDebugStateR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12 - Represents mvd module FSM info."]
    #[inline(always)]
    pub fn mvd_debug_state(&self) -> MvdDebugStateR {
        MvdDebugStateR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - Represents mc_chroma_ip module FSM info."]
    #[inline(always)]
    pub fn mc_chroma_ip_debug_state(&self) -> McChromaIpDebugStateR {
        McChromaIpDebugStateR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:17 - Represents intra_16x16_chroma_ctrl module FSM info."]
    #[inline(always)]
    pub fn intra_16x16_chroma_ctrl_debug_state(&self) -> Intra16x16ChromaCtrlDebugStateR {
        Intra16x16ChromaCtrlDebugStateR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - Represents intra_4x4_ctrl module FSM info."]
    #[inline(always)]
    pub fn intra_4x4_ctrl_debug_state(&self) -> Intra4x4CtrlDebugStateR {
        Intra4x4CtrlDebugStateR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:24 - Represents intra_top_ctrl module FSM info."]
    #[inline(always)]
    pub fn intra_top_ctrl_debug_state(&self) -> IntraTopCtrlDebugStateR {
        IntraTopCtrlDebugStateR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - Represents ime_ctrl module FSM info."]
    #[inline(always)]
    pub fn ime_ctrl_debug_state(&self) -> ImeCtrlDebugStateR {
        ImeCtrlDebugStateR::new(((self.bits >> 25) & 7) as u8)
    }
}
#[doc = "Debug information register0.\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_info0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugInfo0Spec;
impl crate::RegisterSpec for DebugInfo0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_info0::R`](R) reader structure"]
impl crate::Readable for DebugInfo0Spec {}
#[doc = "`reset()` method sets DEBUG_INFO0 to value 0"]
impl crate::Resettable for DebugInfo0Spec {}
