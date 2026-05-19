#[doc = "Register `CACHE_FCTRL` reader"]
pub type R = crate::R<CacheFctrlSpec>;
#[doc = "Register `CACHE_FCTRL` writer"]
pub type W = crate::W<CacheFctrlSpec>;
#[doc = "Field `AXI_REQ_EN` reader - For SPI0, AXI master access enable, 1: enable, 0:disable."]
pub type AxiReqEnR = crate::BitReader;
#[doc = "Field `AXI_REQ_EN` writer - For SPI0, AXI master access enable, 1: enable, 0:disable."]
pub type AxiReqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_USR_ADDR_4BYTE` reader - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
pub type CacheUsrAddr4byteR = crate::BitReader;
#[doc = "Field `CACHE_USR_ADDR_4BYTE` writer - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
pub type CacheUsrAddr4byteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_FLASH_USR_CMD` reader - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
pub type CacheFlashUsrCmdR = crate::BitReader;
#[doc = "Field `CACHE_FLASH_USR_CMD` writer - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
pub type CacheFlashUsrCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDIN_DUAL` reader - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type FdinDualR = crate::BitReader;
#[doc = "Field `FDIN_DUAL` writer - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type FdinDualW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOUT_DUAL` reader - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type FdoutDualR = crate::BitReader;
#[doc = "Field `FDOUT_DUAL` writer - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type FdoutDualW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FADDR_DUAL` reader - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type FaddrDualR = crate::BitReader;
#[doc = "Field `FADDR_DUAL` writer - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type FaddrDualW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDIN_QUAD` reader - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type FdinQuadR = crate::BitReader;
#[doc = "Field `FDIN_QUAD` writer - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type FdinQuadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOUT_QUAD` reader - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type FdoutQuadR = crate::BitReader;
#[doc = "Field `FDOUT_QUAD` writer - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type FdoutQuadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FADDR_QUAD` reader - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type FaddrQuadR = crate::BitReader;
#[doc = "Field `FADDR_QUAD` writer - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type FaddrQuadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARB_WEI_EN` reader - To enable SPI0 arbiter weight func while AXI read/write access SPI0 1: enable 0: disable."]
pub type ArbWeiEnR = crate::BitReader;
#[doc = "Field `ARB_WEI_EN` writer - To enable SPI0 arbiter weight func while AXI read/write access SPI0 1: enable 0: disable."]
pub type ArbWeiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARB_REQ0_PRI` reader - To set AXI read priority in SPI0 arbiter. The larger the value, the greater the priority."]
pub type ArbReq0PriR = crate::BitReader;
#[doc = "Field `ARB_REQ0_PRI` writer - To set AXI read priority in SPI0 arbiter. The larger the value, the greater the priority."]
pub type ArbReq0PriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARB_REQ1_PRI` reader - To set AXI write priority in SPI0 arbiter. The larger the value, the greater the priority."]
pub type ArbReq1PriR = crate::BitReader;
#[doc = "Field `ARB_REQ1_PRI` writer - To set AXI write priority in SPI0 arbiter. The larger the value, the greater the priority."]
pub type ArbReq1PriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARB_REQ0_WEI` reader - To set AXI read priority weight in SPI0 arbiter. While the priority are same, the larger the value, the greater the weight."]
pub type ArbReq0WeiR = crate::FieldReader;
#[doc = "Field `ARB_REQ0_WEI` writer - To set AXI read priority weight in SPI0 arbiter. While the priority are same, the larger the value, the greater the weight."]
pub type ArbReq0WeiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ARB_REQ1_WEI` reader - To set AXI write priority weight in SPI0 arbiter. While the priority are same, the larger the value, the greater the weight."]
pub type ArbReq1WeiR = crate::FieldReader;
#[doc = "Field `ARB_REQ1_WEI` writer - To set AXI write priority weight in SPI0 arbiter. While the priority are same, the larger the value, the greater the weight."]
pub type ArbReq1WeiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPI_SAME_AW_AR_ADDR_CHK_EN` reader - Set this bit to check AXI read/write the same address region."]
pub type SpiSameAwArAddrChkEnR = crate::BitReader;
#[doc = "Field `SPI_SAME_AW_AR_ADDR_CHK_EN` writer - Set this bit to check AXI read/write the same address region."]
pub type SpiSameAwArAddrChkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CLOSE_AXI_INF_EN` reader - Set this bit to close AXI read/write transfer to MSPI, which means that only SLV_ERR will be replied to BRESP/RRESP."]
pub type SpiCloseAxiInfEnR = crate::BitReader;
#[doc = "Field `SPI_CLOSE_AXI_INF_EN` writer - Set this bit to close AXI read/write transfer to MSPI, which means that only SLV_ERR will be replied to BRESP/RRESP."]
pub type SpiCloseAxiInfEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - For SPI0, AXI master access enable, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn axi_req_en(&self) -> AxiReqEnR {
        AxiReqEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn cache_usr_addr_4byte(&self) -> CacheUsrAddr4byteR {
        CacheUsrAddr4byteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn cache_flash_usr_cmd(&self) -> CacheFlashUsrCmdR {
        CacheFlashUsrCmdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn fdin_dual(&self) -> FdinDualR {
        FdinDualR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn fdout_dual(&self) -> FdoutDualR {
        FdoutDualR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn faddr_dual(&self) -> FaddrDualR {
        FaddrDualR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn fdin_quad(&self) -> FdinQuadR {
        FdinQuadR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn fdout_quad(&self) -> FdoutQuadR {
        FdoutQuadR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn faddr_quad(&self) -> FaddrQuadR {
        FaddrQuadR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - To enable SPI0 arbiter weight func while AXI read/write access SPI0 1: enable 0: disable."]
    #[inline(always)]
    pub fn arb_wei_en(&self) -> ArbWeiEnR {
        ArbWeiEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - To set AXI read priority in SPI0 arbiter. The larger the value, the greater the priority."]
    #[inline(always)]
    pub fn arb_req0_pri(&self) -> ArbReq0PriR {
        ArbReq0PriR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - To set AXI write priority in SPI0 arbiter. The larger the value, the greater the priority."]
    #[inline(always)]
    pub fn arb_req1_pri(&self) -> ArbReq1PriR {
        ArbReq1PriR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - To set AXI read priority weight in SPI0 arbiter. While the priority are same, the larger the value, the greater the weight."]
    #[inline(always)]
    pub fn arb_req0_wei(&self) -> ArbReq0WeiR {
        ArbReq0WeiR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - To set AXI write priority weight in SPI0 arbiter. While the priority are same, the larger the value, the greater the weight."]
    #[inline(always)]
    pub fn arb_req1_wei(&self) -> ArbReq1WeiR {
        ArbReq1WeiR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Set this bit to check AXI read/write the same address region."]
    #[inline(always)]
    pub fn spi_same_aw_ar_addr_chk_en(&self) -> SpiSameAwArAddrChkEnR {
        SpiSameAwArAddrChkEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to close AXI read/write transfer to MSPI, which means that only SLV_ERR will be replied to BRESP/RRESP."]
    #[inline(always)]
    pub fn spi_close_axi_inf_en(&self) -> SpiCloseAxiInfEnR {
        SpiCloseAxiInfEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - For SPI0, AXI master access enable, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn axi_req_en(&mut self) -> AxiReqEnW<'_, CacheFctrlSpec> {
        AxiReqEnW::new(self, 0)
    }
    #[doc = "Bit 1 - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn cache_usr_addr_4byte(&mut self) -> CacheUsrAddr4byteW<'_, CacheFctrlSpec> {
        CacheUsrAddr4byteW::new(self, 1)
    }
    #[doc = "Bit 2 - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn cache_flash_usr_cmd(&mut self) -> CacheFlashUsrCmdW<'_, CacheFctrlSpec> {
        CacheFlashUsrCmdW::new(self, 2)
    }
    #[doc = "Bit 3 - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn fdin_dual(&mut self) -> FdinDualW<'_, CacheFctrlSpec> {
        FdinDualW::new(self, 3)
    }
    #[doc = "Bit 4 - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn fdout_dual(&mut self) -> FdoutDualW<'_, CacheFctrlSpec> {
        FdoutDualW::new(self, 4)
    }
    #[doc = "Bit 5 - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn faddr_dual(&mut self) -> FaddrDualW<'_, CacheFctrlSpec> {
        FaddrDualW::new(self, 5)
    }
    #[doc = "Bit 6 - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn fdin_quad(&mut self) -> FdinQuadW<'_, CacheFctrlSpec> {
        FdinQuadW::new(self, 6)
    }
    #[doc = "Bit 7 - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn fdout_quad(&mut self) -> FdoutQuadW<'_, CacheFctrlSpec> {
        FdoutQuadW::new(self, 7)
    }
    #[doc = "Bit 8 - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn faddr_quad(&mut self) -> FaddrQuadW<'_, CacheFctrlSpec> {
        FaddrQuadW::new(self, 8)
    }
    #[doc = "Bit 9 - To enable SPI0 arbiter weight func while AXI read/write access SPI0 1: enable 0: disable."]
    #[inline(always)]
    pub fn arb_wei_en(&mut self) -> ArbWeiEnW<'_, CacheFctrlSpec> {
        ArbWeiEnW::new(self, 9)
    }
    #[doc = "Bit 10 - To set AXI read priority in SPI0 arbiter. The larger the value, the greater the priority."]
    #[inline(always)]
    pub fn arb_req0_pri(&mut self) -> ArbReq0PriW<'_, CacheFctrlSpec> {
        ArbReq0PriW::new(self, 10)
    }
    #[doc = "Bit 11 - To set AXI write priority in SPI0 arbiter. The larger the value, the greater the priority."]
    #[inline(always)]
    pub fn arb_req1_pri(&mut self) -> ArbReq1PriW<'_, CacheFctrlSpec> {
        ArbReq1PriW::new(self, 11)
    }
    #[doc = "Bits 12:15 - To set AXI read priority weight in SPI0 arbiter. While the priority are same, the larger the value, the greater the weight."]
    #[inline(always)]
    pub fn arb_req0_wei(&mut self) -> ArbReq0WeiW<'_, CacheFctrlSpec> {
        ArbReq0WeiW::new(self, 12)
    }
    #[doc = "Bits 16:19 - To set AXI write priority weight in SPI0 arbiter. While the priority are same, the larger the value, the greater the weight."]
    #[inline(always)]
    pub fn arb_req1_wei(&mut self) -> ArbReq1WeiW<'_, CacheFctrlSpec> {
        ArbReq1WeiW::new(self, 16)
    }
    #[doc = "Bit 30 - Set this bit to check AXI read/write the same address region."]
    #[inline(always)]
    pub fn spi_same_aw_ar_addr_chk_en(&mut self) -> SpiSameAwArAddrChkEnW<'_, CacheFctrlSpec> {
        SpiSameAwArAddrChkEnW::new(self, 30)
    }
    #[doc = "Bit 31 - Set this bit to close AXI read/write transfer to MSPI, which means that only SLV_ERR will be replied to BRESP/RRESP."]
    #[inline(always)]
    pub fn spi_close_axi_inf_en(&mut self) -> SpiCloseAxiInfEnW<'_, CacheFctrlSpec> {
        SpiCloseAxiInfEnW::new(self, 31)
    }
}
#[doc = "SPI0 bit mode control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_fctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_fctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CacheFctrlSpec;
impl crate::RegisterSpec for CacheFctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_fctrl::R`](R) reader structure"]
impl crate::Readable for CacheFctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cache_fctrl::W`](W) writer structure"]
impl crate::Writable for CacheFctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_FCTRL to value 0xc000_0000"]
impl crate::Resettable for CacheFctrlSpec {
    const RESET_VALUE: u32 = 0xc000_0000;
}
