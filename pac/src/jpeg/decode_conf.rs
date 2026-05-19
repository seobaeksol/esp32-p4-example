#[doc = "Register `DECODE_CONF` reader"]
pub type R = crate::R<DecodeConfSpec>;
#[doc = "Register `DECODE_CONF` writer"]
pub type W = crate::W<DecodeConfSpec>;
#[doc = "Field `RESTART_INTERVAL` reader - configure restart interval in DRI marker when decode"]
pub type RestartIntervalR = crate::FieldReader<u16>;
#[doc = "Field `RESTART_INTERVAL` writer - configure restart interval in DRI marker when decode"]
pub type RestartIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `COMPONENT_NUM` reader - configure number of components in frame when decode"]
pub type ComponentNumR = crate::FieldReader;
#[doc = "Field `COMPONENT_NUM` writer - configure number of components in frame when decode"]
pub type ComponentNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_DHT_EN` reader - software decode dht table enable"]
pub type SwDhtEnR = crate::BitReader;
#[doc = "Field `SOS_CHECK_BYTE_NUM` reader - Configure the byte number to check next sos marker in the multi-scan picture after one scan is decoded down. The real check number is reg_sos_check_byte_num+1"]
pub type SosCheckByteNumR = crate::FieldReader;
#[doc = "Field `SOS_CHECK_BYTE_NUM` writer - Configure the byte number to check next sos marker in the multi-scan picture after one scan is decoded down. The real check number is reg_sos_check_byte_num+1"]
pub type SosCheckByteNumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RST_CHECK_BYTE_NUM` reader - Configure the byte number to check next rst marker after one rst interval is decoded down. The real check number is reg_rst_check_byte_num+1"]
pub type RstCheckByteNumR = crate::FieldReader;
#[doc = "Field `RST_CHECK_BYTE_NUM` writer - Configure the byte number to check next rst marker after one rst interval is decoded down. The real check number is reg_rst_check_byte_num+1"]
pub type RstCheckByteNumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MULTI_SCAN_ERR_CHECK` reader - reserved for decoder"]
pub type MultiScanErrCheckR = crate::BitReader;
#[doc = "Field `MULTI_SCAN_ERR_CHECK` writer - reserved for decoder"]
pub type MultiScanErrCheckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEZIGZAG_READY_CTL` reader - reserved for decoder"]
pub type DezigzagReadyCtlR = crate::BitReader;
#[doc = "Field `DEZIGZAG_READY_CTL` writer - reserved for decoder"]
pub type DezigzagReadyCtlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - configure restart interval in DRI marker when decode"]
    #[inline(always)]
    pub fn restart_interval(&self) -> RestartIntervalR {
        RestartIntervalR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - configure number of components in frame when decode"]
    #[inline(always)]
    pub fn component_num(&self) -> ComponentNumR {
        ComponentNumR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - software decode dht table enable"]
    #[inline(always)]
    pub fn sw_dht_en(&self) -> SwDhtEnR {
        SwDhtEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Configure the byte number to check next sos marker in the multi-scan picture after one scan is decoded down. The real check number is reg_sos_check_byte_num+1"]
    #[inline(always)]
    pub fn sos_check_byte_num(&self) -> SosCheckByteNumR {
        SosCheckByteNumR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - Configure the byte number to check next rst marker after one rst interval is decoded down. The real check number is reg_rst_check_byte_num+1"]
    #[inline(always)]
    pub fn rst_check_byte_num(&self) -> RstCheckByteNumR {
        RstCheckByteNumR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - reserved for decoder"]
    #[inline(always)]
    pub fn multi_scan_err_check(&self) -> MultiScanErrCheckR {
        MultiScanErrCheckR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - reserved for decoder"]
    #[inline(always)]
    pub fn dezigzag_ready_ctl(&self) -> DezigzagReadyCtlR {
        DezigzagReadyCtlR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - configure restart interval in DRI marker when decode"]
    #[inline(always)]
    pub fn restart_interval(&mut self) -> RestartIntervalW<'_, DecodeConfSpec> {
        RestartIntervalW::new(self, 0)
    }
    #[doc = "Bits 16:23 - configure number of components in frame when decode"]
    #[inline(always)]
    pub fn component_num(&mut self) -> ComponentNumW<'_, DecodeConfSpec> {
        ComponentNumW::new(self, 16)
    }
    #[doc = "Bits 25:26 - Configure the byte number to check next sos marker in the multi-scan picture after one scan is decoded down. The real check number is reg_sos_check_byte_num+1"]
    #[inline(always)]
    pub fn sos_check_byte_num(&mut self) -> SosCheckByteNumW<'_, DecodeConfSpec> {
        SosCheckByteNumW::new(self, 25)
    }
    #[doc = "Bits 27:28 - Configure the byte number to check next rst marker after one rst interval is decoded down. The real check number is reg_rst_check_byte_num+1"]
    #[inline(always)]
    pub fn rst_check_byte_num(&mut self) -> RstCheckByteNumW<'_, DecodeConfSpec> {
        RstCheckByteNumW::new(self, 27)
    }
    #[doc = "Bit 29 - reserved for decoder"]
    #[inline(always)]
    pub fn multi_scan_err_check(&mut self) -> MultiScanErrCheckW<'_, DecodeConfSpec> {
        MultiScanErrCheckW::new(self, 29)
    }
    #[doc = "Bit 30 - reserved for decoder"]
    #[inline(always)]
    pub fn dezigzag_ready_ctl(&mut self) -> DezigzagReadyCtlW<'_, DecodeConfSpec> {
        DezigzagReadyCtlW::new(self, 30)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`decode_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decode_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DecodeConfSpec;
impl crate::RegisterSpec for DecodeConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decode_conf::R`](R) reader structure"]
impl crate::Readable for DecodeConfSpec {}
#[doc = "`write(|w| ..)` method takes [`decode_conf::W`](W) writer structure"]
impl crate::Writable for DecodeConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DECODE_CONF to value 0x5f03_0000"]
impl crate::Resettable for DecodeConfSpec {
    const RESET_VALUE: u32 = 0x5f03_0000;
}
