#[doc = "Register `RX_TIMING` reader"]
pub type R = crate::R<RxTimingSpec>;
#[doc = "Register `RX_TIMING` writer"]
pub type W = crate::W<RxTimingSpec>;
#[doc = "Field `RX_SD_IN_DM` reader - The delay mode of I2S Rx SD input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RxSdInDmR = crate::FieldReader;
#[doc = "Field `RX_SD_IN_DM` writer - The delay mode of I2S Rx SD input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RxSdInDmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_SD1_IN_DM` reader - The delay mode of I2S Rx SD1 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RxSd1InDmR = crate::FieldReader;
#[doc = "Field `RX_SD1_IN_DM` writer - The delay mode of I2S Rx SD1 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RxSd1InDmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_SD2_IN_DM` reader - The delay mode of I2S Rx SD2 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RxSd2InDmR = crate::FieldReader;
#[doc = "Field `RX_SD2_IN_DM` writer - The delay mode of I2S Rx SD2 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RxSd2InDmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_SD3_IN_DM` reader - The delay mode of I2S Rx SD3 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RxSd3InDmR = crate::FieldReader;
#[doc = "Field `RX_SD3_IN_DM` writer - The delay mode of I2S Rx SD3 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RxSd3InDmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_WS_OUT_DM` reader - The delay mode of I2S Rx WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RxWsOutDmR = crate::FieldReader;
#[doc = "Field `RX_WS_OUT_DM` writer - The delay mode of I2S Rx WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RxWsOutDmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_BCK_OUT_DM` reader - The delay mode of I2S Rx BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RxBckOutDmR = crate::FieldReader;
#[doc = "Field `RX_BCK_OUT_DM` writer - The delay mode of I2S Rx BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RxBckOutDmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_WS_IN_DM` reader - The delay mode of I2S Rx WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RxWsInDmR = crate::FieldReader;
#[doc = "Field `RX_WS_IN_DM` writer - The delay mode of I2S Rx WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RxWsInDmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_BCK_IN_DM` reader - The delay mode of I2S Rx BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RxBckInDmR = crate::FieldReader;
#[doc = "Field `RX_BCK_IN_DM` writer - The delay mode of I2S Rx BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type RxBckInDmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - The delay mode of I2S Rx SD input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_sd_in_dm(&self) -> RxSdInDmR {
        RxSdInDmR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - The delay mode of I2S Rx SD1 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_sd1_in_dm(&self) -> RxSd1InDmR {
        RxSd1InDmR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - The delay mode of I2S Rx SD2 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_sd2_in_dm(&self) -> RxSd2InDmR {
        RxSd2InDmR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - The delay mode of I2S Rx SD3 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_sd3_in_dm(&self) -> RxSd3InDmR {
        RxSd3InDmR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - The delay mode of I2S Rx WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_ws_out_dm(&self) -> RxWsOutDmR {
        RxWsOutDmR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - The delay mode of I2S Rx BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_bck_out_dm(&self) -> RxBckOutDmR {
        RxBckOutDmR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - The delay mode of I2S Rx WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_ws_in_dm(&self) -> RxWsInDmR {
        RxWsInDmR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - The delay mode of I2S Rx BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_bck_in_dm(&self) -> RxBckInDmR {
        RxBckInDmR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - The delay mode of I2S Rx SD input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_sd_in_dm(&mut self) -> RxSdInDmW<'_, RxTimingSpec> {
        RxSdInDmW::new(self, 0)
    }
    #[doc = "Bits 4:5 - The delay mode of I2S Rx SD1 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_sd1_in_dm(&mut self) -> RxSd1InDmW<'_, RxTimingSpec> {
        RxSd1InDmW::new(self, 4)
    }
    #[doc = "Bits 8:9 - The delay mode of I2S Rx SD2 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_sd2_in_dm(&mut self) -> RxSd2InDmW<'_, RxTimingSpec> {
        RxSd2InDmW::new(self, 8)
    }
    #[doc = "Bits 12:13 - The delay mode of I2S Rx SD3 input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_sd3_in_dm(&mut self) -> RxSd3InDmW<'_, RxTimingSpec> {
        RxSd3InDmW::new(self, 12)
    }
    #[doc = "Bits 16:17 - The delay mode of I2S Rx WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_ws_out_dm(&mut self) -> RxWsOutDmW<'_, RxTimingSpec> {
        RxWsOutDmW::new(self, 16)
    }
    #[doc = "Bits 20:21 - The delay mode of I2S Rx BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_bck_out_dm(&mut self) -> RxBckOutDmW<'_, RxTimingSpec> {
        RxBckOutDmW::new(self, 20)
    }
    #[doc = "Bits 24:25 - The delay mode of I2S Rx WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_ws_in_dm(&mut self) -> RxWsInDmW<'_, RxTimingSpec> {
        RxWsInDmW::new(self, 24)
    }
    #[doc = "Bits 28:29 - The delay mode of I2S Rx BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn rx_bck_in_dm(&mut self) -> RxBckInDmW<'_, RxTimingSpec> {
        RxBckInDmW::new(self, 28)
    }
}
#[doc = "I2S RX timing control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxTimingSpec;
impl crate::RegisterSpec for RxTimingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_timing::R`](R) reader structure"]
impl crate::Readable for RxTimingSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_timing::W`](W) writer structure"]
impl crate::Writable for RxTimingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_TIMING to value 0"]
impl crate::Resettable for RxTimingSpec {}
