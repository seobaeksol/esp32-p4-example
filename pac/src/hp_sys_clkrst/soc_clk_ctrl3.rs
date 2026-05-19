#[doc = "Register `SOC_CLK_CTRL3` reader"]
pub type R = crate::R<SocClkCtrl3Spec>;
#[doc = "Register `SOC_CLK_CTRL3` writer"]
pub type W = crate::W<SocClkCtrl3Spec>;
#[doc = "Field `LEDC_APB_CLK_EN` reader - Reserved"]
pub type LedcApbClkEnR = crate::BitReader;
#[doc = "Field `LEDC_APB_CLK_EN` writer - Reserved"]
pub type LedcApbClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCAM_APB_CLK_EN` reader - Reserved"]
pub type LcdcamApbClkEnR = crate::BitReader;
#[doc = "Field `LCDCAM_APB_CLK_EN` writer - Reserved"]
pub type LcdcamApbClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_APB_CLK_EN` reader - Reserved"]
pub type EtmApbClkEnR = crate::BitReader;
#[doc = "Field `ETM_APB_CLK_EN` writer - Reserved"]
pub type EtmApbClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOMUX_APB_CLK_EN` reader - Reserved"]
pub type IomuxApbClkEnR = crate::BitReader;
#[doc = "Field `IOMUX_APB_CLK_EN` writer - Reserved"]
pub type IomuxApbClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2CACHE_L2MEM_CLK_EN` reader - Reserved"]
pub type L2cacheL2memClkEnR = crate::BitReader;
#[doc = "Field `L2CACHE_L2MEM_CLK_EN` writer - Reserved"]
pub type L2cacheL2memClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn ledc_apb_clk_en(&self) -> LedcApbClkEnR {
        LedcApbClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn lcdcam_apb_clk_en(&self) -> LcdcamApbClkEnR {
        LcdcamApbClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn etm_apb_clk_en(&self) -> EtmApbClkEnR {
        EtmApbClkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn iomux_apb_clk_en(&self) -> IomuxApbClkEnR {
        IomuxApbClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn l2cache_l2mem_clk_en(&self) -> L2cacheL2memClkEnR {
        L2cacheL2memClkEnR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn ledc_apb_clk_en(&mut self) -> LedcApbClkEnW<'_, SocClkCtrl3Spec> {
        LedcApbClkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn lcdcam_apb_clk_en(&mut self) -> LcdcamApbClkEnW<'_, SocClkCtrl3Spec> {
        LcdcamApbClkEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn etm_apb_clk_en(&mut self) -> EtmApbClkEnW<'_, SocClkCtrl3Spec> {
        EtmApbClkEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn iomux_apb_clk_en(&mut self) -> IomuxApbClkEnW<'_, SocClkCtrl3Spec> {
        IomuxApbClkEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn l2cache_l2mem_clk_en(&mut self) -> L2cacheL2memClkEnW<'_, SocClkCtrl3Spec> {
        L2cacheL2memClkEnW::new(self, 4)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`soc_clk_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soc_clk_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocClkCtrl3Spec;
impl crate::RegisterSpec for SocClkCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_clk_ctrl3::R`](R) reader structure"]
impl crate::Readable for SocClkCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`soc_clk_ctrl3::W`](W) writer structure"]
impl crate::Writable for SocClkCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOC_CLK_CTRL3 to value 0x18"]
impl crate::Resettable for SocClkCtrl3Spec {
    const RESET_VALUE: u32 = 0x18;
}
