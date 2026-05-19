#[doc = "Register `SYS_CTRL` reader"]
pub type R = crate::R<SysCtrlSpec>;
#[doc = "Register `SYS_CTRL` writer"]
pub type W = crate::W<SysCtrlSpec>;
#[doc = "Field `LP_CORE_DISABLE` reader - lp cpu disable"]
pub type LpCoreDisableR = crate::BitReader;
#[doc = "Field `LP_CORE_DISABLE` writer - lp cpu disable"]
pub type LpCoreDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_SW_RST` writer - digital system software reset bit"]
pub type SysSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_DOWNLOAD_BOOT` reader - need_des"]
pub type ForceDownloadBootR = crate::BitReader;
#[doc = "Field `FORCE_DOWNLOAD_BOOT` writer - need_des"]
pub type ForceDownloadBootW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIG_FIB` reader - need_des"]
pub type DigFibR = crate::FieldReader;
#[doc = "Field `DIG_FIB` writer - need_des"]
pub type DigFibW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IO_MUX_RESET_DISABLE` reader - reset disable bit for LP IOMUX"]
pub type IoMuxResetDisableR = crate::BitReader;
#[doc = "Field `IO_MUX_RESET_DISABLE` writer - reset disable bit for LP IOMUX"]
pub type IoMuxResetDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANA_FIB` reader - need_des"]
pub type AnaFibR = crate::FieldReader;
#[doc = "Field `LP_FIB_SEL` reader - need_des"]
pub type LpFibSelR = crate::FieldReader;
#[doc = "Field `LP_FIB_SEL` writer - need_des"]
pub type LpFibSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_CORE_ETM_WAKEUP_FLAG_CLR` writer - need_des"]
pub type LpCoreEtmWakeupFlagClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_CORE_ETM_WAKEUP_FLAG` reader - need_des"]
pub type LpCoreEtmWakeupFlagR = crate::BitReader;
#[doc = "Field `LP_CORE_ETM_WAKEUP_FLAG` writer - need_des"]
pub type LpCoreEtmWakeupFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_STALL_SEL` reader - 0: use systimer_stall signal from hp_core0, 1: use systimer_stall signal from hp_core1"]
pub type SystimerStallSelR = crate::BitReader;
#[doc = "Field `SYSTIMER_STALL_SEL` writer - 0: use systimer_stall signal from hp_core0, 1: use systimer_stall signal from hp_core1"]
pub type SystimerStallSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lp cpu disable"]
    #[inline(always)]
    pub fn lp_core_disable(&self) -> LpCoreDisableR {
        LpCoreDisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_download_boot(&self) -> ForceDownloadBootR {
        ForceDownloadBootR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:10 - need_des"]
    #[inline(always)]
    pub fn dig_fib(&self) -> DigFibR {
        DigFibR::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - reset disable bit for LP IOMUX"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&self) -> IoMuxResetDisableR {
        IoMuxResetDisableR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 14:20 - need_des"]
    #[inline(always)]
    pub fn ana_fib(&self) -> AnaFibR {
        AnaFibR::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bits 21:28 - need_des"]
    #[inline(always)]
    pub fn lp_fib_sel(&self) -> LpFibSelR {
        LpFibSelR::new(((self.bits >> 21) & 0xff) as u8)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_core_etm_wakeup_flag(&self) -> LpCoreEtmWakeupFlagR {
        LpCoreEtmWakeupFlagR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0: use systimer_stall signal from hp_core0, 1: use systimer_stall signal from hp_core1"]
    #[inline(always)]
    pub fn systimer_stall_sel(&self) -> SystimerStallSelR {
        SystimerStallSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lp cpu disable"]
    #[inline(always)]
    pub fn lp_core_disable(&mut self) -> LpCoreDisableW<'_, SysCtrlSpec> {
        LpCoreDisableW::new(self, 0)
    }
    #[doc = "Bit 1 - digital system software reset bit"]
    #[inline(always)]
    pub fn sys_sw_rst(&mut self) -> SysSwRstW<'_, SysCtrlSpec> {
        SysSwRstW::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_download_boot(&mut self) -> ForceDownloadBootW<'_, SysCtrlSpec> {
        ForceDownloadBootW::new(self, 2)
    }
    #[doc = "Bits 3:10 - need_des"]
    #[inline(always)]
    pub fn dig_fib(&mut self) -> DigFibW<'_, SysCtrlSpec> {
        DigFibW::new(self, 3)
    }
    #[doc = "Bit 11 - reset disable bit for LP IOMUX"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&mut self) -> IoMuxResetDisableW<'_, SysCtrlSpec> {
        IoMuxResetDisableW::new(self, 11)
    }
    #[doc = "Bits 21:28 - need_des"]
    #[inline(always)]
    pub fn lp_fib_sel(&mut self) -> LpFibSelW<'_, SysCtrlSpec> {
        LpFibSelW::new(self, 21)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_core_etm_wakeup_flag_clr(&mut self) -> LpCoreEtmWakeupFlagClrW<'_, SysCtrlSpec> {
        LpCoreEtmWakeupFlagClrW::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_core_etm_wakeup_flag(&mut self) -> LpCoreEtmWakeupFlagW<'_, SysCtrlSpec> {
        LpCoreEtmWakeupFlagW::new(self, 30)
    }
    #[doc = "Bit 31 - 0: use systimer_stall signal from hp_core0, 1: use systimer_stall signal from hp_core1"]
    #[inline(always)]
    pub fn systimer_stall_sel(&mut self) -> SystimerStallSelW<'_, SysCtrlSpec> {
        SystimerStallSelW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysCtrlSpec;
impl crate::RegisterSpec for SysCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_ctrl::R`](R) reader structure"]
impl crate::Readable for SysCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_ctrl::W`](W) writer structure"]
impl crate::Writable for SysCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_CTRL to value 0x1fff_c7f8"]
impl crate::Resettable for SysCtrlSpec {
    const RESET_VALUE: u32 = 0x1fff_c7f8;
}
