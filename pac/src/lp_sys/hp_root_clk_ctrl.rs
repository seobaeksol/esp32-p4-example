#[doc = "Register `HP_ROOT_CLK_CTRL` reader"]
pub type R = crate::R<HpRootClkCtrlSpec>;
#[doc = "Register `HP_ROOT_CLK_CTRL` writer"]
pub type W = crate::W<HpRootClkCtrlSpec>;
#[doc = "Field `CPU_CLK_EN` reader - clock gate enable for hp cpu root 400M clk"]
pub type CpuClkEnR = crate::BitReader;
#[doc = "Field `CPU_CLK_EN` writer - clock gate enable for hp cpu root 400M clk"]
pub type CpuClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_CLK_EN` reader - clock gate enable for hp sys root 480M clk"]
pub type SysClkEnR = crate::BitReader;
#[doc = "Field `SYS_CLK_EN` writer - clock gate enable for hp sys root 480M clk"]
pub type SysClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - clock gate enable for hp cpu root 400M clk"]
    #[inline(always)]
    pub fn cpu_clk_en(&self) -> CpuClkEnR {
        CpuClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clock gate enable for hp sys root 480M clk"]
    #[inline(always)]
    pub fn sys_clk_en(&self) -> SysClkEnR {
        SysClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clock gate enable for hp cpu root 400M clk"]
    #[inline(always)]
    pub fn cpu_clk_en(&mut self) -> CpuClkEnW<'_, HpRootClkCtrlSpec> {
        CpuClkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - clock gate enable for hp sys root 480M clk"]
    #[inline(always)]
    pub fn sys_clk_en(&mut self) -> SysClkEnW<'_, HpRootClkCtrlSpec> {
        SysClkEnW::new(self, 1)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_root_clk_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_root_clk_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpRootClkCtrlSpec;
impl crate::RegisterSpec for HpRootClkCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_root_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for HpRootClkCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hp_root_clk_ctrl::W`](W) writer structure"]
impl crate::Writable for HpRootClkCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_ROOT_CLK_CTRL to value 0x03"]
impl crate::Resettable for HpRootClkCtrlSpec {
    const RESET_VALUE: u32 = 0x03;
}
