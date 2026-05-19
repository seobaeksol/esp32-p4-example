#[doc = "Register `REF_CLK_CTRL2` reader"]
pub type R = crate::R<RefClkCtrl2Spec>;
#[doc = "Register `REF_CLK_CTRL2` writer"]
pub type W = crate::W<RefClkCtrl2Spec>;
#[doc = "Field `REF_160M_CLK_EN` reader - Reserved"]
pub type Ref160mClkEnR = crate::BitReader;
#[doc = "Field `REF_160M_CLK_EN` writer - Reserved"]
pub type Ref160mClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_160M_CLK_EN` reader - Reserved"]
pub type Tm160mClkEnR = crate::BitReader;
#[doc = "Field `TM_160M_CLK_EN` writer - Reserved"]
pub type Tm160mClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_120M_CLK_EN` reader - Reserved"]
pub type Ref120mClkEnR = crate::BitReader;
#[doc = "Field `REF_120M_CLK_EN` writer - Reserved"]
pub type Ref120mClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_120M_CLK_EN` reader - Reserved"]
pub type Tm120mClkEnR = crate::BitReader;
#[doc = "Field `TM_120M_CLK_EN` writer - Reserved"]
pub type Tm120mClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_80M_CLK_EN` reader - Reserved"]
pub type Ref80mClkEnR = crate::BitReader;
#[doc = "Field `REF_80M_CLK_EN` writer - Reserved"]
pub type Ref80mClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_80M_CLK_EN` reader - Reserved"]
pub type Tm80mClkEnR = crate::BitReader;
#[doc = "Field `TM_80M_CLK_EN` writer - Reserved"]
pub type Tm80mClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_60M_CLK_EN` reader - Reserved"]
pub type Tm60mClkEnR = crate::BitReader;
#[doc = "Field `TM_60M_CLK_EN` writer - Reserved"]
pub type Tm60mClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_48M_CLK_EN` reader - Reserved"]
pub type Tm48mClkEnR = crate::BitReader;
#[doc = "Field `TM_48M_CLK_EN` writer - Reserved"]
pub type Tm48mClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_20M_CLK_EN` reader - Reserved"]
pub type Ref20mClkEnR = crate::BitReader;
#[doc = "Field `REF_20M_CLK_EN` writer - Reserved"]
pub type Ref20mClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_20M_CLK_EN` reader - Reserved"]
pub type Tm20mClkEnR = crate::BitReader;
#[doc = "Field `TM_20M_CLK_EN` writer - Reserved"]
pub type Tm20mClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn ref_160m_clk_en(&self) -> Ref160mClkEnR {
        Ref160mClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn tm_160m_clk_en(&self) -> Tm160mClkEnR {
        Tm160mClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn ref_120m_clk_en(&self) -> Ref120mClkEnR {
        Ref120mClkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn tm_120m_clk_en(&self) -> Tm120mClkEnR {
        Tm120mClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn ref_80m_clk_en(&self) -> Ref80mClkEnR {
        Ref80mClkEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn tm_80m_clk_en(&self) -> Tm80mClkEnR {
        Tm80mClkEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn tm_60m_clk_en(&self) -> Tm60mClkEnR {
        Tm60mClkEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn tm_48m_clk_en(&self) -> Tm48mClkEnR {
        Tm48mClkEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn ref_20m_clk_en(&self) -> Ref20mClkEnR {
        Ref20mClkEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn tm_20m_clk_en(&self) -> Tm20mClkEnR {
        Tm20mClkEnR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn ref_160m_clk_en(&mut self) -> Ref160mClkEnW<'_, RefClkCtrl2Spec> {
        Ref160mClkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn tm_160m_clk_en(&mut self) -> Tm160mClkEnW<'_, RefClkCtrl2Spec> {
        Tm160mClkEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn ref_120m_clk_en(&mut self) -> Ref120mClkEnW<'_, RefClkCtrl2Spec> {
        Ref120mClkEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn tm_120m_clk_en(&mut self) -> Tm120mClkEnW<'_, RefClkCtrl2Spec> {
        Tm120mClkEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn ref_80m_clk_en(&mut self) -> Ref80mClkEnW<'_, RefClkCtrl2Spec> {
        Ref80mClkEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn tm_80m_clk_en(&mut self) -> Tm80mClkEnW<'_, RefClkCtrl2Spec> {
        Tm80mClkEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn tm_60m_clk_en(&mut self) -> Tm60mClkEnW<'_, RefClkCtrl2Spec> {
        Tm60mClkEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn tm_48m_clk_en(&mut self) -> Tm48mClkEnW<'_, RefClkCtrl2Spec> {
        Tm48mClkEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn ref_20m_clk_en(&mut self) -> Ref20mClkEnW<'_, RefClkCtrl2Spec> {
        Ref20mClkEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn tm_20m_clk_en(&mut self) -> Tm20mClkEnW<'_, RefClkCtrl2Spec> {
        Tm20mClkEnW::new(self, 9)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_clk_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_clk_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefClkCtrl2Spec;
impl crate::RegisterSpec for RefClkCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_clk_ctrl2::R`](R) reader structure"]
impl crate::Readable for RefClkCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ref_clk_ctrl2::W`](W) writer structure"]
impl crate::Writable for RefClkCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REF_CLK_CTRL2 to value 0x0115"]
impl crate::Resettable for RefClkCtrl2Spec {
    const RESET_VALUE: u32 = 0x0115;
}
