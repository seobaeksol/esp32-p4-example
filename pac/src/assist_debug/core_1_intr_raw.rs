#[doc = "Register `CORE_1_INTR_RAW` reader"]
pub type R = crate::R<Core1IntrRawSpec>;
#[doc = "Field `CORE_1_AREA_DRAM0_0_RD_RAW` reader - Core1 dram0 area0 read monitor interrupt status"]
pub type Core1AreaDram0_0RdRawR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_0_WR_RAW` reader - Core1 dram0 area0 write monitor interrupt status"]
pub type Core1AreaDram0_0WrRawR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_1_RD_RAW` reader - Core1 dram0 area1 read monitor interrupt status"]
pub type Core1AreaDram0_1RdRawR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_1_WR_RAW` reader - Core1 dram0 area1 write monitor interrupt status"]
pub type Core1AreaDram0_1WrRawR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_0_RD_RAW` reader - Core1 PIF area0 read monitor interrupt status"]
pub type Core1AreaPif0RdRawR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_0_WR_RAW` reader - Core1 PIF area0 write monitor interrupt status"]
pub type Core1AreaPif0WrRawR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_1_RD_RAW` reader - Core1 PIF area1 read monitor interrupt status"]
pub type Core1AreaPif1RdRawR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_1_WR_RAW` reader - Core1 PIF area1 write monitor interrupt status"]
pub type Core1AreaPif1WrRawR = crate::BitReader;
#[doc = "Field `CORE_1_SP_SPILL_MIN_RAW` reader - Core1 stackpoint underflow monitor interrupt status"]
pub type Core1SpSpillMinRawR = crate::BitReader;
#[doc = "Field `CORE_1_SP_SPILL_MAX_RAW` reader - Core1 stackpoint overflow monitor interrupt status"]
pub type Core1SpSpillMaxRawR = crate::BitReader;
#[doc = "Field `CORE_1_IRAM0_EXCEPTION_MONITOR_RAW` reader - IBUS busy monitor interrupt status"]
pub type Core1Iram0ExceptionMonitorRawR = crate::BitReader;
#[doc = "Field `CORE_1_DRAM0_EXCEPTION_MONITOR_RAW` reader - DBUS busy monitor initerrupt status"]
pub type Core1Dram0ExceptionMonitorRawR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Core1 dram0 area0 read monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_rd_raw(&self) -> Core1AreaDram0_0RdRawR {
        Core1AreaDram0_0RdRawR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core1 dram0 area0 write monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_wr_raw(&self) -> Core1AreaDram0_0WrRawR {
        Core1AreaDram0_0WrRawR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Core1 dram0 area1 read monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_rd_raw(&self) -> Core1AreaDram0_1RdRawR {
        Core1AreaDram0_1RdRawR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Core1 dram0 area1 write monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_wr_raw(&self) -> Core1AreaDram0_1WrRawR {
        Core1AreaDram0_1WrRawR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Core1 PIF area0 read monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_area_pif_0_rd_raw(&self) -> Core1AreaPif0RdRawR {
        Core1AreaPif0RdRawR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Core1 PIF area0 write monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_area_pif_0_wr_raw(&self) -> Core1AreaPif0WrRawR {
        Core1AreaPif0WrRawR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Core1 PIF area1 read monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_area_pif_1_rd_raw(&self) -> Core1AreaPif1RdRawR {
        Core1AreaPif1RdRawR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Core1 PIF area1 write monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_area_pif_1_wr_raw(&self) -> Core1AreaPif1WrRawR {
        Core1AreaPif1WrRawR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Core1 stackpoint underflow monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_sp_spill_min_raw(&self) -> Core1SpSpillMinRawR {
        Core1SpSpillMinRawR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Core1 stackpoint overflow monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_sp_spill_max_raw(&self) -> Core1SpSpillMaxRawR {
        Core1SpSpillMaxRawR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_iram0_exception_monitor_raw(&self) -> Core1Iram0ExceptionMonitorRawR {
        Core1Iram0ExceptionMonitorRawR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBUS busy monitor initerrupt status"]
    #[inline(always)]
    pub fn core_1_dram0_exception_monitor_raw(&self) -> Core1Dram0ExceptionMonitorRawR {
        Core1Dram0ExceptionMonitorRawR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "core1 monitor interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_intr_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1IntrRawSpec;
impl crate::RegisterSpec for Core1IntrRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_intr_raw::R`](R) reader structure"]
impl crate::Readable for Core1IntrRawSpec {}
#[doc = "`reset()` method sets CORE_1_INTR_RAW to value 0"]
impl crate::Resettable for Core1IntrRawSpec {}
