#[doc = "Register `PIN%s` reader"]
pub type R = crate::R<PinSpec>;
#[doc = "Register `PIN%s` writer"]
pub type W = crate::W<PinSpec>;
#[doc = "Field `SYNC2_BYPASS` reader - set GPIO input_sync2 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
pub type Sync2BypassR = crate::FieldReader;
#[doc = "Field `SYNC2_BYPASS` writer - set GPIO input_sync2 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
pub type Sync2BypassW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PAD_DRIVER` reader - set this bit to select pad driver. 1:open-drain. 0:normal."]
pub type PadDriverR = crate::BitReader;
#[doc = "Field `PAD_DRIVER` writer - set this bit to select pad driver. 1:open-drain. 0:normal."]
pub type PadDriverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC1_BYPASS` reader - set GPIO input_sync1 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
pub type Sync1BypassR = crate::FieldReader;
#[doc = "Field `SYNC1_BYPASS` writer - set GPIO input_sync1 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
pub type Sync1BypassW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INT_TYPE` reader - set this value to choose interrupt mode. 0:disable GPIO interrupt. 1:trigger at posedge. 2:trigger at negedge. 3:trigger at any edge. 4:valid at low level. 5:valid at high level"]
pub type IntTypeR = crate::FieldReader;
#[doc = "Field `INT_TYPE` writer - set this value to choose interrupt mode. 0:disable GPIO interrupt. 1:trigger at posedge. 2:trigger at negedge. 3:trigger at any edge. 4:valid at low level. 5:valid at high level"]
pub type IntTypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WAKEUP_ENABLE` reader - set this bit to enable GPIO wakeup.(can only wakeup CPU from Light-sleep Mode)"]
pub type WakeupEnableR = crate::BitReader;
#[doc = "Field `WAKEUP_ENABLE` writer - set this bit to enable GPIO wakeup.(can only wakeup CPU from Light-sleep Mode)"]
pub type WakeupEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONFIG` reader - reserved"]
pub type ConfigR = crate::FieldReader;
#[doc = "Field `CONFIG` writer - reserved"]
pub type ConfigW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INT_ENA` reader - set bit 13 to enable CPU interrupt. set bit 14 to enable CPU(not shielded) interrupt."]
pub type IntEnaR = crate::FieldReader;
#[doc = "Field `INT_ENA` writer - set bit 13 to enable CPU interrupt. set bit 14 to enable CPU(not shielded) interrupt."]
pub type IntEnaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - set GPIO input_sync2 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    #[inline(always)]
    pub fn sync2_bypass(&self) -> Sync2BypassR {
        Sync2BypassR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - set this bit to select pad driver. 1:open-drain. 0:normal."]
    #[inline(always)]
    pub fn pad_driver(&self) -> PadDriverR {
        PadDriverR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - set GPIO input_sync1 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    #[inline(always)]
    pub fn sync1_bypass(&self) -> Sync1BypassR {
        Sync1BypassR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 7:9 - set this value to choose interrupt mode. 0:disable GPIO interrupt. 1:trigger at posedge. 2:trigger at negedge. 3:trigger at any edge. 4:valid at low level. 5:valid at high level"]
    #[inline(always)]
    pub fn int_type(&self) -> IntTypeR {
        IntTypeR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - set this bit to enable GPIO wakeup.(can only wakeup CPU from Light-sleep Mode)"]
    #[inline(always)]
    pub fn wakeup_enable(&self) -> WakeupEnableR {
        WakeupEnableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - reserved"]
    #[inline(always)]
    pub fn config(&self) -> ConfigR {
        ConfigR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:17 - set bit 13 to enable CPU interrupt. set bit 14 to enable CPU(not shielded) interrupt."]
    #[inline(always)]
    pub fn int_ena(&self) -> IntEnaR {
        IntEnaR::new(((self.bits >> 13) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - set GPIO input_sync2 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    #[inline(always)]
    pub fn sync2_bypass(&mut self) -> Sync2BypassW<'_, PinSpec> {
        Sync2BypassW::new(self, 0)
    }
    #[doc = "Bit 2 - set this bit to select pad driver. 1:open-drain. 0:normal."]
    #[inline(always)]
    pub fn pad_driver(&mut self) -> PadDriverW<'_, PinSpec> {
        PadDriverW::new(self, 2)
    }
    #[doc = "Bits 3:4 - set GPIO input_sync1 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    #[inline(always)]
    pub fn sync1_bypass(&mut self) -> Sync1BypassW<'_, PinSpec> {
        Sync1BypassW::new(self, 3)
    }
    #[doc = "Bits 7:9 - set this value to choose interrupt mode. 0:disable GPIO interrupt. 1:trigger at posedge. 2:trigger at negedge. 3:trigger at any edge. 4:valid at low level. 5:valid at high level"]
    #[inline(always)]
    pub fn int_type(&mut self) -> IntTypeW<'_, PinSpec> {
        IntTypeW::new(self, 7)
    }
    #[doc = "Bit 10 - set this bit to enable GPIO wakeup.(can only wakeup CPU from Light-sleep Mode)"]
    #[inline(always)]
    pub fn wakeup_enable(&mut self) -> WakeupEnableW<'_, PinSpec> {
        WakeupEnableW::new(self, 10)
    }
    #[doc = "Bits 11:12 - reserved"]
    #[inline(always)]
    pub fn config(&mut self) -> ConfigW<'_, PinSpec> {
        ConfigW::new(self, 11)
    }
    #[doc = "Bits 13:17 - set bit 13 to enable CPU interrupt. set bit 14 to enable CPU(not shielded) interrupt."]
    #[inline(always)]
    pub fn int_ena(&mut self) -> IntEnaW<'_, PinSpec> {
        IntEnaW::new(self, 13)
    }
}
#[doc = "GPIO pin configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PinSpec;
impl crate::RegisterSpec for PinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin::R`](R) reader structure"]
impl crate::Readable for PinSpec {}
#[doc = "`write(|w| ..)` method takes [`pin::W`](W) writer structure"]
impl crate::Writable for PinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIN%s to value 0"]
impl crate::Resettable for PinSpec {}
