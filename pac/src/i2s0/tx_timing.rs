#[doc = "Register `TX_TIMING` reader"]
pub type R = crate::R<TxTimingSpec>;
#[doc = "Register `TX_TIMING` writer"]
pub type W = crate::W<TxTimingSpec>;
#[doc = "Field `TX_SD_OUT_DM` reader - The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type TxSdOutDmR = crate::FieldReader;
#[doc = "Field `TX_SD_OUT_DM` writer - The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type TxSdOutDmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_SD1_OUT_DM` reader - The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type TxSd1OutDmR = crate::FieldReader;
#[doc = "Field `TX_SD1_OUT_DM` writer - The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type TxSd1OutDmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_WS_OUT_DM` reader - The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type TxWsOutDmR = crate::FieldReader;
#[doc = "Field `TX_WS_OUT_DM` writer - The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type TxWsOutDmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_BCK_OUT_DM` reader - The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type TxBckOutDmR = crate::FieldReader;
#[doc = "Field `TX_BCK_OUT_DM` writer - The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type TxBckOutDmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_WS_IN_DM` reader - The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type TxWsInDmR = crate::FieldReader;
#[doc = "Field `TX_WS_IN_DM` writer - The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type TxWsInDmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_BCK_IN_DM` reader - The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type TxBckInDmR = crate::FieldReader;
#[doc = "Field `TX_BCK_IN_DM` writer - The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type TxBckInDmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_sd_out_dm(&self) -> TxSdOutDmR {
        TxSdOutDmR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_sd1_out_dm(&self) -> TxSd1OutDmR {
        TxSd1OutDmR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:17 - The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_ws_out_dm(&self) -> TxWsOutDmR {
        TxWsOutDmR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_bck_out_dm(&self) -> TxBckOutDmR {
        TxBckOutDmR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_ws_in_dm(&self) -> TxWsInDmR {
        TxWsInDmR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_bck_in_dm(&self) -> TxBckInDmR {
        TxBckInDmR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_sd_out_dm(&mut self) -> TxSdOutDmW<'_, TxTimingSpec> {
        TxSdOutDmW::new(self, 0)
    }
    #[doc = "Bits 4:5 - The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_sd1_out_dm(&mut self) -> TxSd1OutDmW<'_, TxTimingSpec> {
        TxSd1OutDmW::new(self, 4)
    }
    #[doc = "Bits 16:17 - The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_ws_out_dm(&mut self) -> TxWsOutDmW<'_, TxTimingSpec> {
        TxWsOutDmW::new(self, 16)
    }
    #[doc = "Bits 20:21 - The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_bck_out_dm(&mut self) -> TxBckOutDmW<'_, TxTimingSpec> {
        TxBckOutDmW::new(self, 20)
    }
    #[doc = "Bits 24:25 - The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_ws_in_dm(&mut self) -> TxWsInDmW<'_, TxTimingSpec> {
        TxWsInDmW::new(self, 24)
    }
    #[doc = "Bits 28:29 - The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn tx_bck_in_dm(&mut self) -> TxBckInDmW<'_, TxTimingSpec> {
        TxBckInDmW::new(self, 28)
    }
}
#[doc = "I2S TX timing control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxTimingSpec;
impl crate::RegisterSpec for TxTimingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_timing::R`](R) reader structure"]
impl crate::Readable for TxTimingSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_timing::W`](W) writer structure"]
impl crate::Writable for TxTimingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_TIMING to value 0"]
impl crate::Resettable for TxTimingSpec {}
