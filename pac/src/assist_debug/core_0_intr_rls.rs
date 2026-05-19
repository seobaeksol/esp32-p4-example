#[doc = "Register `CORE_0_INTR_RLS` reader"]
pub type R = crate::R<Core0IntrRlsSpec>;
#[doc = "Register `CORE_0_INTR_RLS` writer"]
pub type W = crate::W<Core0IntrRlsSpec>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_RLS` reader - Core0 dram0 area0 read monitor interrupt enable"]
pub type Core0AreaDram0_0RdRlsR = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_RLS` writer - Core0 dram0 area0 read monitor interrupt enable"]
pub type Core0AreaDram0_0RdRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_RLS` reader - Core0 dram0 area0 write monitor interrupt enable"]
pub type Core0AreaDram0_0WrRlsR = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_RLS` writer - Core0 dram0 area0 write monitor interrupt enable"]
pub type Core0AreaDram0_0WrRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_RLS` reader - Core0 dram0 area1 read monitor interrupt enable"]
pub type Core0AreaDram0_1RdRlsR = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_RLS` writer - Core0 dram0 area1 read monitor interrupt enable"]
pub type Core0AreaDram0_1RdRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_RLS` reader - Core0 dram0 area1 write monitor interrupt enable"]
pub type Core0AreaDram0_1WrRlsR = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_RLS` writer - Core0 dram0 area1 write monitor interrupt enable"]
pub type Core0AreaDram0_1WrRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_PIF_0_RD_RLS` reader - Core0 PIF area0 read monitor interrupt enable"]
pub type Core0AreaPif0RdRlsR = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_0_RD_RLS` writer - Core0 PIF area0 read monitor interrupt enable"]
pub type Core0AreaPif0RdRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_PIF_0_WR_RLS` reader - Core0 PIF area0 write monitor interrupt enable"]
pub type Core0AreaPif0WrRlsR = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_0_WR_RLS` writer - Core0 PIF area0 write monitor interrupt enable"]
pub type Core0AreaPif0WrRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_PIF_1_RD_RLS` reader - Core0 PIF area1 read monitor interrupt enable"]
pub type Core0AreaPif1RdRlsR = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_1_RD_RLS` writer - Core0 PIF area1 read monitor interrupt enable"]
pub type Core0AreaPif1RdRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_AREA_PIF_1_WR_RLS` reader - Core0 PIF area1 write monitor interrupt enable"]
pub type Core0AreaPif1WrRlsR = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_1_WR_RLS` writer - Core0 PIF area1 write monitor interrupt enable"]
pub type Core0AreaPif1WrRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_SP_SPILL_MIN_RLS` reader - Core0 stackpoint underflow monitor interrupt enable"]
pub type Core0SpSpillMinRlsR = crate::BitReader;
#[doc = "Field `CORE_0_SP_SPILL_MIN_RLS` writer - Core0 stackpoint underflow monitor interrupt enable"]
pub type Core0SpSpillMinRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_SP_SPILL_MAX_RLS` reader - Core0 stackpoint overflow monitor interrupt enable"]
pub type Core0SpSpillMaxRlsR = crate::BitReader;
#[doc = "Field `CORE_0_SP_SPILL_MAX_RLS` writer - Core0 stackpoint overflow monitor interrupt enable"]
pub type Core0SpSpillMaxRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_RLS` reader - IBUS busy monitor interrupt enable"]
pub type Core0Iram0ExceptionMonitorRlsR = crate::BitReader;
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_RLS` writer - IBUS busy monitor interrupt enable"]
pub type Core0Iram0ExceptionMonitorRlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_RLS` reader - DBUS busy monitor interrupt enbale"]
pub type Core0Dram0ExceptionMonitorRlsR = crate::BitReader;
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_RLS` writer - DBUS busy monitor interrupt enbale"]
pub type Core0Dram0ExceptionMonitorRlsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Core0 dram0 area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_rls(&self) -> Core0AreaDram0_0RdRlsR {
        Core0AreaDram0_0RdRlsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core0 dram0 area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_rls(&self) -> Core0AreaDram0_0WrRlsR {
        Core0AreaDram0_0WrRlsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Core0 dram0 area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_rls(&self) -> Core0AreaDram0_1RdRlsR {
        Core0AreaDram0_1RdRlsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Core0 dram0 area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_rls(&self) -> Core0AreaDram0_1WrRlsR {
        Core0AreaDram0_1WrRlsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Core0 PIF area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_rls(&self) -> Core0AreaPif0RdRlsR {
        Core0AreaPif0RdRlsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Core0 PIF area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_rls(&self) -> Core0AreaPif0WrRlsR {
        Core0AreaPif0WrRlsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Core0 PIF area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_rls(&self) -> Core0AreaPif1RdRlsR {
        Core0AreaPif1RdRlsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Core0 PIF area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_rls(&self) -> Core0AreaPif1WrRlsR {
        Core0AreaPif1WrRlsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Core0 stackpoint underflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_rls(&self) -> Core0SpSpillMinRlsR {
        Core0SpSpillMinRlsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Core0 stackpoint overflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_rls(&self) -> Core0SpSpillMaxRlsR {
        Core0SpSpillMaxRlsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_rls(&self) -> Core0Iram0ExceptionMonitorRlsR {
        Core0Iram0ExceptionMonitorRlsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt enbale"]
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_rls(&self) -> Core0Dram0ExceptionMonitorRlsR {
        Core0Dram0ExceptionMonitorRlsR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core0 dram0 area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_rls(&mut self) -> Core0AreaDram0_0RdRlsW<'_, Core0IntrRlsSpec> {
        Core0AreaDram0_0RdRlsW::new(self, 0)
    }
    #[doc = "Bit 1 - Core0 dram0 area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_rls(&mut self) -> Core0AreaDram0_0WrRlsW<'_, Core0IntrRlsSpec> {
        Core0AreaDram0_0WrRlsW::new(self, 1)
    }
    #[doc = "Bit 2 - Core0 dram0 area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_rls(&mut self) -> Core0AreaDram0_1RdRlsW<'_, Core0IntrRlsSpec> {
        Core0AreaDram0_1RdRlsW::new(self, 2)
    }
    #[doc = "Bit 3 - Core0 dram0 area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_rls(&mut self) -> Core0AreaDram0_1WrRlsW<'_, Core0IntrRlsSpec> {
        Core0AreaDram0_1WrRlsW::new(self, 3)
    }
    #[doc = "Bit 4 - Core0 PIF area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_rls(&mut self) -> Core0AreaPif0RdRlsW<'_, Core0IntrRlsSpec> {
        Core0AreaPif0RdRlsW::new(self, 4)
    }
    #[doc = "Bit 5 - Core0 PIF area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_rls(&mut self) -> Core0AreaPif0WrRlsW<'_, Core0IntrRlsSpec> {
        Core0AreaPif0WrRlsW::new(self, 5)
    }
    #[doc = "Bit 6 - Core0 PIF area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_rls(&mut self) -> Core0AreaPif1RdRlsW<'_, Core0IntrRlsSpec> {
        Core0AreaPif1RdRlsW::new(self, 6)
    }
    #[doc = "Bit 7 - Core0 PIF area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_rls(&mut self) -> Core0AreaPif1WrRlsW<'_, Core0IntrRlsSpec> {
        Core0AreaPif1WrRlsW::new(self, 7)
    }
    #[doc = "Bit 8 - Core0 stackpoint underflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_rls(&mut self) -> Core0SpSpillMinRlsW<'_, Core0IntrRlsSpec> {
        Core0SpSpillMinRlsW::new(self, 8)
    }
    #[doc = "Bit 9 - Core0 stackpoint overflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_rls(&mut self) -> Core0SpSpillMaxRlsW<'_, Core0IntrRlsSpec> {
        Core0SpSpillMaxRlsW::new(self, 9)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt enable"]
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_rls(
        &mut self,
    ) -> Core0Iram0ExceptionMonitorRlsW<'_, Core0IntrRlsSpec> {
        Core0Iram0ExceptionMonitorRlsW::new(self, 10)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt enbale"]
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_rls(
        &mut self,
    ) -> Core0Dram0ExceptionMonitorRlsW<'_, Core0IntrRlsSpec> {
        Core0Dram0ExceptionMonitorRlsW::new(self, 11)
    }
}
#[doc = "core0 monitor interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_rls::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_intr_rls::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core0IntrRlsSpec;
impl crate::RegisterSpec for Core0IntrRlsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_intr_rls::R`](R) reader structure"]
impl crate::Readable for Core0IntrRlsSpec {}
#[doc = "`write(|w| ..)` method takes [`core_0_intr_rls::W`](W) writer structure"]
impl crate::Writable for Core0IntrRlsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_0_INTR_RLS to value 0"]
impl crate::Resettable for Core0IntrRlsSpec {}
