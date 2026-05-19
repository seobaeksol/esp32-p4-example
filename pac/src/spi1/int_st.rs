#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `PER_END` reader - The status bit for SPI_MEM_PER_END_INT interrupt."]
pub type PerEndR = crate::BitReader;
#[doc = "Field `PES_END` reader - The status bit for SPI_MEM_PES_END_INT interrupt."]
pub type PesEndR = crate::BitReader;
#[doc = "Field `WPE_END` reader - The status bit for SPI_MEM_WPE_END_INT interrupt."]
pub type WpeEndR = crate::BitReader;
#[doc = "Field `SLV_ST_END` reader - The status bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SlvStEndR = crate::BitReader;
#[doc = "Field `MST_ST_END` reader - The status bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type MstStEndR = crate::BitReader;
#[doc = "Field `BROWN_OUT` reader - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
pub type BrownOutR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The status bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    pub fn per_end(&self) -> PerEndR {
        PerEndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The status bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    pub fn pes_end(&self) -> PesEndR {
        PesEndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The status bit for SPI_MEM_WPE_END_INT interrupt."]
    #[inline(always)]
    pub fn wpe_end(&self) -> WpeEndR {
        WpeEndR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The status bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn slv_st_end(&self) -> SlvStEndR {
        SlvStEndR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The status bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mst_st_end(&self) -> MstStEndR {
        MstStEndR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 10 - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
    #[inline(always)]
    pub fn brown_out(&self) -> BrownOutR {
        BrownOutR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "SPI1 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}
