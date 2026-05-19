#[doc = "Register `AHB2AXI_BRESP_ERR_INT_ENA` reader"]
pub type R = crate::R<Ahb2axiBrespErrIntEnaSpec>;
#[doc = "Register `AHB2AXI_BRESP_ERR_INT_ENA` writer"]
pub type W = crate::W<Ahb2axiBrespErrIntEnaSpec>;
#[doc = "Field `CPU_ICM_H2X_BRESP_ERR_INT_ENA` reader - Write 1 to enable cpu_icm_h2x_bresp_err int"]
pub type CpuIcmH2xBrespErrIntEnaR = crate::BitReader;
#[doc = "Field `CPU_ICM_H2X_BRESP_ERR_INT_ENA` writer - Write 1 to enable cpu_icm_h2x_bresp_err int"]
pub type CpuIcmH2xBrespErrIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Write 1 to enable cpu_icm_h2x_bresp_err int"]
    #[inline(always)]
    pub fn cpu_icm_h2x_bresp_err_int_ena(&self) -> CpuIcmH2xBrespErrIntEnaR {
        CpuIcmH2xBrespErrIntEnaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Write 1 to enable cpu_icm_h2x_bresp_err int"]
    #[inline(always)]
    pub fn cpu_icm_h2x_bresp_err_int_ena(
        &mut self,
    ) -> CpuIcmH2xBrespErrIntEnaW<'_, Ahb2axiBrespErrIntEnaSpec> {
        CpuIcmH2xBrespErrIntEnaW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2axi_bresp_err_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2axi_bresp_err_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2axiBrespErrIntEnaSpec;
impl crate::RegisterSpec for Ahb2axiBrespErrIntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2axi_bresp_err_int_ena::R`](R) reader structure"]
impl crate::Readable for Ahb2axiBrespErrIntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2axi_bresp_err_int_ena::W`](W) writer structure"]
impl crate::Writable for Ahb2axiBrespErrIntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB2AXI_BRESP_ERR_INT_ENA to value 0"]
impl crate::Resettable for Ahb2axiBrespErrIntEnaSpec {}
