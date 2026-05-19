#[doc = "Register `L1_ICACHE2_AUTOLOAD_CTRL` reader"]
pub type R = crate::R<L1Icache2AutoloadCtrlSpec>;
#[doc = "Field `L1_ICACHE2_AUTOLOAD_ENA` reader - The bit is used to enable and disable autoload operation on L1-ICache2. 1: enable, 0: disable."]
pub type L1Icache2AutoloadEnaR = crate::BitReader;
#[doc = "Field `L1_ICACHE2_AUTOLOAD_DONE` reader - The bit is used to indicate whether autoload operation on L1-ICache2 is finished or not. 0: not finished. 1: finished."]
pub type L1Icache2AutoloadDoneR = crate::BitReader;
#[doc = "Field `L1_ICACHE2_AUTOLOAD_ORDER` reader - The bit is used to configure the direction of autoload operation on L1-ICache2. 0: ascending. 1: descending."]
pub type L1Icache2AutoloadOrderR = crate::BitReader;
#[doc = "Field `L1_ICACHE2_AUTOLOAD_TRIGGER_MODE` reader - The field is used to configure trigger mode of autoload operation on L1-ICache2. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
pub type L1Icache2AutoloadTriggerModeR = crate::FieldReader;
#[doc = "Field `L1_ICACHE2_AUTOLOAD_SCT0_ENA` reader - The bit is used to enable the first section for autoload operation on L1-ICache2."]
pub type L1Icache2AutoloadSct0EnaR = crate::BitReader;
#[doc = "Field `L1_ICACHE2_AUTOLOAD_SCT1_ENA` reader - The bit is used to enable the second section for autoload operation on L1-ICache2."]
pub type L1Icache2AutoloadSct1EnaR = crate::BitReader;
#[doc = "Field `L1_ICACHE2_AUTOLOAD_RGID` reader - The bit is used to set the gid of l1 icache2 autoload."]
pub type L1Icache2AutoloadRgidR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable and disable autoload operation on L1-ICache2. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn l1_icache2_autoload_ena(&self) -> L1Icache2AutoloadEnaR {
        L1Icache2AutoloadEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate whether autoload operation on L1-ICache2 is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn l1_icache2_autoload_done(&self) -> L1Icache2AutoloadDoneR {
        L1Icache2AutoloadDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of autoload operation on L1-ICache2. 0: ascending. 1: descending."]
    #[inline(always)]
    pub fn l1_icache2_autoload_order(&self) -> L1Icache2AutoloadOrderR {
        L1Icache2AutoloadOrderR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - The field is used to configure trigger mode of autoload operation on L1-ICache2. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
    #[inline(always)]
    pub fn l1_icache2_autoload_trigger_mode(&self) -> L1Icache2AutoloadTriggerModeR {
        L1Icache2AutoloadTriggerModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 8 - The bit is used to enable the first section for autoload operation on L1-ICache2."]
    #[inline(always)]
    pub fn l1_icache2_autoload_sct0_ena(&self) -> L1Icache2AutoloadSct0EnaR {
        L1Icache2AutoloadSct0EnaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is used to enable the second section for autoload operation on L1-ICache2."]
    #[inline(always)]
    pub fn l1_icache2_autoload_sct1_ena(&self) -> L1Icache2AutoloadSct1EnaR {
        L1Icache2AutoloadSct1EnaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - The bit is used to set the gid of l1 icache2 autoload."]
    #[inline(always)]
    pub fn l1_icache2_autoload_rgid(&self) -> L1Icache2AutoloadRgidR {
        L1Icache2AutoloadRgidR::new(((self.bits >> 10) & 0x0f) as u8)
    }
}
#[doc = "L1 instruction Cache 2 autoload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache2_autoload_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache2AutoloadCtrlSpec;
impl crate::RegisterSpec for L1Icache2AutoloadCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache2_autoload_ctrl::R`](R) reader structure"]
impl crate::Readable for L1Icache2AutoloadCtrlSpec {}
#[doc = "`reset()` method sets L1_ICACHE2_AUTOLOAD_CTRL to value 0x02"]
impl crate::Resettable for L1Icache2AutoloadCtrlSpec {
    const RESET_VALUE: u32 = 0x02;
}
