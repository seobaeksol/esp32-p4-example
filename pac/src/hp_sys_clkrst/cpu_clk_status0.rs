#[doc = "Register `CPU_CLK_STATUS0` reader"]
pub type R = crate::R<CpuClkStatus0Spec>;
#[doc = "Field `ASIC_OR_FPGA` reader - 0: ASIC mode, 1: FPGA mode"]
pub type AsicOrFpgaR = crate::BitReader;
#[doc = "Field `CPU_DIV_EFFECT` reader - 0: Divider bypass, 1: Divider takes effect"]
pub type CpuDivEffectR = crate::BitReader;
#[doc = "Field `CPU_SRC_IS_CPLL` reader - 0: CPU source isn't cpll_400m, 1: CPU Source is cll_400m"]
pub type CpuSrcIsCpllR = crate::BitReader;
#[doc = "Field `CPU_DIV_NUM_CUR` reader - cpu current div number"]
pub type CpuDivNumCurR = crate::FieldReader;
#[doc = "Field `CPU_DIV_NUMERATOR_CUR` reader - cpu current div numerator"]
pub type CpuDivNumeratorCurR = crate::FieldReader;
#[doc = "Field `CPU_DIV_DENOMINATOR_CUR` reader - cpu current div denominator"]
pub type CpuDivDenominatorCurR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0: ASIC mode, 1: FPGA mode"]
    #[inline(always)]
    pub fn asic_or_fpga(&self) -> AsicOrFpgaR {
        AsicOrFpgaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: Divider bypass, 1: Divider takes effect"]
    #[inline(always)]
    pub fn cpu_div_effect(&self) -> CpuDivEffectR {
        CpuDivEffectR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: CPU source isn't cpll_400m, 1: CPU Source is cll_400m"]
    #[inline(always)]
    pub fn cpu_src_is_cpll(&self) -> CpuSrcIsCpllR {
        CpuSrcIsCpllR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:10 - cpu current div number"]
    #[inline(always)]
    pub fn cpu_div_num_cur(&self) -> CpuDivNumCurR {
        CpuDivNumCurR::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bits 11:18 - cpu current div numerator"]
    #[inline(always)]
    pub fn cpu_div_numerator_cur(&self) -> CpuDivNumeratorCurR {
        CpuDivNumeratorCurR::new(((self.bits >> 11) & 0xff) as u8)
    }
    #[doc = "Bits 19:26 - cpu current div denominator"]
    #[inline(always)]
    pub fn cpu_div_denominator_cur(&self) -> CpuDivDenominatorCurR {
        CpuDivDenominatorCurR::new(((self.bits >> 19) & 0xff) as u8)
    }
}
#[doc = "CPU Clock Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_clk_status0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuClkStatus0Spec;
impl crate::RegisterSpec for CpuClkStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_clk_status0::R`](R) reader structure"]
impl crate::Readable for CpuClkStatus0Spec {}
#[doc = "`reset()` method sets CPU_CLK_STATUS0 to value 0"]
impl crate::Resettable for CpuClkStatus0Spec {}
