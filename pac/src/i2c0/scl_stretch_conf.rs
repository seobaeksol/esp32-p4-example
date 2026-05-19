#[doc = "Register `SCL_STRETCH_CONF` reader"]
pub type R = crate::R<SclStretchConfSpec>;
#[doc = "Register `SCL_STRETCH_CONF` writer"]
pub type W = crate::W<SclStretchConfSpec>;
#[doc = "Field `STRETCH_PROTECT_NUM` reader - Configures the time period to release the SCL line from stretching to avoid timing violation. Usually it should be larger than the SDA setup time.\\\\ Measurement unit: i2c_sclk \\\\"]
pub type StretchProtectNumR = crate::FieldReader<u16>;
#[doc = "Field `STRETCH_PROTECT_NUM` writer - Configures the time period to release the SCL line from stretching to avoid timing violation. Usually it should be larger than the SDA setup time.\\\\ Measurement unit: i2c_sclk \\\\"]
pub type StretchProtectNumW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SLAVE_SCL_STRETCH_EN` reader - Configures to enable slave SCL stretch function. The SCL output line will be stretched low when I2C_SLAVE_SCL_STRETCH_EN is 1 and stretch event happens. The stretch cause can be seen in I2C_STRETCH_CAUSE.\\\\ 0: Disable \\\\ 1: Enable \\\\"]
pub type SlaveSclStretchEnR = crate::BitReader;
#[doc = "Field `SLAVE_SCL_STRETCH_EN` writer - Configures to enable slave SCL stretch function. The SCL output line will be stretched low when I2C_SLAVE_SCL_STRETCH_EN is 1 and stretch event happens. The stretch cause can be seen in I2C_STRETCH_CAUSE.\\\\ 0: Disable \\\\ 1: Enable \\\\"]
pub type SlaveSclStretchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_SCL_STRETCH_CLR` writer - Configures to clear the I2C slave SCL stretch function.\\\\ 0: No effect \\\\ 1: Clear\\\\"]
pub type SlaveSclStretchClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_BYTE_ACK_CTL_EN` reader - Configures to enable the function for slave to control ACK level.\\\\ 0: Disable \\\\ 1: Enable \\\\"]
pub type SlaveByteAckCtlEnR = crate::BitReader;
#[doc = "Field `SLAVE_BYTE_ACK_CTL_EN` writer - Configures to enable the function for slave to control ACK level.\\\\ 0: Disable \\\\ 1: Enable \\\\"]
pub type SlaveByteAckCtlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_BYTE_ACK_LVL` reader - Set the ACK level when slave controlling ACK level function enables.\\\\ 0: Low level\\\\ 1: High level \\\\"]
pub type SlaveByteAckLvlR = crate::BitReader;
#[doc = "Field `SLAVE_BYTE_ACK_LVL` writer - Set the ACK level when slave controlling ACK level function enables.\\\\ 0: Low level\\\\ 1: High level \\\\"]
pub type SlaveByteAckLvlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Configures the time period to release the SCL line from stretching to avoid timing violation. Usually it should be larger than the SDA setup time.\\\\ Measurement unit: i2c_sclk \\\\"]
    #[inline(always)]
    pub fn stretch_protect_num(&self) -> StretchProtectNumR {
        StretchProtectNumR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Configures to enable slave SCL stretch function. The SCL output line will be stretched low when I2C_SLAVE_SCL_STRETCH_EN is 1 and stretch event happens. The stretch cause can be seen in I2C_STRETCH_CAUSE.\\\\ 0: Disable \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn slave_scl_stretch_en(&self) -> SlaveSclStretchEnR {
        SlaveSclStretchEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Configures to enable the function for slave to control ACK level.\\\\ 0: Disable \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn slave_byte_ack_ctl_en(&self) -> SlaveByteAckCtlEnR {
        SlaveByteAckCtlEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set the ACK level when slave controlling ACK level function enables.\\\\ 0: Low level\\\\ 1: High level \\\\"]
    #[inline(always)]
    pub fn slave_byte_ack_lvl(&self) -> SlaveByteAckLvlR {
        SlaveByteAckLvlR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Configures the time period to release the SCL line from stretching to avoid timing violation. Usually it should be larger than the SDA setup time.\\\\ Measurement unit: i2c_sclk \\\\"]
    #[inline(always)]
    pub fn stretch_protect_num(&mut self) -> StretchProtectNumW<'_, SclStretchConfSpec> {
        StretchProtectNumW::new(self, 0)
    }
    #[doc = "Bit 10 - Configures to enable slave SCL stretch function. The SCL output line will be stretched low when I2C_SLAVE_SCL_STRETCH_EN is 1 and stretch event happens. The stretch cause can be seen in I2C_STRETCH_CAUSE.\\\\ 0: Disable \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn slave_scl_stretch_en(&mut self) -> SlaveSclStretchEnW<'_, SclStretchConfSpec> {
        SlaveSclStretchEnW::new(self, 10)
    }
    #[doc = "Bit 11 - Configures to clear the I2C slave SCL stretch function.\\\\ 0: No effect \\\\ 1: Clear\\\\"]
    #[inline(always)]
    pub fn slave_scl_stretch_clr(&mut self) -> SlaveSclStretchClrW<'_, SclStretchConfSpec> {
        SlaveSclStretchClrW::new(self, 11)
    }
    #[doc = "Bit 12 - Configures to enable the function for slave to control ACK level.\\\\ 0: Disable \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn slave_byte_ack_ctl_en(&mut self) -> SlaveByteAckCtlEnW<'_, SclStretchConfSpec> {
        SlaveByteAckCtlEnW::new(self, 12)
    }
    #[doc = "Bit 13 - Set the ACK level when slave controlling ACK level function enables.\\\\ 0: Low level\\\\ 1: High level \\\\"]
    #[inline(always)]
    pub fn slave_byte_ack_lvl(&mut self) -> SlaveByteAckLvlW<'_, SclStretchConfSpec> {
        SlaveByteAckLvlW::new(self, 13)
    }
}
#[doc = "Set SCL stretch of I2C slave\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_stretch_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_stretch_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SclStretchConfSpec;
impl crate::RegisterSpec for SclStretchConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_stretch_conf::R`](R) reader structure"]
impl crate::Readable for SclStretchConfSpec {}
#[doc = "`write(|w| ..)` method takes [`scl_stretch_conf::W`](W) writer structure"]
impl crate::Writable for SclStretchConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCL_STRETCH_CONF to value 0"]
impl crate::Resettable for SclStretchConfSpec {}
