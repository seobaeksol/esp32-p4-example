#[doc = "Register `HW_CFG` reader"]
pub type R = crate::R<HwCfgSpec>;
#[doc = "Field `QOS_SUPPORT` reader - "]
pub type QosSupportR = crate::BitReader;
#[doc = "Field `APB3_SUPPORT` reader - "]
pub type Apb3SupportR = crate::BitReader;
#[doc = "Field `AXI4_SUPPORT` reader - "]
pub type Axi4SupportR = crate::BitReader;
#[doc = "Field `LOCK_EN` reader - "]
pub type LockEnR = crate::BitReader;
#[doc = "Field `TRUST_ZONE_EN` reader - "]
pub type TrustZoneEnR = crate::BitReader;
#[doc = "Field `DECODER_TYPE` reader - "]
pub type DecoderTypeR = crate::BitReader;
#[doc = "Field `REMAP_EN` reader - "]
pub type RemapEnR = crate::BitReader;
#[doc = "Field `BI_DIR_CMD_EN` reader - "]
pub type BiDirCmdEnR = crate::BitReader;
#[doc = "Field `LOW_POWER_INF_EN` reader - "]
pub type LowPowerInfEnR = crate::BitReader;
#[doc = "Field `AXI_NUM_MASTERS` reader - "]
pub type AxiNumMastersR = crate::FieldReader;
#[doc = "Field `AXI_NUM_SLAVES` reader - "]
pub type AxiNumSlavesR = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn qos_support(&self) -> QosSupportR {
        QosSupportR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn apb3_support(&self) -> Apb3SupportR {
        Apb3SupportR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn axi4_support(&self) -> Axi4SupportR {
        Axi4SupportR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lock_en(&self) -> LockEnR {
        LockEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn trust_zone_en(&self) -> TrustZoneEnR {
        TrustZoneEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn decoder_type(&self) -> DecoderTypeR {
        DecoderTypeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn remap_en(&self) -> RemapEnR {
        RemapEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bi_dir_cmd_en(&self) -> BiDirCmdEnR {
        BiDirCmdEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn low_power_inf_en(&self) -> LowPowerInfEnR {
        LowPowerInfEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn axi_num_masters(&self) -> AxiNumMastersR {
        AxiNumMastersR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn axi_num_slaves(&self) -> AxiNumSlavesR {
        AxiNumSlavesR::new(((self.bits >> 20) & 0x1f) as u8)
    }
}
#[doc = "QoS hardware configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_cfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwCfgSpec;
impl crate::RegisterSpec for HwCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_cfg::R`](R) reader structure"]
impl crate::Readable for HwCfgSpec {}
#[doc = "`reset()` method sets HW_CFG to value 0"]
impl crate::Resettable for HwCfgSpec {}
