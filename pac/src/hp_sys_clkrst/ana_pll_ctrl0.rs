#[doc = "Register `ANA_PLL_CTRL0` reader"]
pub type R = crate::R<AnaPllCtrl0Spec>;
#[doc = "Register `ANA_PLL_CTRL0` writer"]
pub type W = crate::W<AnaPllCtrl0Spec>;
#[doc = "Field `PLLA_CAL_END` reader - Reserved"]
pub type PllaCalEndR = crate::BitReader;
#[doc = "Field `PLLA_CAL_STOP` reader - Reserved"]
pub type PllaCalStopR = crate::BitReader;
#[doc = "Field `PLLA_CAL_STOP` writer - Reserved"]
pub type PllaCalStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_PLL_CAL_END` reader - Reserved"]
pub type CpuPllCalEndR = crate::BitReader;
#[doc = "Field `CPU_PLL_CAL_STOP` reader - Reserved"]
pub type CpuPllCalStopR = crate::BitReader;
#[doc = "Field `CPU_PLL_CAL_STOP` writer - Reserved"]
pub type CpuPllCalStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_PLL_CAL_END` reader - Reserved"]
pub type SdioPllCalEndR = crate::BitReader;
#[doc = "Field `SDIO_PLL_CAL_STOP` reader - Reserved"]
pub type SdioPllCalStopR = crate::BitReader;
#[doc = "Field `SDIO_PLL_CAL_STOP` writer - Reserved"]
pub type SdioPllCalStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_PLL_CAL_END` reader - Reserved"]
pub type SysPllCalEndR = crate::BitReader;
#[doc = "Field `SYS_PLL_CAL_STOP` reader - Reserved"]
pub type SysPllCalStopR = crate::BitReader;
#[doc = "Field `SYS_PLL_CAL_STOP` writer - Reserved"]
pub type SysPllCalStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSPI_CAL_END` reader - Reserved"]
pub type MspiCalEndR = crate::BitReader;
#[doc = "Field `MSPI_CAL_STOP` reader - Reserved"]
pub type MspiCalStopR = crate::BitReader;
#[doc = "Field `MSPI_CAL_STOP` writer - Reserved"]
pub type MspiCalStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn plla_cal_end(&self) -> PllaCalEndR {
        PllaCalEndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn plla_cal_stop(&self) -> PllaCalStopR {
        PllaCalStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn cpu_pll_cal_end(&self) -> CpuPllCalEndR {
        CpuPllCalEndR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn cpu_pll_cal_stop(&self) -> CpuPllCalStopR {
        CpuPllCalStopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn sdio_pll_cal_end(&self) -> SdioPllCalEndR {
        SdioPllCalEndR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn sdio_pll_cal_stop(&self) -> SdioPllCalStopR {
        SdioPllCalStopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn sys_pll_cal_end(&self) -> SysPllCalEndR {
        SysPllCalEndR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn sys_pll_cal_stop(&self) -> SysPllCalStopR {
        SysPllCalStopR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn mspi_cal_end(&self) -> MspiCalEndR {
        MspiCalEndR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn mspi_cal_stop(&self) -> MspiCalStopR {
        MspiCalStopR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn plla_cal_stop(&mut self) -> PllaCalStopW<'_, AnaPllCtrl0Spec> {
        PllaCalStopW::new(self, 1)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn cpu_pll_cal_stop(&mut self) -> CpuPllCalStopW<'_, AnaPllCtrl0Spec> {
        CpuPllCalStopW::new(self, 3)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn sdio_pll_cal_stop(&mut self) -> SdioPllCalStopW<'_, AnaPllCtrl0Spec> {
        SdioPllCalStopW::new(self, 5)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn sys_pll_cal_stop(&mut self) -> SysPllCalStopW<'_, AnaPllCtrl0Spec> {
        SysPllCalStopW::new(self, 7)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn mspi_cal_stop(&mut self) -> MspiCalStopW<'_, AnaPllCtrl0Spec> {
        MspiCalStopW::new(self, 9)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_pll_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_pll_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaPllCtrl0Spec;
impl crate::RegisterSpec for AnaPllCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_pll_ctrl0::R`](R) reader structure"]
impl crate::Readable for AnaPllCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ana_pll_ctrl0::W`](W) writer structure"]
impl crate::Writable for AnaPllCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANA_PLL_CTRL0 to value 0"]
impl crate::Resettable for AnaPllCtrl0Spec {}
