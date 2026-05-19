#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<IntClrSpec>;
#[doc = "Field `PER_END` writer - The clear bit for SPI_MEM_PER_END_INT interrupt."]
pub type PerEndW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PES_END` writer - The clear bit for SPI_MEM_PES_END_INT interrupt."]
pub type PesEndW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WPE_END` writer - The clear bit for SPI_MEM_WPE_END_INT interrupt."]
pub type WpeEndW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLV_ST_END` writer - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SlvStEndW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MST_ST_END` writer - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type MstStEndW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BROWN_OUT` writer - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
pub type BrownOutW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - The clear bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    pub fn per_end(&mut self) -> PerEndW<'_, IntClrSpec> {
        PerEndW::new(self, 0)
    }
    #[doc = "Bit 1 - The clear bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    pub fn pes_end(&mut self) -> PesEndW<'_, IntClrSpec> {
        PesEndW::new(self, 1)
    }
    #[doc = "Bit 2 - The clear bit for SPI_MEM_WPE_END_INT interrupt."]
    #[inline(always)]
    pub fn wpe_end(&mut self) -> WpeEndW<'_, IntClrSpec> {
        WpeEndW::new(self, 2)
    }
    #[doc = "Bit 3 - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn slv_st_end(&mut self) -> SlvStEndW<'_, IntClrSpec> {
        SlvStEndW::new(self, 3)
    }
    #[doc = "Bit 4 - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mst_st_end(&mut self) -> MstStEndW<'_, IntClrSpec> {
        MstStEndW::new(self, 4)
    }
    #[doc = "Bit 10 - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
    #[inline(always)]
    pub fn brown_out(&mut self) -> BrownOutW<'_, IntClrSpec> {
        BrownOutW::new(self, 10)
    }
}
#[doc = "SPI1 interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClrSpec;
impl crate::RegisterSpec for IntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for IntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x041f;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for IntClrSpec {}
