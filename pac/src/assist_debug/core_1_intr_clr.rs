#[doc = "Register `CORE_1_INTR_CLR` writer"]
pub type W = crate::W<Core1IntrClrSpec>;
#[doc = "Field `CORE_1_AREA_DRAM0_0_RD_CLR` writer - Core1 dram0 area0 read monitor interrupt clr"]
pub type Core1AreaDram0_0RdClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_DRAM0_0_WR_CLR` writer - Core1 dram0 area0 write monitor interrupt clr"]
pub type Core1AreaDram0_0WrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_DRAM0_1_RD_CLR` writer - Core1 dram0 area1 read monitor interrupt clr"]
pub type Core1AreaDram0_1RdClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_DRAM0_1_WR_CLR` writer - Core1 dram0 area1 write monitor interrupt clr"]
pub type Core1AreaDram0_1WrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_PIF_0_RD_CLR` writer - Core1 PIF area0 read monitor interrupt clr"]
pub type Core1AreaPif0RdClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_PIF_0_WR_CLR` writer - Core1 PIF area0 write monitor interrupt clr"]
pub type Core1AreaPif0WrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_PIF_1_RD_CLR` writer - Core1 PIF area1 read monitor interrupt clr"]
pub type Core1AreaPif1RdClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_PIF_1_WR_CLR` writer - Core1 PIF area1 write monitor interrupt clr"]
pub type Core1AreaPif1WrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_SP_SPILL_MIN_CLR` writer - Core1 stackpoint underflow monitor interrupt clr"]
pub type Core1SpSpillMinClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_SP_SPILL_MAX_CLR` writer - Core1 stackpoint overflow monitor interrupt clr"]
pub type Core1SpSpillMaxClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_IRAM0_EXCEPTION_MONITOR_CLR` writer - IBUS busy monitor interrupt clr"]
pub type Core1Iram0ExceptionMonitorClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_DRAM0_EXCEPTION_MONITOR_CLR` writer - DBUS busy monitor interrupt clr"]
pub type Core1Dram0ExceptionMonitorClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Core1 dram0 area0 read monitor interrupt clr"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_rd_clr(&mut self) -> Core1AreaDram0_0RdClrW<'_, Core1IntrClrSpec> {
        Core1AreaDram0_0RdClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Core1 dram0 area0 write monitor interrupt clr"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_wr_clr(&mut self) -> Core1AreaDram0_0WrClrW<'_, Core1IntrClrSpec> {
        Core1AreaDram0_0WrClrW::new(self, 1)
    }
    #[doc = "Bit 2 - Core1 dram0 area1 read monitor interrupt clr"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_rd_clr(&mut self) -> Core1AreaDram0_1RdClrW<'_, Core1IntrClrSpec> {
        Core1AreaDram0_1RdClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Core1 dram0 area1 write monitor interrupt clr"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_wr_clr(&mut self) -> Core1AreaDram0_1WrClrW<'_, Core1IntrClrSpec> {
        Core1AreaDram0_1WrClrW::new(self, 3)
    }
    #[doc = "Bit 4 - Core1 PIF area0 read monitor interrupt clr"]
    #[inline(always)]
    pub fn core_1_area_pif_0_rd_clr(&mut self) -> Core1AreaPif0RdClrW<'_, Core1IntrClrSpec> {
        Core1AreaPif0RdClrW::new(self, 4)
    }
    #[doc = "Bit 5 - Core1 PIF area0 write monitor interrupt clr"]
    #[inline(always)]
    pub fn core_1_area_pif_0_wr_clr(&mut self) -> Core1AreaPif0WrClrW<'_, Core1IntrClrSpec> {
        Core1AreaPif0WrClrW::new(self, 5)
    }
    #[doc = "Bit 6 - Core1 PIF area1 read monitor interrupt clr"]
    #[inline(always)]
    pub fn core_1_area_pif_1_rd_clr(&mut self) -> Core1AreaPif1RdClrW<'_, Core1IntrClrSpec> {
        Core1AreaPif1RdClrW::new(self, 6)
    }
    #[doc = "Bit 7 - Core1 PIF area1 write monitor interrupt clr"]
    #[inline(always)]
    pub fn core_1_area_pif_1_wr_clr(&mut self) -> Core1AreaPif1WrClrW<'_, Core1IntrClrSpec> {
        Core1AreaPif1WrClrW::new(self, 7)
    }
    #[doc = "Bit 8 - Core1 stackpoint underflow monitor interrupt clr"]
    #[inline(always)]
    pub fn core_1_sp_spill_min_clr(&mut self) -> Core1SpSpillMinClrW<'_, Core1IntrClrSpec> {
        Core1SpSpillMinClrW::new(self, 8)
    }
    #[doc = "Bit 9 - Core1 stackpoint overflow monitor interrupt clr"]
    #[inline(always)]
    pub fn core_1_sp_spill_max_clr(&mut self) -> Core1SpSpillMaxClrW<'_, Core1IntrClrSpec> {
        Core1SpSpillMaxClrW::new(self, 9)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt clr"]
    #[inline(always)]
    pub fn core_1_iram0_exception_monitor_clr(
        &mut self,
    ) -> Core1Iram0ExceptionMonitorClrW<'_, Core1IntrClrSpec> {
        Core1Iram0ExceptionMonitorClrW::new(self, 10)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt clr"]
    #[inline(always)]
    pub fn core_1_dram0_exception_monitor_clr(
        &mut self,
    ) -> Core1Dram0ExceptionMonitorClrW<'_, Core1IntrClrSpec> {
        Core1Dram0ExceptionMonitorClrW::new(self, 11)
    }
}
#[doc = "core1 monitor interrupt clr register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_intr_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1IntrClrSpec;
impl crate::RegisterSpec for Core1IntrClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`core_1_intr_clr::W`](W) writer structure"]
impl crate::Writable for Core1IntrClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_1_INTR_CLR to value 0"]
impl crate::Resettable for Core1IntrClrSpec {}
