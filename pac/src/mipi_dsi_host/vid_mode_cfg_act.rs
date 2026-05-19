#[doc = "Register `VID_MODE_CFG_ACT` reader"]
pub type R = crate::R<VidModeCfgActSpec>;
#[doc = "Field `VID_MODE_TYPE_ACT` reader - NA"]
pub type VidModeTypeActR = crate::FieldReader;
#[doc = "Field `LP_VSA_EN_ACT` reader - NA"]
pub type LpVsaEnActR = crate::BitReader;
#[doc = "Field `LP_VBP_EN_ACT` reader - NA"]
pub type LpVbpEnActR = crate::BitReader;
#[doc = "Field `LP_VFP_EN_ACT` reader - NA"]
pub type LpVfpEnActR = crate::BitReader;
#[doc = "Field `LP_VACT_EN_ACT` reader - NA"]
pub type LpVactEnActR = crate::BitReader;
#[doc = "Field `LP_HBP_EN_ACT` reader - NA"]
pub type LpHbpEnActR = crate::BitReader;
#[doc = "Field `LP_HFP_EN_ACT` reader - NA"]
pub type LpHfpEnActR = crate::BitReader;
#[doc = "Field `FRAME_BTA_ACK_EN_ACT` reader - NA"]
pub type FrameBtaAckEnActR = crate::BitReader;
#[doc = "Field `LP_CMD_EN_ACT` reader - NA"]
pub type LpCmdEnActR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn vid_mode_type_act(&self) -> VidModeTypeActR {
        VidModeTypeActR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn lp_vsa_en_act(&self) -> LpVsaEnActR {
        LpVsaEnActR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn lp_vbp_en_act(&self) -> LpVbpEnActR {
        LpVbpEnActR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn lp_vfp_en_act(&self) -> LpVfpEnActR {
        LpVfpEnActR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn lp_vact_en_act(&self) -> LpVactEnActR {
        LpVactEnActR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn lp_hbp_en_act(&self) -> LpHbpEnActR {
        LpHbpEnActR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn lp_hfp_en_act(&self) -> LpHfpEnActR {
        LpHfpEnActR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn frame_bta_ack_en_act(&self) -> FrameBtaAckEnActR {
        FrameBtaAckEnActR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn lp_cmd_en_act(&self) -> LpCmdEnActR {
        LpCmdEnActR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_mode_cfg_act::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidModeCfgActSpec;
impl crate::RegisterSpec for VidModeCfgActSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_mode_cfg_act::R`](R) reader structure"]
impl crate::Readable for VidModeCfgActSpec {}
#[doc = "`reset()` method sets VID_MODE_CFG_ACT to value 0"]
impl crate::Resettable for VidModeCfgActSpec {}
