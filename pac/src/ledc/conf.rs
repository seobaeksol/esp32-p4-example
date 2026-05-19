#[doc = "Register `CONF` reader"]
pub type R = crate::R<ConfSpec>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<ConfSpec>;
#[doc = "Field `APB_CLK_SEL` reader - Configures the clock source for the four timers.\\\\0: APB_CLK\\\\1: RC_FAST_CLK\\\\2: XTAL_CLK\\\\3: Invalid. No clock"]
pub type ApbClkSelR = crate::FieldReader;
#[doc = "Field `APB_CLK_SEL` writer - Configures the clock source for the four timers.\\\\0: APB_CLK\\\\1: RC_FAST_CLK\\\\2: XTAL_CLK\\\\3: Invalid. No clock"]
pub type ApbClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH0` reader - Configures whether or not to open LEDC ch0 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch0 gamma ram\\\\1: Force open the clock gate for LEDC ch0 gamma ram"]
pub type GammaRamClkEnCh0R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH0` writer - Configures whether or not to open LEDC ch0 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch0 gamma ram\\\\1: Force open the clock gate for LEDC ch0 gamma ram"]
pub type GammaRamClkEnCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH1` reader - Configures whether or not to open LEDC ch1 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch1 gamma ram\\\\1: Force open the clock gate for LEDC ch1 gamma ram"]
pub type GammaRamClkEnCh1R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH1` writer - Configures whether or not to open LEDC ch1 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch1 gamma ram\\\\1: Force open the clock gate for LEDC ch1 gamma ram"]
pub type GammaRamClkEnCh1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH2` reader - Configures whether or not to open LEDC ch2 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch2 gamma ram\\\\1: Force open the clock gate for LEDC ch2 gamma ram"]
pub type GammaRamClkEnCh2R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH2` writer - Configures whether or not to open LEDC ch2 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch2 gamma ram\\\\1: Force open the clock gate for LEDC ch2 gamma ram"]
pub type GammaRamClkEnCh2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH3` reader - Configures whether or not to open LEDC ch3 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch3 gamma ram\\\\1: Force open the clock gate for LEDC ch3 gamma ram"]
pub type GammaRamClkEnCh3R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH3` writer - Configures whether or not to open LEDC ch3 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch3 gamma ram\\\\1: Force open the clock gate for LEDC ch3 gamma ram"]
pub type GammaRamClkEnCh3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH4` reader - Configures whether or not to open LEDC ch4 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch4 gamma ram\\\\1: Force open the clock gate for LEDC ch4 gamma ram"]
pub type GammaRamClkEnCh4R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH4` writer - Configures whether or not to open LEDC ch4 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch4 gamma ram\\\\1: Force open the clock gate for LEDC ch4 gamma ram"]
pub type GammaRamClkEnCh4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH5` reader - Configures whether or not to open LEDC ch5 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch5 gamma ram\\\\1: Force open the clock gate for LEDC ch5 gamma ram"]
pub type GammaRamClkEnCh5R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH5` writer - Configures whether or not to open LEDC ch5 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch5 gamma ram\\\\1: Force open the clock gate for LEDC ch5 gamma ram"]
pub type GammaRamClkEnCh5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH6` reader - Configures whether or not to open LEDC ch6 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch6 gamma ram\\\\1: Force open the clock gate for LEDC ch6 gamma ram"]
pub type GammaRamClkEnCh6R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH6` writer - Configures whether or not to open LEDC ch6 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch6 gamma ram\\\\1: Force open the clock gate for LEDC ch6 gamma ram"]
pub type GammaRamClkEnCh6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH7` reader - Configures whether or not to open LEDC ch7 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch7 gamma ram\\\\1: Force open the clock gate for LEDC ch7 gamma ram"]
pub type GammaRamClkEnCh7R = crate::BitReader;
#[doc = "Field `GAMMA_RAM_CLK_EN_CH7` writer - Configures whether or not to open LEDC ch7 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch7 gamma ram\\\\1: Force open the clock gate for LEDC ch7 gamma ram"]
pub type GammaRamClkEnCh7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - Configures whether or not to open register clock gate.\\\\0: Open the clock gate only when application writes registers\\\\1: Force open the clock gate for register"]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Configures whether or not to open register clock gate.\\\\0: Open the clock gate only when application writes registers\\\\1: Force open the clock gate for register"]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures the clock source for the four timers.\\\\0: APB_CLK\\\\1: RC_FAST_CLK\\\\2: XTAL_CLK\\\\3: Invalid. No clock"]
    #[inline(always)]
    pub fn apb_clk_sel(&self) -> ApbClkSelR {
        ApbClkSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Configures whether or not to open LEDC ch0 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch0 gamma ram\\\\1: Force open the clock gate for LEDC ch0 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch0(&self) -> GammaRamClkEnCh0R {
        GammaRamClkEnCh0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not to open LEDC ch1 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch1 gamma ram\\\\1: Force open the clock gate for LEDC ch1 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch1(&self) -> GammaRamClkEnCh1R {
        GammaRamClkEnCh1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to open LEDC ch2 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch2 gamma ram\\\\1: Force open the clock gate for LEDC ch2 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch2(&self) -> GammaRamClkEnCh2R {
        GammaRamClkEnCh2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to open LEDC ch3 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch3 gamma ram\\\\1: Force open the clock gate for LEDC ch3 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch3(&self) -> GammaRamClkEnCh3R {
        GammaRamClkEnCh3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether or not to open LEDC ch4 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch4 gamma ram\\\\1: Force open the clock gate for LEDC ch4 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch4(&self) -> GammaRamClkEnCh4R {
        GammaRamClkEnCh4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether or not to open LEDC ch5 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch5 gamma ram\\\\1: Force open the clock gate for LEDC ch5 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch5(&self) -> GammaRamClkEnCh5R {
        GammaRamClkEnCh5R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures whether or not to open LEDC ch6 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch6 gamma ram\\\\1: Force open the clock gate for LEDC ch6 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch6(&self) -> GammaRamClkEnCh6R {
        GammaRamClkEnCh6R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures whether or not to open LEDC ch7 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch7 gamma ram\\\\1: Force open the clock gate for LEDC ch7 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch7(&self) -> GammaRamClkEnCh7R {
        GammaRamClkEnCh7R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Configures whether or not to open register clock gate.\\\\0: Open the clock gate only when application writes registers\\\\1: Force open the clock gate for register"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures the clock source for the four timers.\\\\0: APB_CLK\\\\1: RC_FAST_CLK\\\\2: XTAL_CLK\\\\3: Invalid. No clock"]
    #[inline(always)]
    pub fn apb_clk_sel(&mut self) -> ApbClkSelW<'_, ConfSpec> {
        ApbClkSelW::new(self, 0)
    }
    #[doc = "Bit 2 - Configures whether or not to open LEDC ch0 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch0 gamma ram\\\\1: Force open the clock gate for LEDC ch0 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch0(&mut self) -> GammaRamClkEnCh0W<'_, ConfSpec> {
        GammaRamClkEnCh0W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to open LEDC ch1 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch1 gamma ram\\\\1: Force open the clock gate for LEDC ch1 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch1(&mut self) -> GammaRamClkEnCh1W<'_, ConfSpec> {
        GammaRamClkEnCh1W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to open LEDC ch2 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch2 gamma ram\\\\1: Force open the clock gate for LEDC ch2 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch2(&mut self) -> GammaRamClkEnCh2W<'_, ConfSpec> {
        GammaRamClkEnCh2W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to open LEDC ch3 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch3 gamma ram\\\\1: Force open the clock gate for LEDC ch3 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch3(&mut self) -> GammaRamClkEnCh3W<'_, ConfSpec> {
        GammaRamClkEnCh3W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to open LEDC ch4 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch4 gamma ram\\\\1: Force open the clock gate for LEDC ch4 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch4(&mut self) -> GammaRamClkEnCh4W<'_, ConfSpec> {
        GammaRamClkEnCh4W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to open LEDC ch5 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch5 gamma ram\\\\1: Force open the clock gate for LEDC ch5 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch5(&mut self) -> GammaRamClkEnCh5W<'_, ConfSpec> {
        GammaRamClkEnCh5W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to open LEDC ch6 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch6 gamma ram\\\\1: Force open the clock gate for LEDC ch6 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch6(&mut self) -> GammaRamClkEnCh6W<'_, ConfSpec> {
        GammaRamClkEnCh6W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to open LEDC ch7 gamma ram clock gate.\\\\0: Open the clock gate only when application writes or reads LEDC ch7 gamma ram\\\\1: Force open the clock gate for LEDC ch7 gamma ram"]
    #[inline(always)]
    pub fn gamma_ram_clk_en_ch7(&mut self) -> GammaRamClkEnCh7W<'_, ConfSpec> {
        GammaRamClkEnCh7W::new(self, 9)
    }
    #[doc = "Bit 31 - Configures whether or not to open register clock gate.\\\\0: Open the clock gate only when application writes registers\\\\1: Force open the clock gate for register"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, ConfSpec> {
        ClkEnW::new(self, 31)
    }
}
#[doc = "LEDC global configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfSpec;
impl crate::RegisterSpec for ConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for ConfSpec {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for ConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for ConfSpec {}
