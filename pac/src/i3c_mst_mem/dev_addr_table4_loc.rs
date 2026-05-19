#[doc = "Register `DEV_ADDR_TABLE4_LOC` reader"]
pub type R = crate::R<DevAddrTable4LocSpec>;
#[doc = "Register `DEV_ADDR_TABLE4_LOC` writer"]
pub type W = crate::W<DevAddrTable4LocSpec>;
#[doc = "Field `REG_DAT_DEV4_STATIC_ADDR` reader - NA"]
pub type RegDatDev4StaticAddrR = crate::FieldReader;
#[doc = "Field `REG_DAT_DEV4_STATIC_ADDR` writer - NA"]
pub type RegDatDev4StaticAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `REG_DAT_DEV4_DYNAMIC_ADDR` reader - Device Dynamic Address with parity, The MSB,bit\\[23\\], should be programmed with parity of dynamic address."]
pub type RegDatDev4DynamicAddrR = crate::FieldReader;
#[doc = "Field `REG_DAT_DEV4_DYNAMIC_ADDR` writer - Device Dynamic Address with parity, The MSB,bit\\[23\\], should be programmed with parity of dynamic address."]
pub type RegDatDev4DynamicAddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_DAT_DEV4_NACK_RETRY_CNT` reader - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state."]
pub type RegDatDev4NackRetryCntR = crate::FieldReader;
#[doc = "Field `REG_DAT_DEV4_NACK_RETRY_CNT` writer - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state."]
pub type RegDatDev4NackRetryCntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_DAT_DEV4_I2C` reader - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device."]
pub type RegDatDev4I2cR = crate::BitReader;
#[doc = "Field `REG_DAT_DEV4_I2C` writer - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device."]
pub type RegDatDev4I2cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - NA"]
    #[inline(always)]
    pub fn reg_dat_dev4_static_addr(&self) -> RegDatDev4StaticAddrR {
        RegDatDev4StaticAddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - Device Dynamic Address with parity, The MSB,bit\\[23\\], should be programmed with parity of dynamic address."]
    #[inline(always)]
    pub fn reg_dat_dev4_dynamic_addr(&self) -> RegDatDev4DynamicAddrR {
        RegDatDev4DynamicAddrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 29:30 - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state."]
    #[inline(always)]
    pub fn reg_dat_dev4_nack_retry_cnt(&self) -> RegDatDev4NackRetryCntR {
        RegDatDev4NackRetryCntR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device."]
    #[inline(always)]
    pub fn reg_dat_dev4_i2c(&self) -> RegDatDev4I2cR {
        RegDatDev4I2cR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - NA"]
    #[inline(always)]
    pub fn reg_dat_dev4_static_addr(&mut self) -> RegDatDev4StaticAddrW<'_, DevAddrTable4LocSpec> {
        RegDatDev4StaticAddrW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Device Dynamic Address with parity, The MSB,bit\\[23\\], should be programmed with parity of dynamic address."]
    #[inline(always)]
    pub fn reg_dat_dev4_dynamic_addr(
        &mut self,
    ) -> RegDatDev4DynamicAddrW<'_, DevAddrTable4LocSpec> {
        RegDatDev4DynamicAddrW::new(self, 16)
    }
    #[doc = "Bits 29:30 - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state."]
    #[inline(always)]
    pub fn reg_dat_dev4_nack_retry_cnt(
        &mut self,
    ) -> RegDatDev4NackRetryCntW<'_, DevAddrTable4LocSpec> {
        RegDatDev4NackRetryCntW::new(self, 29)
    }
    #[doc = "Bit 31 - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device."]
    #[inline(always)]
    pub fn reg_dat_dev4_i2c(&mut self) -> RegDatDev4I2cW<'_, DevAddrTable4LocSpec> {
        RegDatDev4I2cW::new(self, 31)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_addr_table4_loc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_addr_table4_loc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevAddrTable4LocSpec;
impl crate::RegisterSpec for DevAddrTable4LocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_addr_table4_loc::R`](R) reader structure"]
impl crate::Readable for DevAddrTable4LocSpec {}
#[doc = "`write(|w| ..)` method takes [`dev_addr_table4_loc::W`](W) writer structure"]
impl crate::Writable for DevAddrTable4LocSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_ADDR_TABLE4_LOC to value 0"]
impl crate::Resettable for DevAddrTable4LocSpec {}
