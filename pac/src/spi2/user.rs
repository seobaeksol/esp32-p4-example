#[doc = "Register `USER` reader"]
pub type R = crate::R<UserSpec>;
#[doc = "Register `USER` writer"]
pub type W = crate::W<UserSpec>;
#[doc = "Field `DOUTDIN` reader - Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state."]
pub type DoutdinR = crate::BitReader;
#[doc = "Field `DOUTDIN` writer - Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state."]
pub type DoutdinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QPI_MODE` reader - Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state."]
pub type QpiModeR = crate::BitReader;
#[doc = "Field `QPI_MODE` writer - Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state."]
pub type QpiModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPI_MODE` reader - Just for master mode. 1: spi controller is in OPI mode (all in 8-b-m). 0: others. Can be configured in CONF state."]
pub type OpiModeR = crate::BitReader;
#[doc = "Field `OPI_MODE` writer - Just for master mode. 1: spi controller is in OPI mode (all in 8-b-m). 0: others. Can be configured in CONF state."]
pub type OpiModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCK_I_EDGE` reader - In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i."]
pub type TsckIEdgeR = crate::BitReader;
#[doc = "Field `TSCK_I_EDGE` writer - In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i."]
pub type TsckIEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_HOLD` reader - spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state."]
pub type CsHoldR = crate::BitReader;
#[doc = "Field `CS_HOLD` writer - spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state."]
pub type CsHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_SETUP` reader - spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state."]
pub type CsSetupR = crate::BitReader;
#[doc = "Field `CS_SETUP` writer - spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state."]
pub type CsSetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSCK_I_EDGE` reader - In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i."]
pub type RsckIEdgeR = crate::BitReader;
#[doc = "Field `RSCK_I_EDGE` writer - In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i."]
pub type RsckIEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_OUT_EDGE` reader - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state."]
pub type CkOutEdgeR = crate::BitReader;
#[doc = "Field `CK_OUT_EDGE` writer - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state."]
pub type CkOutEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWRITE_DUAL` reader - In the write operations read-data phase apply 2 signals. Can be configured in CONF state."]
pub type FwriteDualR = crate::BitReader;
#[doc = "Field `FWRITE_DUAL` writer - In the write operations read-data phase apply 2 signals. Can be configured in CONF state."]
pub type FwriteDualW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWRITE_QUAD` reader - In the write operations read-data phase apply 4 signals. Can be configured in CONF state."]
pub type FwriteQuadR = crate::BitReader;
#[doc = "Field `FWRITE_QUAD` writer - In the write operations read-data phase apply 4 signals. Can be configured in CONF state."]
pub type FwriteQuadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWRITE_OCT` reader - In the write operations read-data phase apply 8 signals. Can be configured in CONF state."]
pub type FwriteOctR = crate::BitReader;
#[doc = "Field `FWRITE_OCT` writer - In the write operations read-data phase apply 8 signals. Can be configured in CONF state."]
pub type FwriteOctW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_CONF_NXT` reader - 1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state."]
pub type UsrConfNxtR = crate::BitReader;
#[doc = "Field `USR_CONF_NXT` writer - 1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state."]
pub type UsrConfNxtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIO` reader - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state."]
pub type SioR = crate::BitReader;
#[doc = "Field `SIO` writer - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state."]
pub type SioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_MISO_HIGHPART` reader - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
pub type UsrMisoHighpartR = crate::BitReader;
#[doc = "Field `USR_MISO_HIGHPART` writer - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
pub type UsrMisoHighpartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_MOSI_HIGHPART` reader - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
pub type UsrMosiHighpartR = crate::BitReader;
#[doc = "Field `USR_MOSI_HIGHPART` writer - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
pub type UsrMosiHighpartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DUMMY_IDLE` reader - spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state."]
pub type UsrDummyIdleR = crate::BitReader;
#[doc = "Field `USR_DUMMY_IDLE` writer - spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state."]
pub type UsrDummyIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_MOSI` reader - This bit enable the write-data phase of an operation. Can be configured in CONF state."]
pub type UsrMosiR = crate::BitReader;
#[doc = "Field `USR_MOSI` writer - This bit enable the write-data phase of an operation. Can be configured in CONF state."]
pub type UsrMosiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_MISO` reader - This bit enable the read-data phase of an operation. Can be configured in CONF state."]
pub type UsrMisoR = crate::BitReader;
#[doc = "Field `USR_MISO` writer - This bit enable the read-data phase of an operation. Can be configured in CONF state."]
pub type UsrMisoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DUMMY` reader - This bit enable the dummy phase of an operation. Can be configured in CONF state."]
pub type UsrDummyR = crate::BitReader;
#[doc = "Field `USR_DUMMY` writer - This bit enable the dummy phase of an operation. Can be configured in CONF state."]
pub type UsrDummyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_ADDR` reader - This bit enable the address phase of an operation. Can be configured in CONF state."]
pub type UsrAddrR = crate::BitReader;
#[doc = "Field `USR_ADDR` writer - This bit enable the address phase of an operation. Can be configured in CONF state."]
pub type UsrAddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_COMMAND` reader - This bit enable the command phase of an operation. Can be configured in CONF state."]
pub type UsrCommandR = crate::BitReader;
#[doc = "Field `USR_COMMAND` writer - This bit enable the command phase of an operation. Can be configured in CONF state."]
pub type UsrCommandW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn doutdin(&self) -> DoutdinR {
        DoutdinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state."]
    #[inline(always)]
    pub fn qpi_mode(&self) -> QpiModeR {
        QpiModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Just for master mode. 1: spi controller is in OPI mode (all in 8-b-m). 0: others. Can be configured in CONF state."]
    #[inline(always)]
    pub fn opi_mode(&self) -> OpiModeR {
        OpiModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i."]
    #[inline(always)]
    pub fn tsck_i_edge(&self) -> TsckIEdgeR {
        TsckIEdgeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold(&self) -> CsHoldR {
        CsHoldR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_setup(&self) -> CsSetupR {
        CsSetupR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i."]
    #[inline(always)]
    pub fn rsck_i_edge(&self) -> RsckIEdgeR {
        RsckIEdgeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ck_out_edge(&self) -> CkOutEdgeR {
        CkOutEdgeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - In the write operations read-data phase apply 2 signals. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fwrite_dual(&self) -> FwriteDualR {
        FwriteDualR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - In the write operations read-data phase apply 4 signals. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fwrite_quad(&self) -> FwriteQuadR {
        FwriteQuadR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In the write operations read-data phase apply 8 signals. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fwrite_oct(&self) -> FwriteOctR {
        FwriteOctR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_conf_nxt(&self) -> UsrConfNxtR {
        UsrConfNxtR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn sio(&self) -> SioR {
        SioR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_miso_highpart(&self) -> UsrMisoHighpartR {
        UsrMisoHighpartR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_mosi_highpart(&self) -> UsrMosiHighpartR {
        UsrMosiHighpartR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_dummy_idle(&self) -> UsrDummyIdleR {
        UsrDummyIdleR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_mosi(&self) -> UsrMosiR {
        UsrMosiR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_miso(&self) -> UsrMisoR {
        UsrMisoR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_dummy(&self) -> UsrDummyR {
        UsrDummyR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_addr(&self) -> UsrAddrR {
        UsrAddrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit enable the command phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_command(&self) -> UsrCommandR {
        UsrCommandR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn doutdin(&mut self) -> DoutdinW<'_, UserSpec> {
        DoutdinW::new(self, 0)
    }
    #[doc = "Bit 3 - Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state."]
    #[inline(always)]
    pub fn qpi_mode(&mut self) -> QpiModeW<'_, UserSpec> {
        QpiModeW::new(self, 3)
    }
    #[doc = "Bit 4 - Just for master mode. 1: spi controller is in OPI mode (all in 8-b-m). 0: others. Can be configured in CONF state."]
    #[inline(always)]
    pub fn opi_mode(&mut self) -> OpiModeW<'_, UserSpec> {
        OpiModeW::new(self, 4)
    }
    #[doc = "Bit 5 - In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i."]
    #[inline(always)]
    pub fn tsck_i_edge(&mut self) -> TsckIEdgeW<'_, UserSpec> {
        TsckIEdgeW::new(self, 5)
    }
    #[doc = "Bit 6 - spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold(&mut self) -> CsHoldW<'_, UserSpec> {
        CsHoldW::new(self, 6)
    }
    #[doc = "Bit 7 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_setup(&mut self) -> CsSetupW<'_, UserSpec> {
        CsSetupW::new(self, 7)
    }
    #[doc = "Bit 8 - In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i."]
    #[inline(always)]
    pub fn rsck_i_edge(&mut self) -> RsckIEdgeW<'_, UserSpec> {
        RsckIEdgeW::new(self, 8)
    }
    #[doc = "Bit 9 - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ck_out_edge(&mut self) -> CkOutEdgeW<'_, UserSpec> {
        CkOutEdgeW::new(self, 9)
    }
    #[doc = "Bit 12 - In the write operations read-data phase apply 2 signals. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fwrite_dual(&mut self) -> FwriteDualW<'_, UserSpec> {
        FwriteDualW::new(self, 12)
    }
    #[doc = "Bit 13 - In the write operations read-data phase apply 4 signals. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fwrite_quad(&mut self) -> FwriteQuadW<'_, UserSpec> {
        FwriteQuadW::new(self, 13)
    }
    #[doc = "Bit 14 - In the write operations read-data phase apply 8 signals. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fwrite_oct(&mut self) -> FwriteOctW<'_, UserSpec> {
        FwriteOctW::new(self, 14)
    }
    #[doc = "Bit 15 - 1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_conf_nxt(&mut self) -> UsrConfNxtW<'_, UserSpec> {
        UsrConfNxtW::new(self, 15)
    }
    #[doc = "Bit 17 - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn sio(&mut self) -> SioW<'_, UserSpec> {
        SioW::new(self, 17)
    }
    #[doc = "Bit 24 - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_miso_highpart(&mut self) -> UsrMisoHighpartW<'_, UserSpec> {
        UsrMisoHighpartW::new(self, 24)
    }
    #[doc = "Bit 25 - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_mosi_highpart(&mut self) -> UsrMosiHighpartW<'_, UserSpec> {
        UsrMosiHighpartW::new(self, 25)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_dummy_idle(&mut self) -> UsrDummyIdleW<'_, UserSpec> {
        UsrDummyIdleW::new(self, 26)
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_mosi(&mut self) -> UsrMosiW<'_, UserSpec> {
        UsrMosiW::new(self, 27)
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_miso(&mut self) -> UsrMisoW<'_, UserSpec> {
        UsrMisoW::new(self, 28)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_dummy(&mut self) -> UsrDummyW<'_, UserSpec> {
        UsrDummyW::new(self, 29)
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_addr(&mut self) -> UsrAddrW<'_, UserSpec> {
        UsrAddrW::new(self, 30)
    }
    #[doc = "Bit 31 - This bit enable the command phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_command(&mut self) -> UsrCommandW<'_, UserSpec> {
        UsrCommandW::new(self, 31)
    }
}
#[doc = "SPI USER control register\n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UserSpec;
impl crate::RegisterSpec for UserSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user::R`](R) reader structure"]
impl crate::Readable for UserSpec {}
#[doc = "`write(|w| ..)` method takes [`user::W`](W) writer structure"]
impl crate::Writable for UserSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USER to value 0x8000_00c0"]
impl crate::Resettable for UserSpec {
    const RESET_VALUE: u32 = 0x8000_00c0;
}
