#[doc = "Register `ROOT_CLK_CTRL2` reader"]
pub type R = crate::R<RootClkCtrl2Spec>;
#[doc = "Register `ROOT_CLK_CTRL2` writer"]
pub type W = crate::W<RootClkCtrl2Spec>;
#[doc = "Field `SYS_CLK_DIV_NUMERATOR` reader - Reserved"]
pub type SysClkDivNumeratorR = crate::FieldReader;
#[doc = "Field `SYS_CLK_DIV_NUMERATOR` writer - Reserved"]
pub type SysClkDivNumeratorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SYS_CLK_DIV_DENOMINATOR` reader - Reserved"]
pub type SysClkDivDenominatorR = crate::FieldReader;
#[doc = "Field `SYS_CLK_DIV_DENOMINATOR` writer - Reserved"]
pub type SysClkDivDenominatorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `APB_CLK_DIV_NUM` reader - Reserved"]
pub type ApbClkDivNumR = crate::FieldReader;
#[doc = "Field `APB_CLK_DIV_NUM` writer - Reserved"]
pub type ApbClkDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `APB_CLK_DIV_NUMERATOR` reader - Reserved"]
pub type ApbClkDivNumeratorR = crate::FieldReader;
#[doc = "Field `APB_CLK_DIV_NUMERATOR` writer - Reserved"]
pub type ApbClkDivNumeratorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn sys_clk_div_numerator(&self) -> SysClkDivNumeratorR {
        SysClkDivNumeratorR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn sys_clk_div_denominator(&self) -> SysClkDivDenominatorR {
        SysClkDivDenominatorR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn apb_clk_div_num(&self) -> ApbClkDivNumR {
        ApbClkDivNumR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Reserved"]
    #[inline(always)]
    pub fn apb_clk_div_numerator(&self) -> ApbClkDivNumeratorR {
        ApbClkDivNumeratorR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn sys_clk_div_numerator(&mut self) -> SysClkDivNumeratorW<'_, RootClkCtrl2Spec> {
        SysClkDivNumeratorW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn sys_clk_div_denominator(&mut self) -> SysClkDivDenominatorW<'_, RootClkCtrl2Spec> {
        SysClkDivDenominatorW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn apb_clk_div_num(&mut self) -> ApbClkDivNumW<'_, RootClkCtrl2Spec> {
        ApbClkDivNumW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Reserved"]
    #[inline(always)]
    pub fn apb_clk_div_numerator(&mut self) -> ApbClkDivNumeratorW<'_, RootClkCtrl2Spec> {
        ApbClkDivNumeratorW::new(self, 24)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`root_clk_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`root_clk_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RootClkCtrl2Spec;
impl crate::RegisterSpec for RootClkCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`root_clk_ctrl2::R`](R) reader structure"]
impl crate::Readable for RootClkCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`root_clk_ctrl2::W`](W) writer structure"]
impl crate::Writable for RootClkCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROOT_CLK_CTRL2 to value 0x0001_0000"]
impl crate::Resettable for RootClkCtrl2Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
