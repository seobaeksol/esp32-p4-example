#[doc = "Register `DEBUG_INFO1` reader"]
pub type R = crate::R<DebugInfo1Spec>;
#[doc = "Field `FME_CTRL_DEBUG_STATE` reader - Represents fme_ctrl module FSM info."]
pub type FmeCtrlDebugStateR = crate::FieldReader;
#[doc = "Field `DECI_CALC_DEBUG_STATE` reader - Represents deci_calc module's FSM info. DEV use only."]
pub type DeciCalcDebugStateR = crate::FieldReader;
#[doc = "Field `DB_DEBUG_STATE` reader - Represents db module FSM info."]
pub type DbDebugStateR = crate::FieldReader;
#[doc = "Field `CAVLC_ENC_DEBUG_STATE` reader - Represents cavlc module enc FSM info."]
pub type CavlcEncDebugStateR = crate::FieldReader;
#[doc = "Field `CAVLC_SCAN_DEBUG_STATE` reader - Represents cavlc module scan FSM info."]
pub type CavlcScanDebugStateR = crate::FieldReader;
#[doc = "Field `CAVLC_CTRL_DEBUG_STATE` reader - Represents cavlc module ctrl FSM info."]
pub type CavlcCtrlDebugStateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Represents fme_ctrl module FSM info."]
    #[inline(always)]
    pub fn fme_ctrl_debug_state(&self) -> FmeCtrlDebugStateR {
        FmeCtrlDebugStateR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Represents deci_calc module's FSM info. DEV use only."]
    #[inline(always)]
    pub fn deci_calc_debug_state(&self) -> DeciCalcDebugStateR {
        DeciCalcDebugStateR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Represents db module FSM info."]
    #[inline(always)]
    pub fn db_debug_state(&self) -> DbDebugStateR {
        DbDebugStateR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Represents cavlc module enc FSM info."]
    #[inline(always)]
    pub fn cavlc_enc_debug_state(&self) -> CavlcEncDebugStateR {
        CavlcEncDebugStateR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Represents cavlc module scan FSM info."]
    #[inline(always)]
    pub fn cavlc_scan_debug_state(&self) -> CavlcScanDebugStateR {
        CavlcScanDebugStateR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Represents cavlc module ctrl FSM info."]
    #[inline(always)]
    pub fn cavlc_ctrl_debug_state(&self) -> CavlcCtrlDebugStateR {
        CavlcCtrlDebugStateR::new(((self.bits >> 16) & 3) as u8)
    }
}
#[doc = "Debug information register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_info1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugInfo1Spec;
impl crate::RegisterSpec for DebugInfo1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_info1::R`](R) reader structure"]
impl crate::Readable for DebugInfo1Spec {}
#[doc = "`reset()` method sets DEBUG_INFO1 to value 0"]
impl crate::Resettable for DebugInfo1Spec {}
