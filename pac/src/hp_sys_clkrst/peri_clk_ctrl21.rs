#[doc = "Register `PERI_CLK_CTRL21` reader"]
pub type R = crate::R<PeriClkCtrl21Spec>;
#[doc = "Register `PERI_CLK_CTRL21` writer"]
pub type W = crate::W<PeriClkCtrl21Spec>;
#[doc = "Field `TIMERGRP0_TGRT_CLK_SRC_SEL` reader - Reserved"]
pub type Timergrp0TgrtClkSrcSelR = crate::FieldReader;
#[doc = "Field `TIMERGRP0_TGRT_CLK_SRC_SEL` writer - Reserved"]
pub type Timergrp0TgrtClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIMERGRP0_TGRT_CLK_DIV_NUM` reader - Reserved"]
pub type Timergrp0TgrtClkDivNumR = crate::FieldReader<u16>;
#[doc = "Field `TIMERGRP0_TGRT_CLK_DIV_NUM` writer - Reserved"]
pub type Timergrp0TgrtClkDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TIMERGRP1_T0_SRC_SEL` reader - Reserved"]
pub type Timergrp1T0SrcSelR = crate::FieldReader;
#[doc = "Field `TIMERGRP1_T0_SRC_SEL` writer - Reserved"]
pub type Timergrp1T0SrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMERGRP1_T0_CLK_EN` reader - Reserved"]
pub type Timergrp1T0ClkEnR = crate::BitReader;
#[doc = "Field `TIMERGRP1_T0_CLK_EN` writer - Reserved"]
pub type Timergrp1T0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERGRP1_T1_SRC_SEL` reader - Reserved"]
pub type Timergrp1T1SrcSelR = crate::FieldReader;
#[doc = "Field `TIMERGRP1_T1_SRC_SEL` writer - Reserved"]
pub type Timergrp1T1SrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMERGRP1_T1_CLK_EN` reader - Reserved"]
pub type Timergrp1T1ClkEnR = crate::BitReader;
#[doc = "Field `TIMERGRP1_T1_CLK_EN` writer - Reserved"]
pub type Timergrp1T1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERGRP1_WDT_SRC_SEL` reader - Reserved"]
pub type Timergrp1WdtSrcSelR = crate::FieldReader;
#[doc = "Field `TIMERGRP1_WDT_SRC_SEL` writer - Reserved"]
pub type Timergrp1WdtSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMERGRP1_WDT_CLK_EN` reader - Reserved"]
pub type Timergrp1WdtClkEnR = crate::BitReader;
#[doc = "Field `TIMERGRP1_WDT_CLK_EN` writer - Reserved"]
pub type Timergrp1WdtClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_CLK_SRC_SEL` reader - Reserved"]
pub type SystimerClkSrcSelR = crate::BitReader;
#[doc = "Field `SYSTIMER_CLK_SRC_SEL` writer - Reserved"]
pub type SystimerClkSrcSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_CLK_EN` reader - Reserved"]
pub type SystimerClkEnR = crate::BitReader;
#[doc = "Field `SYSTIMER_CLK_EN` writer - Reserved"]
pub type SystimerClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Reserved"]
    #[inline(always)]
    pub fn timergrp0_tgrt_clk_src_sel(&self) -> Timergrp0TgrtClkSrcSelR {
        Timergrp0TgrtClkSrcSelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:19 - Reserved"]
    #[inline(always)]
    pub fn timergrp0_tgrt_clk_div_num(&self) -> Timergrp0TgrtClkDivNumR {
        Timergrp0TgrtClkDivNumR::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bits 20:21 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_t0_src_sel(&self) -> Timergrp1T0SrcSelR {
        Timergrp1T0SrcSelR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_t0_clk_en(&self) -> Timergrp1T0ClkEnR {
        Timergrp1T0ClkEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_t1_src_sel(&self) -> Timergrp1T1SrcSelR {
        Timergrp1T1SrcSelR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_t1_clk_en(&self) -> Timergrp1T1ClkEnR {
        Timergrp1T1ClkEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_wdt_src_sel(&self) -> Timergrp1WdtSrcSelR {
        Timergrp1WdtSrcSelR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_wdt_clk_en(&self) -> Timergrp1WdtClkEnR {
        Timergrp1WdtClkEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn systimer_clk_src_sel(&self) -> SystimerClkSrcSelR {
        SystimerClkSrcSelR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn systimer_clk_en(&self) -> SystimerClkEnR {
        SystimerClkEnR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Reserved"]
    #[inline(always)]
    pub fn timergrp0_tgrt_clk_src_sel(&mut self) -> Timergrp0TgrtClkSrcSelW<'_, PeriClkCtrl21Spec> {
        Timergrp0TgrtClkSrcSelW::new(self, 0)
    }
    #[doc = "Bits 4:19 - Reserved"]
    #[inline(always)]
    pub fn timergrp0_tgrt_clk_div_num(&mut self) -> Timergrp0TgrtClkDivNumW<'_, PeriClkCtrl21Spec> {
        Timergrp0TgrtClkDivNumW::new(self, 4)
    }
    #[doc = "Bits 20:21 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_t0_src_sel(&mut self) -> Timergrp1T0SrcSelW<'_, PeriClkCtrl21Spec> {
        Timergrp1T0SrcSelW::new(self, 20)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_t0_clk_en(&mut self) -> Timergrp1T0ClkEnW<'_, PeriClkCtrl21Spec> {
        Timergrp1T0ClkEnW::new(self, 22)
    }
    #[doc = "Bits 23:24 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_t1_src_sel(&mut self) -> Timergrp1T1SrcSelW<'_, PeriClkCtrl21Spec> {
        Timergrp1T1SrcSelW::new(self, 23)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_t1_clk_en(&mut self) -> Timergrp1T1ClkEnW<'_, PeriClkCtrl21Spec> {
        Timergrp1T1ClkEnW::new(self, 25)
    }
    #[doc = "Bits 26:27 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_wdt_src_sel(&mut self) -> Timergrp1WdtSrcSelW<'_, PeriClkCtrl21Spec> {
        Timergrp1WdtSrcSelW::new(self, 26)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_wdt_clk_en(&mut self) -> Timergrp1WdtClkEnW<'_, PeriClkCtrl21Spec> {
        Timergrp1WdtClkEnW::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn systimer_clk_src_sel(&mut self) -> SystimerClkSrcSelW<'_, PeriClkCtrl21Spec> {
        SystimerClkSrcSelW::new(self, 29)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn systimer_clk_en(&mut self) -> SystimerClkEnW<'_, PeriClkCtrl21Spec> {
        SystimerClkEnW::new(self, 30)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriClkCtrl21Spec;
impl crate::RegisterSpec for PeriClkCtrl21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl21::R`](R) reader structure"]
impl crate::Readable for PeriClkCtrl21Spec {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl21::W`](W) writer structure"]
impl crate::Writable for PeriClkCtrl21Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL21 to value 0x5240_0000"]
impl crate::Resettable for PeriClkCtrl21Spec {
    const RESET_VALUE: u32 = 0x5240_0000;
}
