#[doc = "Register `SCL_SP_CONF` reader"]
pub type R = crate::R<SclSpConfSpec>;
#[doc = "Register `SCL_SP_CONF` writer"]
pub type W = crate::W<SclSpConfSpec>;
#[doc = "Field `SCL_RST_SLV_EN` reader - Configures to send out SCL pulses when I2C master is IDLE. The number of pulses equals to I2C_SCL_RST_SLV_NUM\\[4:0\\]."]
pub type SclRstSlvEnR = crate::BitReader;
#[doc = "Field `SCL_RST_SLV_EN` writer - Configures to send out SCL pulses when I2C master is IDLE. The number of pulses equals to I2C_SCL_RST_SLV_NUM\\[4:0\\]."]
pub type SclRstSlvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_RST_SLV_NUM` reader - Configure the pulses of SCL generated in I2C master mode. \\\\ Valid when I2C_SCL_RST_SLV_EN is 1.\\\\ Measurement unit: i2c_sclk \\\\"]
pub type SclRstSlvNumR = crate::FieldReader;
#[doc = "Field `SCL_RST_SLV_NUM` writer - Configure the pulses of SCL generated in I2C master mode. \\\\ Valid when I2C_SCL_RST_SLV_EN is 1.\\\\ Measurement unit: i2c_sclk \\\\"]
pub type SclRstSlvNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SCL_PD_EN` reader - Configures to power down the I2C output SCL line. \\\\ 0: Not power down.\\\\ 1: Not work and power down.\\\\ Valid only when I2C_SCL_FORCE_OUT is 1."]
pub type SclPdEnR = crate::BitReader;
#[doc = "Field `SCL_PD_EN` writer - Configures to power down the I2C output SCL line. \\\\ 0: Not power down.\\\\ 1: Not work and power down.\\\\ Valid only when I2C_SCL_FORCE_OUT is 1."]
pub type SclPdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDA_PD_EN` reader - Configures to power down the I2C output SDA line. \\\\ 0: Not power down.\\\\ 1: Not work and power down.\\\\ Valid only when I2C_SDA_FORCE_OUT is 1."]
pub type SdaPdEnR = crate::BitReader;
#[doc = "Field `SDA_PD_EN` writer - Configures to power down the I2C output SDA line. \\\\ 0: Not power down.\\\\ 1: Not work and power down.\\\\ Valid only when I2C_SDA_FORCE_OUT is 1."]
pub type SdaPdEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures to send out SCL pulses when I2C master is IDLE. The number of pulses equals to I2C_SCL_RST_SLV_NUM\\[4:0\\]."]
    #[inline(always)]
    pub fn scl_rst_slv_en(&self) -> SclRstSlvEnR {
        SclRstSlvEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Configure the pulses of SCL generated in I2C master mode. \\\\ Valid when I2C_SCL_RST_SLV_EN is 1.\\\\ Measurement unit: i2c_sclk \\\\"]
    #[inline(always)]
    pub fn scl_rst_slv_num(&self) -> SclRstSlvNumR {
        SclRstSlvNumR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Configures to power down the I2C output SCL line. \\\\ 0: Not power down.\\\\ 1: Not work and power down.\\\\ Valid only when I2C_SCL_FORCE_OUT is 1."]
    #[inline(always)]
    pub fn scl_pd_en(&self) -> SclPdEnR {
        SclPdEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures to power down the I2C output SDA line. \\\\ 0: Not power down.\\\\ 1: Not work and power down.\\\\ Valid only when I2C_SDA_FORCE_OUT is 1."]
    #[inline(always)]
    pub fn sda_pd_en(&self) -> SdaPdEnR {
        SdaPdEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures to send out SCL pulses when I2C master is IDLE. The number of pulses equals to I2C_SCL_RST_SLV_NUM\\[4:0\\]."]
    #[inline(always)]
    pub fn scl_rst_slv_en(&mut self) -> SclRstSlvEnW<'_, SclSpConfSpec> {
        SclRstSlvEnW::new(self, 0)
    }
    #[doc = "Bits 1:5 - Configure the pulses of SCL generated in I2C master mode. \\\\ Valid when I2C_SCL_RST_SLV_EN is 1.\\\\ Measurement unit: i2c_sclk \\\\"]
    #[inline(always)]
    pub fn scl_rst_slv_num(&mut self) -> SclRstSlvNumW<'_, SclSpConfSpec> {
        SclRstSlvNumW::new(self, 1)
    }
    #[doc = "Bit 6 - Configures to power down the I2C output SCL line. \\\\ 0: Not power down.\\\\ 1: Not work and power down.\\\\ Valid only when I2C_SCL_FORCE_OUT is 1."]
    #[inline(always)]
    pub fn scl_pd_en(&mut self) -> SclPdEnW<'_, SclSpConfSpec> {
        SclPdEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Configures to power down the I2C output SDA line. \\\\ 0: Not power down.\\\\ 1: Not work and power down.\\\\ Valid only when I2C_SDA_FORCE_OUT is 1."]
    #[inline(always)]
    pub fn sda_pd_en(&mut self) -> SdaPdEnW<'_, SclSpConfSpec> {
        SdaPdEnW::new(self, 7)
    }
}
#[doc = "Power configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_sp_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_sp_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SclSpConfSpec;
impl crate::RegisterSpec for SclSpConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_sp_conf::R`](R) reader structure"]
impl crate::Readable for SclSpConfSpec {}
#[doc = "`write(|w| ..)` method takes [`scl_sp_conf::W`](W) writer structure"]
impl crate::Writable for SclSpConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCL_SP_CONF to value 0"]
impl crate::Resettable for SclSpConfSpec {}
