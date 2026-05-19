#[doc = "Register `DEV_ADDR_TABLE2_LOC` reader"]
pub type R = crate::R<DevAddrTable2LocSpec>;
#[doc = "Register `DEV_ADDR_TABLE2_LOC` writer"]
pub type W = crate::W<DevAddrTable2LocSpec>;
#[doc = "Field `REG_DAT_DEV2_STATIC_ADDR` reader - NA"]
pub type RegDatDev2StaticAddrR = crate::FieldReader;
#[doc = "Field `REG_DAT_DEV2_STATIC_ADDR` writer - NA"]
pub type RegDatDev2StaticAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `REG_DAT_DEV2_DYNAMIC_ADDR` reader - Device Dynamic Address with parity, The MSB,bit\\[23\\], should be programmed with parity of dynamic address."]
pub type RegDatDev2DynamicAddrR = crate::FieldReader;
#[doc = "Field `REG_DAT_DEV2_DYNAMIC_ADDR` writer - Device Dynamic Address with parity, The MSB,bit\\[23\\], should be programmed with parity of dynamic address."]
pub type RegDatDev2DynamicAddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_DAT_DEV2_NACK_RETRY_CNT` reader - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state."]
pub type RegDatDev2NackRetryCntR = crate::FieldReader;
#[doc = "Field `REG_DAT_DEV2_NACK_RETRY_CNT` writer - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state."]
pub type RegDatDev2NackRetryCntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_DAT_DEV2_I2C` reader - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device."]
pub type RegDatDev2I2cR = crate::BitReader;
#[doc = "Field `REG_DAT_DEV2_I2C` writer - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device."]
pub type RegDatDev2I2cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - NA"]
    #[inline(always)]
    pub fn reg_dat_dev2_static_addr(&self) -> RegDatDev2StaticAddrR {
        RegDatDev2StaticAddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - Device Dynamic Address with parity, The MSB,bit\\[23\\], should be programmed with parity of dynamic address."]
    #[inline(always)]
    pub fn reg_dat_dev2_dynamic_addr(&self) -> RegDatDev2DynamicAddrR {
        RegDatDev2DynamicAddrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 29:30 - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state."]
    #[inline(always)]
    pub fn reg_dat_dev2_nack_retry_cnt(&self) -> RegDatDev2NackRetryCntR {
        RegDatDev2NackRetryCntR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device."]
    #[inline(always)]
    pub fn reg_dat_dev2_i2c(&self) -> RegDatDev2I2cR {
        RegDatDev2I2cR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - NA"]
    #[inline(always)]
    pub fn reg_dat_dev2_static_addr(&mut self) -> RegDatDev2StaticAddrW<'_, DevAddrTable2LocSpec> {
        RegDatDev2StaticAddrW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Device Dynamic Address with parity, The MSB,bit\\[23\\], should be programmed with parity of dynamic address."]
    #[inline(always)]
    pub fn reg_dat_dev2_dynamic_addr(
        &mut self,
    ) -> RegDatDev2DynamicAddrW<'_, DevAddrTable2LocSpec> {
        RegDatDev2DynamicAddrW::new(self, 16)
    }
    #[doc = "Bits 29:30 - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state."]
    #[inline(always)]
    pub fn reg_dat_dev2_nack_retry_cnt(
        &mut self,
    ) -> RegDatDev2NackRetryCntW<'_, DevAddrTable2LocSpec> {
        RegDatDev2NackRetryCntW::new(self, 29)
    }
    #[doc = "Bit 31 - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device."]
    #[inline(always)]
    pub fn reg_dat_dev2_i2c(&mut self) -> RegDatDev2I2cW<'_, DevAddrTable2LocSpec> {
        RegDatDev2I2cW::new(self, 31)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_addr_table2_loc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_addr_table2_loc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevAddrTable2LocSpec;
impl crate::RegisterSpec for DevAddrTable2LocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_addr_table2_loc::R`](R) reader structure"]
impl crate::Readable for DevAddrTable2LocSpec {}
#[doc = "`write(|w| ..)` method takes [`dev_addr_table2_loc::W`](W) writer structure"]
impl crate::Writable for DevAddrTable2LocSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_ADDR_TABLE2_LOC to value 0"]
impl crate::Resettable for DevAddrTable2LocSpec {}
