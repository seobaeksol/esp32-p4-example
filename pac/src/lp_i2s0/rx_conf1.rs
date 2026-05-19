#[doc = "Register `RX_CONF1` reader"]
pub type R = crate::R<RxConf1Spec>;
#[doc = "Register `RX_CONF1` writer"]
pub type W = crate::W<RxConf1Spec>;
#[doc = "Field `RX_TDM_WS_WIDTH` reader - The width of rx_ws_out in TDM mode is (I2S_RX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
pub type RxTdmWsWidthR = crate::FieldReader;
#[doc = "Field `RX_TDM_WS_WIDTH` writer - The width of rx_ws_out in TDM mode is (I2S_RX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
pub type RxTdmWsWidthW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RX_BCK_DIV_NUM` reader - Bit clock configuration bits in receiver mode."]
pub type RxBckDivNumR = crate::FieldReader;
#[doc = "Field `RX_BCK_DIV_NUM` writer - Bit clock configuration bits in receiver mode."]
pub type RxBckDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RX_BITS_MOD` reader - Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
pub type RxBitsModR = crate::FieldReader;
#[doc = "Field `RX_BITS_MOD` writer - Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
pub type RxBitsModW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RX_HALF_SAMPLE_BITS` reader - I2S Rx half sample bits -1."]
pub type RxHalfSampleBitsR = crate::FieldReader;
#[doc = "Field `RX_HALF_SAMPLE_BITS` writer - I2S Rx half sample bits -1."]
pub type RxHalfSampleBitsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RX_TDM_CHAN_BITS` reader - The Rx bit number for each channel minus 1in TDM mode."]
pub type RxTdmChanBitsR = crate::FieldReader;
#[doc = "Field `RX_TDM_CHAN_BITS` writer - The Rx bit number for each channel minus 1in TDM mode."]
pub type RxTdmChanBitsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RX_MSB_SHIFT` reader - Set this bit to enable receiver in Phillips standard mode"]
pub type RxMsbShiftR = crate::BitReader;
#[doc = "Field `RX_MSB_SHIFT` writer - Set this bit to enable receiver in Phillips standard mode"]
pub type RxMsbShiftW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - The width of rx_ws_out in TDM mode is (I2S_RX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
    #[inline(always)]
    pub fn rx_tdm_ws_width(&self) -> RxTdmWsWidthR {
        RxTdmWsWidthR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:12 - Bit clock configuration bits in receiver mode."]
    #[inline(always)]
    pub fn rx_bck_div_num(&self) -> RxBckDivNumR {
        RxBckDivNumR::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bits 13:17 - Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
    #[inline(always)]
    pub fn rx_bits_mod(&self) -> RxBitsModR {
        RxBitsModR::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:23 - I2S Rx half sample bits -1."]
    #[inline(always)]
    pub fn rx_half_sample_bits(&self) -> RxHalfSampleBitsR {
        RxHalfSampleBitsR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:28 - The Rx bit number for each channel minus 1in TDM mode."]
    #[inline(always)]
    pub fn rx_tdm_chan_bits(&self) -> RxTdmChanBitsR {
        RxTdmChanBitsR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - Set this bit to enable receiver in Phillips standard mode"]
    #[inline(always)]
    pub fn rx_msb_shift(&self) -> RxMsbShiftR {
        RxMsbShiftR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - The width of rx_ws_out in TDM mode is (I2S_RX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
    #[inline(always)]
    pub fn rx_tdm_ws_width(&mut self) -> RxTdmWsWidthW<'_, RxConf1Spec> {
        RxTdmWsWidthW::new(self, 0)
    }
    #[doc = "Bits 7:12 - Bit clock configuration bits in receiver mode."]
    #[inline(always)]
    pub fn rx_bck_div_num(&mut self) -> RxBckDivNumW<'_, RxConf1Spec> {
        RxBckDivNumW::new(self, 7)
    }
    #[doc = "Bits 13:17 - Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
    #[inline(always)]
    pub fn rx_bits_mod(&mut self) -> RxBitsModW<'_, RxConf1Spec> {
        RxBitsModW::new(self, 13)
    }
    #[doc = "Bits 18:23 - I2S Rx half sample bits -1."]
    #[inline(always)]
    pub fn rx_half_sample_bits(&mut self) -> RxHalfSampleBitsW<'_, RxConf1Spec> {
        RxHalfSampleBitsW::new(self, 18)
    }
    #[doc = "Bits 24:28 - The Rx bit number for each channel minus 1in TDM mode."]
    #[inline(always)]
    pub fn rx_tdm_chan_bits(&mut self) -> RxTdmChanBitsW<'_, RxConf1Spec> {
        RxTdmChanBitsW::new(self, 24)
    }
    #[doc = "Bit 29 - Set this bit to enable receiver in Phillips standard mode"]
    #[inline(always)]
    pub fn rx_msb_shift(&mut self) -> RxMsbShiftW<'_, RxConf1Spec> {
        RxMsbShiftW::new(self, 29)
    }
}
#[doc = "I2S RX configure register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxConf1Spec;
impl crate::RegisterSpec for RxConf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_conf1::R`](R) reader structure"]
impl crate::Readable for RxConf1Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_conf1::W`](W) writer structure"]
impl crate::Writable for RxConf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CONF1 to value 0x2f3d_e300"]
impl crate::Resettable for RxConf1Spec {
    const RESET_VALUE: u32 = 0x2f3d_e300;
}
