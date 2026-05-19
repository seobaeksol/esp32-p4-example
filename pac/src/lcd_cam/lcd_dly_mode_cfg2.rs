#[doc = "Register `LCD_DLY_MODE_CFG2` reader"]
pub type R = crate::R<LcdDlyModeCfg2Spec>;
#[doc = "Register `LCD_DLY_MODE_CFG2` writer"]
pub type W = crate::W<LcdDlyModeCfg2Spec>;
#[doc = "The output data bit %s is delayed by module clock LCD_CLK"]
pub use super::lcd_dly_mode_cfg1::DelayMode;
#[doc = "Field `DOUT_MODE(0-15)` reader - The output data bit %s is delayed by module clock LCD_CLK"]
pub use super::lcd_dly_mode_cfg1::DoutModeR;
#[doc = "Field `DOUT_MODE(0-15)` writer - The output data bit %s is delayed by module clock LCD_CLK"]
pub use super::lcd_dly_mode_cfg1::DoutModeW;
impl R {
    #[doc = "The output data bit (0-15) is delayed by module clock LCD_CLK"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DOUT0_MODE` field.</div>"]
    #[inline(always)]
    pub fn dout_mode(&self, n: u8) -> DoutModeR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        DoutModeR::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The output data bit (0-15) is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout_mode_iter(&self) -> impl Iterator<Item = DoutModeR> + '_ {
        (0..16).map(move |n| DoutModeR::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - The output data bit 0 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout0_mode(&self) -> DoutModeR {
        DoutModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The output data bit 1 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout1_mode(&self) -> DoutModeR {
        DoutModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - The output data bit 2 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout2_mode(&self) -> DoutModeR {
        DoutModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - The output data bit 3 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout3_mode(&self) -> DoutModeR {
        DoutModeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - The output data bit 4 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout4_mode(&self) -> DoutModeR {
        DoutModeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - The output data bit 5 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout5_mode(&self) -> DoutModeR {
        DoutModeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - The output data bit 6 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout6_mode(&self) -> DoutModeR {
        DoutModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - The output data bit 7 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout7_mode(&self) -> DoutModeR {
        DoutModeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - The output data bit 8 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout8_mode(&self) -> DoutModeR {
        DoutModeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - The output data bit 9 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout9_mode(&self) -> DoutModeR {
        DoutModeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - The output data bit 10 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout10_mode(&self) -> DoutModeR {
        DoutModeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - The output data bit 11 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout11_mode(&self) -> DoutModeR {
        DoutModeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - The output data bit 12 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout12_mode(&self) -> DoutModeR {
        DoutModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - The output data bit 13 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout13_mode(&self) -> DoutModeR {
        DoutModeR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - The output data bit 14 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout14_mode(&self) -> DoutModeR {
        DoutModeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - The output data bit 15 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout15_mode(&self) -> DoutModeR {
        DoutModeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "The output data bit (0-15) is delayed by module clock LCD_CLK"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DOUT0_MODE` field.</div>"]
    #[inline(always)]
    pub fn dout_mode(&mut self, n: u8) -> DoutModeW<'_, LcdDlyModeCfg2Spec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        DoutModeW::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - The output data bit 0 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout0_mode(&mut self) -> DoutModeW<'_, LcdDlyModeCfg2Spec> {
        DoutModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - The output data bit 1 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout1_mode(&mut self) -> DoutModeW<'_, LcdDlyModeCfg2Spec> {
        DoutModeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - The output data bit 2 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout2_mode(&mut self) -> DoutModeW<'_, LcdDlyModeCfg2Spec> {
        DoutModeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - The output data bit 3 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout3_mode(&mut self) -> DoutModeW<'_, LcdDlyModeCfg2Spec> {
        DoutModeW::new(self, 6)
    }
    #[doc = "Bits 8:9 - The output data bit 4 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout4_mode(&mut self) -> DoutModeW<'_, LcdDlyModeCfg2Spec> {
        DoutModeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - The output data bit 5 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout5_mode(&mut self) -> DoutModeW<'_, LcdDlyModeCfg2Spec> {
        DoutModeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - The output data bit 6 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout6_mode(&mut self) -> DoutModeW<'_, LcdDlyModeCfg2Spec> {
        DoutModeW::new(self, 12)
    }
    #[doc = "Bits 14:15 - The output data bit 7 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout7_mode(&mut self) -> DoutModeW<'_, LcdDlyModeCfg2Spec> {
        DoutModeW::new(self, 14)
    }
    #[doc = "Bits 16:17 - The output data bit 8 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout8_mode(&mut self) -> DoutModeW<'_, LcdDlyModeCfg2Spec> {
        DoutModeW::new(self, 16)
    }
    #[doc = "Bits 18:19 - The output data bit 9 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout9_mode(&mut self) -> DoutModeW<'_, LcdDlyModeCfg2Spec> {
        DoutModeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - The output data bit 10 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout10_mode(&mut self) -> DoutModeW<'_, LcdDlyModeCfg2Spec> {
        DoutModeW::new(self, 20)
    }
    #[doc = "Bits 22:23 - The output data bit 11 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout11_mode(&mut self) -> DoutModeW<'_, LcdDlyModeCfg2Spec> {
        DoutModeW::new(self, 22)
    }
    #[doc = "Bits 24:25 - The output data bit 12 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout12_mode(&mut self) -> DoutModeW<'_, LcdDlyModeCfg2Spec> {
        DoutModeW::new(self, 24)
    }
    #[doc = "Bits 26:27 - The output data bit 13 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout13_mode(&mut self) -> DoutModeW<'_, LcdDlyModeCfg2Spec> {
        DoutModeW::new(self, 26)
    }
    #[doc = "Bits 28:29 - The output data bit 14 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout14_mode(&mut self) -> DoutModeW<'_, LcdDlyModeCfg2Spec> {
        DoutModeW::new(self, 28)
    }
    #[doc = "Bits 30:31 - The output data bit 15 is delayed by module clock LCD_CLK"]
    #[inline(always)]
    pub fn dout15_mode(&mut self) -> DoutModeW<'_, LcdDlyModeCfg2Spec> {
        DoutModeW::new(self, 30)
    }
}
#[doc = "LCD config register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_dly_mode_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_dly_mode_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdDlyModeCfg2Spec;
impl crate::RegisterSpec for LcdDlyModeCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_dly_mode_cfg2::R`](R) reader structure"]
impl crate::Readable for LcdDlyModeCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_dly_mode_cfg2::W`](W) writer structure"]
impl crate::Writable for LcdDlyModeCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_DLY_MODE_CFG2 to value 0"]
impl crate::Resettable for LcdDlyModeCfg2Spec {}
