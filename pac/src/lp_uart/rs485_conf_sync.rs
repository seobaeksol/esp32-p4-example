#[doc = "Register `RS485_CONF_SYNC` reader"]
pub type R = crate::R<Rs485ConfSyncSpec>;
#[doc = "Register `RS485_CONF_SYNC` writer"]
pub type W = crate::W<Rs485ConfSyncSpec>;
#[doc = "Field `DL0_EN` reader - Set this bit to delay the stop bit by 1 bit."]
pub type Dl0EnR = crate::BitReader;
#[doc = "Field `DL0_EN` writer - Set this bit to delay the stop bit by 1 bit."]
pub type Dl0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL1_EN` reader - Set this bit to delay the stop bit by 1 bit."]
pub type Dl1EnR = crate::BitReader;
#[doc = "Field `DL1_EN` writer - Set this bit to delay the stop bit by 1 bit."]
pub type Dl1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl0_en(&self) -> Dl0EnR {
        Dl0EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl1_en(&self) -> Dl1EnR {
        Dl1EnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl0_en(&mut self) -> Dl0EnW<'_, Rs485ConfSyncSpec> {
        Dl0EnW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl1_en(&mut self) -> Dl1EnW<'_, Rs485ConfSyncSpec> {
        Dl1EnW::new(self, 2)
    }
}
#[doc = "RS485 mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rs485_conf_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs485_conf_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rs485ConfSyncSpec;
impl crate::RegisterSpec for Rs485ConfSyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rs485_conf_sync::R`](R) reader structure"]
impl crate::Readable for Rs485ConfSyncSpec {}
#[doc = "`write(|w| ..)` method takes [`rs485_conf_sync::W`](W) writer structure"]
impl crate::Writable for Rs485ConfSyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RS485_CONF_SYNC to value 0"]
impl crate::Resettable for Rs485ConfSyncSpec {}
