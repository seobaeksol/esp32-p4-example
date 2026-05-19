#[doc = "Register `SDA_HOLD_TIME` reader"]
pub type R = crate::R<SdaHoldTimeSpec>;
#[doc = "Register `SDA_HOLD_TIME` writer"]
pub type W = crate::W<SdaHoldTimeSpec>;
#[doc = "Field `REG_SDA_OD_TX_HOLD_TIME` reader - It is used to adjust sda drive point after scl neg under open drain speed"]
pub type RegSdaOdTxHoldTimeR = crate::FieldReader<u16>;
#[doc = "Field `REG_SDA_OD_TX_HOLD_TIME` writer - It is used to adjust sda drive point after scl neg under open drain speed"]
pub type RegSdaOdTxHoldTimeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `REG_SDA_PP_TX_HOLD_TIME` reader - It is used to adjust sda dirve point after scl neg under push pull speed"]
pub type RegSdaPpTxHoldTimeR = crate::FieldReader;
#[doc = "Field `REG_SDA_PP_TX_HOLD_TIME` writer - It is used to adjust sda dirve point after scl neg under push pull speed"]
pub type RegSdaPpTxHoldTimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:8 - It is used to adjust sda drive point after scl neg under open drain speed"]
    #[inline(always)]
    pub fn reg_sda_od_tx_hold_time(&self) -> RegSdaOdTxHoldTimeR {
        RegSdaOdTxHoldTimeR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:13 - It is used to adjust sda dirve point after scl neg under push pull speed"]
    #[inline(always)]
    pub fn reg_sda_pp_tx_hold_time(&self) -> RegSdaPpTxHoldTimeR {
        RegSdaPpTxHoldTimeR::new(((self.bits >> 9) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - It is used to adjust sda drive point after scl neg under open drain speed"]
    #[inline(always)]
    pub fn reg_sda_od_tx_hold_time(&mut self) -> RegSdaOdTxHoldTimeW<'_, SdaHoldTimeSpec> {
        RegSdaOdTxHoldTimeW::new(self, 0)
    }
    #[doc = "Bits 9:13 - It is used to adjust sda dirve point after scl neg under push pull speed"]
    #[inline(always)]
    pub fn reg_sda_pp_tx_hold_time(&mut self) -> RegSdaPpTxHoldTimeW<'_, SdaHoldTimeSpec> {
        RegSdaPpTxHoldTimeW::new(self, 9)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`sda_hold_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sda_hold_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdaHoldTimeSpec;
impl crate::RegisterSpec for SdaHoldTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sda_hold_time::R`](R) reader structure"]
impl crate::Readable for SdaHoldTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`sda_hold_time::W`](W) writer structure"]
impl crate::Writable for SdaHoldTimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDA_HOLD_TIME to value 0x01"]
impl crate::Resettable for SdaHoldTimeSpec {
    const RESET_VALUE: u32 = 0x01;
}
