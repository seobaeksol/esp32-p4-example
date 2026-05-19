#[doc = "Register `LP_RST_CTRL` reader"]
pub type R = crate::R<LpRstCtrlSpec>;
#[doc = "Register `LP_RST_CTRL` writer"]
pub type W = crate::W<LpRstCtrlSpec>;
#[doc = "Field `ANA_RST_BYPASS` reader - analog source reset bypass : wdt,brown out,super wdt,glitch"]
pub type AnaRstBypassR = crate::BitReader;
#[doc = "Field `ANA_RST_BYPASS` writer - analog source reset bypass : wdt,brown out,super wdt,glitch"]
pub type AnaRstBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_RST_BYPASS` reader - system source reset bypass : software reset,hp wdt,lp wdt,efuse"]
pub type SysRstBypassR = crate::BitReader;
#[doc = "Field `SYS_RST_BYPASS` writer - system source reset bypass : software reset,hp wdt,lp wdt,efuse"]
pub type SysRstBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_FORCE_NORST` reader - efuse force no reset control"]
pub type EfuseForceNorstR = crate::BitReader;
#[doc = "Field `EFUSE_FORCE_NORST` writer - efuse force no reset control"]
pub type EfuseForceNorstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - analog source reset bypass : wdt,brown out,super wdt,glitch"]
    #[inline(always)]
    pub fn ana_rst_bypass(&self) -> AnaRstBypassR {
        AnaRstBypassR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - system source reset bypass : software reset,hp wdt,lp wdt,efuse"]
    #[inline(always)]
    pub fn sys_rst_bypass(&self) -> SysRstBypassR {
        SysRstBypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - efuse force no reset control"]
    #[inline(always)]
    pub fn efuse_force_norst(&self) -> EfuseForceNorstR {
        EfuseForceNorstR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - analog source reset bypass : wdt,brown out,super wdt,glitch"]
    #[inline(always)]
    pub fn ana_rst_bypass(&mut self) -> AnaRstBypassW<'_, LpRstCtrlSpec> {
        AnaRstBypassW::new(self, 0)
    }
    #[doc = "Bit 1 - system source reset bypass : software reset,hp wdt,lp wdt,efuse"]
    #[inline(always)]
    pub fn sys_rst_bypass(&mut self) -> SysRstBypassW<'_, LpRstCtrlSpec> {
        SysRstBypassW::new(self, 1)
    }
    #[doc = "Bit 2 - efuse force no reset control"]
    #[inline(always)]
    pub fn efuse_force_norst(&mut self) -> EfuseForceNorstW<'_, LpRstCtrlSpec> {
        EfuseForceNorstW::new(self, 2)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpRstCtrlSpec;
impl crate::RegisterSpec for LpRstCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for LpRstCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for LpRstCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_RST_CTRL to value 0x03"]
impl crate::Resettable for LpRstCtrlSpec {
    const RESET_VALUE: u32 = 0x03;
}
