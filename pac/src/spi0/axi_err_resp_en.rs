#[doc = "Register `AXI_ERR_RESP_EN` reader"]
pub type R = crate::R<AxiErrRespEnSpec>;
#[doc = "Register `AXI_ERR_RESP_EN` writer"]
pub type W = crate::W<AxiErrRespEnSpec>;
#[doc = "Field `AW_RESP_EN_MMU_VLD` reader - Set this bit to enable AXI response function for mmu valid err in axi write trans."]
pub type AwRespEnMmuVldR = crate::BitReader;
#[doc = "Field `AW_RESP_EN_MMU_VLD` writer - Set this bit to enable AXI response function for mmu valid err in axi write trans."]
pub type AwRespEnMmuVldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AW_RESP_EN_MMU_GID` reader - Set this bit to enable AXI response function for mmu gid err in axi write trans."]
pub type AwRespEnMmuGidR = crate::BitReader;
#[doc = "Field `AW_RESP_EN_MMU_GID` writer - Set this bit to enable AXI response function for mmu gid err in axi write trans."]
pub type AwRespEnMmuGidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AW_RESP_EN_AXI_SIZE` reader - Set this bit to enable AXI response function for axi size err in axi write trans."]
pub type AwRespEnAxiSizeR = crate::BitReader;
#[doc = "Field `AW_RESP_EN_AXI_SIZE` writer - Set this bit to enable AXI response function for axi size err in axi write trans."]
pub type AwRespEnAxiSizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AW_RESP_EN_AXI_FLASH` reader - Set this bit to enable AXI response function for axi flash err in axi write trans."]
pub type AwRespEnAxiFlashR = crate::BitReader;
#[doc = "Field `AW_RESP_EN_AXI_FLASH` writer - Set this bit to enable AXI response function for axi flash err in axi write trans."]
pub type AwRespEnAxiFlashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AW_RESP_EN_MMU_ECC` reader - Set this bit to enable AXI response function for mmu ecc err in axi write trans."]
pub type AwRespEnMmuEccR = crate::BitReader;
#[doc = "Field `AW_RESP_EN_MMU_ECC` writer - Set this bit to enable AXI response function for mmu ecc err in axi write trans."]
pub type AwRespEnMmuEccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AW_RESP_EN_MMU_SENS` reader - Set this bit to enable AXI response function for mmu sens in err axi write trans."]
pub type AwRespEnMmuSensR = crate::BitReader;
#[doc = "Field `AW_RESP_EN_MMU_SENS` writer - Set this bit to enable AXI response function for mmu sens in err axi write trans."]
pub type AwRespEnMmuSensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AW_RESP_EN_AXI_WSTRB` reader - Set this bit to enable AXI response function for axi wstrb err in axi write trans."]
pub type AwRespEnAxiWstrbR = crate::BitReader;
#[doc = "Field `AW_RESP_EN_AXI_WSTRB` writer - Set this bit to enable AXI response function for axi wstrb err in axi write trans."]
pub type AwRespEnAxiWstrbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AR_RESP_EN_MMU_VLD` reader - Set this bit to enable AXI response function for mmu valid err in axi read trans."]
pub type ArRespEnMmuVldR = crate::BitReader;
#[doc = "Field `AR_RESP_EN_MMU_VLD` writer - Set this bit to enable AXI response function for mmu valid err in axi read trans."]
pub type ArRespEnMmuVldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AR_RESP_EN_MMU_GID` reader - Set this bit to enable AXI response function for mmu gid err in axi read trans."]
pub type ArRespEnMmuGidR = crate::BitReader;
#[doc = "Field `AR_RESP_EN_MMU_GID` writer - Set this bit to enable AXI response function for mmu gid err in axi read trans."]
pub type ArRespEnMmuGidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AR_RESP_EN_MMU_ECC` reader - Set this bit to enable AXI response function for mmu ecc err in axi read trans."]
pub type ArRespEnMmuEccR = crate::BitReader;
#[doc = "Field `AR_RESP_EN_MMU_ECC` writer - Set this bit to enable AXI response function for mmu ecc err in axi read trans."]
pub type ArRespEnMmuEccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AR_RESP_EN_MMU_SENS` reader - Set this bit to enable AXI response function for mmu sensitive err in axi read trans."]
pub type ArRespEnMmuSensR = crate::BitReader;
#[doc = "Field `AR_RESP_EN_MMU_SENS` writer - Set this bit to enable AXI response function for mmu sensitive err in axi read trans."]
pub type ArRespEnMmuSensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AR_RESP_EN_AXI_SIZE` reader - Set this bit to enable AXI response function for axi size err in axi read trans."]
pub type ArRespEnAxiSizeR = crate::BitReader;
#[doc = "Field `AR_RESP_EN_AXI_SIZE` writer - Set this bit to enable AXI response function for axi size err in axi read trans."]
pub type ArRespEnAxiSizeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable AXI response function for mmu valid err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_mmu_vld(&self) -> AwRespEnMmuVldR {
        AwRespEnMmuVldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable AXI response function for mmu gid err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_mmu_gid(&self) -> AwRespEnMmuGidR {
        AwRespEnMmuGidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable AXI response function for axi size err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_axi_size(&self) -> AwRespEnAxiSizeR {
        AwRespEnAxiSizeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable AXI response function for axi flash err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_axi_flash(&self) -> AwRespEnAxiFlashR {
        AwRespEnAxiFlashR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable AXI response function for mmu ecc err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_mmu_ecc(&self) -> AwRespEnMmuEccR {
        AwRespEnMmuEccR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable AXI response function for mmu sens in err axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_mmu_sens(&self) -> AwRespEnMmuSensR {
        AwRespEnMmuSensR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable AXI response function for axi wstrb err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_axi_wstrb(&self) -> AwRespEnAxiWstrbR {
        AwRespEnAxiWstrbR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable AXI response function for mmu valid err in axi read trans."]
    #[inline(always)]
    pub fn ar_resp_en_mmu_vld(&self) -> ArRespEnMmuVldR {
        ArRespEnMmuVldR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to enable AXI response function for mmu gid err in axi read trans."]
    #[inline(always)]
    pub fn ar_resp_en_mmu_gid(&self) -> ArRespEnMmuGidR {
        ArRespEnMmuGidR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to enable AXI response function for mmu ecc err in axi read trans."]
    #[inline(always)]
    pub fn ar_resp_en_mmu_ecc(&self) -> ArRespEnMmuEccR {
        ArRespEnMmuEccR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to enable AXI response function for mmu sensitive err in axi read trans."]
    #[inline(always)]
    pub fn ar_resp_en_mmu_sens(&self) -> ArRespEnMmuSensR {
        ArRespEnMmuSensR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to enable AXI response function for axi size err in axi read trans."]
    #[inline(always)]
    pub fn ar_resp_en_axi_size(&self) -> ArRespEnAxiSizeR {
        ArRespEnAxiSizeR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable AXI response function for mmu valid err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_mmu_vld(&mut self) -> AwRespEnMmuVldW<'_, AxiErrRespEnSpec> {
        AwRespEnMmuVldW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to enable AXI response function for mmu gid err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_mmu_gid(&mut self) -> AwRespEnMmuGidW<'_, AxiErrRespEnSpec> {
        AwRespEnMmuGidW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to enable AXI response function for axi size err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_axi_size(&mut self) -> AwRespEnAxiSizeW<'_, AxiErrRespEnSpec> {
        AwRespEnAxiSizeW::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to enable AXI response function for axi flash err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_axi_flash(&mut self) -> AwRespEnAxiFlashW<'_, AxiErrRespEnSpec> {
        AwRespEnAxiFlashW::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable AXI response function for mmu ecc err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_mmu_ecc(&mut self) -> AwRespEnMmuEccW<'_, AxiErrRespEnSpec> {
        AwRespEnMmuEccW::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to enable AXI response function for mmu sens in err axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_mmu_sens(&mut self) -> AwRespEnMmuSensW<'_, AxiErrRespEnSpec> {
        AwRespEnMmuSensW::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to enable AXI response function for axi wstrb err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_axi_wstrb(&mut self) -> AwRespEnAxiWstrbW<'_, AxiErrRespEnSpec> {
        AwRespEnAxiWstrbW::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to enable AXI response function for mmu valid err in axi read trans."]
    #[inline(always)]
    pub fn ar_resp_en_mmu_vld(&mut self) -> ArRespEnMmuVldW<'_, AxiErrRespEnSpec> {
        ArRespEnMmuVldW::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to enable AXI response function for mmu gid err in axi read trans."]
    #[inline(always)]
    pub fn ar_resp_en_mmu_gid(&mut self) -> ArRespEnMmuGidW<'_, AxiErrRespEnSpec> {
        ArRespEnMmuGidW::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to enable AXI response function for mmu ecc err in axi read trans."]
    #[inline(always)]
    pub fn ar_resp_en_mmu_ecc(&mut self) -> ArRespEnMmuEccW<'_, AxiErrRespEnSpec> {
        ArRespEnMmuEccW::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to enable AXI response function for mmu sensitive err in axi read trans."]
    #[inline(always)]
    pub fn ar_resp_en_mmu_sens(&mut self) -> ArRespEnMmuSensW<'_, AxiErrRespEnSpec> {
        ArRespEnMmuSensW::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to enable AXI response function for axi size err in axi read trans."]
    #[inline(always)]
    pub fn ar_resp_en_axi_size(&mut self) -> ArRespEnAxiSizeW<'_, AxiErrRespEnSpec> {
        ArRespEnAxiSizeW::new(self, 11)
    }
}
#[doc = "SPI0 AXI error response enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_err_resp_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_err_resp_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiErrRespEnSpec;
impl crate::RegisterSpec for AxiErrRespEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_err_resp_en::R`](R) reader structure"]
impl crate::Readable for AxiErrRespEnSpec {}
#[doc = "`write(|w| ..)` method takes [`axi_err_resp_en::W`](W) writer structure"]
impl crate::Writable for AxiErrRespEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_ERR_RESP_EN to value 0"]
impl crate::Resettable for AxiErrRespEnSpec {}
