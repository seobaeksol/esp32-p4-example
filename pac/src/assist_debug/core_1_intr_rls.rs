#[doc = "Register `CORE_1_INTR_RLS` reader"]
pub type R = crate::R<Core1IntrRlsSpec>;
#[doc = "Register `CORE_1_INTR_RLS` writer"]
pub type W = crate::W<Core1IntrRlsSpec>;
#[doc = "Field `CORE_1_AREA_DRAM0_0_RD_RLS` reader - Core1 dram0 area0 read monitor interrupt enable"]
pub type Core1AreaDram0_0RdRlsR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_0_RD_RLS` writer - Core1 dram0 area0 read monitor interrupt enable"]
pub type Core1AreaDram0_0RdRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_DRAM0_0_WR_RLS` reader - Core1 dram0 area0 write monitor interrupt enable"]
pub type Core1AreaDram0_0WrRlsR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_0_WR_RLS` writer - Core1 dram0 area0 write monitor interrupt enable"]
pub type Core1AreaDram0_0WrRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_DRAM0_1_RD_RLS` reader - Core1 dram0 area1 read monitor interrupt enable"]
pub type Core1AreaDram0_1RdRlsR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_1_RD_RLS` writer - Core1 dram0 area1 read monitor interrupt enable"]
pub type Core1AreaDram0_1RdRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_DRAM0_1_WR_RLS` reader - Core1 dram0 area1 write monitor interrupt enable"]
pub type Core1AreaDram0_1WrRlsR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_1_WR_RLS` writer - Core1 dram0 area1 write monitor interrupt enable"]
pub type Core1AreaDram0_1WrRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_PIF_0_RD_RLS` reader - Core1 PIF area0 read monitor interrupt enable"]
pub type Core1AreaPif0RdRlsR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_0_RD_RLS` writer - Core1 PIF area0 read monitor interrupt enable"]
pub type Core1AreaPif0RdRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_PIF_0_WR_RLS` reader - Core1 PIF area0 write monitor interrupt enable"]
pub type Core1AreaPif0WrRlsR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_0_WR_RLS` writer - Core1 PIF area0 write monitor interrupt enable"]
pub type Core1AreaPif0WrRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_PIF_1_RD_RLS` reader - Core1 PIF area1 read monitor interrupt enable"]
pub type Core1AreaPif1RdRlsR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_1_RD_RLS` writer - Core1 PIF area1 read monitor interrupt enable"]
pub type Core1AreaPif1RdRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_AREA_PIF_1_WR_RLS` reader - Core1 PIF area1 write monitor interrupt enable"]
pub type Core1AreaPif1WrRlsR = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_1_WR_RLS` writer - Core1 PIF area1 write monitor interrupt enable"]
pub type Core1AreaPif1WrRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_SP_SPILL_MIN_RLS` reader - Core1 stackpoint underflow monitor interrupt enable"]
pub type Core1SpSpillMinRlsR = crate::BitReader;
#[doc = "Field `CORE_1_SP_SPILL_MIN_RLS` writer - Core1 stackpoint underflow monitor interrupt enable"]
pub type Core1SpSpillMinRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_SP_SPILL_MAX_RLS` reader - Core1 stackpoint overflow monitor interrupt enable"]
pub type Core1SpSpillMaxRlsR = crate::BitReader;
#[doc = "Field `CORE_1_SP_SPILL_MAX_RLS` writer - Core1 stackpoint overflow monitor interrupt enable"]
pub type Core1SpSpillMaxRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_IRAM0_EXCEPTION_MONITOR_RLS` reader - IBUS busy monitor interrupt enable"]
pub type Core1Iram0ExceptionMonitorRlsR = crate::BitReader;
#[doc = "Field `CORE_1_IRAM0_EXCEPTION_MONITOR_RLS` writer - IBUS busy monitor interrupt enable"]
pub type Core1Iram0ExceptionMonitorRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_DRAM0_EXCEPTION_MONITOR_RLS` reader - DBUS busy monitor interrupt enbale"]
pub type Core1Dram0ExceptionMonitorRlsR = crate::BitReader;
#[doc = "Field `CORE_1_DRAM0_EXCEPTION_MONITOR_RLS` writer - DBUS busy monitor interrupt enbale"]
pub type Core1Dram0ExceptionMonitorRlsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Core1 dram0 area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_rd_rls(&self) -> Core1AreaDram0_0RdRlsR {
        Core1AreaDram0_0RdRlsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core1 dram0 area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_wr_rls(&self) -> Core1AreaDram0_0WrRlsR {
        Core1AreaDram0_0WrRlsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Core1 dram0 area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_rd_rls(&self) -> Core1AreaDram0_1RdRlsR {
        Core1AreaDram0_1RdRlsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Core1 dram0 area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_wr_rls(&self) -> Core1AreaDram0_1WrRlsR {
        Core1AreaDram0_1WrRlsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Core1 PIF area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_pif_0_rd_rls(&self) -> Core1AreaPif0RdRlsR {
        Core1AreaPif0RdRlsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Core1 PIF area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_pif_0_wr_rls(&self) -> Core1AreaPif0WrRlsR {
        Core1AreaPif0WrRlsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Core1 PIF area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_pif_1_rd_rls(&self) -> Core1AreaPif1RdRlsR {
        Core1AreaPif1RdRlsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Core1 PIF area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_pif_1_wr_rls(&self) -> Core1AreaPif1WrRlsR {
        Core1AreaPif1WrRlsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Core1 stackpoint underflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_sp_spill_min_rls(&self) -> Core1SpSpillMinRlsR {
        Core1SpSpillMinRlsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Core1 stackpoint overflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_sp_spill_max_rls(&self) -> Core1SpSpillMaxRlsR {
        Core1SpSpillMaxRlsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_iram0_exception_monitor_rls(&self) -> Core1Iram0ExceptionMonitorRlsR {
        Core1Iram0ExceptionMonitorRlsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt enbale"]
    #[inline(always)]
    pub fn core_1_dram0_exception_monitor_rls(&self) -> Core1Dram0ExceptionMonitorRlsR {
        Core1Dram0ExceptionMonitorRlsR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core1 dram0 area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_rd_rls(&mut self) -> Core1AreaDram0_0RdRlsW<'_, Core1IntrRlsSpec> {
        Core1AreaDram0_0RdRlsW::new(self, 0)
    }
    #[doc = "Bit 1 - Core1 dram0 area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_wr_rls(&mut self) -> Core1AreaDram0_0WrRlsW<'_, Core1IntrRlsSpec> {
        Core1AreaDram0_0WrRlsW::new(self, 1)
    }
    #[doc = "Bit 2 - Core1 dram0 area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_rd_rls(&mut self) -> Core1AreaDram0_1RdRlsW<'_, Core1IntrRlsSpec> {
        Core1AreaDram0_1RdRlsW::new(self, 2)
    }
    #[doc = "Bit 3 - Core1 dram0 area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_wr_rls(&mut self) -> Core1AreaDram0_1WrRlsW<'_, Core1IntrRlsSpec> {
        Core1AreaDram0_1WrRlsW::new(self, 3)
    }
    #[doc = "Bit 4 - Core1 PIF area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_pif_0_rd_rls(&mut self) -> Core1AreaPif0RdRlsW<'_, Core1IntrRlsSpec> {
        Core1AreaPif0RdRlsW::new(self, 4)
    }
    #[doc = "Bit 5 - Core1 PIF area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_pif_0_wr_rls(&mut self) -> Core1AreaPif0WrRlsW<'_, Core1IntrRlsSpec> {
        Core1AreaPif0WrRlsW::new(self, 5)
    }
    #[doc = "Bit 6 - Core1 PIF area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_pif_1_rd_rls(&mut self) -> Core1AreaPif1RdRlsW<'_, Core1IntrRlsSpec> {
        Core1AreaPif1RdRlsW::new(self, 6)
    }
    #[doc = "Bit 7 - Core1 PIF area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_pif_1_wr_rls(&mut self) -> Core1AreaPif1WrRlsW<'_, Core1IntrRlsSpec> {
        Core1AreaPif1WrRlsW::new(self, 7)
    }
    #[doc = "Bit 8 - Core1 stackpoint underflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_sp_spill_min_rls(&mut self) -> Core1SpSpillMinRlsW<'_, Core1IntrRlsSpec> {
        Core1SpSpillMinRlsW::new(self, 8)
    }
    #[doc = "Bit 9 - Core1 stackpoint overflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_sp_spill_max_rls(&mut self) -> Core1SpSpillMaxRlsW<'_, Core1IntrRlsSpec> {
        Core1SpSpillMaxRlsW::new(self, 9)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_iram0_exception_monitor_rls(
        &mut self,
    ) -> Core1Iram0ExceptionMonitorRlsW<'_, Core1IntrRlsSpec> {
        Core1Iram0ExceptionMonitorRlsW::new(self, 10)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt enbale"]
    #[inline(always)]
    pub fn core_1_dram0_exception_monitor_rls(
        &mut self,
    ) -> Core1Dram0ExceptionMonitorRlsW<'_, Core1IntrRlsSpec> {
        Core1Dram0ExceptionMonitorRlsW::new(self, 11)
    }
}
#[doc = "core1 monitor interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_intr_rls::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_intr_rls::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1IntrRlsSpec;
impl crate::RegisterSpec for Core1IntrRlsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_intr_rls::R`](R) reader structure"]
impl crate::Readable for Core1IntrRlsSpec {}
#[doc = "`write(|w| ..)` method takes [`core_1_intr_rls::W`](W) writer structure"]
impl crate::Writable for Core1IntrRlsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_1_INTR_RLS to value 0"]
impl crate::Resettable for Core1IntrRlsSpec {}
