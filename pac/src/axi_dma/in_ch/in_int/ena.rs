#[doc = "Register `ENA` reader"]
pub type R = crate::R<EnaSpec>;
#[doc = "Register `ENA` writer"]
pub type W = crate::W<EnaSpec>;
#[doc = "Field `IN_DONE` reader - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
pub type InDoneR = crate::BitReader;
#[doc = "Field `IN_DONE` writer - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
pub type InDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF` reader - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
pub type InSucEofR = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` writer - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
pub type InSucEofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF` reader - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
pub type InErrEofR = crate::BitReader;
#[doc = "Field `IN_ERR_EOF` writer - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
pub type InErrEofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR` reader - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub type InDscrErrR = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR` writer - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub type InDscrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY` reader - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type InDscrEmptyR = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY` writer - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type InDscrEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L1_OVF` reader - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub type InfifoL1OvfR = crate::BitReader;
#[doc = "Field `INFIFO_L1_OVF` writer - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub type InfifoL1OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L1_UDF` reader - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub type InfifoL1UdfR = crate::BitReader;
#[doc = "Field `INFIFO_L1_UDF` writer - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub type InfifoL1UdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L2_OVF` reader - The interrupt enable bit for the INFIFO_OVF_L2_CH_INT interrupt."]
pub type InfifoL2OvfR = crate::BitReader;
#[doc = "Field `INFIFO_L2_OVF` writer - The interrupt enable bit for the INFIFO_OVF_L2_CH_INT interrupt."]
pub type InfifoL2OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L2_UDF` reader - The interrupt enable bit for the INFIFO_UDF_L2_CH_INT interrupt."]
pub type InfifoL2UdfR = crate::BitReader;
#[doc = "Field `INFIFO_L2_UDF` writer - The interrupt enable bit for the INFIFO_UDF_L2_CH_INT interrupt."]
pub type InfifoL2UdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L3_OVF` reader - The interrupt enable bit for the INFIFO_OVF_L3_CH_INT interrupt."]
pub type InfifoL3OvfR = crate::BitReader;
#[doc = "Field `INFIFO_L3_OVF` writer - The interrupt enable bit for the INFIFO_OVF_L3_CH_INT interrupt."]
pub type InfifoL3OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L3_UDF` reader - The interrupt enable bit for the INFIFO_UDF_L3_CH_INT interrupt."]
pub type InfifoL3UdfR = crate::BitReader;
#[doc = "Field `INFIFO_L3_UDF` writer - The interrupt enable bit for the INFIFO_UDF_L3_CH_INT interrupt."]
pub type InfifoL3UdfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done(&self) -> InDoneR {
        InDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> InSucEofR {
        InSucEofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof(&self) -> InErrEofR {
        InErrEofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err(&self) -> InDscrErrR {
        InDscrErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> InDscrEmptyR {
        InDscrEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l1_ovf(&self) -> InfifoL1OvfR {
        InfifoL1OvfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l1_udf(&self) -> InfifoL1UdfR {
        InfifoL1UdfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the INFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l2_ovf(&self) -> InfifoL2OvfR {
        InfifoL2OvfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the INFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l2_udf(&self) -> InfifoL2UdfR {
        InfifoL2UdfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the INFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l3_ovf(&self) -> InfifoL3OvfR {
        InfifoL3OvfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the INFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l3_udf(&self) -> InfifoL3UdfR {
        InfifoL3UdfR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done(&mut self) -> InDoneW<'_, EnaSpec> {
        InDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof(&mut self) -> InSucEofW<'_, EnaSpec> {
        InSucEofW::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof(&mut self) -> InErrEofW<'_, EnaSpec> {
        InErrEofW::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err(&mut self) -> InDscrErrW<'_, EnaSpec> {
        InDscrErrW::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty(&mut self) -> InDscrEmptyW<'_, EnaSpec> {
        InDscrEmptyW::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l1_ovf(&mut self) -> InfifoL1OvfW<'_, EnaSpec> {
        InfifoL1OvfW::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l1_udf(&mut self) -> InfifoL1UdfW<'_, EnaSpec> {
        InfifoL1UdfW::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the INFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l2_ovf(&mut self) -> InfifoL2OvfW<'_, EnaSpec> {
        InfifoL2OvfW::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the INFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l2_udf(&mut self) -> InfifoL2UdfW<'_, EnaSpec> {
        InfifoL2UdfW::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the INFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l3_ovf(&mut self) -> InfifoL3OvfW<'_, EnaSpec> {
        InfifoL3OvfW::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the INFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l3_udf(&mut self) -> InfifoL3UdfW<'_, EnaSpec> {
        InfifoL3UdfW::new(self, 10)
    }
}
#[doc = "Interrupt enable bits of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnaSpec;
impl crate::RegisterSpec for EnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ena::R`](R) reader structure"]
impl crate::Readable for EnaSpec {}
#[doc = "`write(|w| ..)` method takes [`ena::W`](W) writer structure"]
impl crate::Writable for EnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENA to value 0"]
impl crate::Resettable for EnaSpec {}
