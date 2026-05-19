#[doc = "Register `CLK_EN` reader"]
pub type R = crate::R<ClkEnSpec>;
#[doc = "Register `CLK_EN` writer"]
pub type W = crate::W<ClkEnSpec>;
#[doc = "Field `CK_EN_RNG` reader - need_des"]
pub type CkEnRngR = crate::BitReader;
#[doc = "Field `CK_EN_RNG` writer - need_des"]
pub type CkEnRngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_TSENS` reader - need_des"]
pub type CkEnLpTsensR = crate::BitReader;
#[doc = "Field `CK_EN_LP_TSENS` writer - need_des"]
pub type CkEnLpTsensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_PMS` reader - need_des"]
pub type CkEnLpPmsR = crate::BitReader;
#[doc = "Field `CK_EN_LP_PMS` writer - need_des"]
pub type CkEnLpPmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_EFUSE` reader - need_des"]
pub type CkEnLpEfuseR = crate::BitReader;
#[doc = "Field `CK_EN_LP_EFUSE` writer - need_des"]
pub type CkEnLpEfuseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_IOMUX` reader - need_des"]
pub type CkEnLpIomuxR = crate::BitReader;
#[doc = "Field `CK_EN_LP_IOMUX` writer - need_des"]
pub type CkEnLpIomuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_TOUCH` reader - need_des"]
pub type CkEnLpTouchR = crate::BitReader;
#[doc = "Field `CK_EN_LP_TOUCH` writer - need_des"]
pub type CkEnLpTouchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_SPI` reader - need_des"]
pub type CkEnLpSpiR = crate::BitReader;
#[doc = "Field `CK_EN_LP_SPI` writer - need_des"]
pub type CkEnLpSpiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_ADC` reader - need_des"]
pub type CkEnLpAdcR = crate::BitReader;
#[doc = "Field `CK_EN_LP_ADC` writer - need_des"]
pub type CkEnLpAdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_I2S_TX` reader - need_des"]
pub type CkEnLpI2sTxR = crate::BitReader;
#[doc = "Field `CK_EN_LP_I2S_TX` writer - need_des"]
pub type CkEnLpI2sTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_I2S_RX` reader - need_des"]
pub type CkEnLpI2sRxR = crate::BitReader;
#[doc = "Field `CK_EN_LP_I2S_RX` writer - need_des"]
pub type CkEnLpI2sRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_I2S` reader - need_des"]
pub type CkEnLpI2sR = crate::BitReader;
#[doc = "Field `CK_EN_LP_I2S` writer - need_des"]
pub type CkEnLpI2sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_I2CMST` reader - need_des"]
pub type CkEnLpI2cmstR = crate::BitReader;
#[doc = "Field `CK_EN_LP_I2CMST` writer - need_des"]
pub type CkEnLpI2cmstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_I2C` reader - need_des"]
pub type CkEnLpI2cR = crate::BitReader;
#[doc = "Field `CK_EN_LP_I2C` writer - need_des"]
pub type CkEnLpI2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_UART` reader - need_des"]
pub type CkEnLpUartR = crate::BitReader;
#[doc = "Field `CK_EN_LP_UART` writer - need_des"]
pub type CkEnLpUartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_INTR` reader - need_des"]
pub type CkEnLpIntrR = crate::BitReader;
#[doc = "Field `CK_EN_LP_INTR` writer - need_des"]
pub type CkEnLpIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_CORE` reader - write 1 to force on lp_core clk"]
pub type CkEnLpCoreR = crate::BitReader;
#[doc = "Field `CK_EN_LP_CORE` writer - write 1 to force on lp_core clk"]
pub type CkEnLpCoreW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn ck_en_rng(&self) -> CkEnRngR {
        CkEnRngR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_tsens(&self) -> CkEnLpTsensR {
        CkEnLpTsensR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_pms(&self) -> CkEnLpPmsR {
        CkEnLpPmsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_efuse(&self) -> CkEnLpEfuseR {
        CkEnLpEfuseR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_iomux(&self) -> CkEnLpIomuxR {
        CkEnLpIomuxR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_touch(&self) -> CkEnLpTouchR {
        CkEnLpTouchR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_spi(&self) -> CkEnLpSpiR {
        CkEnLpSpiR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_adc(&self) -> CkEnLpAdcR {
        CkEnLpAdcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_i2s_tx(&self) -> CkEnLpI2sTxR {
        CkEnLpI2sTxR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_i2s_rx(&self) -> CkEnLpI2sRxR {
        CkEnLpI2sRxR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_i2s(&self) -> CkEnLpI2sR {
        CkEnLpI2sR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_i2cmst(&self) -> CkEnLpI2cmstR {
        CkEnLpI2cmstR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_i2c(&self) -> CkEnLpI2cR {
        CkEnLpI2cR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_uart(&self) -> CkEnLpUartR {
        CkEnLpUartR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_intr(&self) -> CkEnLpIntrR {
        CkEnLpIntrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - write 1 to force on lp_core clk"]
    #[inline(always)]
    pub fn ck_en_lp_core(&self) -> CkEnLpCoreR {
        CkEnLpCoreR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn ck_en_rng(&mut self) -> CkEnRngW<'_, ClkEnSpec> {
        CkEnRngW::new(self, 16)
    }
    #[doc = "Bit 17 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_tsens(&mut self) -> CkEnLpTsensW<'_, ClkEnSpec> {
        CkEnLpTsensW::new(self, 17)
    }
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_pms(&mut self) -> CkEnLpPmsW<'_, ClkEnSpec> {
        CkEnLpPmsW::new(self, 18)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_efuse(&mut self) -> CkEnLpEfuseW<'_, ClkEnSpec> {
        CkEnLpEfuseW::new(self, 19)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_iomux(&mut self) -> CkEnLpIomuxW<'_, ClkEnSpec> {
        CkEnLpIomuxW::new(self, 20)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_touch(&mut self) -> CkEnLpTouchW<'_, ClkEnSpec> {
        CkEnLpTouchW::new(self, 21)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_spi(&mut self) -> CkEnLpSpiW<'_, ClkEnSpec> {
        CkEnLpSpiW::new(self, 22)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_adc(&mut self) -> CkEnLpAdcW<'_, ClkEnSpec> {
        CkEnLpAdcW::new(self, 23)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_i2s_tx(&mut self) -> CkEnLpI2sTxW<'_, ClkEnSpec> {
        CkEnLpI2sTxW::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_i2s_rx(&mut self) -> CkEnLpI2sRxW<'_, ClkEnSpec> {
        CkEnLpI2sRxW::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_i2s(&mut self) -> CkEnLpI2sW<'_, ClkEnSpec> {
        CkEnLpI2sW::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_i2cmst(&mut self) -> CkEnLpI2cmstW<'_, ClkEnSpec> {
        CkEnLpI2cmstW::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_i2c(&mut self) -> CkEnLpI2cW<'_, ClkEnSpec> {
        CkEnLpI2cW::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_uart(&mut self) -> CkEnLpUartW<'_, ClkEnSpec> {
        CkEnLpUartW::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_intr(&mut self) -> CkEnLpIntrW<'_, ClkEnSpec> {
        CkEnLpIntrW::new(self, 30)
    }
    #[doc = "Bit 31 - write 1 to force on lp_core clk"]
    #[inline(always)]
    pub fn ck_en_lp_core(&mut self) -> CkEnLpCoreW<'_, ClkEnSpec> {
        CkEnLpCoreW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkEnSpec;
impl crate::RegisterSpec for ClkEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_en::R`](R) reader structure"]
impl crate::Readable for ClkEnSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_en::W`](W) writer structure"]
impl crate::Writable for ClkEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_EN to value 0x7fff_0000"]
impl crate::Resettable for ClkEnSpec {
    const RESET_VALUE: u32 = 0x7fff_0000;
}
