#[doc = "Register `CLK_FORCE_ON_CTRL0` reader"]
pub type R = crate::R<ClkForceOnCtrl0Spec>;
#[doc = "Register `CLK_FORCE_ON_CTRL0` writer"]
pub type W = crate::W<ClkForceOnCtrl0Spec>;
#[doc = "Field `CPUICM_GATED_CLK_FORCE_ON` reader - Reserved"]
pub type CpuicmGatedClkForceOnR = crate::BitReader;
#[doc = "Field `CPUICM_GATED_CLK_FORCE_ON` writer - Reserved"]
pub type CpuicmGatedClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCM_CPU_CLK_FORCE_ON` reader - Reserved"]
pub type TcmCpuClkForceOnR = crate::BitReader;
#[doc = "Field `TCM_CPU_CLK_FORCE_ON` writer - Reserved"]
pub type TcmCpuClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSMON_CPU_CLK_FORCE_ON` reader - Reserved"]
pub type BusmonCpuClkForceOnR = crate::BitReader;
#[doc = "Field `BUSMON_CPU_CLK_FORCE_ON` writer - Reserved"]
pub type BusmonCpuClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1CACHE_CPU_CLK_FORCE_ON` reader - Reserved"]
pub type L1cacheCpuClkForceOnR = crate::BitReader;
#[doc = "Field `L1CACHE_CPU_CLK_FORCE_ON` writer - Reserved"]
pub type L1cacheCpuClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1CACHE_D_CPU_CLK_FORCE_ON` reader - Reserved"]
pub type L1cacheDCpuClkForceOnR = crate::BitReader;
#[doc = "Field `L1CACHE_D_CPU_CLK_FORCE_ON` writer - Reserved"]
pub type L1cacheDCpuClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1CACHE_I0_CPU_CLK_FORCE_ON` reader - Reserved"]
pub type L1cacheI0CpuClkForceOnR = crate::BitReader;
#[doc = "Field `L1CACHE_I0_CPU_CLK_FORCE_ON` writer - Reserved"]
pub type L1cacheI0CpuClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1CACHE_I1_CPU_CLK_FORCE_ON` reader - Reserved"]
pub type L1cacheI1CpuClkForceOnR = crate::BitReader;
#[doc = "Field `L1CACHE_I1_CPU_CLK_FORCE_ON` writer - Reserved"]
pub type L1cacheI1CpuClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_CPU_CLK_FORCE_ON` reader - Reserved"]
pub type TraceCpuClkForceOnR = crate::BitReader;
#[doc = "Field `TRACE_CPU_CLK_FORCE_ON` writer - Reserved"]
pub type TraceCpuClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_SYS_CLK_FORCE_ON` reader - Reserved"]
pub type TraceSysClkForceOnR = crate::BitReader;
#[doc = "Field `TRACE_SYS_CLK_FORCE_ON` writer - Reserved"]
pub type TraceSysClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1CACHE_MEM_CLK_FORCE_ON` reader - Reserved"]
pub type L1cacheMemClkForceOnR = crate::BitReader;
#[doc = "Field `L1CACHE_MEM_CLK_FORCE_ON` writer - Reserved"]
pub type L1cacheMemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1CACHE_D_MEM_CLK_FORCE_ON` reader - Reserved"]
pub type L1cacheDMemClkForceOnR = crate::BitReader;
#[doc = "Field `L1CACHE_D_MEM_CLK_FORCE_ON` writer - Reserved"]
pub type L1cacheDMemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1CACHE_I0_MEM_CLK_FORCE_ON` reader - Reserved"]
pub type L1cacheI0MemClkForceOnR = crate::BitReader;
#[doc = "Field `L1CACHE_I0_MEM_CLK_FORCE_ON` writer - Reserved"]
pub type L1cacheI0MemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1CACHE_I1_MEM_CLK_FORCE_ON` reader - Reserved"]
pub type L1cacheI1MemClkForceOnR = crate::BitReader;
#[doc = "Field `L1CACHE_I1_MEM_CLK_FORCE_ON` writer - Reserved"]
pub type L1cacheI1MemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2CACHE_MEM_CLK_FORCE_ON` reader - Reserved"]
pub type L2cacheMemClkForceOnR = crate::BitReader;
#[doc = "Field `L2CACHE_MEM_CLK_FORCE_ON` writer - Reserved"]
pub type L2cacheMemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2MEM_MEM_CLK_FORCE_ON` reader - Reserved"]
pub type L2memMemClkForceOnR = crate::BitReader;
#[doc = "Field `L2MEM_MEM_CLK_FORCE_ON` writer - Reserved"]
pub type L2memMemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR1_CLK_FORCE_ON` reader - Reserved"]
pub type Sar1ClkForceOnR = crate::BitReader;
#[doc = "Field `SAR1_CLK_FORCE_ON` writer - Reserved"]
pub type Sar1ClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_CLK_FORCE_ON` reader - Reserved"]
pub type Sar2ClkForceOnR = crate::BitReader;
#[doc = "Field `SAR2_CLK_FORCE_ON` writer - Reserved"]
pub type Sar2ClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GMAC_TX_CLK_FORCE_ON` reader - Reserved"]
pub type GmacTxClkForceOnR = crate::BitReader;
#[doc = "Field `GMAC_TX_CLK_FORCE_ON` writer - Reserved"]
pub type GmacTxClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2CACHE_L2MEM_CLK_FORCE_ON` reader - Reserved"]
pub type L2cacheL2memClkForceOnR = crate::BitReader;
#[doc = "Field `L2CACHE_L2MEM_CLK_FORCE_ON` writer - Reserved"]
pub type L2cacheL2memClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn cpuicm_gated_clk_force_on(&self) -> CpuicmGatedClkForceOnR {
        CpuicmGatedClkForceOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn tcm_cpu_clk_force_on(&self) -> TcmCpuClkForceOnR {
        TcmCpuClkForceOnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn busmon_cpu_clk_force_on(&self) -> BusmonCpuClkForceOnR {
        BusmonCpuClkForceOnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1cache_cpu_clk_force_on(&self) -> L1cacheCpuClkForceOnR {
        L1cacheCpuClkForceOnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn l1cache_d_cpu_clk_force_on(&self) -> L1cacheDCpuClkForceOnR {
        L1cacheDCpuClkForceOnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn l1cache_i0_cpu_clk_force_on(&self) -> L1cacheI0CpuClkForceOnR {
        L1cacheI0CpuClkForceOnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn l1cache_i1_cpu_clk_force_on(&self) -> L1cacheI1CpuClkForceOnR {
        L1cacheI1CpuClkForceOnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn trace_cpu_clk_force_on(&self) -> TraceCpuClkForceOnR {
        TraceCpuClkForceOnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn trace_sys_clk_force_on(&self) -> TraceSysClkForceOnR {
        TraceSysClkForceOnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn l1cache_mem_clk_force_on(&self) -> L1cacheMemClkForceOnR {
        L1cacheMemClkForceOnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn l1cache_d_mem_clk_force_on(&self) -> L1cacheDMemClkForceOnR {
        L1cacheDMemClkForceOnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn l1cache_i0_mem_clk_force_on(&self) -> L1cacheI0MemClkForceOnR {
        L1cacheI0MemClkForceOnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn l1cache_i1_mem_clk_force_on(&self) -> L1cacheI1MemClkForceOnR {
        L1cacheI1MemClkForceOnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn l2cache_mem_clk_force_on(&self) -> L2cacheMemClkForceOnR {
        L2cacheMemClkForceOnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn l2mem_mem_clk_force_on(&self) -> L2memMemClkForceOnR {
        L2memMemClkForceOnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn sar1_clk_force_on(&self) -> Sar1ClkForceOnR {
        Sar1ClkForceOnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn sar2_clk_force_on(&self) -> Sar2ClkForceOnR {
        Sar2ClkForceOnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn gmac_tx_clk_force_on(&self) -> GmacTxClkForceOnR {
        GmacTxClkForceOnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn l2cache_l2mem_clk_force_on(&self) -> L2cacheL2memClkForceOnR {
        L2cacheL2memClkForceOnR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn cpuicm_gated_clk_force_on(&mut self) -> CpuicmGatedClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        CpuicmGatedClkForceOnW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn tcm_cpu_clk_force_on(&mut self) -> TcmCpuClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        TcmCpuClkForceOnW::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn busmon_cpu_clk_force_on(&mut self) -> BusmonCpuClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        BusmonCpuClkForceOnW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1cache_cpu_clk_force_on(&mut self) -> L1cacheCpuClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        L1cacheCpuClkForceOnW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn l1cache_d_cpu_clk_force_on(
        &mut self,
    ) -> L1cacheDCpuClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        L1cacheDCpuClkForceOnW::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn l1cache_i0_cpu_clk_force_on(
        &mut self,
    ) -> L1cacheI0CpuClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        L1cacheI0CpuClkForceOnW::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn l1cache_i1_cpu_clk_force_on(
        &mut self,
    ) -> L1cacheI1CpuClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        L1cacheI1CpuClkForceOnW::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn trace_cpu_clk_force_on(&mut self) -> TraceCpuClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        TraceCpuClkForceOnW::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn trace_sys_clk_force_on(&mut self) -> TraceSysClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        TraceSysClkForceOnW::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn l1cache_mem_clk_force_on(&mut self) -> L1cacheMemClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        L1cacheMemClkForceOnW::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn l1cache_d_mem_clk_force_on(
        &mut self,
    ) -> L1cacheDMemClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        L1cacheDMemClkForceOnW::new(self, 10)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn l1cache_i0_mem_clk_force_on(
        &mut self,
    ) -> L1cacheI0MemClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        L1cacheI0MemClkForceOnW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn l1cache_i1_mem_clk_force_on(
        &mut self,
    ) -> L1cacheI1MemClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        L1cacheI1MemClkForceOnW::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn l2cache_mem_clk_force_on(&mut self) -> L2cacheMemClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        L2cacheMemClkForceOnW::new(self, 13)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn l2mem_mem_clk_force_on(&mut self) -> L2memMemClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        L2memMemClkForceOnW::new(self, 14)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn sar1_clk_force_on(&mut self) -> Sar1ClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        Sar1ClkForceOnW::new(self, 15)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn sar2_clk_force_on(&mut self) -> Sar2ClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        Sar2ClkForceOnW::new(self, 16)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn gmac_tx_clk_force_on(&mut self) -> GmacTxClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        GmacTxClkForceOnW::new(self, 17)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn l2cache_l2mem_clk_force_on(
        &mut self,
    ) -> L2cacheL2memClkForceOnW<'_, ClkForceOnCtrl0Spec> {
        L2cacheL2memClkForceOnW::new(self, 18)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_force_on_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_force_on_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkForceOnCtrl0Spec;
impl crate::RegisterSpec for ClkForceOnCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_force_on_ctrl0::R`](R) reader structure"]
impl crate::Readable for ClkForceOnCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_force_on_ctrl0::W`](W) writer structure"]
impl crate::Writable for ClkForceOnCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_FORCE_ON_CTRL0 to value 0x0007_ffff"]
impl crate::Resettable for ClkForceOnCtrl0Spec {
    const RESET_VALUE: u32 = 0x0007_ffff;
}
