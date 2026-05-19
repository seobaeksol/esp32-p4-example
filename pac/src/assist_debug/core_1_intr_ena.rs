#[doc = "Register `CORE_1_INTR_ENA` reader"]
pub type R = crate::R<Core1IntrEnaSpec>;
#[doc = "Register `CORE_1_INTR_ENA` writer"]
pub type W = crate::W<Core1IntrEnaSpec>;
#[doc = "Field `CORE_1_AREA_DRAM0_0_RD_ENA` reader - Core1 dram0 area0 read monitor enable"]
pub type Core1AreaDram0_0RdEnaR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_0_RD_ENA` writer - Core1 dram0 area0 read monitor enable"]
pub type Core1AreaDram0_0RdEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_DRAM0_0_WR_ENA` reader - Core1 dram0 area0 write monitor enable"]
pub type Core1AreaDram0_0WrEnaR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_0_WR_ENA` writer - Core1 dram0 area0 write monitor enable"]
pub type Core1AreaDram0_0WrEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_DRAM0_1_RD_ENA` reader - Core1 dram0 area1 read monitor enable"]
pub type Core1AreaDram0_1RdEnaR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_1_RD_ENA` writer - Core1 dram0 area1 read monitor enable"]
pub type Core1AreaDram0_1RdEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_DRAM0_1_WR_ENA` reader - Core1 dram0 area1 write monitor enable"]
pub type Core1AreaDram0_1WrEnaR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_1_WR_ENA` writer - Core1 dram0 area1 write monitor enable"]
pub type Core1AreaDram0_1WrEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_PIF_0_RD_ENA` reader - Core1 PIF area0 read monitor enable"]
pub type Core1AreaPif0RdEnaR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_0_RD_ENA` writer - Core1 PIF area0 read monitor enable"]
pub type Core1AreaPif0RdEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_PIF_0_WR_ENA` reader - Core1 PIF area0 write monitor enable"]
pub type Core1AreaPif0WrEnaR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_0_WR_ENA` writer - Core1 PIF area0 write monitor enable"]
pub type Core1AreaPif0WrEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_PIF_1_RD_ENA` reader - Core1 PIF area1 read monitor enable"]
pub type Core1AreaPif1RdEnaR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_1_RD_ENA` writer - Core1 PIF area1 read monitor enable"]
pub type Core1AreaPif1RdEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_PIF_1_WR_ENA` reader - Core1 PIF area1 write monitor enable"]
pub type Core1AreaPif1WrEnaR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_1_WR_ENA` writer - Core1 PIF area1 write monitor enable"]
pub type Core1AreaPif1WrEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_SP_SPILL_MIN_ENA` reader - Core1 stackpoint underflow monitor enable"]
pub type Core1SpSpillMinEnaR = crate::BitReader;
#[doc = "Field `CORE_1_SP_SPILL_MIN_ENA` writer - Core1 stackpoint underflow monitor enable"]
pub type Core1SpSpillMinEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_SP_SPILL_MAX_ENA` reader - Core1 stackpoint overflow monitor enable"]
pub type Core1SpSpillMaxEnaR = crate::BitReader;
#[doc = "Field `CORE_1_SP_SPILL_MAX_ENA` writer - Core1 stackpoint overflow monitor enable"]
pub type Core1SpSpillMaxEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_IRAM0_EXCEPTION_MONITOR_ENA` reader - IBUS busy monitor enable"]
pub type Core1Iram0ExceptionMonitorEnaR = crate::BitReader;
#[doc = "Field `CORE_1_IRAM0_EXCEPTION_MONITOR_ENA` writer - IBUS busy monitor enable"]
pub type Core1Iram0ExceptionMonitorEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_DRAM0_EXCEPTION_MONITOR_ENA` reader - DBUS busy monitor enbale"]
pub type Core1Dram0ExceptionMonitorEnaR = crate::BitReader;
#[doc = "Field `CORE_1_DRAM0_EXCEPTION_MONITOR_ENA` writer - DBUS busy monitor enbale"]
pub type Core1Dram0ExceptionMonitorEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Core1 dram0 area0 read monitor enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_rd_ena(&self) -> Core1AreaDram0_0RdEnaR {
        Core1AreaDram0_0RdEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core1 dram0 area0 write monitor enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_wr_ena(&self) -> Core1AreaDram0_0WrEnaR {
        Core1AreaDram0_0WrEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Core1 dram0 area1 read monitor enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_rd_ena(&self) -> Core1AreaDram0_1RdEnaR {
        Core1AreaDram0_1RdEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Core1 dram0 area1 write monitor enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_wr_ena(&self) -> Core1AreaDram0_1WrEnaR {
        Core1AreaDram0_1WrEnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Core1 PIF area0 read monitor enable"]
    #[inline(always)]
    pub fn core_1_area_pif_0_rd_ena(&self) -> Core1AreaPif0RdEnaR {
        Core1AreaPif0RdEnaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Core1 PIF area0 write monitor enable"]
    #[inline(always)]
    pub fn core_1_area_pif_0_wr_ena(&self) -> Core1AreaPif0WrEnaR {
        Core1AreaPif0WrEnaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Core1 PIF area1 read monitor enable"]
    #[inline(always)]
    pub fn core_1_area_pif_1_rd_ena(&self) -> Core1AreaPif1RdEnaR {
        Core1AreaPif1RdEnaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Core1 PIF area1 write monitor enable"]
    #[inline(always)]
    pub fn core_1_area_pif_1_wr_ena(&self) -> Core1AreaPif1WrEnaR {
        Core1AreaPif1WrEnaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Core1 stackpoint underflow monitor enable"]
    #[inline(always)]
    pub fn core_1_sp_spill_min_ena(&self) -> Core1SpSpillMinEnaR {
        Core1SpSpillMinEnaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Core1 stackpoint overflow monitor enable"]
    #[inline(always)]
    pub fn core_1_sp_spill_max_ena(&self) -> Core1SpSpillMaxEnaR {
        Core1SpSpillMaxEnaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IBUS busy monitor enable"]
    #[inline(always)]
    pub fn core_1_iram0_exception_monitor_ena(&self) -> Core1Iram0ExceptionMonitorEnaR {
        Core1Iram0ExceptionMonitorEnaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBUS busy monitor enbale"]
    #[inline(always)]
    pub fn core_1_dram0_exception_monitor_ena(&self) -> Core1Dram0ExceptionMonitorEnaR {
        Core1Dram0ExceptionMonitorEnaR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core1 dram0 area0 read monitor enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_rd_ena(&mut self) -> Core1AreaDram0_0RdEnaW<'_, Core1IntrEnaSpec> {
        Core1AreaDram0_0RdEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - Core1 dram0 area0 write monitor enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_wr_ena(&mut self) -> Core1AreaDram0_0WrEnaW<'_, Core1IntrEnaSpec> {
        Core1AreaDram0_0WrEnaW::new(self, 1)
    }
    #[doc = "Bit 2 - Core1 dram0 area1 read monitor enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_rd_ena(&mut self) -> Core1AreaDram0_1RdEnaW<'_, Core1IntrEnaSpec> {
        Core1AreaDram0_1RdEnaW::new(self, 2)
    }
    #[doc = "Bit 3 - Core1 dram0 area1 write monitor enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_wr_ena(&mut self) -> Core1AreaDram0_1WrEnaW<'_, Core1IntrEnaSpec> {
        Core1AreaDram0_1WrEnaW::new(self, 3)
    }
    #[doc = "Bit 4 - Core1 PIF area0 read monitor enable"]
    #[inline(always)]
    pub fn core_1_area_pif_0_rd_ena(&mut self) -> Core1AreaPif0RdEnaW<'_, Core1IntrEnaSpec> {
        Core1AreaPif0RdEnaW::new(self, 4)
    }
    #[doc = "Bit 5 - Core1 PIF area0 write monitor enable"]
    #[inline(always)]
    pub fn core_1_area_pif_0_wr_ena(&mut self) -> Core1AreaPif0WrEnaW<'_, Core1IntrEnaSpec> {
        Core1AreaPif0WrEnaW::new(self, 5)
    }
    #[doc = "Bit 6 - Core1 PIF area1 read monitor enable"]
    #[inline(always)]
    pub fn core_1_area_pif_1_rd_ena(&mut self) -> Core1AreaPif1RdEnaW<'_, Core1IntrEnaSpec> {
        Core1AreaPif1RdEnaW::new(self, 6)
    }
    #[doc = "Bit 7 - Core1 PIF area1 write monitor enable"]
    #[inline(always)]
    pub fn core_1_area_pif_1_wr_ena(&mut self) -> Core1AreaPif1WrEnaW<'_, Core1IntrEnaSpec> {
        Core1AreaPif1WrEnaW::new(self, 7)
    }
    #[doc = "Bit 8 - Core1 stackpoint underflow monitor enable"]
    #[inline(always)]
    pub fn core_1_sp_spill_min_ena(&mut self) -> Core1SpSpillMinEnaW<'_, Core1IntrEnaSpec> {
        Core1SpSpillMinEnaW::new(self, 8)
    }
    #[doc = "Bit 9 - Core1 stackpoint overflow monitor enable"]
    #[inline(always)]
    pub fn core_1_sp_spill_max_ena(&mut self) -> Core1SpSpillMaxEnaW<'_, Core1IntrEnaSpec> {
        Core1SpSpillMaxEnaW::new(self, 9)
    }
    #[doc = "Bit 10 - IBUS busy monitor enable"]
    #[inline(always)]
    pub fn core_1_iram0_exception_monitor_ena(
        &mut self,
    ) -> Core1Iram0ExceptionMonitorEnaW<'_, Core1IntrEnaSpec> {
        Core1Iram0ExceptionMonitorEnaW::new(self, 10)
    }
    #[doc = "Bit 11 - DBUS busy monitor enbale"]
    #[inline(always)]
    pub fn core_1_dram0_exception_monitor_ena(
        &mut self,
    ) -> Core1Dram0ExceptionMonitorEnaW<'_, Core1IntrEnaSpec> {
        Core1Dram0ExceptionMonitorEnaW::new(self, 11)
    }
}
#[doc = "core1 monitor enable configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_intr_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_intr_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1IntrEnaSpec;
impl crate::RegisterSpec for Core1IntrEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_intr_ena::R`](R) reader structure"]
impl crate::Readable for Core1IntrEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`core_1_intr_ena::W`](W) writer structure"]
impl crate::Writable for Core1IntrEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_1_INTR_ENA to value 0"]
impl crate::Resettable for Core1IntrEnaSpec {}
