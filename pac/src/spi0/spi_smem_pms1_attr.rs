#[doc = "Register `SPI_SMEM_PMS1_ATTR` reader"]
pub type R = crate::R<SpiSmemPms1AttrSpec>;
#[doc = "Register `SPI_SMEM_PMS1_ATTR` writer"]
pub type W = crate::W<SpiSmemPms1AttrSpec>;
#[doc = "Field `SPI_SMEM_PMS1_RD_ATTR` reader - 1: SPI1 external RAM PMS section 1 read accessible. 0: Not allowed."]
pub type SpiSmemPms1RdAttrR = crate::BitReader;
#[doc = "Field `SPI_SMEM_PMS1_RD_ATTR` writer - 1: SPI1 external RAM PMS section 1 read accessible. 0: Not allowed."]
pub type SpiSmemPms1RdAttrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_PMS1_WR_ATTR` reader - 1: SPI1 external RAM PMS section 1 write accessible. 0: Not allowed."]
pub type SpiSmemPms1WrAttrR = crate::BitReader;
#[doc = "Field `SPI_SMEM_PMS1_WR_ATTR` writer - 1: SPI1 external RAM PMS section 1 write accessible. 0: Not allowed."]
pub type SpiSmemPms1WrAttrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_PMS1_ECC` reader - SPI1 external RAM PMS section 1 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 1 is configured by registers SPI_SMEM_PMS1_ADDR_REG and SPI_SMEM_PMS1_SIZE_REG."]
pub type SpiSmemPms1EccR = crate::BitReader;
#[doc = "Field `SPI_SMEM_PMS1_ECC` writer - SPI1 external RAM PMS section 1 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 1 is configured by registers SPI_SMEM_PMS1_ADDR_REG and SPI_SMEM_PMS1_SIZE_REG."]
pub type SpiSmemPms1EccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_PMS1_NONSECURE_RD_ATTR` reader - 1: SPI1 external RAM non-secure PMS section 1 read accessible. 0: Not allowed."]
pub type SpiSmemPms1NonsecureRdAttrR = crate::BitReader;
#[doc = "Field `SPI_SMEM_PMS1_NONSECURE_RD_ATTR` writer - 1: SPI1 external RAM non-secure PMS section 1 read accessible. 0: Not allowed."]
pub type SpiSmemPms1NonsecureRdAttrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_PMS1_NONSECURE_WR_ATTR` reader - 1: SPI1 external RAM non-secure PMS section 1 write accessible. 0: Not allowed."]
pub type SpiSmemPms1NonsecureWrAttrR = crate::BitReader;
#[doc = "Field `SPI_SMEM_PMS1_NONSECURE_WR_ATTR` writer - 1: SPI1 external RAM non-secure PMS section 1 write accessible. 0: Not allowed."]
pub type SpiSmemPms1NonsecureWrAttrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_PMS1_NONSECURE_ECC` reader - SPI1 external RAM non-secure PMS section 1 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 1 is configured by registers SPI_SMEM_PMS1_ADDR_REG and SPI_SMEM_PMS1_SIZE_REG."]
pub type SpiSmemPms1NonsecureEccR = crate::BitReader;
#[doc = "Field `SPI_SMEM_PMS1_NONSECURE_ECC` writer - SPI1 external RAM non-secure PMS section 1 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 1 is configured by registers SPI_SMEM_PMS1_ADDR_REG and SPI_SMEM_PMS1_SIZE_REG."]
pub type SpiSmemPms1NonsecureEccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: SPI1 external RAM PMS section 1 read accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_smem_pms1_rd_attr(&self) -> SpiSmemPms1RdAttrR {
        SpiSmemPms1RdAttrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: SPI1 external RAM PMS section 1 write accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_smem_pms1_wr_attr(&self) -> SpiSmemPms1WrAttrR {
        SpiSmemPms1WrAttrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI1 external RAM PMS section 1 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 1 is configured by registers SPI_SMEM_PMS1_ADDR_REG and SPI_SMEM_PMS1_SIZE_REG."]
    #[inline(always)]
    pub fn spi_smem_pms1_ecc(&self) -> SpiSmemPms1EccR {
        SpiSmemPms1EccR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: SPI1 external RAM non-secure PMS section 1 read accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_smem_pms1_nonsecure_rd_attr(&self) -> SpiSmemPms1NonsecureRdAttrR {
        SpiSmemPms1NonsecureRdAttrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: SPI1 external RAM non-secure PMS section 1 write accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_smem_pms1_nonsecure_wr_attr(&self) -> SpiSmemPms1NonsecureWrAttrR {
        SpiSmemPms1NonsecureWrAttrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI1 external RAM non-secure PMS section 1 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 1 is configured by registers SPI_SMEM_PMS1_ADDR_REG and SPI_SMEM_PMS1_SIZE_REG."]
    #[inline(always)]
    pub fn spi_smem_pms1_nonsecure_ecc(&self) -> SpiSmemPms1NonsecureEccR {
        SpiSmemPms1NonsecureEccR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: SPI1 external RAM PMS section 1 read accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_smem_pms1_rd_attr(&mut self) -> SpiSmemPms1RdAttrW<'_, SpiSmemPms1AttrSpec> {
        SpiSmemPms1RdAttrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: SPI1 external RAM PMS section 1 write accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_smem_pms1_wr_attr(&mut self) -> SpiSmemPms1WrAttrW<'_, SpiSmemPms1AttrSpec> {
        SpiSmemPms1WrAttrW::new(self, 1)
    }
    #[doc = "Bit 2 - SPI1 external RAM PMS section 1 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 1 is configured by registers SPI_SMEM_PMS1_ADDR_REG and SPI_SMEM_PMS1_SIZE_REG."]
    #[inline(always)]
    pub fn spi_smem_pms1_ecc(&mut self) -> SpiSmemPms1EccW<'_, SpiSmemPms1AttrSpec> {
        SpiSmemPms1EccW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: SPI1 external RAM non-secure PMS section 1 read accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_smem_pms1_nonsecure_rd_attr(
        &mut self,
    ) -> SpiSmemPms1NonsecureRdAttrW<'_, SpiSmemPms1AttrSpec> {
        SpiSmemPms1NonsecureRdAttrW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: SPI1 external RAM non-secure PMS section 1 write accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_smem_pms1_nonsecure_wr_attr(
        &mut self,
    ) -> SpiSmemPms1NonsecureWrAttrW<'_, SpiSmemPms1AttrSpec> {
        SpiSmemPms1NonsecureWrAttrW::new(self, 4)
    }
    #[doc = "Bit 5 - SPI1 external RAM non-secure PMS section 1 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 1 is configured by registers SPI_SMEM_PMS1_ADDR_REG and SPI_SMEM_PMS1_SIZE_REG."]
    #[inline(always)]
    pub fn spi_smem_pms1_nonsecure_ecc(
        &mut self,
    ) -> SpiSmemPms1NonsecureEccW<'_, SpiSmemPms1AttrSpec> {
        SpiSmemPms1NonsecureEccW::new(self, 5)
    }
}
#[doc = "SPI1 external RAM PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_pms1_attr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_pms1_attr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSmemPms1AttrSpec;
impl crate::RegisterSpec for SpiSmemPms1AttrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_pms1_attr::R`](R) reader structure"]
impl crate::Readable for SpiSmemPms1AttrSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_smem_pms1_attr::W`](W) writer structure"]
impl crate::Writable for SpiSmemPms1AttrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SMEM_PMS1_ATTR to value 0x1b"]
impl crate::Resettable for SpiSmemPms1AttrSpec {
    const RESET_VALUE: u32 = 0x1b;
}
