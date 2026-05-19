#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `MST_ST` reader - The current status of SPI1 master FSM."]
pub type MstStR = crate::FieldReader;
#[doc = "Field `SLV_ST` reader - The current status of SPI1 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state."]
pub type SlvStR = crate::FieldReader;
#[doc = "Field `FLASH_PE` reader - In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashPeR = crate::BitReader;
#[doc = "Field `FLASH_PE` writer - In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashPeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR` reader - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type UsrR = crate::BitReader;
#[doc = "Field `USR` writer - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type UsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_HPM` reader - Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashHpmR = crate::BitReader;
#[doc = "Field `FLASH_HPM` writer - Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashHpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_RES` reader - This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashResR = crate::BitReader;
#[doc = "Field `FLASH_RES` writer - This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashResW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_DP` reader - Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashDpR = crate::BitReader;
#[doc = "Field `FLASH_DP` writer - Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashDpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_CE` reader - Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashCeR = crate::BitReader;
#[doc = "Field `FLASH_CE` writer - Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashCeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_BE` reader - Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashBeR = crate::BitReader;
#[doc = "Field `FLASH_BE` writer - Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashBeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_SE` reader - Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashSeR = crate::BitReader;
#[doc = "Field `FLASH_SE` writer - Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashSeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PP` reader - Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable."]
pub type FlashPpR = crate::BitReader;
#[doc = "Field `FLASH_PP` writer - Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable."]
pub type FlashPpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_WRSR` reader - Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashWrsrR = crate::BitReader;
#[doc = "Field `FLASH_WRSR` writer - Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashWrsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_RDSR` reader - Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashRdsrR = crate::BitReader;
#[doc = "Field `FLASH_RDSR` writer - Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashRdsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_RDID` reader - Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FlashRdidR = crate::BitReader;
#[doc = "Field `FLASH_RDID` writer - Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FlashRdidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_WRDI` reader - Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FlashWrdiR = crate::BitReader;
#[doc = "Field `FLASH_WRDI` writer - Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FlashWrdiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_WREN` reader - Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FlashWrenR = crate::BitReader;
#[doc = "Field `FLASH_WREN` writer - Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FlashWrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_READ` reader - Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FlashReadR = crate::BitReader;
#[doc = "Field `FLASH_READ` writer - Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
pub type FlashReadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - The current status of SPI1 master FSM."]
    #[inline(always)]
    pub fn mst_st(&self) -> MstStR {
        MstStR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The current status of SPI1 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state."]
    #[inline(always)]
    pub fn slv_st(&self) -> SlvStR {
        SlvStR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pe(&self) -> FlashPeR {
        FlashPeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn usr(&self) -> UsrR {
        UsrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_hpm(&self) -> FlashHpmR {
        FlashHpmR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_res(&self) -> FlashResR {
        FlashResR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_dp(&self) -> FlashDpR {
        FlashDpR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_ce(&self) -> FlashCeR {
        FlashCeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_be(&self) -> FlashBeR {
        FlashBeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_se(&self) -> FlashSeR {
        FlashSeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pp(&self) -> FlashPpR {
        FlashPpR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_wrsr(&self) -> FlashWrsrR {
        FlashWrsrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_rdsr(&self) -> FlashRdsrR {
        FlashRdsrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_rdid(&self) -> FlashRdidR {
        FlashRdidR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_wrdi(&self) -> FlashWrdiR {
        FlashWrdiR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_wren(&self) -> FlashWrenR {
        FlashWrenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_read(&self) -> FlashReadR {
        FlashReadR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pe(&mut self) -> FlashPeW<'_, CmdSpec> {
        FlashPeW::new(self, 17)
    }
    #[doc = "Bit 18 - User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn usr(&mut self) -> UsrW<'_, CmdSpec> {
        UsrW::new(self, 18)
    }
    #[doc = "Bit 19 - Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_hpm(&mut self) -> FlashHpmW<'_, CmdSpec> {
        FlashHpmW::new(self, 19)
    }
    #[doc = "Bit 20 - This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_res(&mut self) -> FlashResW<'_, CmdSpec> {
        FlashResW::new(self, 20)
    }
    #[doc = "Bit 21 - Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_dp(&mut self) -> FlashDpW<'_, CmdSpec> {
        FlashDpW::new(self, 21)
    }
    #[doc = "Bit 22 - Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_ce(&mut self) -> FlashCeW<'_, CmdSpec> {
        FlashCeW::new(self, 22)
    }
    #[doc = "Bit 23 - Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_be(&mut self) -> FlashBeW<'_, CmdSpec> {
        FlashBeW::new(self, 23)
    }
    #[doc = "Bit 24 - Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_se(&mut self) -> FlashSeW<'_, CmdSpec> {
        FlashSeW::new(self, 24)
    }
    #[doc = "Bit 25 - Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pp(&mut self) -> FlashPpW<'_, CmdSpec> {
        FlashPpW::new(self, 25)
    }
    #[doc = "Bit 26 - Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_wrsr(&mut self) -> FlashWrsrW<'_, CmdSpec> {
        FlashWrsrW::new(self, 26)
    }
    #[doc = "Bit 27 - Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_rdsr(&mut self) -> FlashRdsrW<'_, CmdSpec> {
        FlashRdsrW::new(self, 27)
    }
    #[doc = "Bit 28 - Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_rdid(&mut self) -> FlashRdidW<'_, CmdSpec> {
        FlashRdidW::new(self, 28)
    }
    #[doc = "Bit 29 - Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_wrdi(&mut self) -> FlashWrdiW<'_, CmdSpec> {
        FlashWrdiW::new(self, 29)
    }
    #[doc = "Bit 30 - Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_wren(&mut self) -> FlashWrenW<'_, CmdSpec> {
        FlashWrenW::new(self, 30)
    }
    #[doc = "Bit 31 - Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_read(&mut self) -> FlashReadW<'_, CmdSpec> {
        FlashReadW::new(self, 31)
    }
}
#[doc = "SPI1 memory command register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {}
