#[doc = "Register `RF_PWC` reader"]
pub type R = crate::R<RfPwcSpec>;
#[doc = "Register `RF_PWC` writer"]
pub type W = crate::W<RfPwcSpec>;
#[doc = "Field `MSPI_PHY_XPD` reader - need_des"]
pub type MspiPhyXpdR = crate::BitReader;
#[doc = "Field `MSPI_PHY_XPD` writer - need_des"]
pub type MspiPhyXpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_PLL_XPD` reader - need_des"]
pub type SdioPllXpdR = crate::BitReader;
#[doc = "Field `SDIO_PLL_XPD` writer - need_des"]
pub type SdioPllXpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIF_I2C_RSTB` reader - need_des"]
pub type PerifI2cRstbR = crate::BitReader;
#[doc = "Field `PERIF_I2C_RSTB` writer - need_des"]
pub type PerifI2cRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_PERIF_I2C` reader - need_des"]
pub type XpdPerifI2cR = crate::BitReader;
#[doc = "Field `XPD_PERIF_I2C` writer - need_des"]
pub type XpdPerifI2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_TXRF_I2C` reader - need_des"]
pub type XpdTxrfI2cR = crate::BitReader;
#[doc = "Field `XPD_TXRF_I2C` writer - need_des"]
pub type XpdTxrfI2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_RFRX_PBUS` reader - need_des"]
pub type XpdRfrxPbusR = crate::BitReader;
#[doc = "Field `XPD_RFRX_PBUS` writer - need_des"]
pub type XpdRfrxPbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_CKGEN_I2C` reader - need_des"]
pub type XpdCkgenI2cR = crate::BitReader;
#[doc = "Field `XPD_CKGEN_I2C` writer - need_des"]
pub type XpdCkgenI2cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn mspi_phy_xpd(&self) -> MspiPhyXpdR {
        MspiPhyXpdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn sdio_pll_xpd(&self) -> SdioPllXpdR {
        SdioPllXpdR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn perif_i2c_rstb(&self) -> PerifI2cRstbR {
        PerifI2cRstbR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn xpd_perif_i2c(&self) -> XpdPerifI2cR {
        XpdPerifI2cR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn xpd_txrf_i2c(&self) -> XpdTxrfI2cR {
        XpdTxrfI2cR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn xpd_rfrx_pbus(&self) -> XpdRfrxPbusR {
        XpdRfrxPbusR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn xpd_ckgen_i2c(&self) -> XpdCkgenI2cR {
        XpdCkgenI2cR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn mspi_phy_xpd(&mut self) -> MspiPhyXpdW<'_, RfPwcSpec> {
        MspiPhyXpdW::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn sdio_pll_xpd(&mut self) -> SdioPllXpdW<'_, RfPwcSpec> {
        SdioPllXpdW::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn perif_i2c_rstb(&mut self) -> PerifI2cRstbW<'_, RfPwcSpec> {
        PerifI2cRstbW::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn xpd_perif_i2c(&mut self) -> XpdPerifI2cW<'_, RfPwcSpec> {
        XpdPerifI2cW::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn xpd_txrf_i2c(&mut self) -> XpdTxrfI2cW<'_, RfPwcSpec> {
        XpdTxrfI2cW::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn xpd_rfrx_pbus(&mut self) -> XpdRfrxPbusW<'_, RfPwcSpec> {
        XpdRfrxPbusW::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn xpd_ckgen_i2c(&mut self) -> XpdCkgenI2cW<'_, RfPwcSpec> {
        XpdCkgenI2cW::new(self, 30)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_pwc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_pwc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfPwcSpec;
impl crate::RegisterSpec for RfPwcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_pwc::R`](R) reader structure"]
impl crate::Readable for RfPwcSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_pwc::W`](W) writer structure"]
impl crate::Writable for RfPwcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF_PWC to value 0x0800_0000"]
impl crate::Resettable for RfPwcSpec {
    const RESET_VALUE: u32 = 0x0800_0000;
}
