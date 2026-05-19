#[doc = "Register `DEV_ADDR_TABLE10_LOC` reader"]
pub type R = crate::R<DevAddrTable10LocSpec>;
#[doc = "Register `DEV_ADDR_TABLE10_LOC` writer"]
pub type W = crate::W<DevAddrTable10LocSpec>;
#[doc = "Field `REG_DAT_DEV10_STATIC_ADDR` reader - NA"]
pub type RegDatDev10StaticAddrR = crate::FieldReader;
#[doc = "Field `REG_DAT_DEV10_STATIC_ADDR` writer - NA"]
pub type RegDatDev10StaticAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `REG_DAT_DEV10_DYNAMIC_ADDR` reader - Device Dynamic Address with parity, The MSB,bit\\[23\\], should be programmed with parity of dynamic address."]
pub type RegDatDev10DynamicAddrR = crate::FieldReader;
#[doc = "Field `REG_DAT_DEV10_DYNAMIC_ADDR` writer - Device Dynamic Address with parity, The MSB,bit\\[23\\], should be programmed with parity of dynamic address."]
pub type RegDatDev10DynamicAddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_DAT_DEV10_NACK_RETRY_CNT` reader - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state."]
pub type RegDatDev10NackRetryCntR = crate::FieldReader;
#[doc = "Field `REG_DAT_DEV10_NACK_RETRY_CNT` writer - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state."]
pub type RegDatDev10NackRetryCntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_DAT_DEV10_I2C` reader - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device."]
pub type RegDatDev10I2cR = crate::BitReader;
#[doc = "Field `REG_DAT_DEV10_I2C` writer - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device."]
pub type RegDatDev10I2cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - NA"]
    #[inline(always)]
    pub fn reg_dat_dev10_static_addr(&self) -> RegDatDev10StaticAddrR {
        RegDatDev10StaticAddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - Device Dynamic Address with parity, The MSB,bit\\[23\\], should be programmed with parity of dynamic address."]
    #[inline(always)]
    pub fn reg_dat_dev10_dynamic_addr(&self) -> RegDatDev10DynamicAddrR {
        RegDatDev10DynamicAddrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 29:30 - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state."]
    #[inline(always)]
    pub fn reg_dat_dev10_nack_retry_cnt(&self) -> RegDatDev10NackRetryCntR {
        RegDatDev10NackRetryCntR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device."]
    #[inline(always)]
    pub fn reg_dat_dev10_i2c(&self) -> RegDatDev10I2cR {
        RegDatDev10I2cR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - NA"]
    #[inline(always)]
    pub fn reg_dat_dev10_static_addr(
        &mut self,
    ) -> RegDatDev10StaticAddrW<'_, DevAddrTable10LocSpec> {
        RegDatDev10StaticAddrW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Device Dynamic Address with parity, The MSB,bit\\[23\\], should be programmed with parity of dynamic address."]
    #[inline(always)]
    pub fn reg_dat_dev10_dynamic_addr(
        &mut self,
    ) -> RegDatDev10DynamicAddrW<'_, DevAddrTable10LocSpec> {
        RegDatDev10DynamicAddrW::new(self, 16)
    }
    #[doc = "Bits 29:30 - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state."]
    #[inline(always)]
    pub fn reg_dat_dev10_nack_retry_cnt(
        &mut self,
    ) -> RegDatDev10NackRetryCntW<'_, DevAddrTable10LocSpec> {
        RegDatDev10NackRetryCntW::new(self, 29)
    }
    #[doc = "Bit 31 - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device."]
    #[inline(always)]
    pub fn reg_dat_dev10_i2c(&mut self) -> RegDatDev10I2cW<'_, DevAddrTable10LocSpec> {
        RegDatDev10I2cW::new(self, 31)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_addr_table10_loc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_addr_table10_loc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevAddrTable10LocSpec;
impl crate::RegisterSpec for DevAddrTable10LocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_addr_table10_loc::R`](R) reader structure"]
impl crate::Readable for DevAddrTable10LocSpec {}
#[doc = "`write(|w| ..)` method takes [`dev_addr_table10_loc::W`](W) writer structure"]
impl crate::Writable for DevAddrTable10LocSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_ADDR_TABLE10_LOC to value 0"]
impl crate::Resettable for DevAddrTable10LocSpec {}
