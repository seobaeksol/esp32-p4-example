#[doc = "Register `IDLE_CONF` reader"]
pub type R = crate::R<IdleConfSpec>;
#[doc = "Register `IDLE_CONF` writer"]
pub type W = crate::W<IdleConfSpec>;
#[doc = "Field `RX_IDLE_THRHD` reader - It will produce frame end signal when receiver takes more time to receive one byte data than this register value."]
pub type RxIdleThrhdR = crate::FieldReader<u16>;
#[doc = "Field `RX_IDLE_THRHD` writer - It will produce frame end signal when receiver takes more time to receive one byte data than this register value."]
pub type RxIdleThrhdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TX_IDLE_NUM` reader - This register is used to configure the duration time between transfers."]
pub type TxIdleNumR = crate::FieldReader<u16>;
#[doc = "Field `TX_IDLE_NUM` writer - This register is used to configure the duration time between transfers."]
pub type TxIdleNumW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - It will produce frame end signal when receiver takes more time to receive one byte data than this register value."]
    #[inline(always)]
    pub fn rx_idle_thrhd(&self) -> RxIdleThrhdR {
        RxIdleThrhdR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - This register is used to configure the duration time between transfers."]
    #[inline(always)]
    pub fn tx_idle_num(&self) -> TxIdleNumR {
        TxIdleNumR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - It will produce frame end signal when receiver takes more time to receive one byte data than this register value."]
    #[inline(always)]
    pub fn rx_idle_thrhd(&mut self) -> RxIdleThrhdW<'_, IdleConfSpec> {
        RxIdleThrhdW::new(self, 0)
    }
    #[doc = "Bits 10:19 - This register is used to configure the duration time between transfers."]
    #[inline(always)]
    pub fn tx_idle_num(&mut self) -> TxIdleNumW<'_, IdleConfSpec> {
        TxIdleNumW::new(self, 10)
    }
}
#[doc = "Frame-end idle configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`idle_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idle_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdleConfSpec;
impl crate::RegisterSpec for IdleConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idle_conf::R`](R) reader structure"]
impl crate::Readable for IdleConfSpec {}
#[doc = "`write(|w| ..)` method takes [`idle_conf::W`](W) writer structure"]
impl crate::Writable for IdleConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDLE_CONF to value 0x0004_0100"]
impl crate::Resettable for IdleConfSpec {
    const RESET_VALUE: u32 = 0x0004_0100;
}
